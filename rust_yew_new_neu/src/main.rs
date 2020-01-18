use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};

use rust_new_neu::{Network};

struct App {
    clicked: bool,
    onclick: Callback<ClickEvent>,
    network: Network,
}

enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
            network: Network::new(&vec![784, 30, 10]),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked { "Clicked!" } else { "Click me!" };

        html! {
            <button onclick=&self.onclick>{ button_text }</button>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
