// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::Ui;

use crate::config::environment::*;

widget_ids!(pub struct Ids {
  layout_header,
  layout_body,
  layout_footer,

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

  pressure_graph,

  branding_container,
  branding_image,
  branding_text,

  controls_wrapper,
  controls_button_run,
  controls_button_advanced,
  controls_image_run,
  controls_image_advanced,

  status_wrapper,
  status_unit_box,
  status_unit_text,
  status_power_box,
  status_power_text,
  status_save_icon,

  heartbeat_ground,
  heartbeat_surround,
  heartbeat_inner,

  telemetry_widgets_right,
  telemetry_widgets_bottom,

  cycles_parent,
  cycles_title,
  cycles_value_measured,
  cycles_value_arrow,
  cycles_value_target,
  cycles_unit,

  peak_parent,
  peak_title,
  peak_value_measured,
  peak_value_arrow,
  peak_value_target,
  peak_unit,

  plateau_parent,
  plateau_title,
  plateau_value_measured,
  plateau_value_arrow,
  plateau_value_target,
  plateau_unit,

  peep_parent,
  peep_title,
  peep_value_measured,
  peep_value_arrow,
  peep_value_target,
  peep_unit,

  ratio_parent,
  ratio_title,
  ratio_value_measured,
  ratio_value_arrow,
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
  tidal_value_arrow,
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

  error,

  initializing_container,
  initializing_logo,
  initializing_text,
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
