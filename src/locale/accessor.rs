// MakAir
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use fluent::concurrent::FluentBundle;
use fluent::FluentResource;
use unic_langid::LanguageIdentifier;

pub struct LocaleAccessor {
    bundle: FluentBundle<FluentResource>,
}

impl LocaleAccessor {
    pub fn new(locale_id: LanguageIdentifier, resource: FluentResource) -> Self {
        let mut bundle = FluentBundle::new(&[locale_id]);

        bundle
            .add_resource(resource)
            .expect("failed to add locale to its bundle");

        LocaleAccessor { bundle: bundle }
    }

    pub fn t(&self, key: &str) -> String {
        let mut errors = vec![];

        let message = self
            .bundle
            .get_message(key)
            .expect(&format!("locale key not found: {}", key));

        // Notice: return the key if the message has no value (eg. not yet translated)
        if let Some(pattern) = message.value {
            let formatted = self.bundle.format_pattern(&pattern, None, &mut errors);

            // Any error? Panic
            if errors.len() > 0 {
                panic!("could not format pattern: {:?}", errors);
            }

            formatted.to_string()
        } else {
            key.to_string()
        }
    }
}
