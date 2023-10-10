use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_slide: Callback<i8>,
    pub default_value: i8,
}

#[function_component]
pub fn Slider(props: &Props) -> Html {
    let input_value_handle = use_state(||props.default_value.to_string());
    let input_value = (*input_value_handle).clone();
    let cb_handle = props.on_slide.clone();

    let on_change = {

        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            
            if let Some(input) = input {
                input_value_handle.set(input.value());
                cb_handle.emit(input.value().parse::<i8>().expect("expected number"));
            }
        })
    };
    

    html! {
        <>
            <input type="range" min="1" max="100" value={input_value.clone()} oninput={on_change} />
            { input_value }
        </>
    }
}
