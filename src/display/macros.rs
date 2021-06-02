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
        #[allow(clippy::large_enum_variant)]
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
            let clicks = DisplayUiEvents::count_clicks(
                $interface,
                &$widget_ids,
            );

            for _ in 0..clicks {
                debug!("pressed the {} widget once", $name);

                $settings_state.open();

                $has = true;
            }
        )+
    }
}

macro_rules! gen_ui_events_generic_settings_clicks {
    (
        $interface:ident,
        $ids:ident,
        $has:ident,

        $({
            $name:expr,

            $([
                $field_name:expr,
                $widget_ids:expr,
                $form_handler:block
            ]),*$(,)?
        }),+,
    ) => {
        $(
            // Handle clicks on dynamic form elements
            $(
                for _ in 0..DisplayUiEvents::count_clicks(
                    $interface,
                    &$widget_ids,
                ) {
                    debug!(
                        "pressed the {} settings {} field button once", $name, $field_name
                    );

                    // Call form handler block
                    $form_handler;

                    $has = true;
                }
            )*
        )+
    }
}

macro_rules! gen_ui_events_modal_settings_clicks {
    (
        $interface:ident,
        $ids:ident,
        $has:ident,

        $({
            $name:expr,
            $settings_state:expr,
            $close_handler:block,
            $save_handler:block,

            $([
                $field_name:expr,
                $widget_ids:expr,
                $form_handler:block
            ]),*
        }),+,
    ) => {
        $(
            if $settings_state.is_open() {
                // Handle clicks on the close button
                for _ in 0..DisplayUiEvents::count_clicks(
                    $interface,
                    &[
                        $ids.modal_background,
                        $ids.modal_close,
                        $ids.modal_close_text,
                    ],
                ) {
                    debug!("pressed the {} settings close button once", $name);

                    $settings_state.close();

                    // Call close handler block
                    $close_handler;

                    $has = true;
                }

                // Handle clicks on the save button (if any)
                for _ in 0..DisplayUiEvents::count_clicks(
                    $interface,
                    &[
                        $ids.modal_save,
                        $ids.modal_save_text,
                    ],
                ) {
                    debug!("pressed the {} settings save button once", $name);

                    $settings_state.close();

                    // Call save handler block
                    $save_handler;

                    $has = true;
                }

                // Handle other clicks
                gen_ui_events_generic_settings_clicks!(
                    $interface, $ids, $has,

                    {
                        $name,

                        $(
                            [
                                $field_name,
                                $widget_ids,
                                $form_handler
                            ],
                        )*
                    },
                );
            }
        )+
    }
}

macro_rules! gen_ui_events_modal_settings_intents_clicks {
    (
        $interface:ident,
        $ids:ident,
        $intents:ident,
        $has:ident,

        $({
            $name:expr,
            $type:tt,
            $settings_prefix:tt,
            $settings_state:expr,
            $settings_intent:tt,
            $close_handler:block,
            $save_handler:block,

            {
                $([
                    $switch_field_name:expr,
                    $switch_intent:tt,
                    $switch_field:tt
                ]),*
            },

            {
                $([
                    $navigate_field_name:expr,
                    $navigate_intent:tt,
                    $navigate_field:tt
                ]),*
            }
        }),+,
    ) => {
        gen_ui_events_modal_settings_clicks!(
            $interface, $ids, $has,

            $({
                $name, $settings_state, $close_handler, $save_handler,

                $([
                    $switch_field_name,

                    paste! {
                        [
                            $ids.[<$settings_prefix _selector_tab_ $switch_field>],
                            $ids.[<$settings_prefix _selector_texts_ $switch_field>],
                        ]
                    },

                    {
                        $intents.push(ChipSettingsIntent::$type(
                            $settings_intent::$switch_intent
                        ));
                    }
                ]),*
            },)+

            $({
                $name, $settings_state, $close_handler, $save_handler,

                $(
                    [
                        $navigate_field_name,

                        paste! {
                            [
                                $ids.[<$settings_prefix _ $navigate_field _less>],
                                $ids.[<$settings_prefix _ $navigate_field _less_text>],
                            ]
                        },

                        {
                            $intents.push(ChipSettingsIntent::$type(
                                $settings_intent::$navigate_intent(
                                    SettingActionRange::Less
                                )
                            ));
                        }
                    ],

                    [
                        $navigate_field_name,

                        paste! {
                            [
                                $ids.[<$settings_prefix _ $navigate_field _more>],
                                $ids.[<$settings_prefix _ $navigate_field _more_text>],
                            ]
                        },

                        {
                            $intents.push(ChipSettingsIntent::$type(
                                $settings_intent::$navigate_intent(
                                    SettingActionRange::More
                                )
                            ));
                        }
                    ]
                ),*
            },)+
        );
    }
}

macro_rules! gen_ui_events_modal_local_clicks {
    (
        $interface:ident,
        $ids:ident,
        $has:ident,

        $({
            $name:expr,
            $settings_state:expr,

            $({
                $field_name:expr,
                $widget_ids:expr,
                $handler:block
            }),*
        }),+,
    ) => {
        $(
            if $settings_state.is_open() {
                // Handle clicks on local elements
                $(
                    for _ in 0..DisplayUiEvents::count_clicks(
                        $interface,
                        &$widget_ids,
                    ) {
                        debug!("pressed the {} local {} field button once", $name, $field_name);

                        // Call handler block
                        $handler;

                        $has = true;
                    }
                )*
            }
        )+
    }
}

macro_rules! gen_render_preset_settings_field_ids {
    ($self:ident, $name:tt) => {
        paste! {
            (
                $self.ids.[<preset_settings_field_ $name _text>],
                $self.ids.[<preset_settings_field_ $name _value_wrapper>],
                $self.ids.[<preset_settings_field_ $name _value>],
                $self.ids.[<preset_settings_field_ $name _more>],
                $self.ids.[<preset_settings_field_ $name _more_text>],
                $self.ids.[<preset_settings_field_ $name _less>],
                $self.ids.[<preset_settings_field_ $name _less_text>],
            )
        }
    };
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

macro_rules! gen_render_mode_settings_alarm_ids {
    ($self:ident, $name:tt) => {
        paste! {
            (
                $self.ids.[<mode_settings_alarm_ $name _text>],
                $self.ids.[<mode_settings_alarm_ $name _value_wrapper>],
                $self.ids.[<mode_settings_alarm_ $name _value>],
                $self.ids.[<mode_settings_alarm_ $name _more>],
                $self.ids.[<mode_settings_alarm_ $name _more_text>],
                $self.ids.[<mode_settings_alarm_ $name _less>],
                $self.ids.[<mode_settings_alarm_ $name _less_text>],
            )
        }
    };
}

macro_rules! gen_render_advanced_settings_field_ids {
    ($self:ident, $name:tt) => {
        paste! {
            (
                $self.ids.[<advanced_field_ $name _text>],
                $self.ids.[<advanced_field_ $name _value_wrapper>],
                $self.ids.[<advanced_field_ $name _value>],
                $self.ids.[<advanced_field_ $name _more>],
                $self.ids.[<advanced_field_ $name _more_text>],
                $self.ids.[<advanced_field_ $name _less>],
                $self.ids.[<advanced_field_ $name _less_text>],
            )
        }
    };
}

macro_rules! gen_render_advanced_settings_text_ids {
    ($self:ident, $name:tt) => {
        paste! {
            (
                $self.ids.[<advanced_text_ $name _text>],
                $self.ids.[<advanced_text_ $name _value>],
            )
        }
    };
}
