mod db;

use yew::prelude::*;
use gloo::timers::callback::Timeout;

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
            <div>
                <button onclick={ctx.link().callback(|_| Msg::ToSleep)}>{ "to sleep" }</button>
                <p>{ "status: " }{ self.content }</p>
                <button onclick={ctx.link().callback(|_| Msg::DBTest)}>{ "setup db" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
