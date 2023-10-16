use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{criterias::Criteria, wages_param::WagesParam};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_parameter_slide: Callback<SliderMessage>,
    pub on_coef_slide: Callback<SliderCoefMessage>,
    pub wages_param: WagesParam,
    pub criteria: Criteria,
}

#[derive(Debug)]
pub struct SliderMessage {
    pub id: String,
    pub value: i8,
}

pub struct SliderCoefMessage {
    pub id: String,
    pub coef: f64,
}

#[function_component]
pub fn Slider(props: &Props) -> Html {
    let input_value_handle = use_state(|| props.wages_param.value.to_string());
    let coef_value_handle = use_state(|| props.criteria.coefficient.to_string());
    let prop_val = props.wages_param.value.to_string();
    let coef_value = (*coef_value_handle).clone();

    let wages_param = props.wages_param.clone();
    let wages_param_2 = props.wages_param.clone();
    let criteria = props.criteria.clone();

    let cb_value_handle = props.on_parameter_slide.clone();
    let on_value_change = {
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                input_value_handle.set(input.value());
                cb_value_handle.emit(SliderMessage {
                    id: wages_param.id.to_string(),
                    value: input.value().parse::<i8>().expect("expected number"),
                });
            }
        })
    };

    let cb_coef_handle: Callback<SliderCoefMessage> = props.on_coef_slide.clone();
    let on_coef_change = {
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                coef_value_handle.set(input.value());
                cb_coef_handle.emit(SliderCoefMessage {
                    id: wages_param_2.id.to_string(),
                    coef: input.value().parse::<f64>().expect("expected number"),
                });
            }
        })
    };

    html! {
        <div class="w3-row">
            <div class="w3-half w3-container">{  criteria.label }</div>
            <div class="sliderDiv w3-quarter">
                <input type="range" min="0" max="100" value={prop_val.clone()} oninput={on_value_change} />
                <div class="sliderValue">{ prop_val.clone() }</div>
            </div>
            <div class="sliderDiv w3-quarter">
                <input class="coefSlider" type="range" min="-2" max="2" step="0.1" value={coef_value.clone()} oninput={on_coef_change} />
                <div class="sliderValue">{ coef_value }</div>
            </div>
        </div>
    }
}
