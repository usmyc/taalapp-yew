use gloo::console::log;
use yew::prelude::*;
use web_sys::HtmlInputElement;
mod event;

#[derive(Clone, Properties, PartialEq)]
struct EventProps {
    name: String,
    age: u32,
}

#[function_component(Card)]
pub fn card() -> Html {
    let  event_vec = use_state(|| 
        EventProps {
            name: String::from("name"),
            age: 25,
        });
    
    let age = use_state(|| event_vec.age);
    let name = use_state(|| event_vec.name.clone());
    fn new_age(age: u32) -> u32 {
        age+1
    }
    let name_node_ref = use_node_ref();
    let age_node_ref = use_node_ref();
 

    let onclick = {
        let age_in = age.clone();
        let name_in = name.clone();
       
        let name_node_ref = name_node_ref.clone();
        let age_node_ref = age_node_ref.clone();
       
        Callback::from(move |_| {
            let name = name_node_ref.cast::<HtmlInputElement>();
            let age = age_node_ref.cast::<HtmlInputElement>();
       
            if let Some(name) = name{
                 name_in.set(name.value())            }
            if let Some(age) = age{
             age_in.set(age.value().parse::<u32>().unwrap_or(0))
            }
   
            
        })
        
    };
   
  
    html! {
        <div class="card">
           <event::EventComponent name={String::from(&*name)} age={*age}/>
            <input ref={name_node_ref}/>
            <input ref={age_node_ref}/>
            <button onclick={onclick}>{"Click me"}</button>
            
        </div>
    }
}