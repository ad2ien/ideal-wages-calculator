use log::info;
use slider::Slider;
use yew::prelude::*;

mod slider;

#[function_component]
fn App() -> Html {
    let on_slide: Callback<i8> = Callback::from(move |value: i8| {
        info!("Value {:?}", value);
    });

    html! {
        <div>
            <Slider on_slide={on_slide} default_value={50} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}
