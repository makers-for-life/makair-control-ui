// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use fluent::concurrent::FluentBundle;
use fluent::FluentResource;
use unic_langid::LanguageIdentifier;

use crate::EmbeddedLocales;

use super::accessor::LocaleAccessor;
use super::locales::LocaleCode;

const LOCALE_EXTENSION: &str = ".ftl";

pub struct LocaleLoader {
    locale_id: LanguageIdentifier,
    resource: FluentResource,
}

impl LocaleLoader {
    pub fn new(locale: &str) -> Self {
        debug!("loading locale: [{}]...", locale);

        if LocaleCode::from_code(locale).is_none() {
            panic!(
                "locale not mapped in the list of allowed locales: {}",
                locale
            );
        }

        let locale_id: LanguageIdentifier = locale.parse().expect("locale code parsing failed");
        let locale_buffer = EmbeddedLocales::get(&format!("{}{}", locale, LOCALE_EXTENSION))
            .expect("locale not found");
        let locale_string =
            String::from_utf8(locale_buffer.into_owned()).expect("locale file is not a string");

        info!("loaded locale: [{}]", locale);

        LocaleLoader {
            locale_id,
            resource: FluentResource::try_new(locale_string).expect("failed to parse locale file"),
        }
    }

    pub fn into_bundle(self) -> FluentBundle<FluentResource> {
        let mut bundle = FluentBundle::new(&[self.locale_id]);

        bundle
            .add_resource(self.resource)
            .expect("failed to add locale to its bundle");

        bundle
    }

    pub fn into_accessor(self) -> LocaleAccessor {
        LocaleAccessor::new(self.into_bundle())
    }
}
