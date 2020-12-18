// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

macro_rules! image_ids {
    (pub struct $name:ident { $($key:ident),+, }) => {
        pub struct $name {
            $(
                pub $key: ImageId,
            )+
        }

        impl $name {
            pub fn new(
                display: &GliumDisplayWinitWrapper,
                image_map: &mut conrod_core::image::Map<texture::SrgbTexture2d>,
            ) -> $name {
                // Insert all images in the image map, returning re-usable image identifiers
                $name {
                    $(
                        $key: image_map.insert(DisplayImages::$key(display)),
                    )+
                }
            }
        }
    };
}

macro_rules! gen_load_font {
    ($embedded_fonts:ident, $name:expr) => {
        conrod_core::text::Font::from_bytes(
            inflate_bytes_zlib(&$embedded_fonts::get(&format!("{}.ttf.zz", $name)).unwrap())
                .unwrap(),
        )
        .unwrap()
    };
}

macro_rules! gen_load_image_reverse {
    ($name:expr, $width:ident) => {
        reverse_rgba(
            &load_from_memory(
                EmbeddedImages::get(&format!("{}.png", $name))
                    .unwrap()
                    .to_mut(),
            )
            .unwrap()
            .into_rgba8()
            .into_raw(),
            $width,
        )
    };
}

macro_rules! gen_draw_cached_image {
    ($display:ident <= $logo_rgba:ident[$width:ident, $height:ident]) => {
        // Notice: build the raw image directly using the texture internals, as to avoid cloning \
        //   the raw image bytes at every refresh.
        glium::texture::SrgbTexture2d::new(
            &$display.0,
            glium::texture::RawImage2d {
                data: Cow::Borrowed(&*$logo_rgba),
                width: $width,
                height: $height,
                format: glium::texture::ClientFormat::U8U8U8U8,
            },
        )
        .unwrap()
    };
}

macro_rules! gen_widget_impls {
    ($($control:tt -> $widget:ident $([$lifetime:lifetime])*),+,) => {
        pub enum ControlWidgetType<'a> {
            $(
                $control($widget::Config<$($lifetime)*>),
            )+
        }

        impl<'a> ControlWidget<'a> {
            pub fn render(&mut self, widget_type: ControlWidgetType<'a>) -> f64 {
                match widget_type {
                    $(
                        ControlWidgetType::$control(config) => $widget::render(self, config),
                    )+
                }
            }
        }
    }
}

macro_rules! gen_ui_events_opener_settings_clicks {
    ($interface:ident, $has:ident, $({$name:expr, $settings_state:expr, $widget_ids:expr}),+,) => {
        $(
            let clicks = DisplayUIEvents::count_clicks(
                $interface,
                &$widget_ids,
            );

            for _ in 0..clicks {
                debug!("pressed the {} widget once", $name);

                $settings_state.toggle();

                $has = true;
            }
        )+
    }
}

macro_rules! gen_ui_events_modal_settings_clicks {
    (
        $interface:ident,
        $ids:ident,
        $has:ident,
        $events:ident,

        $({
            $name:expr,
            $type:tt,
            $settings_state:expr,

            $({
                $action:expr,
                $field_name:expr,
                $widget_ids:expr,
                $forced_state:expr
            }),*
        }),+,
    ) => {
        $(
            if $settings_state == DisplayRendererSettingsState::Opened {
                // Handle clicks on the close button
                for _ in 0..DisplayUIEvents::count_clicks(
                    $interface,
                    &[
                        $ids.modal_background,
                        $ids.modal_validate,
                        $ids.modal_validate_text,
                    ],
                ) {
                    debug!("pressed the {} settings close button once", $name);

                    $settings_state.toggle();

                    $has = true;
                }

                // Handle clicks on dynamic form elements
                $(
                    for _ in 0..DisplayUIEvents::count_clicks(
                        $interface,
                        &$widget_ids,
                    ) {
                        debug!("pressed the {} settings {} field button once", $name, $field_name);

                        $events.push(ChipSettingsEvent::$type(
                            $action,
                        ));

                        $has = true;

                        // Force to provided state?
                        if let Some(forced_state) = $forced_state {
                            debug!(
                                "forced state of the {} settings modal upon button press to: {:?}",
                                $name,
                                forced_state
                            );

                            $settings_state = forced_state;
                        }
                    }
                )*
            }
        )+
    }
}

macro_rules! gen_ui_events_modal_local_clicks {
    (
        $interface:ident,
        $ids:ident,
        $has:ident,

        $({
            $name:expr,
            $settings_chip:expr,
            $settings_state:expr,

            $({
                $target_value:expr,
                $field_key:tt,
                $field_name:expr,
                $widget_ids:expr
            }),*
        }),+,
    ) => {
        $(
            if $settings_state == DisplayRendererSettingsState::Opened {
                // Handle clicks on local elements
                $(
                    for _ in 0..DisplayUIEvents::count_clicks(
                        $interface,
                        &$widget_ids,
                    ) {
                        debug!("pressed the {} local {} field button once", $name, $field_name);

                        // Force state to target value
                        $settings_chip.$field_key = $target_value;

                        $has = true;
                    }
                )*
            }
        )+
    }
}

macro_rules! gen_render_mode_settings_field_ids {
    ($self:ident, $name:tt) => {
        paste! {
            (
                $self.ids.[<mode_settings_field_ $name _text>],
                $self.ids.[<mode_settings_field_ $name _value_wrapper>],
                $self.ids.[<mode_settings_field_ $name _value>],
                $self.ids.[<mode_settings_field_ $name _more>],
                $self.ids.[<mode_settings_field_ $name _more_text>],
                $self.ids.[<mode_settings_field_ $name _less>],
                $self.ids.[<mode_settings_field_ $name _less_text>],
            )
        }
    };
}
