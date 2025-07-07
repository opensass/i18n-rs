#![doc = include_str!("../DIOXUS.md")]

use crate::config::{I18n, I18nConfig, StorageType};
use dioxus::prelude::*;
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use web_sys::{wasm_bindgen::JsCast, window, Storage};

/// Properties for the `I18nProvider` component.
///
/// This configuration struct allows you to specify supported translations,
/// storage preferences, and define callbacks for handling language changes and errors.
#[derive(Props, PartialEq, Clone)]
pub struct I18nProviderProps {
    /// The translations raw content.
    ///
    /// A map where keys are language codes (e.g. `"en"`, `"fr"`) and values are the corresponding translation strings or raw content.
    /// Defaults to an empty `HashMap` if not provided.
    #[props(default)]
    pub translations: HashMap<&'static str, &'static str>,

    /// The child components wrapped with the `I18n` context.
    ///
    /// These elements will have access to the internationalization features provided by the `I18nProvider`.
    pub children: Element,

    /// The type of browser storage to use.
    ///
    /// Determines how the selected language is persisted in the user's browser.
    /// Options typically include `StorageType::LocalStorage` or `StorageType::SessionStorage`.
    /// Defaults to `StorageType::LocalStorage`.
    #[props(default)]
    pub storage_type: StorageType,

    /// The key for storing the selected language.
    ///
    /// Used as the key in the browser storage system to persist the selected language.
    /// Defaults to `"i18nrs"`.
    #[props(default = "i18nrs".to_string())]
    pub storage_name: String,

    /// Default language if no language is found in storage.
    ///
    /// This will be used as the fallback language when there is no saved language in storage.
    /// Defaults to `"en"`.
    #[props(default = "en".to_string())]
    pub default_language: String,

    /// Callback when the language changes.
    ///
    /// Invoked whenever the language is updated.
    /// Receives the new language code as a `String`.
    #[props(default)]
    pub onchange: EventHandler<String>,

    /// Callback for handling errors.
    ///
    /// Triggered when an error occurs during the internationalization process.
    /// Receives an error message as a `String`.
    #[props(default)]
    pub onerror: EventHandler<String>,
}

/// The context provided to children by the `I18nProvider`.
///
/// Contains the current `I18n` instance and a method to change the language.
#[derive(Clone)]
pub struct I18nContext {
    /// Reactive signal containing the current internationalization state.
    pub i18n: Signal<I18n>,

    /// Function to change the current language.
    ///
    /// Triggers re-rendering of any components using the `i18n` signal.
    pub set_language: EventHandler<String>,
}

/// I18nProvider Component
///
/// A Dioxus component that provides internationalization (i18n) context to its child components.
/// It initializes translations, manages language persistence using browser storage, and provides
/// reactive access to the current language and i18n instance. It also handles text direction (RTL/LTR)
/// updates based on the selected language.
///
/// # Features
/// - Loads and provides translations via a context.
/// - Dynamically updates the `dir` attribute of the HTML document based on RTL/LTR languages.
/// - Handles language change events and provides reactive updates to subscribers.
/// - Persists user-selected language in browser storage.
/// - Emits callbacks for changes and errors.
///
/// # Behavior
/// - Reads the initial language from browser storage using the configured `storage_type` and `storage_name`.
/// - Falls back to `default_language` if no language is stored.
/// - Initializes the i18n instance using provided translations.
/// - If the language cannot be set, the `onerror` callback is triggered with the error message.
/// - On language change:
///   - Updates the browser storage.
///   - Applies text direction (`dir="rtl"` or `dir="ltr"`) on the `<html>` element.
///   - Calls the `onchange` callback.
///   - Updates the context state.
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use i18nrs::dioxus::I18nProvider;
/// use i18nrs::StorageType;
/// use std::collections::HashMap;
///
/// fn app() -> Element {
///     let translations = HashMap::from([
///         ("en", r#"{"hello": "Hello!"}"#),
///         ("ar", r#"{"hello": "مرحبا!"}"#),
///     ]);
///
///     rsx! {
///         I18nProvider {
///             translations: translations,
///             storage_type: StorageType::LocalStorage,
///             storage_name: "my_i18n_key".to_string(),
///             default_language: "en".to_string(),
///             onchange: move |lang| log::info!("Language changed to {lang}"),
///             onerror: move |err| log::error!("i18n error: {err}"),
///             children: rsx! {
///                 div { "Hello, world!" }
///             }
///         }
///     }
/// }
/// ```
///
/// # Notes
/// - Right-to-left (RTL) languages like Arabic, Hebrew, Persian, and Urdu automatically set the HTML `dir` attribute.
/// - If initialization fails (e.g., missing or malformed translation data), the `onerror` callback is triggered.
/// - The `I18nContext` with `i18n` and `set_language` is made available via Dioxus's context API.
#[component]
pub fn I18nProvider(props: I18nProviderProps) -> Element {
    let initial_language =
        use_initial_language(props.storage_type.clone(), props.storage_name.clone())()
            .unwrap_or(props.default_language.clone());

    #[cfg(target_arch = "wasm32")]
    fn is_rtl_language(lang: &str) -> bool {
        matches!(lang, "ar" | "he" | "fa" | "ur" | "ps" | "ku" | "sd")
    }

    let update_text_direction = |_lang: &str| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(document) = window().and_then(|win| win.document()) {
                let dir = if is_rtl_language(_lang) { "rtl" } else { "ltr" };
                if let Some(html_element) = document.document_element() {
                    let _ = html_element.set_attribute("dir", dir);
                }
            }
        }
    };

    update_text_direction(&initial_language.clone());

    let mut i18n = use_signal(|| {
        I18n::new(
            I18nConfig {
                translations: props.translations.clone(),
            },
            props.translations.clone(),
        )
        .map(|mut instance| {
            if let Err(err) = instance.set_translation_language(
                &initial_language.clone(),
                &props.storage_type,
                &props.storage_name,
            ) {
                props.onerror.call(err.clone());
            }
            instance
        })
        .unwrap_or_else(|err| {
            props.onerror.call(err.clone());
            panic!("Failed to initialize I18n: {}", err);
        })
    });

    let set_language = EventHandler::new({
        move |language: String| {
            let mut i18n_val = i18n();
            update_text_direction(&language);

            let lang = language.clone();
            if i18n_val
                .set_translation_language(&language, &props.storage_type, &props.storage_name)
                .is_ok()
            {
                i18n.set(i18n_val);
                props.onchange.call(language);
                let storage_name = props.storage_name.clone();

                #[cfg(feature = "dio-ssr")]
                spawn(async move {
                    let lang = lang.clone();
                    let _ = set_cookie(storage_name, lang).await;
                });
            }
        }
    });

    let context = I18nContext { i18n, set_language };
    provide_context(context);

    rsx! { {props.children} }
}

pub fn use_i18n() -> I18nContext {
    consume_context::<I18nContext>()
}

#[allow(unused)]
pub fn use_initial_language(storage_type: StorageType, key: String) -> Signal<Option<String>> {
    let mut language = use_signal(|| None);

    #[cfg(target_arch = "wasm32")]
    {
        let stored: Option<String> = match storage_type {
            StorageType::LocalStorage => window()
                .expect("No window object")
                .local_storage()
                .expect("Failed to access localStorage")
                .and_then(|s| s.get_item(&key).ok())
                .expect("Stored language not found in localStorage"),
            StorageType::SessionStorage => window()
                .expect("No window object")
                .session_storage()
                .expect("Failed to access sessionStorage")
                .and_then(|s| s.get_item(&key).ok())
                .expect("Stored language not found in sessionStorage"),
        };
        language.set(stored);

        // TODO: Why no cookie?
        #[cfg(feature = "dio-ssr")]
        let cookie = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.dyn_into::<web_sys::HtmlDocument>().ok())
            .and_then(|html_doc| html_doc.cookie().ok())
            .and_then(|c| {
                c.split(';')
                    .map(|c| c.trim())
                    .find_map(|c| c.strip_prefix(&format!("{key}=")))
                    .map(|v| v.to_owned())
            });

        #[cfg(feature = "dio-ssr")]
        language.set(cookie);

        #[cfg(feature = "dio-ssr")]
        spawn(async move {
            let key = key.clone();
            let _cookie = get_cookie(key).await.unwrap();
            // language.set(Some(cookie));
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "dio-ssr")]
    {
        server_only! {
            use_future({
                use http::header::{COOKIE, SET_COOKIE};

                let key = key.to_owned();
                move || {let value = key.clone();
                    async move {
                        let ctx = server_context();

                        let headers: http::HeaderMap = ctx.extract().await.unwrap();

                        if let Some(raw) = headers.get(COOKIE).and_then(|v| v.to_str().ok()) {
                            if let Some(v) = raw.split(';')
                                                .map(|c| c.trim())
                                                .find_map(|c| c.strip_prefix(&format!("{value}=")))
                            {
                                language.set(Some(v.to_string()));
                                return;
                            }
                        }

                        if let Some(al) = headers.get("Accept-Language")
                                                .and_then(|v| v.to_str().ok()) {
                            let v = al.split(',').next().unwrap_or("en").trim().to_owned();
                            language.set(Some(v.clone()));

                            ctx.response_parts_mut().headers.append(
                                SET_COOKIE,
                                http::HeaderValue::from_str(
                                    &format!("{value}={v}; Path=/; Max-Age=31536000; SameSite=Lax"))
                                .unwrap()
                            );
                        }
                    }
                }
            });
        }
    }

    language
}

#[cfg(feature = "dio-ssr")]
#[server]
pub async fn set_cookie(key: String, lang: String) -> Result<(), ServerFnError> {
    use http::header::SET_COOKIE;

    let ctx = server_context();
    ctx.response_parts_mut().headers.append(
        SET_COOKIE,
        http::HeaderValue::from_str(&format!(
            "{key}={lang}; Path=/; SameSite=Lax; Max-Age=31536000"
        ))?,
    );

    Ok(())
}

#[cfg(feature = "dio-ssr")]
#[server]
pub async fn get_cookie(key: String) -> Result<String, ServerFnError> {
    use http::header::COOKIE;

    let ctx = server_context();

    let headers: http::HeaderMap = ctx.extract().await.unwrap();

    if let Some(raw) = headers.get(COOKIE).and_then(|v| v.to_str().ok()) {
        if let Some(v) = raw
            .split(';')
            .map(|c| c.trim())
            .find_map(|c| c.strip_prefix(&format!("{key}=")))
        {
            return Ok(v.to_string());
        }
    }

    Ok("en".to_string())
}
