// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{image::Id as ImageId, Ui};
use glium::texture;

use crate::config::environment::*;

use super::images::DisplayImages;
use super::support::GliumDisplayWinitWrapper;

widget_ids!(pub struct Ids {
  layout_container,
  layout_header,
  layout_body,
  layout_footer,
  layout_texture_header,

  alarm_container,
  alarm_title_wrapper,
  alarm_title,
  alarm_empty,
  alarm_alarms[],
  alarm_codes_containers[],
  alarm_codes[],
  alarm_messages_containers[],
  alarm_messages[],

  background,

  graph_pressure,

  branding_container,
  branding_text,

  controls_wrapper,
  controls_button_run,
  controls_button_snooze,
  controls_button_advanced,

  status_wrapper,
  status_unit_box,
  status_unit_text,
  status_power_box,
  status_power_text,
  status_recording_outer,
  status_recording_inner,

  heartbeat_ground,
  heartbeat_surround,
  heartbeat_inner,

  telemetry_widgets_right,

  cycles_parent,
  cycles_title,
  cycles_value_measured,
  cycles_value_arrow_main,
  cycles_value_arrow_line,
  cycles_value_target,
  cycles_unit,

  peak_parent,
  peak_title,
  peak_value_measured,
  peak_value_arrow_main,
  peak_value_arrow_line,
  peak_value_target,
  peak_unit,

  plateau_parent,
  plateau_title,
  plateau_value_measured,
  plateau_value_arrow_main,
  plateau_value_arrow_line,
  plateau_value_target,
  plateau_unit,

  peep_parent,
  peep_title,
  peep_value_measured,
  peep_value_arrow_main,
  peep_value_arrow_line,
  peep_value_target,
  peep_unit,

  ratio_parent,
  ratio_title,
  ratio_value_measured,
  ratio_value_arrow_main,
  ratio_value_arrow_line,
  ratio_value_target,
  ratio_unit,

  expiration_term_container,
  expiration_term_more_button,
  expiration_term_more_button_text,
  expiration_term_less_button,
  expiration_term_less_button_text,
  expiration_term_text,
  expiration_term_value,

  pressure_container,
  pressure_plateau_more_button,
  pressure_plateau_more_button_text,
  pressure_plateau_less_button,
  pressure_plateau_less_button_text,
  pressure_plateau_text,
  pressure_plateau_value,
  pressure_peep_more_button,
  pressure_peep_more_button_text,
  pressure_peep_less_button,
  pressure_peep_less_button_text,
  pressure_peep_text,
  pressure_peep_value,

  cycles_container,
  cycles_more_button,
  cycles_more_button_text,
  cycles_less_button,
  cycles_less_button_text,
  cycles_text,
  cycles_value,

  tidal_parent,
  tidal_title,
  tidal_value_measured,
  tidal_value_arrow_main,
  tidal_value_arrow_line,
  tidal_value_target,
  tidal_unit,

  trigger_overview_container,
  trigger_overview_border,
  trigger_overview_title,
  trigger_overview_status_label,
  trigger_overview_status_value,
  trigger_overview_offset_label,
  trigger_overview_offset_value,
  trigger_overview_configure,
  trigger_overview_expiratory_term,
  trigger_overview_plateau_duration,

  trigger_status_container,
  trigger_status_text,
  trigger_status_button,
  trigger_status_button_text,
  trigger_offset_container,
  trigger_offset_more_button,
  trigger_offset_more_button_text,
  trigger_offset_less_button,
  trigger_offset_less_button_text,
  trigger_offset_text,
  trigger_offset_value,

  run_status_container,
  run_status_text,
  run_status_button,
  run_status_button_text,

  snooze_container,
  snooze_alarms_text,
  snooze_alarms_button,
  snooze_alarms_button_text,

  advanced_container,
  advanced_line_labels[],
  advanced_line_values[],

  modal_background,
  modal_container_borders,
  modal_container,
  modal_validate,
  modal_validate_text,

  stop_background,
  stop_container_borders,
  stop_container,
  stop_title,
  stop_message,

  error_container,
  error_icon,
  error_text_wrapper,
  error_text_title,
  error_text_message,

  initializing_container,
  initializing_logo,
  initializing_text,
});

image_ids!(pub struct ImageIds {
  // Important: please use only a minimum number of images at the same time per view, as it has \
  //   been found out that, under the 'smooth' framerate mode (which is around ~20 FPS), drawing \
  //   a single image (no matter its size, tiny to huge), eats a constant 10% CPU. This all means \
  //   that if you need to draw multiple images on different widgets at once, you should nest them \
  //   all in textures. For instance, if the texture contains 5 different images, that would be a \
  //   CPU saving of 40% (as 5 images would render using 50% of the CPU, while 1 would use 10%). \
  //   All those measurements were made using a release mode build.

  // Initializing screen
  bootloader_logo,

  // Error screen
  error_icon,

  // Running / stopped screens (most images are textures here)
  header_stopped,
  header_stopped_snoozed,
  header_running,
  header_running_snoozed,
  graph_pressure,
});

impl Ids {
    pub fn allocate(&mut self, interface: &mut Ui) {
        self.advanced_line_labels.resize(
            ADVANCED_SETTINGS_LINES_COUNT,
            &mut interface.widget_id_generator(),
        );

        self.advanced_line_values.resize(
            ADVANCED_SETTINGS_LINES_COUNT,
            &mut interface.widget_id_generator(),
        );
    }
}
