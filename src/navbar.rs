use yew::prelude::*;
#[derive(Clone, PartialEq)]
pub struct People {
    pub name: String,
    pub age: u8,
}

#[derive(Properties, PartialEq)]
pub struct NavProps {
    pub esra: People,
}

#[function_component(Nav)]
pub fn nav(NavProps { esra }: &NavProps) -> Html {
    html! {
       <div class="nav">
               <p>{"Osman"}</p>
                  <img  style="display: none;" class="usr-img" src="/assets/userimage.png" alt="userimage" />
        </div>
    }
}
