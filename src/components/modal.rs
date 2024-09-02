use yew::prelude::*;
use web_sys::HtmlInputElement;

#[function_component(Modal)]
pub fn Modal_show() -> Html {
  html! {
        <div class="modal">
            <div class="modal-background"></div>
            <div class="modal-content">
                <h2>{ "This is a modal" }</h2>
                <p>{ "Click outside to close." }</p >
            </div>
            <button class="modal-close is-large" aria-label="close"></button>
        </div>       

    
  }
}