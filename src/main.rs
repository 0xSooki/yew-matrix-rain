use matrix_rain::MatrixRain;

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <MatrixRain />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
