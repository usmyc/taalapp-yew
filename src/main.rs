mod leftbar;
mod navbar;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
        <navbar::Nav />
        <leftbar::Leftbar />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
