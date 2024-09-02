use yew::prelude::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketMessage {    
} 

pub struct Basic {
    _wss          : WebSocketService,
    _producer     : Box<dyn Bridge<EventBus>>,
    _http         : HttpService,
    account       : WF_Account,
    index_quote   : Data,
    position_quote: Data,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub sidebar_is_collapsed: bool,
    pub time                : DateTime<Local>,
}


//#[derive(Debug)]
pub enum Msg {
    HandleEventBusMsg(String),
    UpdateProfitMsg(f64),
}

impl Component for Basic{
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        //let wss = WebSocketService::new();
        let account_id = "2630104180";

        Self { 
            _wss: WebSocketService::new(),
            _producer:EventBus::bridge(ctx.link().callback(Msg::HandleEventBusMsg)),     
            _http: HttpService::new(account_id),  
            account:WF_Account::new(account_id),
            index_quote:Data::new(),   
            position_quote:Data::new(),  
        }
        
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        
        match msg{
            // 处理在Event_bus上出现的消息
            Msg::HandleEventBusMsg(s) => {
                if s.contains("MarketIndex") == true {
                    self.index_quote = serde_json::from_str(&s).unwrap();
                    //log::info!("basic update get index_quote from event_bus:{:#?}", self.index_quote);
                }

                if s.contains("Position") == true {
                    self.position_quote = serde_json::from_str(&s).unwrap();
                    //log::info!("basic update get position_quote from event_bus:{:#?}", self.position_quote);
                }

                if s.contains("Notice") == true {
                    log::info!("Notice recieved:{}",s);
                }

                if s.contains("account_id") == true {
                    self.account = serde_json::from_str(&s).unwrap();
                    //log::info!("basic update get account from event_bus:{:#?}", self.account);
                }
                true
            },
            Msg::UpdateProfitMsg(p) => {
                self.account.account_detail.close_profit = p;
                //log::info!("basic update close profit from message:{:#?}", self.account.account_detail.close_profit);
                false
            }, 
        }
        
    }

    
    fn view(&self, ctx: &Context<Self>) -> Html {
        // 处理接收到的数据，整理成页面显示所需的数据格式
        let pos_info = self.data_handle();
        //log::info!("basic view handle data:{:#?}", data);
        
        let data = pos_info.into_iter()
                                    .map(|(_k,v)| v)
                                    .collect::<Vec<_>>(); 
        // 更新account中的当日盈亏
        let today_profit_sum = data.clone().into_iter()
                                        .map(|v| v.today_profit)
                                        .sum::<f64>();
        let link = ctx.link().clone();      
        link.send_message(Msg::UpdateProfitMsg(today_profit_sum));


        let class_text;
        if ctx.props().sidebar_is_collapsed == false {
            class_text = "main";
        } else {
            class_text = "main sidebar--closed";
        }

        html!{
            <>
                <main class={class_text} id="main">
                    <nav class="breadcrumb is-small" aria-label="breadcrumbs" id="navigator">
                        <ul>
                            <li><a href="/">{"Home"}</a></li>
                            <li class="is-active"><a href="#" aria-current="page">{"Stock"}</a></li>
                            // form control - select 
                            <a class="select">
                                <select>
                                <option>{"银河证券"}</option>
                                <option selected = true>{"海通证券"}</option>
                                <option>{"国信证券"}</option>
                                </select>
                            </a>          
                        </ul>                     
                    </nav>

                    if self.index_quote == Data::new() || self.position_quote == Data::new() {
                    <section class="section">
                        <div class="container is-fullhd">
                            <center>
                             <progress class="progress is-small is-primary" max="100">{"15%"}</progress>
                            </center>
                        </div>
                    </section>   
                    } else {
                    <section class="section">
                        <div class="columns is-multiline">
                            <div class="column is-4"> 
                                <div class="level mt-2 mr-2 content-title">
                                    <div class="level-left">
                                        <div class="level-item">
                                            <div class="title has-text-primary">
                                                <span class="icon is-small icon-title">
                                                    <i class="fa fa-money-check-alt"></i>
                                                </span>
                                                <span>{"账户信息"}</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>   
                            </div> 
                            <div class="column is-8"> 
                                <div class="level mt-2 mr-2 content-title">
                                    <div class="level-left">
                                        <div class="level-item">
                                            <div class="title has-text-primary">
                                                <span class="icon is-small icon-title">
                                                    <i class="fa fa-poll"></i>
                                                </span>
                                                <span>{"主要指数"}</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>   
                            </div>               
                        </div>

                    <div class="columns">               
                        <crate::components::account::AccountSummary account = {self.account.clone()} />    
                        <crate::components::market::MarketIndex index={self.index_quote.data.clone()} />
                    </div>
                    </section> 
                    <crate::components::chart::Chart profit={today_profit_sum.clone()} /> 
                    <crate::components::tables::TableData data = {data.clone()} />                    
                    }

                </main>  
            </>
        }         
    }

    /* fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let chart_data = vec![ChartData {
            x:ctx.props().time.timestamp(),
            y:self.account.account_detail.close_profit}]; 
        log::info!("chart data is {:#?}", chart_data);  
        false
    } */

    
}


impl Basic {
    fn data_handle(&self) -> HashMap<&String, RowData> {
        let quote = self.position_quote.data.clone();  
        let mut pos_info = HashMap::new();

        for (id, pos) in self.account.positions.iter() {
            let mut quote_data = SinaData::default();

            if let Some(v) = quote.get(id) {
                //log::info!("{:#?}", v);
                quote_data = v.clone();
            } 

            //log::info!("pos quote:{:#?}", quote_data);

            let data = RowData{
                stock_name     : pos.stock_name.clone(),
                stock_code     : pos.stock_code.clone(),
                pct_change     : (quote_data.lastest_price.clone() - quote_data.last_close.clone()) / quote_data.last_close.clone(),
                lastest_price  : quote_data.lastest_price.clone(),
                last_close     : quote_data.last_close.clone(),
                price_change   : quote_data.lastest_price.clone() - quote_data.last_close.clone(),
                volume         : pos.volume.clone(),
                cost           : pos.cost.clone(),
                floating_profit: pos.volume.clone() * (quote_data.lastest_price.clone() - pos.cost.clone()),
                today_profit   : pos.volume.clone() * (quote_data.lastest_price.clone() - quote_data.last_close.clone()),
            };
            pos_info.insert(id, data);
        }

        // sort by value:pct_change
        //log::info!("{:#?}", pos_info);
        //let mut data = pos_info.into_iter().map(|(_k,v)| v)
        //                                              .collect::<Vec<_>>();        
        //data.sort_by(|a,b|b.pct_change.partial_cmp(&a.pct_change).unwrap());
        //log::info!("{:#?}", data);

        //let today_profit_sum = data.clone().into_iter().map(|v| v.today_profit).sum::<f64>();
        //log::info!("today's profit is:{:#?}", today_profit_sum);
        //ctx.props().profit_update.emit(today_profit_sum);

        //let float_profit_sum = data.clone().into_iter().map(|v| v.floating_profit).sum::<f64>();
        pos_info
    }
}