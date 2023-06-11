mod card;
mod leftbar;
mod navbar;
use yew::prelude::*;
#[function_component(App)]
fn app() -> Html {
    let card_list = use_state(|| 0);
    let card_listed = card_list.clone();
    let onclick = Callback::from(move |_| {
        card_listed.set(*card_listed + 1);
    });

    html! {
        <div>
        <navbar::Nav />
        <div class="container">
            <div class="hero">
                <h1>{"Learn with AI"}</h1>
            </div>
            <img onclick={onclick} class="main-plus" src="/assets/PLUS.svg" alt="plus"/>

            <div class="cards">
                   if *card_list < 4{
             {for (0..(*card_list)).map(|_| html!{<card::Card />})}
            }
             else{
              <p style="color:red">{"You can't add more than 4 cards"}</p>
              {for (0..4).map(|_| html!{<card::Card />})}
             }
            </div>


        </div>
        <leftbar::Leftbar />
        </div>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
