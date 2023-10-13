use yew::{html, function_component, Html};

#[function_component]
pub fn Header() -> Html {
    html! {
        <div class="title">
            <h1>{"Ideal Wages Calculator"}</h1>
            <h2>{"How much should you earn?"}</h2>
        </div>
    }
}