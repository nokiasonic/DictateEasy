use yew::{function_component, Html, html};

#[function_component]
pub fn Description() -> Html {
   html!{
    <>
        <section class="section pb-2">
            <h1 class="title has-text-primary">{"听写易"}</h1>
            <h2 class="subtitle">
            {"这是一个用于英语单词听写的应用，用“听写易”还算“挺写意”。"}
            </h2>
    
            <div class="container">
                <div class="notification is-primary is-size-6" >
                    {"可以通过自行输入或从文本导入需要听写的英语单词。输入时每行写一个单词，一行一个单词。从文本导入的单词也遵循同样的格式。"}<br />
                    {"输入示例:"}<br />
                    {"hello"}<br />
                    {"world"}<br />
                    {"friend"}<br />
                    {"..."}
                </div>
                //<input type="text" name={props.content.clone()} onchange={call_to_action} />  
                //<textarea class="textarea is-primary" placeholder="输入需要听写的英语单词" rows="10" name={props.content.clone()} onchange={call_to_action}></textarea>
            </div>
        </section>
    </>
   } 
}
