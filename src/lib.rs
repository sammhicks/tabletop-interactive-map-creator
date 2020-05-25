use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!(
            <div>
                <h1>{"Hello World"}</h1>
            </div>
        )
    }
}

#[wasm_bindgen]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
