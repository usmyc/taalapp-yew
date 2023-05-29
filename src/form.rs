use yew::prelude::*;

#[derive(Properties, PartialEq)]

pub struct Props {
    pub name: String,
}

#[function_component(FormComponent)]
pub fn form_component(props: &Props) -> Html {
    html! {
      <input type ="text" name={props.name.clone()} />
    }
}
