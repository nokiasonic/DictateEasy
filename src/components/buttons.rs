use yew::prelude::*;
use yew_hooks::prelude::*;
use web_sys::HtmlInputElement;

#[function_component]
pub fn ButtonConfirm() -> Html {
    let content = use_state(||String::from(""));
    let onclick_confirm = {
        let content = content.clone();
        log::info!("dictation content:{:#?}", content);
        Callback::from(move |e:SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            //content.run();
        })
    };

  html! {
    <section class="section  my-0 py-2">    
        <nav class="level">
            //<!-- Left side -->
            <div class="level-left">                
                <div class="level-item">
                    <div class="file">
                        <label class="file-label">
                            <input class="file-input" type="file" name="resume" />
                            <span class="file-cta">
                            <span class="file-icon">
                                <i class="fas fa-upload"></i>
                            </span>
                            <span class="file-label"> {"上传文件..."} </span>
                            </span>
                        </label>
                    </div> 
                </div>
            </div>
        
            //<!-- Right side -->
            <div class="level-right">                
                <p class="level-item"><a class="button is-success" type="submit">{"确定"}</a></p>
            </div>
        </nav>  
    </section>
    
  }
}