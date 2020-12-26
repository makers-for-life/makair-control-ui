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

macro_rules! gen_set_mode_new_draft_value {
    ($self:ident, $setting:expr, $action:ident, $field:tt, $step:ident) => {{
        let old_value = gen_get_mode_value!($self, $field);
        let new_value = $action.to_new_value(&$setting, old_value, $step);

        gen_set_mode_draft!($self, $field, old_value, new_value);
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
