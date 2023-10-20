use yew::{function_component, html, Html};

#[function_component]
pub fn Header() -> Html {
    html! {
        <div class="title">
            <h1 class="w3-tooltip">
                {"Ideal Wages Calculator 💸"}
                <span class="w3-text w3-tag w3-round-xlarge tooltip">
                    { "(says me)" }
                </span>
            </h1>
            <h2>{"🫵 How much should I earn?"}</h2>
        </div>
    }
}
