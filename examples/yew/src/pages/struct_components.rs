use i18nrs::yew::use_translation;
use i18nrs::yew::I18nProvider;
use i18nrs::yew::I18nProviderConfig;
use std::collections::HashMap;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct StructComponents {}

pub struct StructComponentsMsg {}
#[derive(Properties, PartialEq, Default)]
pub struct StructComponentsProps {}

impl Component for StructComponents {
    type Message = StructComponentsMsg;
    type Properties = StructComponentsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let translations = HashMap::from([
            ("us", r#"{"greeting": "Hello"}"#),
            ("fr", r#"{"greeting": "Bonjour"}"#),
            ("es", r#"{"greeting": "Hola"}"#),
        ]);

        let config = I18nProviderConfig {
            translations,
            default_language: "us".to_string(),
            ..Default::default()
        };

        html! {
            <I18nProvider ..config>
                <FMainStructComponents />
            </I18nProvider>
        }
    }
}

#[yew::function_component(FMainStructComponents)]
pub fn f_main_struct_components() -> yew::Html {
    let (i18n, set_language) = use_translation();
    yew::html! {<MainStructComponents i18n={i18n} set_language={set_language} />}
}

pub struct MainStructComponents {}

pub struct MainMsg {}
#[derive(Properties, PartialEq)]
pub struct MainProps {
    i18n: i18nrs::I18n,
    set_language: Callback<String>,
}

impl Component for MainStructComponents {
    type Message = MainMsg;
    type Properties = MainProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let greeting = ctx.props().i18n.t("greeting"); // Retrieves translation for key "greeting"
        let language = (&ctx.props().set_language).clone();
        let callback = Callback::from(move |e: MouseEvent| {
            let id = JsCast::unchecked_into::<HtmlInputElement>(e.target().unwrap()).id();
            language.emit(id.to_string())
        });

        html! {
            <div class="m-6 min-h-screen flex flex-col items-center justify-center">
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h1 class="text-xl font-semibold mb-4 text-gray-800">{ greeting }</h1>
                    <button class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600" id={"us"} onclick={&callback}>
                        { "Switch to Us" }
                    </button>
                    <button class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600" id={"fr"} onclick={&callback}>
                        { "Switch to Fr" }
                    </button>
                    <button class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600" id={"es"} onclick={&callback}>
                        { "Switch to Es" }
                    </button>
                </div>
            </div>
        }
    }
}
