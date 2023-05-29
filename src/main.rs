mod card;
mod event;
mod form;
mod leftbar;
mod navbar;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let name = String::from("name");

    html! {
        <div>
        <navbar::Nav />
        <div class="container">
            <img class="main-plus" src="/assets/PLUS.svg" alt="plus"/>
            // <card::Card new_name={name.clone()} />
            // <event::EventComponent />
            <form::FormComponent name={name} />
        </div>
        <leftbar::Leftbar />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
