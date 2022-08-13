use yew::prelude::*;

enum Msg {
    AddOne,
    SubOne,
}
struct CounterComponent {
    count: i64,
}
impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true // re-render component
            }
            Msg::SubOne => {
                if self.count > 0 {
                    self.count -= 1;
                    true
                } else {
                    false
                }
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
            <p>{self.count}</p>
            <button id ="increment" onclick={link.callback(|_| Msg::AddOne)}>{"+1"}</button>
            <button id = "decrement" onclick={link.callback(|_|Msg::SubOne)}>{"-1"}</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
