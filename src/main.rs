use salary_param::SalaryParam;
use slider::Slider;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::{header::Header, slider::SliderMessage};

mod header;
mod salary_param;
mod slider;

const PARAMETERS_MEANING: i8 = 10;

const DATA: [SalaryParam; 7] = [
    SalaryParam {
        id: "like",
        label: "How much I hate what I'm doing?",
        coefficient: 1.0,
        value: 50,
    },
    SalaryParam {
        id: "body",
        label: "How much I destroy my body at work?",
        coefficient: 1.0,
        value: 50,
    },
    SalaryParam {
        id: "pain",
        label: "How much pain I get from doing my job?",
        coefficient: 1.0,
        value: 50,
    },
    SalaryParam {
        id: "mental",
        label: "How much I'm emotionally impacted by my work?",
        coefficient: 1.0,
        value: 50,
    },
    SalaryParam {
        id: "value",
        label: "How much value I bring with my work?",
        coefficient: 1.0,
        value: 50,
    },
    SalaryParam {
        id: "skills",
        label: "How rare are my skills?",
        coefficient: 1.0,
        value: 50,
    },
    SalaryParam {
        id: "training",
        label: "How much I sacrifice to train my skills?",
        coefficient: 1.0,
        value: 50,
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
            <Header />
            <div class="w3-row firstRow">
                <div class="w3-half contentBlock">
                    <form class="w3-container">
                        <label>{"Base salary : "}</label>
                        <input class="w3-border w3-round-large parameterTextInput" type="number" value={input_base_salary.to_string()} oninput={on_base_change} />
                        <label>{"€"}</label>
                    </form>
                </div>
                <div class="w3-half contentBlock">
                        <span>{"Result : "}{result.to_string()}{"€"}</span>
                </div>
            </div>
            <div class="contentBlock">
                <div class="w3-row parameterHeader">
                    <div class="w3-half">{ "Criteria" }</div>
                    <div class="w3-quarter">{ "my job" }</div>
                    <div class="w3-quarter">{ "How it maters" }</div>
                </div>
                { for (*app_state).clone().into_iter().map(|param: SalaryParam| {
                    html! {
                        <div>
                            <Slider on_slide={on_slide.clone()} salary_param={param} />
                        </div>
                    }
                })}
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
    param.coefficient = slider_message.coef;
    new_state
}

fn compute_result(state: Vec<SalaryParam>, base: i32) -> f64 {
    let mut variable_wage_part = 0.0;
    for param in state {
        variable_wage_part += param.coefficient * param.value as f64;
    }
    (variable_wage_part as f64 * PARAMETERS_MEANING as f64 + base as f64).round()
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}
