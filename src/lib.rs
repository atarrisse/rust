extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
  arrows: u8
}

#[derive(Debug, Clone)]
pub enum Msg {}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model { arrows: 5 }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }
}

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    html! {
      <section class="hunt",>
        <h1 class="header",>{"Hunt the Wumpus"}</h1>
        <div class="body",>
          <span class="arrows",>{&format!("Arrows: {}", self.arrows)}</span>
        </div>
      </section>
    }
  }
}
