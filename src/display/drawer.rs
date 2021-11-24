// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::cmp::max;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};

use conrod_core::Ui;
use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder};
use glium::{texture, Surface};
use makair_telemetry::{self, TelemetryChannelType};

use crate::chip::{Chip, ChipEventUpdate, ChipState};
use crate::config::arguments::RunMode;
use crate::config::environment::*;
use crate::serial::poller::{PollEvent, SerialPoller, SerialPollerBuilder};
use crate::APP_ARGS;

use super::events::{DisplayEventsBuilder, DisplayEventsHandleOutcome};
use super::fonts::Fonts;
use super::identifiers::{Ids, ImageIds};
use super::renderer::{DisplayRenderer, DisplayRendererBuilder};
use super::support::GliumDisplayWinitWrapper;

const FRAMERATE_SLEEP_THROTTLE_SMOOTH_HEAVY: Duration =
    Duration::from_millis(1000 / DISPLAY_FRAMERATE_SMOOTH_HEAVY);
const FRAMERATE_SLEEP_THROTTLE_MODERATE_FAST: Duration =
    Duration::from_millis(1000 / DISPLAY_FRAMERATE_MODERATE_FAST);
const FRAMERATE_SLEEP_THROTTLE_JERKY_FAST: Duration =
    Duration::from_millis(1000 / DISPLAY_FRAMERATE_JERKY_FAST);
const FRAMERATE_SLEEP_THROTTLE_MINIMUM: Duration = Duration::from_millis(10);

const FORCE_REFRESH_INACTIVE_PERIOD: Duration = Duration::from_secs(5);

pub struct DisplayDrawerBuilder<'a> {
    _phantom: &'a std::marker::PhantomData<u8>,
}

pub struct DisplayDrawer<'a> {
    renderer: DisplayRenderer,
    glium_renderer: conrod_glium::Renderer,
    display: GliumDisplayWinitWrapper,
    interface: &'a mut Ui,
    events_loop: EventsLoop,
    image_map: conrod_core::image::Map<texture::SrgbTexture2d>,
    chip: Chip,
}

impl<'a> DisplayDrawerBuilder<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        window: WindowBuilder,
        context: ContextBuilder,
        events_loop: EventsLoop,
        mut interface: &'a mut Ui,
        fonts: Fonts,
        chip: Chip,
    ) -> DisplayDrawer<'a> {
        // Create display
        let display =
            GliumDisplayWinitWrapper(glium::Display::new(window, context, &events_loop).unwrap());

        // Create widget IDs (simplified to 'IDs')
        let mut ids = Ids::new(interface.widget_id_generator());

        ids.allocate(&mut interface);

        // Create image IDs
        let mut image_map = conrod_core::image::Map::<texture::SrgbTexture2d>::new();

        let images = ImageIds::new(&display, &mut image_map);

        // Create drawer
        DisplayDrawer {
            renderer: DisplayRendererBuilder::new(fonts, ids, images),
            glium_renderer: conrod_glium::Renderer::new(&display.0).unwrap(),
            display,
            interface,
            events_loop,
            image_map,
            chip,
        }
    }
}

impl<'a> DisplayDrawer<'a> {
    pub fn run(&mut self) {
        // Create handlers
        let (mut serial_poller, mut events_handler) =
            (SerialPollerBuilder::new(), DisplayEventsBuilder::new());

        // Start gathering telemetry
        let rx = self.bind_telemetry();

        // Start drawer loop
        // Flow: cycles through telemetry events, and refreshes the view every time there is an \
        //   update on the machines state.
        let now_time = Instant::now();

        let (mut last_chip_state, mut last_refresh, mut last_heartbeat, mut is_first_frame) =
            (ChipState::WaitingData(now_time), now_time, now_time, true);

        'main: loop {
            // Measure loop tick start time
            // Notice: this will be used to prevent the FPS throttler from skipping frames, as the \
            //   methods called here may add a bit of overhead time to the final thread sleep \
            //   time, so we need to substract this overhead time to the sleep time in order to \
            //   guarantee the real on-screen FPS rate.
            let tick_start_time = Instant::now();

            // Receive telemetry data (from the input serial from the motherboard)
            // Empty the events queue before doing anything else
            let has_poll_events = self.poll_telemetry(&mut serial_poller, &rx);

            // Handle incoming UI events (ie. from the window, eg. 'ESC' key is pressed)
            match events_handler.handle(&self.display, &mut self.interface, &mut self.events_loop) {
                DisplayEventsHandleOutcome::Break => break 'main,
                DisplayEventsHandleOutcome::Continue => {}
            }

            // Catch chip state changes
            let mut has_chip_state_change = false;

            if self.chip.state != last_chip_state {
                last_chip_state = self.chip.state.to_owned();

                has_chip_state_change = true;
            }

            // Run events since the last render
            let (has_heartbeat, has_user_events, user_intents, user_events) =
                self.renderer.run_events(
                    &mut self.interface,
                    &mut self.chip,
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
                self.chip.dispatch_heartbeat_event();

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
            {
                // Unstack all user events & intents
                if has_user_events {
                    if !user_intents.is_empty() {
                        self.chip.dispatch_settings_intents(user_intents);
                    }

                    if !user_events.is_empty() {
                        self.chip.dispatch_settings_events(user_events);
                    }
                }

                // Proceed UI refresh?
                self.refresh();

                // Mark as refreshed now
                is_first_frame = false;
                last_refresh = tick_start_time;
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
                if self.renderer.has_state_moderate_framerate() {
                    FRAMERATE_SLEEP_THROTTLE_MODERATE_FAST
                } else {
                    FRAMERATE_SLEEP_THROTTLE_SMOOTH_HEAVY
                }
            } else {
                FRAMERATE_SLEEP_THROTTLE_JERKY_FAST
            };

            if throttle_sleep_duration > tick_spent_time {
                throttle_sleep_duration = max(
                    throttle_sleep_duration - tick_spent_time,
                    FRAMERATE_SLEEP_THROTTLE_MINIMUM,
                );
            } else {
                throttle_sleep_duration = FRAMERATE_SLEEP_THROTTLE_MINIMUM;
            }

            std::thread::sleep(throttle_sleep_duration);
        }
    }

    fn bind_telemetry(&mut self) -> Receiver<TelemetryChannelType> {
        // Start gathering telemetry
        let (tx, rx): (Sender<TelemetryChannelType>, Receiver<TelemetryChannelType>) =
            std::sync::mpsc::channel();

        match &APP_ARGS.mode {
            RunMode::Port { port, output_dir } => {
                let optional_file_buffer = output_dir.as_ref().map(|dir| {
                    let file_count: Vec<std::io::Result<std::fs::DirEntry>> =
                        std::fs::read_dir(dir)
                            .expect("should read directory")
                            .collect();

                    let path = format!(
                        "{}/{}-{}.record",
                        &dir,
                        chrono::Local::now().format("%Y%m%d-%H%M%S"),
                        file_count.len() + 1
                    );

                    let file = std::fs::File::create(&path)
                        .unwrap_or_else(|_| panic!("could not create file '{}'", &path));

                    std::io::BufWriter::new(file)
                });

                let settings_receiver = self.chip.init_settings_receiver();

                std::thread::spawn(move || {
                    makair_telemetry::gather_telemetry(
                        &port,
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
                let mut simulator = makair_simulator::MakAirSimulator::new(tx);
                simulator.initialize();

                let settings_receiver = self.chip.init_settings_receiver();
                std::thread::spawn(move || {
                    while let Ok(message) = settings_receiver.recv() {
                        simulator.send_control_message(message);
                    }
                });
            }
        }

        rx
    }

    fn poll_telemetry(
        &mut self,
        poller: &mut SerialPoller,
        rx: &Receiver<TelemetryChannelType>,
    ) -> bool {
        let mut has_poll_events = false;

        'poll_serial: loop {
            match poller.poll(rx) {
                Ok(PollEvent::Ready(event)) => {
                    // Do we need to mark this event as resulting in an UI update? (due to an \
                    //   internal data point having been updated)
                    if self.chip.new_event(event) == ChipEventUpdate::May {
                        has_poll_events = true;
                    }
                }
                Ok(PollEvent::Corrupted(error)) => {
                    // Handle unrecoverable corruption errors
                    self.chip.new_telemetry_error(error);

                    break 'poll_serial;
                }
                Ok(PollEvent::Pending) => break 'poll_serial,
                Err(error) => {
                    // Handle unrecoverable core errors
                    self.chip.new_core_error(error);

                    break 'poll_serial;
                }
            };
        }

        has_poll_events
    }

    fn refresh(&mut self) {
        // Render screen to an image
        self.renderer.render(&mut self.interface, &self.chip);

        // Draw interface if it changed
        if let Some(primitives) = self.interface.draw_if_changed() {
            self.glium_renderer
                .fill(&self.display.0, primitives, &self.image_map);

            let mut target = self.display.0.draw();

            target.clear_color(0.0, 0.0, 0.0, 1.0);

            self.glium_renderer
                .draw(&self.display.0, &mut target, &self.image_map)
                .unwrap();

            target.finish().unwrap();
        }
    }
}
