use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub width: u64,
    pub height: u64
}

pub struct Icon {}

impl Component for Icon {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div
                class={format!("icon-{}", ctx.props().name)}
                style={format!("width:{}px;height:{}px", ctx.props().width, ctx.props().height)}
            />
        }
    }
}
