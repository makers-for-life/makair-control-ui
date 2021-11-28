// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::cmp::max;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};

use conrod_core::Ui;
use glium::glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::{ContextBuilder, ContextCurrentState};
use glium::{texture, Surface};
use makair_telemetry::{self, TelemetryChannelType};

use crate::chip::{Chip, ChipEventUpdate, ChipState};
use crate::config::arguments::RunMode;
use crate::config::environment::*;
use crate::data::poller::{PollEvent, SerialPoller, SerialPollerBuilder};
use crate::APP_ARGS;

use super::fonts::Fonts;
use super::identifiers::{Ids, ImageIds};
use super::renderer::{DisplayRendererBuilder, DisplayRendererRunEventsResult};
use super::support::GliumDisplayWinitWrapper;

const FRAMERATE_SLEEP_THROTTLE_SMOOTH_HEAVY: Duration =
    Duration::from_millis(1000 / DISPLAY_FRAMERATE_SMOOTH_HEAVY);
const FRAMERATE_SLEEP_THROTTLE_MODERATE_FAST: Duration =
    Duration::from_millis(1000 / DISPLAY_FRAMERATE_MODERATE_FAST);
const FRAMERATE_SLEEP_THROTTLE_JERKY_FAST: Duration =
    Duration::from_millis(1000 / DISPLAY_FRAMERATE_JERKY_FAST);
const FRAMERATE_SLEEP_THROTTLE_MINIMUM: Duration = Duration::from_millis(10);

const FORCE_REFRESH_INACTIVE_PERIOD: Duration = Duration::from_secs(5);

pub struct DisplayDrawer;

impl DisplayDrawer {
    pub fn run<S: ContextCurrentState, E: 'static>(
        window_builder: WindowBuilder,
        context: ContextBuilder<S>,
        event_loop: EventLoop<E>,
        mut interface: Ui,
        fonts: Fonts,
        mut chip: Chip,
    ) -> ! {
        // Create display
        let display = GliumDisplayWinitWrapper(
            glium::Display::new(window_builder, context, &event_loop).unwrap(),
        );

        // Create widget IDs (simplified to 'IDs')
        let mut ids = Ids::new(interface.widget_id_generator());

        ids.allocate(&mut interface);

        // Create image IDs
        let mut image_map = conrod_core::image::Map::<texture::SrgbTexture2d>::new();

        let images = ImageIds::new(&display, &mut image_map);

        let mut renderer = DisplayRendererBuilder::new(fonts, ids, images);
        let mut glium_renderer = conrod_glium::Renderer::new(&display.0).unwrap();

        debug!("window built, will spawn now");

        // Create handlers
        let mut serial_poller = SerialPollerBuilder::new();

        // Start gathering telemetry
        let rx = Self::bind_telemetry(&mut chip);

        // Start drawer loop
        // Flow: cycles through telemetry events, and refreshes the view every time there is an \
        //   update on the machines state.
        let now_time = Instant::now();

        let (mut last_chip_state, mut last_refresh, mut last_heartbeat, mut is_first_frame) =
            (ChipState::WaitingData(now_time), now_time, now_time, true);
        let mut window_updated = true;

        event_loop.run(move |event, _window_target, control_flow| {
            // Measure loop tick start time
            // Notice: this will be used to prevent the FPS throttler from skipping frames, as the \
            //   methods called here may add a bit of overhead time to the final thread sleep \
            //   time, so we need to substract this overhead time to the sleep time in order to \
            //   guarantee the real on-screen FPS rate.
            let tick_start_time = Instant::now();

            // Receive telemetry data (from the input serial from the motherboard)
            // Empty the events queue before doing anything else
            let has_poll_events = Self::poll_telemetry(&mut chip, &mut serial_poller, &rx);

            // Handle incoming UI events (ie. from the window)
            if let Some(event) =
                super::support::convert_event(&event, display.0.gl_window().window())
            {
                interface.handle_event(event);
                window_updated = true;
            }

            // Break from the loop upon `Escape` or closed window.
            if let Event::WindowEvent { event, .. } = &event {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                }
            } else {
                match &event {
                    glium::glutin::event::Event::MainEventsCleared => {
                        // Catch chip state changes
                        let mut has_chip_state_change = false;

                        if chip.state != last_chip_state {
                            last_chip_state = chip.state.to_owned();

                            has_chip_state_change = true;
                        }

                        // Run events since the last render
                        let DisplayRendererRunEventsResult {
                            has_heartbeat,
                            has_user_events,
                            user_intents,
                            user_events,
                            #[cfg(feature = "simulator")]
                            user_simulator_events,
                        } = renderer.run_events(
                            &mut interface,
                            &mut chip,
                            &last_heartbeat,
                            &tick_start_time,
                        );

                        // Dispatch heartbeat?
                        // Notice: heartbeats are critical, as they indicate the firmware that the Control UI \
                        //   is still up and running. If the firmware misses a certain number of heartbeats, \
                        //   it will think that the Control UI is frozen, and thus will power cycle the \
                        //   computer that runs it. Note that, while this may look dangerous, heartbeats \
                        //   should be sent from the drawer loop itself, as this is the main Control UI event \
                        //   loop. If the Control UI drawer loop freezes, then the Control UI would be \
                        //   unusable and unresponsive, and thus heartbeats MUST be stopped so that a power \
                        //   cycle gets triggered by the firmware.
                        if has_heartbeat {
                            chip.dispatch_heartbeat_event();

                            last_heartbeat = tick_start_time;
                        }

                        // Refresh UI? (if any event occured, either user-based or poll-based)
                        // Notice: if this is the first frame, do not wait for an event to occur, refresh \
                        //   immediately. Only check those if the chip state is stopped, as to minimize \
                        //   CPU usage when no graph needs to be drawn and animated.
                        if last_chip_state == ChipState::Running
                            || has_poll_events
                            || has_user_events
                            || has_chip_state_change
                            || is_first_frame
                            || last_refresh.elapsed() >= FORCE_REFRESH_INACTIVE_PERIOD
                            || window_updated
                        {
                            window_updated = false;

                            // Unstack all user events & intents
                            if has_user_events {
                                if !user_intents.is_empty() {
                                    chip.dispatch_settings_intents(user_intents);
                                }

                                if !user_events.is_empty() {
                                    chip.dispatch_settings_events(user_events);
                                }

                                #[cfg(feature = "simulator")]
                                if !user_simulator_events.is_empty() {
                                    chip.dispatch_simulator_settings_events(user_simulator_events);
                                }
                            }

                            // Render screen to an image
                            renderer.render(&mut interface, &chip);

                            // Mark as refreshed now
                            is_first_frame = false;
                            last_refresh = tick_start_time;

                            // Request a redraw
                            display.0.gl_window().window().request_redraw();
                        }
                    }
                    glium::glutin::event::Event::RedrawRequested(_) => {
                        // A redraw was requested
                        // Let's see if it is really necessary
                        if let Some(primitives) = interface.draw_if_changed() {
                            glium_renderer.fill(&display.0, primitives, &image_map);

                            let mut target = display.0.draw();

                            target.clear_color(0.0, 0.0, 0.0, 1.0);

                            glium_renderer
                                .draw(&display.0, &mut target, &image_map)
                                .unwrap();

                            target.finish().unwrap();
                        }
                    }
                    _ => (),
                }

                // Measure loop tick total elapsed time
                let tick_spent_time = tick_start_time.elapsed();

                // Limit framerate to 'FRAMERATE_SLEEP_THROTTLE_SMOOTH_HEAVY' or \
                //   'DISPLAY_FRAMERATE_STATIC_LIGHT' (depending on current chip state, as we do not \
                //   need as many frames in stopped states; this way we can guarantee the CPU usage \
                //   will be minimal, even if something asks for frequent UI refreshes while in \
                //   stopped state; also depends on other states like if a settings modal is open)
                // Notice #1: limit the speed at the drawer loop is called. If this is not limited, \
                //   CPU usage can grow as high as 5% residual under release mode, and 40% residual \
                //   under debug mode.
                // Notice #2: substract the time it took to perform this loop tick, as to converge to \
                //   the real on-screen FPS rate. If this tick took longer than the throttle sleep \
                //   time, then fallback on a minimum guaranteed throttle time as to release the \
                //   thread for some time.
                let mut throttle_sleep_duration = if last_chip_state == ChipState::Running {
                    // A state requests that framerate should be moderated (lighter middle-ground \
                    //   between the fast but jerky framerate and the smooth but heavy framerate)
                    if renderer.has_state_moderate_framerate() {
                        FRAMERATE_SLEEP_THROTTLE_MODERATE_FAST
                    } else {
                        FRAMERATE_SLEEP_THROTTLE_SMOOTH_HEAVY
                    }
                } else {
                    FRAMERATE_SLEEP_THROTTLE_JERKY_FAST
                };

                throttle_sleep_duration = if throttle_sleep_duration > tick_spent_time {
                    max(
                        throttle_sleep_duration - tick_spent_time,
                        FRAMERATE_SLEEP_THROTTLE_MINIMUM,
                    )
                } else {
                    FRAMERATE_SLEEP_THROTTLE_MINIMUM
                };

                *control_flow = ControlFlow::WaitUntil(Instant::now() + throttle_sleep_duration);
            }
        })
    }

    fn bind_telemetry(chip: &mut Chip) -> Receiver<TelemetryChannelType> {
        // Start gathering telemetry
        let (tx, rx): (Sender<TelemetryChannelType>, Receiver<TelemetryChannelType>) =
            std::sync::mpsc::channel();

        match &APP_ARGS.mode {
            #[cfg(feature = "serial")]
            RunMode::Port { port, output_dir } => {
                let optional_file_buffer = output_dir.as_ref().map(|dir| {
                    let file_count = std::fs::read_dir(dir)
                        .expect("should read directory")
                        .count();

                    let path = format!(
                        "{}/{}-{}.record",
                        &dir,
                        chrono::Local::now().format("%Y%m%d-%H%M%S"),
                        file_count + 1
                    );

                    let file = std::fs::File::create(&path)
                        .unwrap_or_else(|_| panic!("could not create file '{}'", &path));

                    std::io::BufWriter::new(file)
                });

                let settings_receiver = chip.init_settings_receiver();

                std::thread::spawn(move || {
                    makair_telemetry::gather_telemetry(
                        port,
                        tx,
                        optional_file_buffer,
                        Some(settings_receiver),
                    );
                });
            }

            RunMode::Input(path) => {
                std::thread::spawn(move || loop {
                    let file = std::fs::File::open(path).expect("input file not found");

                    makair_telemetry::gather_telemetry_from_file(file, tx.clone(), true);
                });
            }

            #[cfg(feature = "simulator")]
            RunMode::Simulator => {
                let simulator = crate::data::simulator::init(tx, chip.init_settings_receiver());
                chip.set_simulator(simulator);
            }
        }

        rx
    }

    fn poll_telemetry(
        chip: &mut Chip,
        poller: &mut SerialPoller,
        rx: &Receiver<TelemetryChannelType>,
    ) -> bool {
        let mut has_poll_events = false;

        'poll_serial: loop {
            match poller.poll(rx) {
                Ok(PollEvent::Ready(event)) => {
                    // Do we need to mark this event as resulting in an UI update? (due to an \
                    //   internal data point having been updated)
                    if chip.new_event(event) == ChipEventUpdate::May {
                        has_poll_events = true;
                    }
                }
                Ok(PollEvent::Corrupted(error)) => {
                    // Handle unrecoverable corruption errors
                    chip.new_telemetry_error(error);

                    break 'poll_serial;
                }
                Ok(PollEvent::Pending) => break 'poll_serial,
                Err(error) => {
                    // Handle unrecoverable core errors
                    chip.new_core_error(error);

                    break 'poll_serial;
                }
            };
        }

        has_poll_events
    }
}
