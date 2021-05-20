// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

macro_rules! gen_get_mode_value {
    ($self:ident, $field:tt) => {{
        if let Some(ref draft) = $self.draft {
            draft.$field
        } else {
            $self.live.$field
        }
    }};
}

macro_rules! gen_set_mode_draft {
    ($self:ident, $field:tt, $old_value:expr, $new_value:expr) => {{
        // Value did change?
        if $new_value != $old_value {
            // Initialize draft?
            if $self.draft.is_none() {
                $self.draft = Some($self.live.clone());
            }

            if let Some(ref mut draft) = $self.draft {
                // Update draft value
                draft.$field = $new_value;
            }
        }
    }};
}

macro_rules! gen_commit_mode_events_numeric {
    ($self:ident, $draft:ident, $events:ident, { $($setting:tt -> $field:tt,)+ }) => {{
        $(
            if $draft.$field != $self.live.$field {
                $events.push(ControlMessage {
                    setting: ControlSetting::$setting,
                    value: $draft.$field as _,
                });
            }
        )+
    }};
}

macro_rules! gen_mode_impls {
    ($($setting:tt |-> $field:tt [$step:ident],)+) => {
        #[derive(Debug)]
        #[allow(clippy::upper_case_acronyms)]
        pub enum SettingsModeIntent {
            ClearDraft,
            ModePcCmv,
            ModePcAc,
            ModePcVsai,
            ModeVcCmv,
            ModeVcAc,

            $(
                $setting(SettingActionRange),
            )+
        }

        #[derive(Debug, Clone)]
        pub struct SettingsModeSettings {
            pub mode: VentilationMode,

            $(
                pub $field: usize,
            )+
        }

        impl Default for SettingsModeSettings {
            fn default() -> Self {
                Self {
                    mode: VentilationMode::default(),

                    $(
                        $field: ControlSetting::$setting.default(),
                    )+
                }
            }
        }

        impl SettingsMode {
            pub fn new_intent(&mut self, intent: SettingsModeIntent) {
                paste! {
                    match intent {
                        SettingsModeIntent::ClearDraft => {
                            self.clear_draft()
                        },
                        SettingsModeIntent::ModePcCmv => {
                            self.switch_mode(VentilationMode::PC_CMV)
                        },
                        SettingsModeIntent::ModePcAc => {
                            self.switch_mode(VentilationMode::PC_AC)
                        },
                        SettingsModeIntent::ModePcVsai => {
                            self.switch_mode(VentilationMode::PC_VSAI)
                        },
                        SettingsModeIntent::ModeVcCmv => {
                            self.switch_mode(VentilationMode::VC_CMV)
                        },
                        SettingsModeIntent::ModeVcAc => {
                            self.switch_mode(VentilationMode::VC_AC)
                        },

                        $(
                            SettingsModeIntent::$setting(action) => {
                                self.[<set_ $field >](action)
                            },
                        )+
                    }
                }
            }

            pub fn commit(&mut self) -> Vec<ControlMessage> {
                let mut events = Vec::new();

                // Generate events from changed draft values?
                if let Some(ref draft) = self.draft {
                    // Append all other numeric values
                    gen_commit_mode_events_numeric!(
                        self, draft, events, {
                            $($setting -> $field,)+
                        }
                    );

                    // Append non-numeric mode value (right after all numeric values have been \
                    //   commited)
                    if draft.mode != self.live.mode {
                        events.push(ControlMessage {
                            setting: ControlSetting::VentilationMode,
                            value: u8::from(&draft.mode) as _,
                        });
                    }
                }

                // Ensure draft is cleared
                self.draft = None;

                events
            }

            $(
                paste! {
                    fn [<set_ $field>](&mut self, action: SettingActionRange) {
                        // Update old value to new value? (this may create a new draft)
                        let old_value = gen_get_mode_value!(self, $field);

                        let new_value = action.to_new_value(
                            &ControlSetting::$setting, old_value, $step
                        );

                        gen_set_mode_draft!(self, $field, old_value, new_value);
                    }
                }
            )+
        }
    };
}
