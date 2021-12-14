use yew::prelude::*;
use gloo::timers::callback::Timeout;

enum Msg {
    ToSleep,
    SleepEnd
}

struct Model {
    content: &'static str,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            content: "running",
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
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::ToSleep)}>{ "to sleep" }</button>
                <p>{ "status: " }{ self.content }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
