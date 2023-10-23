use yew::prelude::*;

use crate::{
    criterias::Criteria,
    slider_component::{Slider, SliderCoefMessage, SliderMessage},
    wages_param::WagesParam,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub wages_param: Vec<WagesParam>,
    pub criterias: Vec<Criteria>,
    pub on_parameter_slide: Callback<Vec<WagesParam>>,
    pub on_coef_slide: Callback<Vec<Criteria>>,
}

#[function_component]
pub fn JobSliders(props: &Props) -> Html {
    let wages_param = props.wages_param.clone();
    let on_parameter_slide = props.on_parameter_slide.clone();
    let criterias = props.criterias.clone();
    let on_coef_slide: Callback<Vec<Criteria>> = props.on_coef_slide.clone();

    let on_param_value_slide: Callback<SliderMessage> = {
        Callback::from(move |msg: SliderMessage| {
            let mut new_params = wages_param.clone();
            let param = new_params
                .iter_mut()
                .find(|param| param.id == msg.id)
                .expect(format!("param not found : {} ", msg.id).as_str());
            param.value = msg.value;
            on_parameter_slide.emit(new_params);
        })
    };

    let on_coef_slide: Callback<SliderCoefMessage> = {
        Callback::from(move |msg: SliderCoefMessage| {
            let mut crit_state = criterias.clone();
            let criteria = crit_state
                .iter_mut()
                .find(|criteria| criteria.id == msg.id)
                .expect(format!("criteria not found : {} ", msg.id).as_str());
            criteria.coefficient = msg.coef;
            on_coef_slide.emit(crit_state);
        })
    };

    {
        let wages_param = props.wages_param.clone();
        html! {
            <div class="contentBlock">
            <div class="w3-row parameterHeader">
                <div class="w3-half">{ "Criteria" }</div>
                <div class="w3-quarter">{ "Mark" }</div>
                <div class="w3-quarter">{ "How it maters" }</div>
            </div>
            if props.criterias.len() == props.wages_param.len() {
                <div>
                {
                    for props.criterias.clone().into_iter().map(|criteria: Criteria| {
                        let param = wages_param.clone().into_iter().find(|param| param.id == criteria.id).unwrap();
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
        }
    }
}
