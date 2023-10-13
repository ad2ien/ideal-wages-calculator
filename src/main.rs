use salary_param::SalaryParam;
use slider::Slider;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
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
    let base_salary_handle = use_state(|| 1000);
    let input_base_salary = (*base_salary_handle).clone();
    let result = use_state(|| compute_result(DATA.to_vec(), input_base_salary));

    let result_clone = result.clone();
    let new_state = app_state.clone();
    let on_slide: Callback<SliderMessage> = {
        Callback::from(move |msg: SliderMessage| {
            let new_data = compute_state((*new_state).clone(), msg);
            new_state.set(new_data);
            result_clone.set(compute_result((*new_state).clone(), input_base_salary));
        })
    };
    
    let result_clone_base = result.clone();
    let new_state_base = app_state.clone();
    let on_base_change = {
        let base_salary_handle = base_salary_handle.clone();

        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            
            if let Some(input) = input {
                let base_salary = input.value().parse::<i32>().expect("expected number");
                base_salary_handle.set(base_salary);
                result_clone_base.set(compute_result((*new_state_base).clone(), base_salary));
            }
        })
    };

    html! {
        <div class="container">
        <div class="contentBlock">
            <span>{"Base salary : "}</span>
            <input type="text" value={input_base_salary.to_string()} oninput={on_base_change} />
        </div>
            <div class="contentBlock">
            { for (*app_state).clone().into_iter().map(|param: SalaryParam| {
                html! {
                    <div>
                        <Slider on_slide={on_slide.clone()} salary_param={param} />
                    </div>
                }
            })}
            </div>
            <div class="contentBlock">
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

fn compute_result(state: Vec<SalaryParam>, base: i32) -> f64 {
    let mut result = 0.0;
    for param in state {
        result += param.coefficient * param.value as f64;
    }
    result + base as f64
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}
