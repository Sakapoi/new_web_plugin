use std::{fmt::format, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "/script.js")]
extern "C" {
    fn getActiveTabUrl(callback: &Closure<dyn FnMut(Vec<JsValue>)>);
    fn changeContent(content: JsValue);
}

#[wasm_bindgen]
pub fn my_plugin_function() {
    // let closure = Closure::wrap(Box::new(|url: JsValue| {
    //   console_log!("qwe");
    //   let url_str = url.as_string().unwrap();
    //   console_log!("{}", url_str);
    //}) as Box<dyn Fn(JsValue)>);
    let closure = Closure::new(move |url: Vec<JsValue>| {
        let url_str = url
            .into_iter()
            .filter_map(|url| url.as_string())
            .collect::<Vec<_>>();
        console_log!("{:?}", url_str);
        let pretty_html_list = format!(
            r#"<ul class="styled-list">
      {}
      </ul>"#,
            url_str
                .into_iter()
                .map(|url| format!("<li>{url}</li>"))
                .collect::<Vec<_>>()
                .join("")
        );
        changeContent(pretty_html_list.into());
    });
    getActiveTabUrl(&closure);

    closure.forget();
}
