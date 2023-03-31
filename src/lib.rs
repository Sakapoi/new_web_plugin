use web_sys::{window, ServiceWorkerGlobalScope, HtmlElement};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub fn my_plugin_function() {
  console_log!("Some text");
}

#[wasm_bindgen]
pub fn get_tab_urls() -> Vec<JsValue> {
  let window = window().unwrap();
  let document = window.document().unwrap();
  let tabs = document.get_elements_by_name("a");
  let mut urls: Vec<JsValue> = Vec::new();

  for i in 0..tabs.length() {
      let tab = tabs.item(i).unwrap();
      let href = tab.dyn_ref::<HtmlElement>().unwrap().get_attribute("w").unwrap();
      urls.push(href.into());
  }

  urls
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
  // отримання об'єкта window
  let window = web_sys::window().unwrap();

  // визначення функції, що буде викликана при події
  let callback = Closure::wrap(Box::new(|| {
    my_plugin_function();
  }) as Box<dyn FnMut()>);

  // приєднання обробника події до кнопки
  let document = window.document().unwrap();
  let button = document.get_element_by_id("my-button").unwrap();
  button.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())?;

  // звільнення пам'яті
  callback.forget();

  Ok(())
}

