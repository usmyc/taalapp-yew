mod leftbar;
mod navbar;
use navbar::People;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let esra = People {
        name: "Zelda".to_string(),
        age: 20,
    };

    html! {
        <div>
        <navbar::Nav esra={esra} />
        <leftbar::Leftbar />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
