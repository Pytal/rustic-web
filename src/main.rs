use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};

struct App {
  clicked: bool,
  onclick: Callback<ClickEvent>,
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
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Click => {
        self.clicked = !self.clicked;
        true // Indicate that the Component should re-render
      }
    }
  }

  fn view(&self) -> Html {
    let button_text = if !self.clicked { "Click me!" } else { "Clicked!" };
    let header = if !self.clicked { "😶" } else { "😮 whoa" };

    let button_style = "
      width: 10rem;
      font-size: 16px;
      border: none;
      outline: none;
      padding: 1rem;
      color: whitesmoke;
      background-color: #5B38D1;
      border-radius: 2rem;
      box-shadow: 0 0 10px 0px rgba(0,0,0,0.5);
    ";

    let box_style = "
      margin-top: 2rem;
      display: grid;
      justify-content: center;
    ";

    html! {
      <div style={ box_style }>
        <button
          style={ button_style }
          onclick=&self.onclick>
          { button_text }
        </button>
        <h1>{ header }</h1>
      </div>
    }
  }
}

fn main() {
  yew::start_app::<App>();
}
