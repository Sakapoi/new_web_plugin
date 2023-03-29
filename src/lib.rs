use wasm_bindgen::prelude::*;

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

