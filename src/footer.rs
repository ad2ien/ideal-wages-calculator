use yew::{html, function_component, Html};

#[function_component]
pub fn Footer() -> Html {
    html! {
        <div class="contentBlock">
            <p>
            <span>{"Made with fun 🤓 and "}</span>
            <a href="https://yew.rs/">
            <img src="https://yew.rs/img/logo.svg " alt="yew" style="height: 2rem" />
               { " Yew " }
            </a> 
            <span>{ " and " } </span>
            <a href="https://trunkrs.dev/">
            <img src="https://trunkrs.dev/rustacean-flat-happy.svg" alt="yew" style="height: 2rem" />
               { " Trunk " }
            </a> 
            <span>{ " and " } </span>
            <a href="https://rust-lang.org/">
                <img src="https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white" alt="rust" />
            </a>
            <span>{ ". Here is the " } </span>
            <a href="https://github.com/ad2ien/ideal-wages-calculator">
                <img src="https://img.shields.io/badge/source_code-gray.svg?logo=github" alt="source code" />
            </a>
             </p>
        </div>
    }
}