use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::salary_param::SalaryParam;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_slide: Callback<SliderMessage>,
    pub salary_param: SalaryParam,
}

pub struct SliderMessage {
    pub id: String,
    pub value: i8,
    pub coef: f64,
}

#[function_component]
pub fn Slider(props: &Props) -> Html {
    let input_value_handle = use_state(|| props.salary_param.value.to_string());
    let coef_value_handle = use_state(|| props.salary_param.coefficient.to_string());
    let input_value = (*input_value_handle).clone();
    let coef_value = (*coef_value_handle).clone();

    let salary_param = props.salary_param.clone();
    
    let coef_for_value = coef_value.clone();
    let cb_value_handle = props.on_slide.clone();
    let on_value_change = {
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                input_value_handle.set(input.value());
                cb_value_handle.emit( SliderMessage { 
                    id: salary_param.id.to_string(),
                    value: input.value().parse::<i8>().expect("expected number"),
                    coef: coef_for_value.parse::<f64>().expect("expected number")
                 });
            }
        })
    };

    let input_value_for_coef = input_value.clone();
    let cb_coef_handle = props.on_slide.clone();
    let on_coef_change = {
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                coef_value_handle.set(input.value());
                cb_coef_handle.emit( SliderMessage { 
                    id: salary_param.id.to_string(),
                    value: input_value_for_coef.parse::<i8>().expect("expected number"),
                    coef: input.value().parse::<f64>().expect("expected number")
                 });
            }
        })
    };

    html! {
        <div class="parameter">
            <div class="sliderLabel">{  salary_param.label }</div>
            <div class="sliderDiv">
                <input type="range" min="0" max="100" value={input_value.clone()} oninput={on_value_change} />
                <div class="sliderValue">{ input_value }</div>
            </div>
            <div class="sliderDiv">
                <input type="range" min="-2" max="2" step="0.1" value={coef_value.clone()} oninput={on_coef_change} />
                <div class="sliderValue">{ coef_value }</div>
            </div>
        </div>
    }
}
