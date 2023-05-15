use yew::prelude::*;
struct Menu {
    profil: String,
    settings: String,
    subscription: String,
    logout: String,
    faq: String,
}
#[function_component(Nav)]
pub fn nav() -> Html {
    let menu = Menu {
        profil: "Profil".to_string(),
        settings: "Settings".to_string(),
        subscription: "Subscription".to_string(),
        logout: "Logout".to_string(),
        faq: "FAQ".to_string(),
    };
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
    let class = if *visible.clone() {
        "profile-menu visible"
    } else {
        "profile-menu"
    };
    html! {
       <div class="nav">
               <p>{"Nav"}</p>
                  <img class="usr-img" src="/assets/userimage.png" alt="userimage" onclick={onclick} />
                  <div class={class}>
                    <p>{"User Name"}</p>
                    <hr class="line"/>
                    <p>{menu.profil}</p>
                    <p>{menu.settings}</p>
                    <p>{menu.subscription}</p>
                    <p>{menu.logout}</p>
                    <p>{menu.faq}</p>
                  </div>
        </div>
    }
}
