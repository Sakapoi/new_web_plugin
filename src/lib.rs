use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::*;
use js_sys::*;

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
    fn listProcesses();
    fn screen();
    fn takeScreenshott();
    fn captureAndCopyToClipboard();
    fn capture();
    fn takeScreenshotAndDownload();
    fn qwe();
}

#[wasm_bindgen]
pub fn my_plugin_function() {
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

#[wasm_bindgen]
pub fn take_screenshot() -> Result<JsValue, JsValue> {
    console_log!("peter");
    let window = web_sys::window().unwrap();
    console_log!("peter");
    let document = window.document().unwrap();
    console_log!("peter");
    let canvas = document.create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;
    let context = canvas.get_context("2d")?.unwrap().dyn_into::<CanvasRenderingContext2d>()?;
    console_log!("sanya");
    canvas.set_width(window.inner_width()?.as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height()?.as_f64().unwrap() as u32);
    console_log!("sanya {:?}", window);
    console_log!("sanya {:?}", context);
    let peter = vec![100; 4*100*100];
    console_log!("{}", peter.len());
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&peter), 100, 100);
    console_log!("sanya {:?}", data);
    context.put_image_data(&data.unwrap(), 100.0, 100.0)?;
    console_log!("sanya");
    let (width, height) = (
        window.inner_width()?.as_f64().unwrap() as u32,
        window.inner_height()?.as_f64().unwrap() as u32,
    );
    canvas.set_width(width);
    canvas.set_height(height);
    console_log!("sanya1");
    let data_url_promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let f = Closure::wrap(Box::new(move |data_url: String| {
            resolve.call1(&JsValue::NULL, &JsValue::from_str(&data_url)).unwrap();
        }) as Box<dyn FnMut(String)>);
        window.request_animation_frame(f.as_ref().unchecked_ref()).unwrap();
        let _ = f.forget();
    });
    console_log!("sanya2 {:?}", data_url_promise);
    let data_url = data_url_promise.as_string().unwrap();
    console_log!("sanya {}", data_url);
    let img = web_sys::HtmlImageElement::new()?;
    img.set_src(&data_url);
    
    context.draw_image_with_html_image_element(&img, width as f64, height as f64)?;
    console_log!("max875");
    Ok(JsValue::from_str(&canvas.to_data_url()?))
}

#[wasm_bindgen]
pub fn take_screen_from_js() {
    let navigator = web_sys::window().unwrap().navigator();
    let media_devices = navigator.media_devices().unwrap();
   
    let promise: Promise = media_devices.get_display_media().unwrap().into();
    let closure = Closure::once_into_js(move |stream: MediaStream| {
        let track: MediaStreamTrack = stream.get_video_tracks().get(0).into();
        let js_track = JsValue::from(track);
        let capture = js_sys::Reflect::get(&js_track, &JsValue::from_str("ImageCapture"))
        .unwrap()
        .dyn_into::<web_sys::ImageCapture>()
        .unwrap();
        let grab_promise = capture.grab_frame();
        
        let canvas: HtmlCanvasElement = window().unwrap().document().unwrap().get_element_by_id("myCanvas").unwrap().dyn_into().unwrap();

        let f = Closure::wrap(Box::new(|bitmap: js_sys::Object| {
            let bitmap: web_sys::ImageData = bitmap.unchecked_into();
            canvas.set_width(bitmap.width() as u32);
            canvas.set_height(bitmap.height() as u32);
            canvas.get_context("2d").unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>().unwrap()
            .draw_image_with_html_image_element(&bitmap.dyn_into::<HtmlImageElement>().unwrap(), 0.0, 0.0).unwrap();

            let url_promise = canvas.to_data_url().unwrap();
            console_log!("{}", url_promise);
            let callback = Function::new_no_args("");
            let f = Closure::wrap(Box::new(move |url: JsValue| {
                callback.call1(&JsValue::NULL, &url).unwrap();
            }) as Box<dyn FnMut(JsValue)>);
            //url_promise.then(&f.as_ref().unchecked_ref()).unwrap();
            f.forget();
        }) as Box<dyn FnMut(js_sys::Object)>);
        
        grab_promise.then(&f.as_ref().unchecked_ref()).unwrap();
        f.forget();

        let _ = track.stop();
    });

    // let _ = promise.then(&closure);
    // closure.forget();
}

#[wasm_bindgen]
pub fn open_new_tab(url: String) {
    let window: Window = window().unwrap();

    window
    .open_with_url(&url)
    .expect("Не вдалося відкрити вкладку");
}

#[wasm_bindgen]
pub fn print_processes() {
    listProcesses();
}

#[wasm_bindgen]
pub fn take_screen() {
    console_log!("peter");
    //qwe();
    take_screen_from_js();
    //console_log!("{:?}", take_screenshot().unwrap());
    // let url = take_screenshot().unwrap().as_string().unwrap();
    // open_new_tab(url);
}

#[wasm_bindgen]
pub fn dont_press() {
    screen();
}
