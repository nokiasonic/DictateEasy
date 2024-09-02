use yew::{Properties, function_component, Html, html, Callback};
use wasm_bindgen::JsCast;
use web_sys::{Event,HtmlInputElement};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub content: String,
}

#[function_component]
pub fn TextArea(props: &Props) -> Html {
    let call_to_action = Callback::from(

        | input_event: Event | {
          let input_event_target = input_event.target().unwrap();
          // * Here, feature 'HtmlInputElement' is used
          let current_input_text = input_event_target.unchecked_into::<HtmlInputElement>();
          // * Simply log "current_input_text" for now..
          log::info!("{}",current_input_text.value());  

        }
        
    );
    log::info!("{}",props.content);

   html!{
    <>
        <section class="section my-0 py-2">        
            <div class="container">
                <textarea class="textarea is-primary" placeholder="输入需要听写的英语单词" rows="10" style ="resize: none" onchange={call_to_action}></textarea>
                <crate::components::buttons::ButtonConfirm />   
            </div>
        </section>
            

    </>
   } 
}

