use salary_param::SalaryParam;
use slider::Slider;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::{
    criterias::Criteria,
    data::DATA,
    header::Header,
    slider::{SliderCoefMessage, SliderMessage},
};

mod criterias;
mod data;
mod header;
mod salary_param;
mod slider;

const PARAMETERS_MEANING: i8 = 10;

#[function_component]
fn App() -> Html {
    let parameter_state = use_state(|| DATA.to_vec());
    let criterias_state = use_state(|| criterias::CRITERIAS.to_vec());
    let base_salary_state = use_state(|| 1000);
    let input_base_salary_state = (*base_salary_state).clone();
    let result_state = use_state(|| {
        compute_result(
            (*parameter_state).clone(),
            (*criterias_state).clone(),
            input_base_salary_state.clone(),
        )
    });

    let result_clone_2 = result_state.clone();

    let criterias_state_2 = criterias_state.clone();
    let criterias_state_3 = criterias_state.clone();
    let criterias_state_4 = criterias_state.clone();
    let criterias_state_5 = criterias_state.clone();

    let result_state_2 = result_state.clone();
    let result_state_3 = result_state.clone();

    let parameter_state_2 = parameter_state.clone();
    let parameter_state_3 = parameter_state.clone();
    let parameter_state_4 = parameter_state.clone();

    let on_slide: Callback<SliderMessage> = {
        Callback::from(move |msg: SliderMessage| {
            let mut state = (*parameter_state_2).clone();
            let param = state
                .iter_mut()
                .find(|param| param.id == msg.id)
                .expect(format!("param not found : {} ", msg.id).as_str());
            param.value = msg.value;
            parameter_state_2.set(state.to_vec());
            result_clone_2.set(compute_result(
                (*parameter_state_2).clone(),
                (*criterias_state_2).clone(),
                input_base_salary_state.clone(),
            ));
        })
    };

    let on_coef_slide: Callback<SliderCoefMessage> = {
        Callback::from(move |msg: SliderCoefMessage| {
            let mut crit_state = (*criterias_state_3).clone();
            let criteria = crit_state
                .iter_mut()
                .find(|criteria| criteria.id == msg.id)
                .expect(format!("criteria not found : {} ", msg.id).as_str());
            criteria.coefficient = msg.coef;
            criterias_state_3.set(crit_state.to_vec());
            result_state_2.set(compute_result(
                (*parameter_state_3).clone(),
                (*criterias_state_3).clone(),
                input_base_salary_state.clone(),
            ));
        })
    };

    let on_base_change = {
        let base_salary_handle = base_salary_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                let base_salary = input.value().parse::<i32>().expect("expected number");
                base_salary_handle.set(base_salary);
            }
            result_state_3.set(compute_result(
                (*parameter_state_4).clone(),
                (*criterias_state_4).clone(),
                input_base_salary_state.clone(),
            ));
        })
    };

    html! {
        <div class="container">
            <Header />
            <div class="w3-row firstRow">
                <div class="w3-half contentBlock">
                    <form class="w3-container">
                        <label>{"Base salary : "}</label>
                        <input class="w3-border w3-round-large parameterTextInput"
                          type="number"
                          value={input_base_salary_state.to_string()}
                          oninput={on_base_change} />
                        <label>{"€"}</label>
                    </form>
                </div>
                <div class="w3-half contentBlock">
                        <span>{"Result : "}{result_state.to_string()}{"€"}</span>
                </div>
            </div>
            <div class="contentBlock">
                <div class="w3-row parameterHeader">
                    <div class="w3-half">{ "Criteria" }</div>
                    <div class="w3-quarter">{ "my job" }</div>
                    <div class="w3-quarter">{ "How it maters" }</div>
                </div>
                { for (*criterias_state_5).clone().into_iter().map(|criteria: Criteria| {
                    let param = (*parameter_state).clone().into_iter().find(|param| param.id == criteria.id).unwrap();
                    html! {
                        <div>
                            <Slider on_parameter_slide={on_slide.clone()} on_coef_slide={on_coef_slide.clone()} salary_param={param} criteria={criteria} />
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

fn compute_result(state: Vec<SalaryParam>, criterias: Vec<Criteria>, base: i32) -> f64 {
    let variable_wage_part = state.clone().into_iter().fold(0.0, |acc, param| {
        acc + (param.value as f64
            * criterias
                .clone()
                .into_iter()
                .find(|criteria| criteria.id == param.id)
                .expect(format!("criteria not found : {} ", param.id).as_str())
                .coefficient)
    });
    (variable_wage_part as f64 * PARAMETERS_MEANING as f64 + base as f64).round()
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}
