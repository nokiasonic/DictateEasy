#![recursion_limit = "256"]
use yew::prelude::*;


mod components;
//mod service;

#[function_component]
fn App() -> Html {
    //let time = Local::now();


    html! {
        <>
           <components::description::Description /> 
           <components::dictation::Dictation />    
     
        </>

    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
