use yew::prelude::*;

#[function_component(Card)]
pub fn card() -> Html {
    html! {
       <div class="card">
            <div class="card-header">
            <img class="plus" src="/assets/PLUS.svg" alt="plus"/>
            </div>
                <h2>{"Card Header"}</h2>
                <p>{"Card Content"}</p>

        </div>
    }
}
