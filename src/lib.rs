#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! In this project, I developed a simple Google Chrome extension that prints the URLs of all opened tabs of the browser in the extension HTML window, when the button of the extension is pressed. This extension also takes the screenshot of a selected file and displays this screenshot in the HTML window. Extension has a button to download the screenshot in the png file. This extension was written in Rust and compiled to WASM to use the Rust functions in the Javascript module of the extension later.
//! 
//! In this project all Rust functions are in the src folder, they perform the main logic of a plugin. manifest.json is the file with all settings of the plugin and is the first file that our browser sees after installing the plugin. popup.html is a file with HTML markup of a plugin window. In style.css there is a style for this HTML window. And the script.js file contains Javascript functions that are exported into Rust.
//! 
//! I used a wasm-bindgen crate to compile functions from Rust module into a wasm file. I also used a web-sys crate to get the information about the URLs of all opened tabs of the browser. With a help of the web-sys crate I also created a canvas to draw an image with the screenshot there.

use wasm_bindgen::prelude::*;
use web_sys::*;

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// imported functions from js
#[wasm_bindgen(module = "/script.js")]
extern "C" {
    fn getActiveTabUrl(callback: &Closure<dyn FnMut(Vec<JsValue>)>);
    fn changeContent(content: JsValue, id: String);
    fn listProcesses();
    fn screen();
    fn take_screen_from_js();
    fn openLinkInNewTab(url: String);
    fn dlCanvas();
}

/// display all open tabs
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
        changeContent(pretty_html_list.into(), "content".to_string());
    });
    getActiveTabUrl(&closure);

    closure.forget();
}

/// starts videostream and captures first frame
/// draw this frame into the canvas
#[wasm_bindgen]
pub fn take_screen_from_rust() {
    // let navigator = web_sys::window().unwrap().navigator();
    // let media_devices = navigator.media_devices().unwrap();

    // let closure = Closure::new(move |stream: JsValue| {
    //     let track: MediaStreamTrack = stream.dyn_into::<web_sys::MediaStream>().unwrap().
    //     get_video_tracks().get(0).into();
    //     let capture = ImageCapture::new(&track).unwrap();

    //     let f = Closure::new(move |bitmap: JsValue| {
    //         track.stop();
    //         let canvas: HtmlCanvasElement = window().unwrap()
    //         .document().unwrap().get_element_by_id("myCanvas").unwrap().dyn_into().unwrap();
    //         let bitmap: web_sys::HtmlImageElement = bitmap.unchecked_into();
    //         canvas.set_width(bitmap.width() as u32);
    //         canvas.set_height(bitmap.height() as u32);
    //         canvas.get_context("2d").unwrap().unwrap()
    //         .dyn_into::<CanvasRenderingContext2d>().unwrap()
    //         .draw_image_with_html_image_element(&bitmap, 0.0, 0.0)
    //         .unwrap();
    
    //         let screenshot_url = canvas.to_data_url().unwrap();

    //         console_log!("{}", screenshot_url);
    //         //open_new_tab(screenshot_url);
    //         //changeContent(screenshot_url.into(), "content".to_string());
    //     });

    //     let _ = capture.grab_frame().then(&f);
    //     f.forget();
    // });
   
    // let _ = media_devices.get_display_media().unwrap().then(&closure);
    // closure.forget();
}

/// opens url in a new tab
#[wasm_bindgen]
pub fn open_new_tab(url: String) {
    let window: Window = window().unwrap();

    window
    .open_with_url(&url)
    .expect("Unable to open a new tab");
}

/// gets the text from the HTML element by it's id
#[wasm_bindgen]
pub fn get_text_by_id(id: &str) -> String {
    let elemet = window().unwrap()
    .document().unwrap().get_element_by_id(id).unwrap();
    elemet.text_content().unwrap()
}

/// buttons functions

#[wasm_bindgen]
pub fn print_processes() {
    listProcesses();
}

/// downloads canvas to the browser screen
#[wasm_bindgen]
pub fn download() {
    dlCanvas();
}

/// takes screenshot
#[wasm_bindgen]
pub fn take_screen() {
    //take_screen_from_js();
    take_screen_from_rust();
}

/// opens another tab with nice content
#[wasm_bindgen]
pub fn dont_press() {
    screen();
}
