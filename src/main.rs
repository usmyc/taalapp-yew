mod card;
mod leftbar;
mod navbar;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
        <navbar::Nav />
        <div class="container">
            <img class="main-plus" src="/assets/PLUS.svg" alt="plus"/>
            <card::Card />
        </div>
        <leftbar::Leftbar />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
