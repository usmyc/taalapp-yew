use yew::prelude::*;

struct Menu {
    profil: String,
    settings: String,
    subscription: String,
    logout: String,
    faq: String,
}

#[function_component(Usermenu)]
pub fn usermenu() -> Html {
    let menu = Menu {
        profil: "Profil".to_string(),
        settings: "Settings".to_string(),
        subscription: "Subscription".to_string(),
        logout: "Logout".to_string(),
        faq: "FAQ".to_string(),
    };

    html! {
        <div class="usermenu">
            <p>{"User Name"}</p>
            <hr class="line"/>
            <p>{menu.profil}</p>
            <p>{menu.settings}</p>
            <p>{menu.subscription}</p>
            <p>{menu.logout}</p>
            <p>{menu.faq}</p>
        </div>
    }
}
