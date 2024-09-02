use yew::prelude::*;
use chrono::prelude::*;


pub struct Header;

/* impl Header {
    fn is_trade_time(dt: DateTime<Local>) -> bool {
        //unimplemented!();
        let time =  dt.naive_local().time(); 
        if (time > NaiveTime::from_hms(9, 29, 59) && time < NaiveTime::from_hms(11, 30, 01)) ||
           (time > NaiveTime::from_hms(12, 59, 59) && time < NaiveTime::from_hms(15, 00, 01)) {
            return true
        } else {
            return false
        } 
    }
} */

pub enum Msg {
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub time: DateTime<Local>,
}

impl Component for Header{
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self 
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let time = ctx.props().time.clone();
        html! {
            <header class="hero">
                <div class="hero-head">
                    <nav class="navbar has-shadow" role="navigation" aria-label="main navigation">
                        <div class="navbar-brand">
                            <p class="navbar-item is--brand">
                                <img class="navbar-brand-logo"
                                    src="logo.png"
                                    alt="Brand logo"
                                />
                            </p>                        
                        </div>

                        <div class="navbar-menu navbar-end" id="navMenu">   
                            <a class="navbar-item nav-tag">
                            <span class="tag is-success">{&time.format("%Y-%m-%d %H:%M:%S").to_string()}</span>
                            </a>
                            /* <a class="navbar-item nav-tag">
                                if is_trade_time(time) {
                                    <span class="tag is-success">{&time.format("%Y-%m-%d %H:%M:%S").to_string()}</span>
                                } else {
                                    <span class="tag is-danger">{&time.format("%Y-%m-%d %H:%M:%S").to_string()}</span>
                                } 
                            </a>  */
                            <a class="navbar-item nav-tag">
                                <span class="icon is-small">
                                    <i class="fa fa-envelope animated"></i>
                                </span>
                                <span class="tag is-success counter">{"0"}</span>
                            </a>
                            <a class="navbar-item nav-tag">
                                <span class="icon is-small">
                                    <i class="far fa-bell animated"></i>
                                </span>
                                <span class="tag is-danger counter">{"0"}</span>
                            </a>
                            <div class="navbar-item has-dropdown is-hoverable">
                                <a class="navbar-link">
                                    <figure class="image is-32x32" style="margin-right: 0.5em">
                                    <img src="https://avatars.githubusercontent.com/u/2519766?s=400&u=f0a7355e38587eb85a60dc63e13e0402f0d23ebc&v=4" />
                                    </figure>
                                    {"吕俊"}
                                </a>

                                <div class="navbar-dropdown is-right">
                                    <a class="navbar-item" href="https://nokiasonic.github.io/" target="_blank" rel="noopener noreferrer">
                                        //使用图标文本Icon text
                                        <span class="icon-text">
                                            <span class="icon has-text-primary">
                                                <i class="fa fa-user"></i>
                                            </span>
                                            <span>{"Profile"}</span>
                                        </span>                            
                                    </a>
                                    //分割线
                                    <hr class="navbar-divider" />
                                    <a class="navbar-item">
                                    //使用图标文本Icon text
                                        <span class="icon-text">
                                            <span class="icon has-text-primary">
                                                <i class="fa fa-power-off"></i>
                                            </span>
                                            <span>{"Logout"}</span>
                                        </span>  
                                    </a>
                                </div>
                            </div>
                        </div> 
                    </nav> 
                </div>
            </header>
        }
    }
}

