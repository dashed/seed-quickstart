// crates

#[macro_use]
extern crate seed;

// imports

use seed::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Model

#[derive(Clone)]
struct Model {
    pub val: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self { val: 0 }
    }
}

// Update

#[derive(Clone)]
enum Msg {
    Increment,
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Increment => Model { val: model.val + 1 },
    }
}

// View

fn view(state: seed::App<Msg, Model>, model: Model) -> El<Msg> {

    let callback = Closure::wrap(Box::new(move || {
        state.update(Msg::Increment);
    }) as Box<dyn Fn()>);

    div![
        did_mount(move |_| {
            let window = web_sys::window().unwrap();
            window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    // Note this method call, which uses `as_ref()` to get a `JsValue`
                    // from our `Closure` which is then converted to a `&Function`
                    // using the `JsCast::unchecked_ref` function.
                    callback.as_ref().unchecked_ref(),
                    1_000,
                )
                .unwrap();
        }),
        button![
            simple_ev("click", Msg::Increment),
            format!("Hello, World × {}", model.val)
        ]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main", None);
}
