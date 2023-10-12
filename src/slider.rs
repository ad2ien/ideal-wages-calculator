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
}

#[function_component]
pub fn Slider(props: &Props) -> Html {
    let input_value_handle = use_state(|| props.salary_param.value.to_string());
    let input_value = (*input_value_handle).clone();
    let cb_handle = props.on_slide.clone();
    let salary_param = props.salary_param.clone();

    let on_change = {
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                input_value_handle.set(input.value());
                cb_handle.emit( SliderMessage { 
                    id: salary_param.id.to_string(),
                    value: input.value().parse::<i8>().expect("expected number")
                 });
            }
        })
    };

    html! {
        <div class="parameter">
            <div class="sliderLabel">{  salary_param.label }</div>
            <div class="sliderDiv">
                <input type="range" min="1" max="100" value={input_value.clone()} oninput={on_change} />
                <div class="sliderValue">{ input_value }</div>
            </div>
        </div>
    }
}
