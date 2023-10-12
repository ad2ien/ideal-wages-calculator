use log::info;
use salary_param::SalaryParam;
use slider::Slider;
use yew::prelude::*;

mod salary_param;
mod slider;

const DATA: [&'static SalaryParam; 3] = [
    &SalaryParam {
        id: "salary",
        label: "Salary",
        coefficient: 1.0,
        value: 50,
    },
    &SalaryParam {
        id: "bonus",
        label: "Bonus",
        coefficient: 0.5,
        value: 100,
    },
    &SalaryParam {
        id: "tax",
        label: "Tax",
        coefficient: 0.2,
        value: 100,
    },
];

#[function_component]
fn App() -> Html {
    html! {
        <div class="container">
            <div class="parameters">
            { for DATA.into_iter().map(|param| {
                let on_slide: Callback<i8> = Callback::from(move |value: i8| {
                    info!("Value {:?}", value);
                });
                html! {
                    <div>
                        <Slider on_slide={on_slide} salary_param={*param} />
                    </div>
                }
            })}
            </div>
            <div class="result">
                <span>{"Result :"}</span>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}
