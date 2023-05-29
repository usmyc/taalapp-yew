
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub age: u32,
}

#[function_component]
pub fn EventComponent( props: &Props ) -> Html {
  

    html! {
    <div class="event">
       <p>{props.name.clone()}</p>
       <p>{props.age}</p>


    </div>
    }
}
