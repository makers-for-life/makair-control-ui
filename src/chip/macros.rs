// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

macro_rules! gen_override_snapshot_values_from_stopped_identity_clone {
    ($snapshot:expr, $stopped:expr, [$($key:ident),+]) => {
        $(
            $snapshot.$key = $stopped.$key.to_owned();
        )+
    };
}

macro_rules! gen_override_snapshot_values_from_stopped_identity {
    ($snapshot:expr, $stopped:expr, [$($key:ident),+]) => {
        $(
            $snapshot.$key = $stopped.$key;
        )+
    };
}

macro_rules! gen_override_snapshot_values_from_stopped_optional {
    ($snapshot:expr, $stopped:expr, [$($key:ident),+]) => {
        $(
            if let Some(value) = $stopped.$key {
                $snapshot.$key = value;
            }
        )+
    };
}

macro_rules! gen_settings_from_parameters_alarm_thresholds {
    ($update:expr, $mode_settings:expr, [$($name:tt),+]) => {
        $(
            paste! {
                if let Some(value) = $update.[<$name _alarm_threshold>] {
                    $mode_settings.[<alarm_threshold_ $name>] = value as usize;
                }
            }
        )+
    };
}

macro_rules! gen_add_data_generic {
    ($self:ident, $container:tt, $value:expr, $systick:expr, $clean_fn:tt) => {
        let snapshot_time = $self.boot_time.unwrap() + Duration::microseconds($systick as i64);

        // Fetch last data value in order to reduce noise, and check if the point should be \
        //   stored as well (there is no need storing points faster than twice the framerate, \
        //   as this is sufficient to ensure that the plot progresses in time smoothly, and that \
        //   the curves look nice on screen)
        let (new_point, may_store) = if let Some(last_value_inner) = $self.$container.get(0) {
            let new_point = last_value_inner.1
                - ((last_value_inner.1 - $value) / TELEMETRY_POINTS_LOW_PASS_DEGREE);

            let may_store = (snapshot_time - last_value_inner.0)
                >= chrono::Duration::milliseconds(DATA_STORE_EVERY_MILLISECONDS);

            (new_point, may_store)
        } else {
            ($value, true)
        };

        // May we store this value point?
        if may_store {
            // Points are stored as mmH20 (for more precision; though we do work in cmH20)
            $self.$container.push_front((snapshot_time, new_point));

            // Clean any now-expired value
            $self.$clean_fn(snapshot_time);
        }
    };
}

macro_rules! gen_clean_expired_data_from_time_generic {
    ($self:ident, $container:tt, $front_time:ident) => {
        if !$self.$container.is_empty() {
            let expired_time = $front_time - chrono::Duration::seconds(GRAPH_DRAW_SECONDS);

            while $self
                .$container
                .back()
                .map(|p| p.0 < expired_time)
                .unwrap_or(false)
            {
                $self.$container.pop_back();
            }
        }
    };
}
