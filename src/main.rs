use log::info;
use salary_param::SalaryParam;
use slider::Slider;
use yew::prelude::*;

use crate::slider::SliderMessage;

mod salary_param;
mod slider;

const DATA: [SalaryParam; 3] = [
    SalaryParam {
        id: "salary",
        label: "Salary",
        coefficient: 1.0,
        value: 50,
    },
    SalaryParam {
        id: "bonus",
        label: "Bonus",
        coefficient: 0.5,
        value: 100,
    },
    SalaryParam {
        id: "tax",
        label: "Tax",
        coefficient: 0.2,
        value: 100,
    },
];

#[function_component]
fn App() -> Html {
    let app_state = use_state(|| DATA.to_vec());
    let result = use_state(|| compute_result(DATA.to_vec()));
    let result_clone = result.clone();
    let new_state = app_state.clone();

    let on_slide: Callback<SliderMessage> = {
        Callback::from(move |msg: SliderMessage| {
            let new_data = compute_state((*new_state).clone(), msg);
            info!("Value {:?}", new_data);
            new_state.set(new_data);
            result_clone.set(compute_result((*new_state).clone()));
        })
    };

    html! {
        <div class="container">
            <div class="parameters">
            { for (*app_state).clone().into_iter().map(|param: SalaryParam| {
                html! {
                    <div>
                        <Slider on_slide={on_slide.clone()} salary_param={param} />
                    </div>
                }
            })}
            </div>
            <div class="result">
                <span>{"Result : "}{result.to_string()}</span>
            </div>
        </div>
    }
}

fn compute_state(state: Vec<SalaryParam>, slider_message: SliderMessage) -> Vec<SalaryParam> {
    let mut new_state = state.clone();
    let param = new_state
        .iter_mut()
        .find(|param| param.id == slider_message.id)
        .unwrap();
    param.value = slider_message.value;
    new_state
}

fn compute_result(state: Vec<SalaryParam>) -> f64 {
    let mut result = 0.0;
    for param in state {
        result += param.coefficient * param.value as f64;
    }
    result
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}
