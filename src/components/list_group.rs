use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ItemGroupProps {
    pub title: String
}

pub struct ItemGroup {}

impl Component for ItemGroup {
    type Message = ();
    type Properties = ItemGroupProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="item-group">
                <div class="group-header">
                    <div class="title">{ ctx.props().title.to_owned() }</div>
                    <div class="extra">{ "extra" }</div>
                </div>
                <div class="group-list">
                    <div class="item">{ "item" }</div>
                    <div class="item">{ "item" }</div>
                    <div class="item">{ "item" }</div>
                    <div class="item">{ "item" }</div>
                    <div class="item">{ "item" }</div>
                </div>
            </div>
        }
    }
}
