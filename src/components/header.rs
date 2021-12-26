use yew::prelude::*;

pub struct Header {}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="header_component">
                <div class="title">
                    <span>{ "YAR" }</span>
                </div>
                <div class="shortcut">
                    <div class="item">{ "早餐" }</div>
                    <div class="item">{ "午餐" }</div>
                    <div class="item">{ "晚餐" }</div>
                    <div class="item">{ "白条还款" }</div>
                    <div class="item">{ "饮料" }</div>
                    <div class="item">{ "地铁充值" }</div>
                    <div class="item">{ "口罩" }</div>
                </div>
            </div>
        }
    }
}
