// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::sync::mpsc::{Receiver, Sender};

use chrono::offset::Utc;
use chrono::Duration;
use conrod_core::Ui;
use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder};
use glium::Surface;
use telemetry::{self, TelemetryChannelType};

use crate::chip::{Chip, ChipState};
use crate::config::arguments::RunMode;
use crate::config::environment::*;
use crate::serial::poller::{PollEvent, SerialPollerBuilder};
use crate::APP_ARGS;

use super::events::{DisplayEventsBuilder, DisplayEventsHandleOutcome};
use super::fonts::Fonts;
use super::identifiers::Ids;
use super::renderer::{DisplayRenderer, DisplayRendererBuilder};
use super::support::GliumDisplayWinitWrapper;

pub struct DisplayDrawerBuilder<'a> {
    _phantom: &'a std::marker::PhantomData<u8>,
}

pub struct DisplayDrawer<'a> {
    renderer: DisplayRenderer,
    glium_renderer: conrod_glium::Renderer,
    display: GliumDisplayWinitWrapper,
    interface: &'a mut Ui,
    events_loop: EventsLoop,
    chip: Chip,
}

impl<'a> DisplayDrawerBuilder<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        window: WindowBuilder,
        context: ContextBuilder,
        events_loop: EventsLoop,
        interface: &'a mut Ui,
        fonts: Fonts,
        chip: Chip,
    ) -> DisplayDrawer<'a> {
        // Create display
        let display =
            GliumDisplayWinitWrapper(glium::Display::new(window, context, &events_loop).unwrap());

        // Create drawer IDs
        let ids = Ids::new(interface.widget_id_generator());

        // Create drawer
        DisplayDrawer {
            renderer: DisplayRendererBuilder::new(fonts, ids),
            glium_renderer: conrod_glium::Renderer::new(&display.0).unwrap(),
            display,
            interface,
            events_loop,
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
        let rx = self.telemetry();

        // Start drawer loop
        // Flow: cycles through telemetry events, and refreshes the view every time there is an \
        //   update on the machines state.
        let mut last_render = Utc::now();

        'main: loop {
            // Receive telemetry data (from the input serial from the motherboard)
            // Empty the events queue before doing anything else
            'poll_serial: loop {
                match serial_poller.poll(&rx) {
                    Ok(PollEvent::Ready(event)) => self.chip.new_event(event),
                    Ok(PollEvent::Pending) => break 'poll_serial,
                    Err(error) => {
                        self.chip.new_error(error);

                        break 'poll_serial;
                    }
                };
            }

            // Handle incoming events
            match events_handler.handle(&self.display, &mut self.interface, &mut self.events_loop) {
                DisplayEventsHandleOutcome::Break => break 'main,
                DisplayEventsHandleOutcome::Continue => {}
            }

            // Refresh the pressure data interface, if we have any data in the buffer
            let now = Utc::now();

            if (now - last_render) > Duration::milliseconds((1000 / DISPLAY_FRAMERATE) as _) {
                if self.chip.get_state() != &ChipState::Stopped {
                    self.chip.clean_events();

                    // Force redraw if we are not stopped
                    // For some reason, with a "shared" Ids struct, conrod won't detect we need to redraw
                    // even though we know we have a different graph each new frame
                    self.interface.needs_redraw();
                }

                last_render = now;

                // Get UI events since the last render
                let ui_events = self.renderer.run_ui_events(&mut self.interface);

                self.chip.new_settings_events(ui_events);

                self.refresh();
            } else {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    }

    fn telemetry(&mut self) -> Receiver<TelemetryChannelType> {
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
                    telemetry::gather_telemetry(
                        &port,
                        tx,
                        optional_file_buffer,
                        Some(settings_receiver),
                    );
                });
            }

            RunMode::Input(path) => {
                std::thread::spawn(move || loop {
                    let file = std::fs::File::open(path).unwrap();

                    telemetry::gather_telemetry_from_file(file, tx.clone(), true);
                });
            }
        }

        rx
    }

    fn refresh(&mut self) {
        let image_map = self.renderer.render(
            &self.chip.data_pressure,
            &self.chip.last_machine_snapshot,
            &self.chip.ongoing_alarms_sorted(),
            &self.display,
            &mut self.interface,
            self.chip.get_battery_level(),
            &self.chip.get_state(),
            &self.chip.settings.inspiratory_trigger,
        );

        if let Some(primitives) = self.interface.draw_if_changed() {
            self.glium_renderer
                .fill(&self.display.0, primitives, &image_map);

            let mut target = self.display.0.draw();

            target.clear_color(0.0, 0.0, 0.0, 1.0);

            self.glium_renderer
                .draw(&self.display.0, &mut target, &image_map)
                .unwrap();

            target.finish().unwrap();
        }
    }
}
