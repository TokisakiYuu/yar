use yew::prelude::*;
use super::ItemGroup;

pub struct Ledger {}

impl Component for Ledger {
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
            <div class="ledger">
                <ItemGroup title="hello1" />
                <ItemGroup title="hello2" />
                <ItemGroup title="hello3" />
            </div>
        }
    }
}
