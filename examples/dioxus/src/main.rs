use dioxus::prelude::*;
use dioxus_logger::tracing;
use i18nrs::dioxus::I18nContext;
use i18nrs::dioxus::I18nProvider;
use std::collections::HashMap;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const MAIN_CSS: Asset = asset!("/assets/styles.css");

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

#[component]
fn app() -> Element {
    let translations = HashMap::from([
        ("en", include_str!("../i18n/en/base.json")),
        ("es", include_str!("../i18n/es/base.json")),
        ("fr", include_str!("../i18n/fr/base.json")),
        ("ar", include_str!("../i18n/ar/base.json")),
    ]);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        div {
            class: "m-6 min-h-screen flex flex-col items-center justify-center",
            I18nProvider {
                translations: translations.clone(),
                default_language: "en".to_string(),
                storage_name: "i18nrs".to_string(),
                onchange: EventHandler::new(|lang| tracing::info!("Language changed to: {}", lang)),
                onerror: EventHandler::new(|err| tracing::error!("i18n error: {}", err)),
                Examples {}
            }
        }
    }
}

#[component]
fn Examples() -> Element {
    rsx! {
        div { class: "m-6 min-h-screen flex flex-col items-center justify-center",
            h1 { class: "text-3xl font-bold mb-8 text-white", "I18n RS Dioxus Examples" }
            div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Greeting Selector" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
fn GreetingSelect() -> Element {{
    let I18nContext {{ i18n, set_language }} = use_context::<I18nContext>();
    let mut language_state = use_signal(|| "en".to_string());

    rsx! {{
        select {{
            class: "w-full border rounded-md p-2 mb-4",
            onchange: move |event| {{
                let value = event.value();
                language_state.set(value.clone());
                set_language.call(value);
            }},
            option {{ value: "en", "🇺🇸 English" }}
            option {{ value: "fr", "🇫🇷 French" }}
            option {{ value: "es", "🇪🇸 Spanish" }}
            option {{ value: "ar", "🇸🇦 Arabic" }}
        }}
        h1 {{ class: "text-2xl font-semibold text-gray-700", "{{i18n().t(\"greeting\")}}" }}
    }}
}}"##
                    }
                    GreetingSelect {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Language Toggles" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
fn LanguageToggles() -> Element {{
    let I18nContext {{ i18n, set_language }} = use_context::<I18nContext>();

    rsx! {{
        div {{ class: "flex gap-4",
            button {{
                class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                onclick: move |_| set_language.call("en".to_string()),
                "🇺🇸"
            }}
            button {{
                class: "px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600",
                onclick: move |_| set_language.call("fr".to_string()),
                "🇫🇷"
            }}
            button {{
                class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                onclick: move |_| set_language.call("es".to_string()),
                "🇪🇸"
            }}
        }}
        h1 {{ class: "text-2xl font-semibold text-gray-700 ml-4", "{{i18n().t(\"greeting\")}}" }}
    }}
}}"##
                    }
                    LanguageToggles {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Search Bar" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
fn SearchBar() -> Element {{
    let I18nContext {{ i18n, .. }} = use_context::<I18nContext>();

    rsx! {{
        input {{
            r#type: "text",
            placeholder: "{{i18n().t(\"search.placeholder\")}}",
            class: "w-full border rounded-md p-2"
        }}
    }}
}}"##
                    }
                    SearchBar {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Navigation Menu" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
fn NavMenu() -> Element {{
    let I18nContext {{ i18n, .. }} = use_context::<I18nContext>();

    rsx! {{
        nav {{ class: "flex gap-4",
            a {{ href: "#home", class: "text-blue-500 hover:underline", "{{i18n().t(\"nav.home\")}}" }}
            a {{ href: "#about", class: "text-blue-500 hover:underline", "{{i18n().t(\"nav.about\")}}" }}
            a {{ href: "#contact", class: "text-blue-500 hover:underline", "{{i18n().t(\"nav.contact\")}}" }}
        }}
    }}
}}"##
                    }
                    NavMenu {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Localized Form" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
fn LocalizedForm() -> Element {{
    let I18nContext {{ i18n, .. }} = use_context::<I18nContext>();

    rsx! {{
        form {{ class: "space-y-4",
            div {{
                label {{ class: "block text-gray-700", "{{i18n().t(\"form.name\")}}" }}
                input {{
                    r#type: "text",
                    placeholder: "{{i18n().t(\"form.name_placeholder\")}}",
                    class: "w-full border rounded-md p-2"
                }}
            }}
            div {{
                label {{ class: "block text-gray-700", "{{i18n().t(\"form.email\")}}" }}
                input {{
                    r#type: "email",
                    placeholder: "{{i18n().t(\"form.email_placeholder\")}}",
                    class: "w-full border rounded-md p-2"
                }}
            }}
            button {{
                r#type: "submit",
                class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                "{{i18n().t(\"form.submit\")}}"
            }}
        }}
    }}
}}"##
                    }
                    LocalizedForm {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Modal Language Selector" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
fn ModalLanguageSelector() -> Element {{
    let I18nContext {{ i18n, set_language }} = use_context::<I18nContext>();
    let mut modal_open = use_signal(|| false);
    let mut language_state = use_signal(|| "en".to_string());

    rsx! {{
        button {{
            class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
            onclick: move |_| modal_open.set(!modal_open()),
            "{{i18n().t(\"change_language\")}}"
        }}
        if modal_open() {{
            div {{ class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center",
                div {{ class: "bg-white rounded-lg p-6",
                    select {{
                        class: "w-full border rounded-md p-2",
                        onchange: move |event| {{
                            let value = event.value();
                            language_state.set(value.clone());
                            set_language.call(value);
                        }},
                        option {{ value: "en", "🇺🇸 English" }}
                        option {{ value: "fr", "🇫🇷 French" }}
                        option {{ value: "es", "🇪🇸 Spanish" }}
                    }}
                    button {{
                        class: "mt-4 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600",
                        onclick: move |_| modal_open.set(false),
                        "{{i18n().t(\"close\")}}"
                    }}
                }}
            }}
        }}
    }}
}}"##
                    }
                    ModalLanguageSelector {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Theme-Based on Language" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
fn ThemeBasedOnLanguage() -> Element {{
    let I18nContext {{ i18n, .. }} = use_context::<I18nContext>();

    let theme_class = match i18n().get_current_language() {{
        "fr" => "text-yellow-400",
        "es" => "text-green-400",
        _ => "text-blue-400",
    }};

    rsx! {{
        h1 {{ class: "text-2xl font-semibold {{theme_class}}", "{{i18n().t(\"theme.dynamic.title\")}}" }}
    }}
}}"##
                    }
                    ThemeBasedOnLanguage {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 { class: "text-xl font-semibold mb-4 text-gray-800", "Tooltips with Translations" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
fn TooltipExample() -> Element {{
    let I18nContext {{ i18n, .. }} = use_context::<I18nContext>();

    rsx! {{
        button {{ 
            class: "tooltip-button",
            "{{i18n().t(\"tooltip.button\")}}"
            span {{  
                class: "tooltip-text",
                "{{i18n().t(\"tooltip.text\")}}"
            }}
        }}
    }}
}}"##
                    }
                    TooltipExample {}
                }
            }
        }
    }
}

#[component]
fn GreetingSelect() -> Element {
    let I18nContext { i18n, set_language } = use_context::<I18nContext>();
    let mut language_state = use_signal(|| "en".to_string());

    rsx! {
        select {
            class: "w-full border rounded-md p-2 mb-4",
            onchange: move |event| {
                let value = event.value();
                language_state.set(value.clone());
                set_language.call(value);
            },
            option { value: "en", "🇺🇸 English" }
            option { value: "fr", "🇫🇷 French" }
            option { value: "es", "🇪🇸 Spanish" }
            option { value: "ar", "🇸🇦 Arabic" }
        }
        h1 { class: "text-2xl font-semibold text-gray-700", "{i18n().t(\"greeting\")}" }
    }
}

#[component]
fn LanguageToggles() -> Element {
    let I18nContext { i18n, set_language } = use_context::<I18nContext>();

    rsx! {
        div { class: "flex gap-4",
            button {
                class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                onclick: move |_| set_language.call("en".to_string()),
                "🇺🇸"
            }
            button {
                class: "px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600",
                onclick: move |_| set_language.call("fr".to_string()),
                "🇫🇷"
            }
            button {
                class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                onclick: move |_| set_language.call("es".to_string()),
                "🇪🇸"
            }
        }
        h1 { class: "text-2xl font-semibold text-gray-700 ml-4", "{i18n().t(\"greeting\")}" }
    }
}

#[component]
fn SearchBar() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    rsx! {
        input {
            r#type: "text",
            placeholder: "{i18n().t(\"search.placeholder\")}",
            class: "w-full border rounded-md p-2"
        }
    }
}

#[component]
fn NavMenu() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    rsx! {
        nav { class: "flex gap-4",
            a { href: "#home", class: "text-blue-500 hover:underline", "{i18n().t(\"nav.home\")}" }
            a { href: "#about", class: "text-blue-500 hover:underline", "{i18n().t(\"nav.about\")}" }
            a { href: "#contact", class: "text-blue-500 hover:underline", "{i18n().t(\"nav.contact\")}" }
        }
    }
}

#[component]
fn LocalizedForm() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    rsx! {
        form { class: "space-y-4",
            div {
                label { class: "block text-gray-700", "{i18n().t(\"form.name\")}" }
                input {
                    r#type: "text",
                    placeholder: "{i18n().t(\"form.name_placeholder\")}",
                    class: "w-full border rounded-md p-2"
                }
            }
            div {
                label { class: "block text-gray-700", "{i18n().t(\"form.email\")}" }
                input {
                    r#type: "email",
                    placeholder: "{i18n().t(\"form.email_placeholder\")}",
                    class: "w-full border rounded-md p-2"
                }
            }
            button {
                r#type: "submit",
                class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                "{i18n().t(\"form.submit\")}"
            }
        }
    }
}

#[component]
fn ModalLanguageSelector() -> Element {
    let I18nContext { i18n, set_language } = use_context::<I18nContext>();
    let mut modal_open = use_signal(|| false);
    let mut language_state = use_signal(|| "en".to_string());

    rsx! {
        button {
            class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
            onclick: move |_| modal_open.set(!modal_open()),
            "{i18n().t(\"change_language\")}"
        }
        if modal_open() {
            div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center",
                div { class: "bg-white rounded-lg p-6",
                    select {
                        class: "w-full border rounded-md p-2",
                        onchange: move |event| {
                            let value = event.value();
                            language_state.set(value.clone());
                            set_language.call(value);
                        },
                        option { value: "en", "🇺🇸 English" }
                        option { value: "fr", "🇫🇷 French" }
                        option { value: "es", "🇪🇸 Spanish" }
                    }
                    button {
                        class: "mt-4 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600",
                        onclick: move |_| modal_open.set(false),
                        "{i18n().t(\"close\")}"
                    }
                }
            }
        }
    }
}

#[component]
fn ThemeBasedOnLanguage() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    let theme_class = match i18n().get_current_language() {
        "fr" => "text-yellow-400",
        "es" => "text-green-400",
        _ => "text-blue-400",
    };

    rsx! {
        h1 { class: "text-2xl font-semibold {theme_class}", "{i18n().t(\"theme.dynamic.title\")}" }
    }
}

#[component]
fn TooltipExample() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    rsx! {
        button {
            class: "tooltip-button",
            "{i18n().t(\"tooltip.button\")}"
            span {
                class: "tooltip-text",
                "{i18n().t(\"tooltip.text\")}"
            }
        }
    }
}
