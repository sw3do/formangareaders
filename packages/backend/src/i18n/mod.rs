use fluent::{FluentBundle, FluentResource};
use fluent_bundle::FluentArgs;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use unic_langid::LanguageIdentifier;

struct SafeFluentBundle(FluentBundle<FluentResource>);

unsafe impl Send for SafeFluentBundle {}
unsafe impl Sync for SafeFluentBundle {}

#[derive(Clone)]
pub struct I18n {
    bundles: Arc<Mutex<HashMap<String, SafeFluentBundle>>>,
}

impl I18n {
    pub fn new() -> Self {
        let mut bundles = HashMap::new();

        let locales = vec!["en", "tr"];

        for locale in locales {
            let lang_id: LanguageIdentifier = locale.parse().expect("Invalid language identifier");
            let mut bundle = FluentBundle::new(vec![lang_id]);

            let ftl_string = match locale {
                "en" => include_str!("locales/en.ftl"),
                "tr" => include_str!("locales/tr.ftl"),
                _ => include_str!("locales/en.ftl"),
            };

            let resource = FluentResource::try_new(ftl_string.to_string())
                .expect("Failed to parse FTL string");

            bundle
                .add_resource(resource)
                .expect("Failed to add resource to bundle");

            bundles.insert(locale.to_string(), SafeFluentBundle(bundle));
        }

        Self {
            bundles: Arc::new(Mutex::new(bundles)),
        }
    }

    pub fn get_message(&self, locale: &str, message_id: &str, args: Option<&FluentArgs>) -> String {
        let bundles = self.bundles.lock().unwrap();
        let bundle = bundles.get(locale).or_else(|| bundles.get("en"));

        if let Some(SafeFluentBundle(bundle)) = bundle {
            if let Some(message) = bundle.get_message(message_id) {
                if let Some(pattern) = message.value() {
                    let mut errors = vec![];
                    let formatted = bundle.format_pattern(pattern, args, &mut errors);
                    return formatted.to_string();
                }
            }
        }

        message_id.to_string()
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}
