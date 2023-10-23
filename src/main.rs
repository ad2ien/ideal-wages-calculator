use gloo_net::http::Request;
use wages_param::WagesParam;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement};
use yew::functional::use_effect;
use yew::prelude::*;

use crate::{
    criterias::Criteria, footer::Footer, header::Header, job::Job,
    job_sliders_component::JobSliders,
};

mod criterias;
mod footer;
mod header;
mod job;
mod job_sliders_component;
mod slider_component;
mod wages_param;

const PARAMETERS_MEANING: i8 = 10;
const DEFAULT_BASE_WAGES: i32 = 700;
const CRITERIAS_URL: &str = "https://raw.githubusercontent.com/ad2ien/ideal-wages-calculator/main/criterias.json";
const DATA_URL: &str = "https://raw.githubusercontent.com/ad2ien/ideal-wages-calculator/main/data.json";

#[function_component]
fn App() -> Html {
    let jobs_state: UseStateHandle<Vec<Job>> = use_state(|| [].to_vec());

    let parameter_state = use_state(|| [].to_vec());
    let criterias_state = use_state(|| [].to_vec());
    let base_wages_state = use_state(|| DEFAULT_BASE_WAGES);
    let input_base_wages_state = (*base_wages_state).clone();
    let result_state = use_state(|| 0.0);
    let jobs_box_state: UseStateHandle<Vec<String>> = use_state(|| [].to_vec());
    let selected_job_state = use_state(|| "".to_string());
    let error_msg_state = use_state(|| "".to_string());

    {
        let parameter_state = parameter_state.clone();
        let result_state = result_state.clone();
        let error_msg_state = error_msg_state.clone();
        let criterias_state = criterias_state.clone();
        use_effect(move || {
            if (*criterias_state).len() == 0 && (*error_msg_state).is_empty() {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_criteria: Vec<Criteria> =
                        match Request::get(CRITERIAS_URL).send().await {
                            Ok(response) => match response.json().await {
                                Ok(jobs) => jobs,
                                Err(_e) => {
                                    error_msg_state.set(format!("Error fetching criterias"));
                                    return;
                                }
                            },
                            Err(_e) => {
                                error_msg_state.set(format!("Error reading criterias"));
                                return;
                            }
                        };
                    criterias_state.set(fetched_criteria.clone());
                    result_state.set(compute_result(
                        (*parameter_state).clone(),
                        fetched_criteria.clone(),
                        input_base_wages_state.clone(),
                    ));
                });
                || ()
            } else {
                || ()
            }
        });
    }

    {
        let criterias_state = criterias_state.clone();
        let error_msg_state = error_msg_state.clone();
        let result_state = result_state.clone();
        let parameter_state = parameter_state.clone();
        let jobs_state = jobs_state.clone();
        let selected_job_state = selected_job_state.clone();
        let jobs_box_state = jobs_box_state.clone();
        use_effect(move || {
            if (*parameter_state).len() == 0 && (*error_msg_state).is_empty() {
                wasm_bindgen_futures::spawn_local(async move {
                    let jobs: Vec<Job> = match Request::get(DATA_URL).send().await {
                        Ok(response) => match response.json().await {
                            Ok(jobs) => jobs,
                            Err(_e) => {
                                error_msg_state.set(format!("Error fetching data"));
                                return;
                            }
                        },
                        Err(_e) => {
                            error_msg_state.set(format!("Error reading data"));
                            return;
                        }
                    };

                    if jobs.len() < 1 {
                        error_msg_state.set(format!("Error no data fetched"));
                        return;
                    }
                    jobs_state.set(jobs.clone());
                    jobs_box_state.set(jobs.clone().into_iter().map(|job| job.job).collect());
                    selected_job_state.set(jobs.first().unwrap().job.clone());
                    parameter_state.set(jobs.first().unwrap().params.clone());
                    result_state.set(compute_result(
                        jobs.first().unwrap().params.clone(),
                        (*criterias_state).clone(),
                        input_base_wages_state.clone(),
                    ));
                });
                || ()
            } else {
                || ()
            }
        });
    }

    let on_param_value_slide: Callback<Vec<WagesParam>> = {
        let criterias_state = criterias_state.clone();
        let parameter_state = parameter_state.clone();
        let result_state = result_state.clone();
        Callback::from(move |new_params: Vec<WagesParam>| {
            parameter_state.set(new_params.clone());
            result_state.set(compute_result(
                new_params.clone(),
                (*criterias_state).clone(),
                input_base_wages_state.clone(),
            ));
        })
    };

    let on_coef_slide: Callback<Vec<Criteria>> = {
        let criterias_state = criterias_state.clone();
        let parameter_state = parameter_state.clone();
        let result_state = result_state.clone();
        Callback::from(move |new_coef: Vec<Criteria>| {
            criterias_state.set(new_coef.clone());
            result_state.set(compute_result(
                (*parameter_state).clone(),
                new_coef.clone(),
                input_base_wages_state.clone(),
            ));
        })
    };

    let on_base_change = {
        let base_wages_handle = base_wages_state.clone();
        let result_state = result_state.clone();
        let parameter_state = parameter_state.clone();
        let criterias_state = criterias_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                let base_wages = input.value().parse::<i32>().expect("expected number");
                base_wages_handle.set(base_wages);
                result_state.set(compute_result(
                    (*parameter_state).clone(),
                    (*criterias_state).clone(),
                    base_wages.clone(),
                ));
            }
        })
    };

    let on_job_select = {
        let result_state = result_state.clone();
        let parameter_state = parameter_state.clone();
        let criterias_state = criterias_state.clone();
        let selected_job_state = selected_job_state.clone();
        let jobs_state = jobs_state.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                let new_job: Job = (*jobs_state)
                    .clone()
                    .into_iter()
                    .find(|job| job.job == input.value())
                    .expect(format!("param not found : {} ", input.value()).as_str());
                selected_job_state.set(new_job.job.clone());
                parameter_state.set(new_job.params.clone());
                result_state.set(compute_result(
                    new_job.params.clone(),
                    (*criterias_state).clone(),
                    input_base_wages_state.clone(),
                ));
            }
        })
    };

    html! {
        <div class="container">
            <Header />
            <div class={ format!("contentBlock w3-amber {}", if (*error_msg_state).clone().is_empty() { "hidden" } else { "" }) }>
                { (*error_msg_state).clone() }
            </div>
            <div class="w3-row firstRow">
                <div class="w3-third contentBlock">
                    <form class="w3-container">
                        <label class="w3-tooltip">
                            <span class="w3-text w3-tag w3-round-xlarge tooltip">
                                { "Maybe as human we should be able to survive whatever you do in life" }
                            </span>
                            {"Base wages : "}
                        </label>
                        <input class="w3-border w3-round-large parameterTextInput"
                          type="number"
                          value={input_base_wages_state.to_string()}
                          oninput={on_base_change} />
                        <label>{"€"}</label>
                    </form>
                </div>
                <div class="w3-third contentBlock">
                    <label>{ "Profile : "}</label>
                    <select class="w3-select" name="job" onchange={on_job_select}>
                        {
                            for (*jobs_box_state).clone().into_iter().map(|job: String| {
                                html! {
                                    <option selected={job.clone() == (*selected_job_state)} value={job.clone()}>{ job.clone() }</option>
                                }
                            })
                        }
                    </select>
                </div>
                <div class="w3-half contentBlock">
                        <span>{"You should get : "}</span>
                        <br/>
                        <span class="result"> {result_state.to_string()}{"€"}</span>
                </div>
            </div>
            <JobSliders
                wages_param={(*parameter_state).clone()}
                criterias={(*criterias_state).clone()}
                on_parameter_slide={on_param_value_slide.clone()}
                on_coef_slide={on_coef_slide.clone()} />
            <Footer />
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
