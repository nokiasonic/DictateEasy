#![allow(dead_code, unused, unused_imports)]
use yew::prelude::*;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};
use gloo::timers::callback::Interval;
use gloo::file::callbacks::FileReader;
use gloo::file::File;
use regex::Regex;
use std::collections::HashMap;

//use gloo::console::{self, Timer};
//use crate::service::http::HttpService;

#[derive(Clone, Debug, Default)]
struct FileDetails {
    name: String,
    file_type: String,
    data: String,
}

pub enum Msg {
    CreateWordListFromFile,
    CreateWordlistFromInput(String),
    UpdateParaNumber(String),
    UpdateParaInterval(String),
    UpdateParaAccent(String),
    ShowModal,
    HideModal,
    Dictate,
    PlaySound(usize),
    Finished,
    Check,
    Files(Vec<File>),
    Loaded(String, String, String),
    Preview,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Status {
    Inputing,
    Preview,
    Ready,
    Dictating,
    Finished,
    Checking,
}

#[derive(PartialEq, Eq, Debug, Default)]
pub struct Para{
    para_number: usize,
    para_interval: u32,
    para_accent: usize,
}

impl Para{
    pub fn default() -> Self {
        Self {
            para_number: 2,
            para_interval: 3,
            para_accent: 2,
        }
    }
    pub fn new(para_number: usize, para_interval: u32, para_accent: usize) -> Self {
        Self {
            para_number,
            para_interval,
            para_accent,
        }
    }
}


pub struct Dictation{
    toggle_modal: bool,
    status: Status,
    wordlist: Vec<String>,
    currentnumber: usize,
    progress_pct: f32,
    para: Para,
    dictations: Html,
    _interval: Option<Interval>,
    readers: HashMap<String, FileReader>,
    files: Vec<FileDetails>,
    preview_content: String,
}

impl Component for Dictation{
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {

        Self {
            toggle_modal: false,
            status: Status::Inputing,
            wordlist: vec!["".to_string()],
            currentnumber: 1,
            progress_pct: 0.0,
            para: Para::default(),
            dictations: html! {},
            _interval: None,
            readers: HashMap::default(),
            files: Vec::default(),
            preview_content: "".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CreateWordListFromFile => {
                let mut total_string = "".to_string();
                //log::debug!("{:#?}", self.files);
                self.files.iter().for_each(|file| {
                     total_string.push_str((file.data.clone() + "\r\n").as_str());
                });
                log::debug!("{:#?}", total_string);
                let re = Regex::new(r"\r\n|\n").unwrap();
                self.wordlist = re.split(total_string.trim()).map(String::from).collect::<Vec<_>>();
                /* self.wordlist= total_string
                    .trim()
                    .split("\r\n")
                    .map(String::from)
                    .collect::<Vec<_>>();  */
                log::debug!("{:#?}", self.wordlist);
                self.status = Status::Ready;

                true
            },
            Msg::CreateWordlistFromInput(value) => {
                
                self.wordlist= value
                    .split("\n")
                    .map(String::from)
                    .collect::<Vec<_>>(); 
                //log::debug!("{:#?}", self.wordlist);
                self.status = Status::Ready;
                true
            },
            Msg::ShowModal => {
                self.toggle_modal = true;
                //self._interval.cancel();
                true
            },
            Msg::HideModal => {
                self.toggle_modal = false;
                self.preview_content = "".to_string();
                /*self.status = Status::Inputing;   
                self.preview_content = "".to_string(); */
                if self.status == Status::Preview {
                    self.status = Status::Inputing;
                } 
               
                //log::debug!("{:#?}",self.status);
                true
            },
            Msg::UpdateParaNumber(value) => {
                self.para.para_number = value.parse::<usize>().unwrap();
                true
            },

            Msg::UpdateParaInterval(value) => {
                self.para.para_interval = value.parse::<u32>().unwrap();
                true
            },

            Msg::UpdateParaAccent(value) => {
                self.para.para_accent = value.parse::<usize>().unwrap();
                false
            },

            Msg::Dictate =>{
                let mut number = 0;
                let link = ctx.link().clone();

                /* if let Some(interval) = self._interval.take() {
                    interval.cancel();
                }  */

                log::info!("dictate begins....");
                log::info!("para_number:{}, para_interval:{}, para_accent:{}", self.para.para_number, self.para.para_interval, self.para.para_accent);
                link.send_message(Msg::PlaySound(number));
                self._interval = Some(Interval::new(1000 * self.para.para_interval , move || {
                    number = number+1;                     
                    //log::info!("number:{}",number);                  
                    link.send_message(Msg::PlaySound(number));                                
                })); 
                 
                true
            },
            Msg::PlaySound(number) => {  
                let link = ctx.link().clone();
                let wordlist_len = self.wordlist.len();
                self.currentnumber = number/self.para.para_number+1;
                if number == wordlist_len * self.para.para_number   {                    
                    link.send_message(Msg::Finished); 
                    return false;
                }; 
                log::info!("number in Msg::PlaySound:{}",number/self.para.para_number+1);                 
                log::info!("play sound of {} word:{}",number/self.para.para_number+1,self.wordlist[number/self.para.para_number]);   
                self.progress_pct = (number+1) as f32 / (wordlist_len*self.para.para_number) as f32 * 100.0;               
                let url = format!("https://dict.youdao.com/dictvoice?audio={}&type={}",self.wordlist[number/self.para.para_number],self.para.para_accent);
                self.dictations = 
                html! {
                        <p key= {number}><audio id="audio" autoplay=true src={url.clone()}></audio> </p>
                }; 

                //log::info!("url:{:#?}",format!("<audio id=\"audio\" autoplay=true src={}></audio>",url.clone())); 
                
                self.status = Status::Dictating;                         
                true
            },
            Msg::Finished => {
                if let Some(interval) = self._interval.take() {
                    interval.cancel();}
                self.status = Status::Finished;
                true
            },
            Msg::Check => {
                log::info!("check");
                self.status = Status::Checking;
                true
            },
            Msg::Files(files) => {
                self.files = Vec::new();
                for file in files.into_iter() {
                    let file_name = file.name();
                    let file_type = file.raw_mime_type();
                    //log::debug!("file: {}, type: {}", file_name, file_type);

                    let task = {
                        let link = ctx.link().clone();
                        let file_name = file_name.clone();

                        gloo::file::callbacks::read_as_text(&file, move |res| {
                            link.send_message(Msg::Loaded(
                                file_name,
                                file_type,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                    //log::debug!("Readers: {:#?}", self.readers);
                }
                true
            },
            Msg::Loaded(file_name, file_type, data) => {
                let link = ctx.link().clone();
                //self.files = Vec::default();
                //self.preview_content = "".to_string();
                self.files.push(FileDetails {
                    data,
                    file_type,
                    name: file_name.clone(),
                });
                //self.files.iter().for_each(|file| log::info!("file data: {:#?}", file.data));

                self.readers.remove(&file_name);
                self.status = Status::Preview;
                self.toggle_modal = true;
                link.send_message(Msg::Preview);
                //log::info!("Readers in loaded: {:#?}", self.readers);
                //log::info!("Toggle Modal: {:#?}", self.toggle_modal);
                true
            },
            Msg::Preview => {
                self.files.iter().for_each(|file| {
                    //log::info!("file data: {:#?}", file.data);
                    self.preview_content.push_str((file.name.clone() + "\r\n").as_str());
                    self.preview_content.push_str((file.data.clone() + "\r\n").as_str());
                    //log::debug!("iter Status for preview content: {:#?}", self.preview_content);
                });

                //log::debug!("Status for preview content: {:#?}", self.preview_content);
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();   

        let oninput = link.callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            //log::info!("Input value: {}", value);
            Msg::CreateWordlistFromInput(value)
        });

        let show_modal = link.callback(move |_e:MouseEvent| {
            Msg::ShowModal
        });


        let hide_modal = link.callback(move |_e:MouseEvent| {
            Msg::HideModal
        }); 

        let oninput_number = link.callback(move |e:InputEvent| {   
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            log::info!("Input number value: {}", &value);                  
            Msg::UpdateParaNumber(value)
        });

        let oninput_interval = link.callback(move |e:InputEvent| {   
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            log::info!("Input interval value: {}", value);                  
            Msg::UpdateParaInterval(value)
        });

        let oninput_accent1 = link.callback(move |e:MouseEvent| {   
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            log::info!("Input accent1 value: {}", value);  
            let res = match value.as_str(){
                "英音" => 1,
                "美音" => 2,
                _ => 2,
            };                
            Msg::UpdateParaAccent(res.to_string())
        });

        let oninput_accent2 = link.callback(move |e:MouseEvent| {   
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            log::info!("Input accent2 value: {}", value);                  
            let res = match value.as_str(){
                "英音" => 1,
                "美音" => 2,
                _ => 2,
            };                
            Msg::UpdateParaAccent(res.to_string())
        });

        let onfileupload = link.callback(move |e:Event| {
            
            let input: HtmlInputElement = e.target_unchecked_into();
            Self::upload_files(input.files()) 
        });
        
        let ondictate = link.callback(move |_e:MouseEvent| {                                
            Msg::Dictate
        });

        /* let oncheck = link.callback(move |_e:MouseEvent| {                                
            Msg::Check
        }); */

        //let check_content = self.wordlist.clone();

        // Conditional rendering of the modal
    
        if self.toggle_modal {
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
                                    <p class="level-item"><button class="delete is-small" aria-label="close" onclick={hide_modal}></button></p>
                                </div>
                            </nav>
                            // <!-- Any other Bulma elements you want -->
                                if self.status == Status::Dictating {
                                    <p class="has-text-centered is-success">{ format!("听写进行中({}/{})...",self.currentnumber, self.wordlist.len()) }</p>
                                    <progress class="progress is-primary" value={self.progress_pct.to_string()} max="100"> </progress>
                                    {self.dictations.clone()}
                                }                                
                                else if self.status == Status::Finished {
                                    <p class="has-text-centered is-success">{ "听写完成！！！" }</p>
                                    //<p class="level-item my-2"><a class="button is-success"  onclick = {oncheck}>{"检查批改"}</a></p>    
                                }
                                else if self.status == Status::Preview {
                                    //<p class="has-text-centered has-text-success">{ for self.files.iter().map(Self::view_file) }</p>
                                    {self.view_file(ctx)}

                                }
                                else if self.status == Status::Checking {
                                    //<p class="has-text-centered is-success">{ check_content }</p>
                                    <crate::components::check::Word />
                                }
                                else if self.status == Status::Ready{                            
                                    <p class="has-text-centered is-success">{ "单词检查正确，准备开始听写，点击“开始听写”按钮进行听写。" }</p>
                                    
                                    <nav class="level">
                                        <div class="level-left my-2">
                                            <div class="level-item">  
                                                    <p class="level-item my-2">{"单词播放次数  "}</p>
                                                    <p><input class="input is-small" type="text" maxlength="1" style="width:30px" placeholder="2" oninput={oninput_number}/> </p>         
                                            </div>
                                            <div class="level-item">  
                                                    <p class="level-item my-2">{"单词播放间隔时间（秒）"}</p>
                                                    <input class="input is-small" type="text" maxlength="2" style="width:30px" placeholder="3" oninput={oninput_interval}/>            
                                            </div>
                                                                                                                            
                                        </div>
                                        <div class="level-right my-2">
                                            <div class="radios" >
                                                <label class="radio">
                                                    <input type="radio" name="answer" value="英音" onclick={oninput_accent1} /> {"英音"}
                                                </label>
                                                <label class="radio">
                                                    <input type="radio" name="answer" value="美音" onclick={oninput_accent2} checked=true /> {"美音"}
                                                </label>
                                            </div>
                                        </div>
                                    </nav>
                                    <p class="level-item my-2">
                                        <a class="button is-success"  onclick = {ondictate}>{"开始听写"}</a>
                                    </p>
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
                                                    <input class="file-input" type="file" multiple=true onchange={onfileupload}/>
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
                                        <p class="level-item"><a class="button is-success"  onclick = {show_modal}>{"确定"}</a></p>
                                        
                                    </div>
                                </nav> 
                            </section>  
                        </div>
                    </section>
                </>
            }
        }
    }
    
    
}

impl Dictation {
    fn upload_files(files: Option<FileList>) -> Msg {
        let mut result = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }
        //log::info!("{:?}",result);

        Msg::Files(result)
    }

    fn view_file(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();  
        let files = self.files.clone();
        let onclick = link.callback(move |_e:MouseEvent| {                                
            Msg::CreateWordListFromFile
        });
        //log::debug!("{:#?}",files);
        //log::debug!("{:#?}",self.wordlist);
        html! {
            <div class="preview-tile">
               {
                files.into_iter().map( |file| {
                        html! {
                            <div class="preview-tile">
                                <p class="preview-name has-text-success">{ format!("文件名：{}", file.name) }</p>
                                <div>
                                    {file.data}
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
               }

                <div class="my-2">
                    <a class="button is-success"  onclick = {onclick} >{"检查正确"}</a>
                </div>

            </div>
        }

    }

    fn view_test(&self) -> Html {
        let names = vec!["Sam","Bob","Ray"];

        html! {
            <div id="introductions">
                {
                    names.into_iter().map(|name| {
                        html!{<div key={name}>{ format!("Hello, I'am {}!",name) }</div>}
                    }).collect::<Html>()
                }
            </div>
        }
    }
}
