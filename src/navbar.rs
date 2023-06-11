use yew::prelude::*;
mod usermenu;

#[function_component(Nav)]
pub fn nav() -> Html {
    // Create a state variable with the initial value of `false`

    let visible = use_state(|| false);
    let onclick = {
        let visible = visible.clone();

        //careful about MouseEvent//

        Callback::from(move |_: MouseEvent| {
            visible.set(!*visible);
        })
    };

    // Clone the state value to use it
    let class = if *visible {
        "profile-menu visible"
    } else {
        "profile-menu"
    };
    html! {
       <div class="nav">
               <p>{"Nav"}</p>
                  <img class="usr-img" src="/assets/userimage.png" alt="userimage" onclick={onclick} />
                  <div class={class}>
                        <usermenu::Usermenu />
                  </div>
        </div>
    }
}
