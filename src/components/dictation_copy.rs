use yew::prelude::*;
use web_sys::{HtmlInputElement, InputEvent};
//use crate::service::http::HttpService;




#[function_component(Dictation)]
pub fn word_input() -> Html {

    let content = use_state(||String::from(""));
    let dictate_flag = use_state(|| 0);
    let wordlistvec = use_state(||Vec::<String>::new());
    let oninput = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            log::info!("{}",input.value());
            content.set(input.value());
        })
    };

    let show_modal = use_state(|| false);
    let toggle_modal = {
        let content = content.clone();
        let show_modal = show_modal.clone();
        //log::debug!("content:{:#?}", content); 
        Callback::from(move |e:MouseEvent| {
            e.prevent_default(); /* Prevent event propagation */
            let info = content.clone();
            log::debug!("content:{}", info.to_string());
            show_modal.set(!*show_modal);
        })
    }; 

    log::debug!("show_modal——state:{:#?}", show_modal); 
    let onclick_close = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| show_modal.set(false))    
    };

    
    let begin_dictate = {
        let content = content.clone();
        let dictate_flag = dictate_flag.clone();
        let wordlistvec = wordlistvec.clone();
        Callback::from(move |_| {
            let owned_content = content.to_string();
            let wordlist= owned_content
                .split("\n")
                .map(String::from)
                .collect::<Vec<_>>();
                //.collect::<Vec<_>>();
                //.iter()
                //.map(|s| s.to_string())
                //.collect::<Vec<_>>();   
            log::info!("word lists in callback:{:#?}", wordlist);        
            dictate_flag.set(1);
            wordlistvec.set(wordlist);
        })
    };

    // handle src url
    log::debug!("word lists:{:#?}", wordlistvec);
    let dictations = wordlistvec.iter().map(|word| {
        log::debug!("word:{}", word);
        let url = format!("http://dict.youdao.com/dictvoice?audio={}&type=1",word);
        log::debug!("url:{}", url);
        html! {
            <audio id="audio" autoplay=true src={url}></audio>
        }                                    
    });

    log::info!("dication lists:{:#?}", dictations);

    // Conditional rendering of the modal
   
    if *show_modal {
        html! {
            <>
                <div id="modal" class="modal is-active" style = "align-items: center;">
                    <div class="modal-background"></div>
                    <div class="modal-content">
                        <div class="box has-background-grey-dark">  
                        <nav class="level">
                            <div class="level-left">  
                                 <p class="level-item"></p>
                            </div>
                            <div class="level-right">  
                                 <p class="level-item"><button class="delete is-small" aria-label="close" onclick={onclick_close}></button></p>
                            </div>
                        </nav>
                        // <!-- Any other Bulma elements you want -->
                            if *dictate_flag == 1 {
                                <p class="has-text-centered is-success">{ "听写进行中..." }</p>
   
                                {for dictations} 
                            
                                /* {
                                    for wordlistvec.iter().map(|word| {
                                        let url = format!("http://dict.youdao.com/dictvoice?audio={}&type=1",word);
                                        log::debug!("url:{}", url);
                                        html! {
                                            <audio id="audio" autoplay=true src={url}></audio>
                                        }                                    
                                    })
                                } */
                                /* {
                                html!{
                                    <audio id="audio" autoplay=true src="http://dict.youdao.com/dictvoice?audio=hello&type=1"></audio>
                                }} */
                                    
                            }                     
                            else {                            
                            <p class="has-text-centered is-success">{ "单词检查正确，准备开始听写，点击“开始听写”按钮进行听写。" }</p>
                            <p class="level-item my-2"><a class="button is-success"  onclick = {begin_dictate}>{"开始听写"}</a></p>
                            }
                        </div>
                    </div>
                    
                </div>
            </>

        }
    } else { 
        html! {
            <>
                <section class="section my-0 py-2">        
                    <div class="container">
                        <textarea class="textarea is-primary" placeholder="输入需要听写的英语单词" rows="10" style ="resize: none" oninput={oninput}></textarea>               
                        <section class="section my-0 py-2">
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
                                    <p class="level-item"><a class="button is-success"  onclick = {toggle_modal}>{"确定"}</a></p>
                                    
                                </div>
                            </nav> 
                        </section>  
                    </div>
                </section>
            </>
        }
    }
  

    
    
}




