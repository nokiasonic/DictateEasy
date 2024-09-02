use yew::prelude::*;

pub struct SideBar{
    sidebar_onoff: u8,
}

pub enum Msg {
    SideBarToggle,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub sidebar_is_collapsed: Callback<bool>,
}

impl Component for SideBar{
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            sidebar_onoff: 0,
        } 
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SideBarToggle => {
                //log::info!("clicked!!");
                if self.sidebar_onoff == 1 {
                    self.sidebar_onoff = 0;
                    ctx.props().sidebar_is_collapsed.emit(true);
                } else {
                    self.sidebar_onoff = 1;
                    ctx.props().sidebar_is_collapsed.emit(false);
                }
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let class_text;
        if self.sidebar_onoff == 1 {
            class_text = "aside animated";
            
        } else {
            class_text = "aside closed anmated";
            
        }
        
        html! {
            <aside id="main-sidebar" class={class_text}>
                <nav class="menu active-menu--<%= activeMenu %>">
                    <ul class="menu-list">
                    <li>
                        <a class="<%= isActiveClass('home') %>" href="<%= data.config.publicPath %>/">
                        <span class="icon is-small"><i class="fas fa-home animated"></i></span> <span class="menu-text">{"Home"}</span>
                        </a>
                    </li>
                    </ul>
                    <ul class="menu-list">
                    <li>
                        <a class="<%= isActiveClass('stock') %>" href="<%= data.config.publicPath %>/forms/">
                        <span class="icon is-small"><i class="fas fa-chart-line"></i></span> <span class="menu-text">{"Stock"}</span>
                        </a>
                    </li>
                    <li>
                        <a class="<%= isActiveClass('future') %>" href="<%= data.config.publicPath %>/ui-elements/">
                        <span class="icon is-small"><i class="fas fa-balance-scale"></i></span> <span class="menu-text">{"Future"}</span>
                        </a>
                    </li>
                    <li>
                        <a class="<%= isActiveClass('option') %>" href="<%= data.config.publicPath %>/tables/">
                        <span class="icon is-small"><i class="fas fa-globe"></i></span> <span class="menu-text">{"Option"}</span>
                        </a>
                    </li>
                    <li>
                        <a class="<%= isActiveClass('fund') %>" href="<%= data.config.publicPath %>/presentations/">
                        <span class="icon is-small"><i class="fas fa-coins"></i></span> <span class="menu-text">{"Fund"}</span>
                        </a>
                    </li>
                    <li>
                        <a class="<%= isActiveClass('settings') %>" href="<%= data.config.publicPath %>/presentations/">
                        <span class="icon is-small"><i class="fa fa-cog"></i></span> <span class="menu-text">{"Settings"}</span>
                        </a>
                    </li>
                    </ul>

                </nav>

                <div id="sidebar-toggler" onclick={ctx.link().callback(|_| Msg::SideBarToggle)} >
                    <span class="icon is-small"><i class="fa fa-angle-double-left"></i></span>                    
                </div> 
            </aside>
        }
    }
}