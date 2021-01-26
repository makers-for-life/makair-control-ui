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

  graph_wrapper,
  graph_pressure,
  graph_flow,

  graph_pressure_label_box,
  graph_pressure_label_text,

  graph_flow_label_box,
  graph_flow_label_text,

  graph_pressure_saturate_low,
  graph_pressure_saturate_high,
  graph_flow_saturate_low,
  graph_flow_saturate_high,

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

  cycles_container,
  cycles_more_button,
  cycles_more_button_text,
  cycles_less_button,
  cycles_less_button_text,
  cycles_text,
  cycles_value,
  cycles_value_wrapper,

  tidal_parent,
  tidal_title,
  tidal_value_measured,
  tidal_value_arrow_main,
  tidal_value_arrow_line,
  tidal_value_target,
  tidal_unit,

  minute_volume_parent,
  minute_volume_title,
  minute_volume_value_measured,
  minute_volume_value_arrow_main,
  minute_volume_value_arrow_line,
  minute_volume_value_target,
  minute_volume_unit,

  preset_settings_container,
  preset_settings_title_primary,
  preset_settings_title_secondary,
  preset_settings_content_wrapper,
  preset_settings_content_image,
  preset_settings_content_separator,
  preset_settings_content_form_wrapper,

  preset_settings_field_gender_text,
  preset_settings_field_gender_value,
  preset_settings_field_gender_value_wrapper,
  preset_settings_field_gender_more,
  preset_settings_field_gender_more_text,
  preset_settings_field_gender_less,
  preset_settings_field_gender_less_text,

  preset_settings_field_age_text,
  preset_settings_field_age_value,
  preset_settings_field_age_value_wrapper,
  preset_settings_field_age_more,
  preset_settings_field_age_more_text,
  preset_settings_field_age_less,
  preset_settings_field_age_less_text,

  preset_settings_field_height_text,
  preset_settings_field_height_value,
  preset_settings_field_height_value_wrapper,
  preset_settings_field_height_more,
  preset_settings_field_height_more_text,
  preset_settings_field_height_less,
  preset_settings_field_height_less_text,

  mode_overview_container,
  mode_overview_separator,
  mode_overview_text_class,
  mode_overview_text_type,

  mode_settings_container,
  mode_settings_selector_wrapper,
  mode_settings_selector_tab_pc_cmv,
  mode_settings_selector_tab_pc_ac,
  mode_settings_selector_tab_pc_vsai,
  mode_settings_selector_tab_vc_cmv,
  mode_settings_selector_tab_vc_ac,
  mode_settings_selector_texts_pc_cmv,
  mode_settings_selector_texts_pc_ac,
  mode_settings_selector_texts_pc_vsai,
  mode_settings_selector_texts_vc_cmv,
  mode_settings_selector_texts_vc_ac,
  mode_settings_group_wrapper,
  mode_settings_content_wrapper,
  mode_settings_form_wrapper,

  mode_settings_group_tab_general_button,
  mode_settings_group_tab_general_text,
  mode_settings_group_tab_alarms_button,
  mode_settings_group_tab_alarms_text,

  mode_settings_field_pressure_inspiratory_text,
  mode_settings_field_pressure_inspiratory_value,
  mode_settings_field_pressure_inspiratory_value_wrapper,
  mode_settings_field_pressure_inspiratory_more,
  mode_settings_field_pressure_inspiratory_more_text,
  mode_settings_field_pressure_inspiratory_less,
  mode_settings_field_pressure_inspiratory_less_text,

  mode_settings_field_pressure_expiratory_text,
  mode_settings_field_pressure_expiratory_value,
  mode_settings_field_pressure_expiratory_value_wrapper,
  mode_settings_field_pressure_expiratory_more,
  mode_settings_field_pressure_expiratory_more_text,
  mode_settings_field_pressure_expiratory_less,
  mode_settings_field_pressure_expiratory_less_text,

  mode_settings_field_time_inspiratory_minimum_text,
  mode_settings_field_time_inspiratory_minimum_value,
  mode_settings_field_time_inspiratory_minimum_value_wrapper,
  mode_settings_field_time_inspiratory_minimum_more,
  mode_settings_field_time_inspiratory_minimum_more_text,
  mode_settings_field_time_inspiratory_minimum_less,
  mode_settings_field_time_inspiratory_minimum_less_text,

  mode_settings_field_time_inspiratory_maximum_text,
  mode_settings_field_time_inspiratory_maximum_value,
  mode_settings_field_time_inspiratory_maximum_value_wrapper,
  mode_settings_field_time_inspiratory_maximum_more,
  mode_settings_field_time_inspiratory_maximum_more_text,
  mode_settings_field_time_inspiratory_maximum_less,
  mode_settings_field_time_inspiratory_maximum_less_text,

  mode_settings_field_cycles_per_minute_text,
  mode_settings_field_cycles_per_minute_value,
  mode_settings_field_cycles_per_minute_value_wrapper,
  mode_settings_field_cycles_per_minute_more,
  mode_settings_field_cycles_per_minute_more_text,
  mode_settings_field_cycles_per_minute_less,
  mode_settings_field_cycles_per_minute_less_text,

  mode_settings_field_tidal_volume_text,
  mode_settings_field_tidal_volume_value,
  mode_settings_field_tidal_volume_value_wrapper,
  mode_settings_field_tidal_volume_more,
  mode_settings_field_tidal_volume_more_text,
  mode_settings_field_tidal_volume_less,
  mode_settings_field_tidal_volume_less_text,

  mode_settings_field_inspiratory_flow_text,
  mode_settings_field_inspiratory_flow_value,
  mode_settings_field_inspiratory_flow_value_wrapper,
  mode_settings_field_inspiratory_flow_more,
  mode_settings_field_inspiratory_flow_more_text,
  mode_settings_field_inspiratory_flow_less,
  mode_settings_field_inspiratory_flow_less_text,

  mode_settings_field_inspiratory_duration_text,
  mode_settings_field_inspiratory_duration_value,
  mode_settings_field_inspiratory_duration_value_wrapper,
  mode_settings_field_inspiratory_duration_more,
  mode_settings_field_inspiratory_duration_more_text,
  mode_settings_field_inspiratory_duration_less,
  mode_settings_field_inspiratory_duration_less_text,

  mode_settings_field_plateau_duration_text,
  mode_settings_field_plateau_duration_value,
  mode_settings_field_plateau_duration_value_wrapper,
  mode_settings_field_plateau_duration_more,
  mode_settings_field_plateau_duration_more_text,
  mode_settings_field_plateau_duration_less,
  mode_settings_field_plateau_duration_less_text,

  mode_settings_field_trigger_offset_text,
  mode_settings_field_trigger_offset_value,
  mode_settings_field_trigger_offset_value_wrapper,
  mode_settings_field_trigger_offset_more,
  mode_settings_field_trigger_offset_more_text,
  mode_settings_field_trigger_offset_less,
  mode_settings_field_trigger_offset_less_text,

  mode_settings_field_trigger_expiratory_text,
  mode_settings_field_trigger_expiratory_value,
  mode_settings_field_trigger_expiratory_value_wrapper,
  mode_settings_field_trigger_expiratory_more,
  mode_settings_field_trigger_expiratory_more_text,
  mode_settings_field_trigger_expiratory_less,
  mode_settings_field_trigger_expiratory_less_text,

  mode_settings_alarm_threshold_low_inspiratory_minute_volume_text,
  mode_settings_alarm_threshold_low_inspiratory_minute_volume_value,
  mode_settings_alarm_threshold_low_inspiratory_minute_volume_value_wrapper,
  mode_settings_alarm_threshold_low_inspiratory_minute_volume_more,
  mode_settings_alarm_threshold_low_inspiratory_minute_volume_more_text,
  mode_settings_alarm_threshold_low_inspiratory_minute_volume_less,
  mode_settings_alarm_threshold_low_inspiratory_minute_volume_less_text,

  mode_settings_alarm_threshold_high_inspiratory_minute_volume_text,
  mode_settings_alarm_threshold_high_inspiratory_minute_volume_value,
  mode_settings_alarm_threshold_high_inspiratory_minute_volume_value_wrapper,
  mode_settings_alarm_threshold_high_inspiratory_minute_volume_more,
  mode_settings_alarm_threshold_high_inspiratory_minute_volume_more_text,
  mode_settings_alarm_threshold_high_inspiratory_minute_volume_less,
  mode_settings_alarm_threshold_high_inspiratory_minute_volume_less_text,

  mode_settings_alarm_threshold_low_expiratory_minute_volume_text,
  mode_settings_alarm_threshold_low_expiratory_minute_volume_value,
  mode_settings_alarm_threshold_low_expiratory_minute_volume_value_wrapper,
  mode_settings_alarm_threshold_low_expiratory_minute_volume_more,
  mode_settings_alarm_threshold_low_expiratory_minute_volume_more_text,
  mode_settings_alarm_threshold_low_expiratory_minute_volume_less,
  mode_settings_alarm_threshold_low_expiratory_minute_volume_less_text,

  mode_settings_alarm_threshold_high_expiratory_minute_volume_text,
  mode_settings_alarm_threshold_high_expiratory_minute_volume_value,
  mode_settings_alarm_threshold_high_expiratory_minute_volume_value_wrapper,
  mode_settings_alarm_threshold_high_expiratory_minute_volume_more,
  mode_settings_alarm_threshold_high_expiratory_minute_volume_more_text,
  mode_settings_alarm_threshold_high_expiratory_minute_volume_less,
  mode_settings_alarm_threshold_high_expiratory_minute_volume_less_text,

  mode_settings_alarm_threshold_low_respiratory_rate_text,
  mode_settings_alarm_threshold_low_respiratory_rate_value,
  mode_settings_alarm_threshold_low_respiratory_rate_value_wrapper,
  mode_settings_alarm_threshold_low_respiratory_rate_more,
  mode_settings_alarm_threshold_low_respiratory_rate_more_text,
  mode_settings_alarm_threshold_low_respiratory_rate_less,
  mode_settings_alarm_threshold_low_respiratory_rate_less_text,

  mode_settings_alarm_threshold_high_respiratory_rate_text,
  mode_settings_alarm_threshold_high_respiratory_rate_value,
  mode_settings_alarm_threshold_high_respiratory_rate_value_wrapper,
  mode_settings_alarm_threshold_high_respiratory_rate_more,
  mode_settings_alarm_threshold_high_respiratory_rate_more_text,
  mode_settings_alarm_threshold_high_respiratory_rate_less,
  mode_settings_alarm_threshold_high_respiratory_rate_less_text,

  mode_settings_alarm_threshold_low_tidal_volume_text,
  mode_settings_alarm_threshold_low_tidal_volume_value,
  mode_settings_alarm_threshold_low_tidal_volume_value_wrapper,
  mode_settings_alarm_threshold_low_tidal_volume_more,
  mode_settings_alarm_threshold_low_tidal_volume_more_text,
  mode_settings_alarm_threshold_low_tidal_volume_less,
  mode_settings_alarm_threshold_low_tidal_volume_less_text,

  mode_settings_alarm_threshold_high_tidal_volume_text,
  mode_settings_alarm_threshold_high_tidal_volume_value,
  mode_settings_alarm_threshold_high_tidal_volume_value_wrapper,
  mode_settings_alarm_threshold_high_tidal_volume_more,
  mode_settings_alarm_threshold_high_tidal_volume_more_text,
  mode_settings_alarm_threshold_high_tidal_volume_less,
  mode_settings_alarm_threshold_high_tidal_volume_less_text,

  mode_settings_alarm_threshold_leak_text,
  mode_settings_alarm_threshold_leak_value,
  mode_settings_alarm_threshold_leak_value_wrapper,
  mode_settings_alarm_threshold_leak_more,
  mode_settings_alarm_threshold_leak_more_text,
  mode_settings_alarm_threshold_leak_less,
  mode_settings_alarm_threshold_leak_less_text,

  mode_settings_alarm_threshold_peak_pressure_text,
  mode_settings_alarm_threshold_peak_pressure_value,
  mode_settings_alarm_threshold_peak_pressure_value_wrapper,
  mode_settings_alarm_threshold_peak_pressure_more,
  mode_settings_alarm_threshold_peak_pressure_more_text,
  mode_settings_alarm_threshold_peak_pressure_less,
  mode_settings_alarm_threshold_peak_pressure_less_text,

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
  advanced_group_wrapper,
  advanced_form_wrapper,
  advanced_group_tab_statistics_button,
  advanced_group_tab_settings_button,
  advanced_group_tab_statistics_text,
  advanced_group_tab_settings_text,

  advanced_field_locale_text,
  advanced_field_locale_value,
  advanced_field_locale_value_wrapper,
  advanced_field_locale_more,
  advanced_field_locale_more_text,
  advanced_field_locale_less,
  advanced_field_locale_less_text,

  advanced_text_date_text,
  advanced_text_date_value,

  advanced_text_time_text,
  advanced_text_time_value,

  advanced_text_timezone_text,
  advanced_text_timezone_value,

  modal_background,
  modal_container_borders,
  modal_container,
  modal_close,
  modal_close_text,
  modal_save,
  modal_save_text,

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

  // Patient modal
  patient_child,
  patient_teenager,
  patient_adult,
});

impl Ids {
    pub fn allocate(&mut self, interface: &mut Ui) {
        // Allocate advanced line
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
