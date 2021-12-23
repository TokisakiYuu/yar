mod db;
mod components;

use yew::prelude::*;
use gloo::timers::callback::Timeout;
use components::{
    Ledger,
    Header,
    AddRecord
};

enum Msg {
    ToSleep,
    SleepEnd,
    DBTest,
    DBSetupComplete
}

struct Model {
    content: &'static str,
    db_state: &'static str
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            content: "running",
            db_state: "unsetup"
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToSleep => {
                let link = ctx.link().clone();
                let timer = Timeout::new(3_000, move || link.send_message(Msg::SleepEnd));
                timer.forget();
                false
            },
            Msg::SleepEnd => {
                self.content = "sleep end";
                true
            },
            Msg::DBTest => {
                ctx.link().send_future(async {
                    db::db_test().await;
                    Msg::DBSetupComplete
                });
                false
            },
            Msg::DBSetupComplete => {
                self.db_state = "complete";
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="main">
                <div class="header">
                    <div>{ "title" }</div>
                    <div>{ "dashboard" }</div>
                </div>
                <div class="content-wrapper">
                    <div class="content">
                        <Ledger />
                    </div>
                </div>
                <div class="toolbar">
                    <AddRecord />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
