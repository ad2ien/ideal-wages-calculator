use log::info;
use salary_param::SalaryParam;
use slider::Slider;
use yew::prelude::*;

use crate::slider::SliderMessage;

mod salary_param;
mod slider;

const DATA: [&'static SalaryParam; 3] = [
    &SalaryParam {
        id: "salary",
        label: "Salary",
        coefficient: 1.0,
        value: 50,
    },
    &SalaryParam {
        id: "bonus",
        label: "Bonus",
        coefficient: 0.5,
        value: 100,
    },
    &SalaryParam {
        id: "tax",
        label: "Tax",
        coefficient: 0.2,
        value: 100,
    },
];

#[function_component]
fn App() -> Html {
    // let app_state = use_state(|| DATA.to_vec());
    let app_state = use_state(|| DATA.to_vec());
    let new_state = app_state.clone();

    let on_slide: Callback<SliderMessage> = {
        Callback::from(move |msg: SliderMessage| {
        info!("Value {:?} {:?}",msg.id, msg.value);
        new_state.set(
            DATA.to_vec());
    })};

    html! {
        <div class="container">
            <div class="parameters">
            { for (*app_state).clone().into_iter().map(|param: &SalaryParam| {
                html! {
                    <div>
                        <Slider on_slide={on_slide.clone()} salary_param={*param} />
                    </div>
                }
            })}
            </div>
            <div class="result">
                <span>{"Result :"}</span>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}
