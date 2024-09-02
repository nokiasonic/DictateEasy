#![allow(dead_code, unused, unused_imports)]
use yew::prelude::*;


pub struct Word{
    pronouce: Html,
    meaning: Html,
}


impl Component for Word{
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        Self {
            pronouce: html!{},
            meaning: html!{},
        }        
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true   
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();     

        html! {


            <div class="card">
                <div class="card-image">
                    <figure class="image is-4by3">
                    <img
                        src="https://bulma.io/assets/images/placeholders/1280x960.png"
                        alt="Placeholder image"
                    />
                    </figure>
                </div>
                <div class="card-content">
                    <div class="media">
                    <div class="media-left">
                        <figure class="image is-48x48">
                        <img
                            src="https://bulma.io/assets/images/placeholders/96x96.png"
                            alt="Placeholder image"
                        />
                        </figure>
                    </div>
                    <div class="media-content">
                        <p class="title is-4">{"John Smith"}</p>
                        <p class="subtitle is-6">{"@johnsmith"}</p>
                    </div>
                    </div>

                    <div class="content">
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec
                    iaculis mauris."} <a>{"@bulmaio"}</a><a href="#">{"#css"}</a>
                    <a href="#">{"#responsive"}</a>
                    <br />
                    <time datetime="2016-1-1">{"11:09 PM - 1 Jan 2016"}</time>
                    </div>
                </div>
            </div>
        }
    } 
}
    
    

