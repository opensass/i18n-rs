use i18nrs::yew::use_translation;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(GreetingSelect)]
pub fn greeting_select() -> Html {
    let (i18n, set_language) = use_translation();

    let language_ref = use_node_ref();
    let language_state = use_state(|| "en".to_string());

    let onchange = {
        let language_ref = language_ref.clone();
        let language_state = language_state.clone();
        Callback::from(move |_| {
            if let Some(input) = language_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                language_state.set(value);
                set_language.emit(input.value());
            }
        })
    };

    html! {
        <>
            <select
                class="w-full border rounded-md p-2 mb-4"
                ref={language_ref}
                onchange={onchange}
            >
                <option value="en">{ "🇺🇸 English" }</option>
                <option value="fr">{ "🇫🇷 French" }</option>
                <option value="es">{ "🇪🇸 Spanish" }</option>
                <option value="ar">{ "🇸🇦 Arabic" }</option>
            </select>
            <h1 class="text-2xl font-semibold text-gray-700">{ i18n.t("greeting") }</h1>
        </>
    }
}

#[function_component(LanguageToggles)]
pub fn language_toggles() -> Html {
    let (i18n, set_language) = use_translation();

    html! {
        <>
            <div class="flex gap-4">
                <button
                    class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
                    onclick={set_language.reform(|_| "en".to_string())}
                >
                    { "🇺🇸" }
                </button>
                <button
                    class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
                    onclick={set_language.reform(|_| "fr".to_string())}
                >
                    { "🇫🇷" }
                </button>
                <button
                    class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
                    onclick={set_language.reform(|_| "es".to_string())}
                >
                    { "🇪🇸" }
                </button>
            </div>
            <h1 class="text-2xl font-semibold text-gray-700 ml-4">{ i18n.t("greeting") }</h1>
        </>
    }
}

#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    let (i18n, _set_language) = use_translation();

    html! {
        <input
            type="text"
            placeholder={i18n.t("search.placeholder")}
            class="w-full border rounded-md p-2"
        />
    }
}

#[function_component(NavMenu)]
pub fn nav_menu() -> Html {
    let (i18n, _set_language) = use_translation();

    html! {
        <nav class="flex gap-4">
            <a href="#home" class="text-blue-500 hover:underline">{ i18n.t("nav.home") }</a>
            <a href="#about" class="text-blue-500 hover:underline">{ i18n.t("nav.about") }</a>
            <a href="#contact" class="text-blue-500 hover:underline">{ i18n.t("nav.contact") }</a>
        </nav>
    }
}

#[function_component(LocalizedForm)]
pub fn localized_form() -> Html {
    let (i18n, _set_language) = use_translation();

    html! {
        <form class="space-y-4">
            <div>
                <label class="block text-gray-700">{ i18n.t("form.name") }</label>
                <input
                    type="text"
                    placeholder={i18n.t("form.name_placeholder")}
                    class="w-full border rounded-md p-2"
                />
            </div>
            <div>
                <label class="block text-gray-700">{ i18n.t("form.email") }</label>
                <input
                    type="email"
                    placeholder={i18n.t("form.email_placeholder")}
                    class="w-full border rounded-md p-2"
                />
            </div>
            <button
                type="submit"
                class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
            >
                { i18n.t("form.submit") }
            </button>
        </form>
    }
}

#[function_component(ModalLanguageSelector)]
pub fn modal_language_selector() -> Html {
    let (i18n, set_language) = use_translation();

    let language_ref = use_node_ref();
    let language_state = use_state(|| "en".to_string());

    let modal_open = use_state(|| false);

    let toggle_modal = {
        let modal_open = modal_open.clone();
        Callback::from(move |_| modal_open.set(!*modal_open))
    };

    let onchange = {
        let language_ref = language_ref.clone();
        let language_state = language_state.clone();
        Callback::from(move |_| {
            if let Some(input) = language_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                language_state.set(value);
                set_language.emit(input.value());
            }
        })
    };

    html! {
        <>
            <button
                onclick={toggle_modal.clone()}
                class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
            >
                { i18n.t("change_language") }
            </button>
            { if *modal_open {
                html! {
                    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
                        <div class="bg-white rounded-lg p-6">
                            <select
                                class="w-full border rounded-md p-2"
                                ref={language_ref}
                                onchange={onchange}
                            >
                                <option value="en">{ "🇺🇸 English" }</option>
                                <option value="fr">{ "🇫🇷 French" }</option>
                                <option value="es">{ "🇪🇸 Spanish" }</option>
                            </select>
                            <button
                                onclick={toggle_modal.clone()}
                                class="mt-4 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
                            >
                                { i18n.t("close") }
                            </button>
                        </div>
                    </div>
                }
            } else {
                html! { <></> }
            } }
        </>
    }
}

#[function_component(ThemeBasedOnLanguage)]
pub fn theme_based_on_language() -> Html {
    let (i18n, _set_language) = use_translation();
    let lang = i18n.get_current_language();

    let theme_class = match lang {
        "fr" => "text-yellow-400",
        "es" => "text-green-400",
        _ => "text-blue-400",
    };

    html! {
        <h1 class={format!("text-2xl font-semibold {}", theme_class)}>
            { i18n.t("theme.dynamic.title") }
        </h1>
    }
}

#[function_component(TooltipExample)]
pub fn tooltip_example() -> Html {
    let (i18n, _set_language) = use_translation();

    html! {
        <button class="relative group">
            { i18n.t("tooltip.button") }
            <span
                class="absolute hidden group-hover:block bg-gray-800 text-white text-xs rounded px-2 py-1"
            >
                { i18n.t("tooltip.text") }
            </span>
        </button>
    }
}

#[function_component(Examples)]
pub fn examples() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "I18n RS Yew Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                // Basic Usage
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "Basic Usage" }
                    </h2>
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use i18nrs::yew::use_translation;

#[function_component(GreetingSelect)]
pub fn greeting_select() -> Html {
    let (i18n, set_language) = use_translation();

    let language_ref = use_node_ref();
    let language_state = use_state(|| "en".to_string());

    let onchange = {
        let language_ref = language_ref.clone();
        let language_state = language_state.clone();
        Callback::from(move |_| {
            if let Some(input) = language_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                language_state.set(value);
                set_language.emit(input.value());
            }
        })
    };

    html! {
        <>
            <select
                class="w-full border rounded-md p-2 mb-4"
                ref={language_ref}
                onchange={onchange}
            >
                <option value="en">{ "🇺🇸 English" }</option>
                <option value="fr">{ "🇫🇷 French" }</option>
                <option value="es">{ "🇪🇸 Spanish" }</option>
                <option value="ar">{ "🇸🇦 Arabic" }</option>
            </select>
            <h1 class="text-2xl font-semibold text-gray-700">{ i18n.t("greeting") }</h1>
        </>
    }
}"# }
                    </pre>
                    <GreetingSelect />
                </div>
                // Language Toggles with Buttons
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "Language Toggles with Buttons" }
                    </h2>
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use i18nrs::yew::use_translation;

#[function_component(LanguageToggles)]
pub fn language_toggles() -> Html {
    let (i18n, set_language) = use_translation();

    html! {
        <>
            <div class="flex gap-4">
                <button
                    class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
                    onclick={set_language.reform(|_| "en".to_string())}
                >
                    { "🇺🇸" }
                </button>
                <button
                    class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
                    onclick={set_language.reform(|_| "fr".to_string())}
                >
                    { "🇫🇷" }
                </button>
                <button
                    class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
                    onclick={set_language.reform(|_| "es".to_string())}
                >
                    { "🇪🇸" }
                </button>
            </div>
            <h1 class="text-2xl font-semibold text-gray-700 ml-4">{ i18n.t("greeting") }</h1>
        </>
    }
}"# }
                    </pre>
                    <LanguageToggles />
                </div>
                // Localized Search Bar Placeholder
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "Localized Search Bar Placeholder" }
                    </h2>
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use i18nrs::yew::use_translation;

#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    let (i18n, _set_language) = use_translation();

    html! {
        <input
            type="text"
            placeholder={i18n.t("search.placeholder")}
            class="w-full border rounded-md p-2"
        />
    }
}"# }
                    </pre>
                    <SearchBar />
                </div>
                // Navigation Menu with Localized Labels
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "Navigation Menu with Localized Labels" }
                    </h2>
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use i18nrs::yew::use_translation;

#[function_component(NavMenu)]
pub fn nav_menu() -> Html {
    let (i18n, _set_language) = use_translation();

    html! {
        <nav class="flex gap-4">
            <a href='#home' class='text-blue-500 hover:underline'>{ i18n.t('nav.home') }</a>
            <a href='#about' class='text-blue-500 hover:underline'>{ i18n.t('nav.about') }</a>
            <a href='#contact' class='text-blue-500 hover:underline'>{ i18n.t('nav.contact') }</a>
        </nav>
    }
}"# }
                    </pre>
                    <NavMenu />
                </div>
                // Localized Form Labels and Placeholders
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "Localized Form Labels and Placeholders" }
                    </h2>
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use i18nrs::yew::use_translation;

#[function_component(LocalizedForm)]
pub fn localized_form() -> Html {
    let (i18n, _set_language) = use_translation();

    html! {
        <form class="space-y-4">
            <div>
                <label class="block text-gray-700">{ i18n.t("form.name") }</label>
                <input
                    type="text"
                    placeholder={i18n.t("form.name_placeholder")}
                    class="w-full border rounded-md p-2"
                />
            </div>
            <div>
                <label class="block text-gray-700">{ i18n.t("form.email") }</label>
                <input
                    type="email"
                    placeholder={i18n.t("form.email_placeholder")}
                    class="w-full border rounded-md p-2"
                />
            </div>
            <button
                type="submit"
                class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
            >
                { i18n.t("form.submit") }
            </button>
        </form>
    }
}"# }
                    </pre>
                    <LocalizedForm />
                </div>
                // Modal-based Language Selector
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "Modal-based Language Selector" }
                    </h2>
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use i18nrs::yew::use_translation;

#[function_component(ModalLanguageSelector)]
pub fn modal_language_selector() -> Html {
    let (i18n, set_language) = use_translation();

    let language_ref = use_node_ref();
    let language_state = use_state(|| "en".to_string());

    let modal_open = use_state(|| false);

    let toggle_modal = {
        let modal_open = modal_open.clone();
        Callback::from(move |_| modal_open.set(!*modal_open))
    };

    let onchange = {
        let language_ref = language_ref.clone();
        let language_state = language_state.clone();
        Callback::from(move |_| {
            if let Some(input) = language_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                language_state.set(value);
                set_language.emit(input.value());
            }
        })
    };

    html! {
        <>
            <button
                onclick={toggle_modal.clone()}
                class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
            >
                { i18n.t("change_language") }
            </button>
            { if *modal_open {
                html! {
                    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
                        <div class="bg-white rounded-lg p-6">
                            <select
                                class="w-full border rounded-md p-2"
                                ref={language_ref}
                                onchange={onchange}
                            >
                                <option value="en">{ "🇺🇸 English" }</option>
                                <option value="fr">{ "🇫🇷 French" }</option>
                                <option value="es">{ "🇪🇸 Spanish" }</option>
                            </select>
                            <button
                                onclick={toggle_modal.clone()}
                                class="mt-4 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
                            >
                                { i18n.t("close") }
                            </button>
                        </div>
                    </div>
                }
            } else {
                html! { <></> }
            }}
        </>
    }
}"# }
                    </pre>
                    <ModalLanguageSelector />
                </div>
                // Theme-Based on Language
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "Theme-Based on Language" }
                    </h2>
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use i18nrs::yew::use_translation;

#[function_component(ThemeBasedOnLanguage)]
pub fn theme_based_on_language() -> Html {
    let (i18n, _set_language) = use_translation();
    let lang = i18n.get_current_language();

    let theme_class = match lang {
        "fr" => "text-yellow-400",
        "es" => "text-green-400",
        _ => "text-blue-400",
    };

    html! {
        <h1 class={format!("text-2xl font-semibold {}", theme_class)}>
            { i18n.t("theme.dynamic.title") }
        </h1>
    }
}"# }
                    </pre>
                    <ThemeBasedOnLanguage />
                </div>
                // Tooltips with Translations
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "Tooltips with Translations" }
                    </h2>
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use i18nrs::yew::use_translation;

#[function_component(TooltipExample)]
pub fn tooltip_example() -> Html {
    let (i18n, _set_language) = use_translation();

    html! {
        <button class="relative group">
            { i18n.t("tooltip.button") }
            <span class="absolute hidden group-hover:block bg-gray-800 text-white text-xs rounded px-2 py-1">
                { i18n.t("tooltip.text") }
            </span>
        </button>
    }
}"# }
                    </pre>
                    <TooltipExample />
                </div>
            </div>
        </div>
    }
}
