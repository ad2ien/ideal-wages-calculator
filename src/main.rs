use gloo_net::http::Request;
use slider_component::Slider;
use wages_param::WagesParam;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement};
use yew::functional::use_effect;
use yew::prelude::*;

use crate::{
    criterias::Criteria,
    header::Header,
    job::Job,
    slider_component::{SliderCoefMessage, SliderMessage},
};

mod criterias;
mod header;
mod job;
mod slider_component;
mod wages_param;

const PARAMETERS_MEANING: i8 = 10;
const CRITERIAS_URL: &str = "http://localhost:1984/criterias";
const DATA_URL: &str = "http://localhost:1984/params";

#[function_component]
fn App() -> Html {
    let jobs_state: UseStateHandle<Vec<Job>> = use_state(|| [].to_vec());

    let parameter_state = use_state(|| [].to_vec());
    let criterias_state = use_state(|| [].to_vec());
    let base_wages_state = use_state(|| 1000);
    let input_base_wages_state = (*base_wages_state).clone();
    let result_state = use_state(|| 0.0);
    let jobs_box_state: UseStateHandle<Vec<String>> = use_state(|| [].to_vec());
    let selected_job_state = use_state(|| "".to_string());

    let result_clone_2 = result_state.clone();
    let result_clone_3 = result_state.clone();
    let result_clone_4 = result_state.clone();

    let criterias_state_2 = criterias_state.clone();
    let criterias_state_3 = criterias_state.clone();
    let criterias_state_4 = criterias_state.clone();
    let criterias_state_5 = criterias_state.clone();
    let criterias_state_6 = criterias_state.clone();
    let criterias_state_7 = criterias_state.clone();

    let result_state_2 = result_state.clone();
    let result_state_3 = result_state.clone();

    let parameter_state_2 = parameter_state.clone();
    let parameter_state_3 = parameter_state.clone();
    let parameter_state_4 = parameter_state.clone();
    let parameter_state_5 = parameter_state.clone();
    let parameter_state_6 = parameter_state.clone();
    let parameter_state_7 = parameter_state.clone();

    let jobs_box_state_2 = jobs_box_state.clone();

    let selected_job_state_2 = selected_job_state.clone();

    let jobs_state_2 = jobs_state.clone();

    use_effect(move || {
        if (*criterias_state).len() == 0 {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_criteria: Vec<Criteria> = Request::get(CRITERIAS_URL)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                criterias_state.set(fetched_criteria.clone());
                result_clone_3.set(compute_result(
                    (*parameter_state_5).clone(),
                    fetched_criteria.clone(),
                    input_base_wages_state.clone(),
                ));
            });
            || ()
        } else {
            || ()
        }
    });

    use_effect(move || {
        if (*parameter_state_6).len() == 0 {
            wasm_bindgen_futures::spawn_local(async move {
                let jobs: Vec<Job> = Request::get(DATA_URL)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                if jobs.len() < 1 {
                    return;
                }
                jobs_state.set(jobs.clone());
                jobs_box_state.set(jobs.clone().into_iter().map(|job| job.job).collect());
                selected_job_state.set(jobs.first().unwrap().job.clone());
                parameter_state_6.set(jobs.first().unwrap().params.clone());
                result_clone_4.set(compute_result(
                    jobs.first().unwrap().params.clone(),
                    (*criterias_state_6).clone(),
                    input_base_wages_state.clone(),
                ));
            });
            || ()
        } else {
            || ()
        }
    });

    let on_param_value_slide: Callback<SliderMessage> = {
        Callback::from(move |msg: SliderMessage| {
            let mut state = (*parameter_state_2).clone();
            let param = state
                .iter_mut()
                .find(|param| param.id == msg.id)
                .expect(format!("param not found : {} ", msg.id).as_str());
            param.value = msg.value;
            parameter_state_2.set(state.to_vec());
            result_clone_2.set(compute_result(
                state.clone(),
                (*criterias_state_2).clone(),
                input_base_wages_state.clone(),
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
                crit_state.clone(),
                input_base_wages_state.clone(),
            ));
        })
    };

    let on_base_change = {
        let base_wages_handle = base_wages_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                let base_wages = input.value().parse::<i32>().expect("expected number");
                base_wages_handle.set(base_wages);
                result_state_3.set(compute_result(
                    (*parameter_state_4).clone(),
                    (*criterias_state_4).clone(),
                    base_wages.clone(),
                ));
            }
        })
    };

    let on_job_select = {
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                let new_job: Job = (*jobs_state_2)
                    .clone()
                    .into_iter()
                    .find(|job| job.job == input.value())
                    .expect(format!("param not found : {} ", input.value()).as_str());
                log::info!("input {:?}", new_job);
                parameter_state_7.set(new_job.params.clone());
                criterias_state_7.set((*criterias_state_7).clone());
            }
        })
    };

    html! {
        <div class="container">
            <Header />
            <div class="w3-row firstRow">
                <div class="w3-half contentBlock">
                    <form class="w3-container">
                        <label>{"Base wages : "}</label>
                        <input class="w3-border w3-round-large parameterTextInput"
                          type="number"
                          value={input_base_wages_state.to_string()}
                          oninput={on_base_change} />
                        <label>{"€"}</label>
                        <select name="job" onchange={on_job_select}>
                            {
                                for (*jobs_box_state_2).clone().into_iter().map(|job: String| {
                                    html! {
                                        <option selected={job.clone() == (*selected_job_state_2)} value={job.clone()}>{ job.clone() }</option>
                                    }
                                })
                            }
                        </select>
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
                if (*criterias_state_5).len() == (*parameter_state).len() {
                    <div>
                    {
                        for (*criterias_state_5).clone().into_iter().map(|criteria: Criteria| {
                            let param = (*parameter_state).clone().into_iter().find(|param| param.id == criteria.id).unwrap();
                            html! {
                                <div>
                                  <Slider on_parameter_slide={on_param_value_slide.clone()} on_coef_slide={on_coef_slide.clone()} wages_param={param} criteria={criteria} />
                                </div>
                            }
                        })
                    }
                    </div>
                } else {
                    <div>{"loading or data mismatch somewhere..."}</div>
                }
            </div>
        </div>
    }
}

fn compute_result(state: Vec<WagesParam>, criterias: Vec<Criteria>, base: i32) -> f64 {
    if criterias.len() == 0 {
        return 0.0;
    }
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
