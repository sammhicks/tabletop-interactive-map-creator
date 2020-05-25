#![recursion_limit = "512"]

use grid::Grid;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Copy, Clone)]
pub struct Cell {}

#[derive(Copy, Clone, Debug)]
pub enum ToolMode {
    Brush,
    Erasor,
}

pub struct App {
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    grid_size: usize,
    cells: Grid<Option<Cell>>,
    cursor_position: Option<(usize, usize)>,
    tool_mode: Option<ToolMode>,
}

#[derive(Debug)]
pub enum Msg {
    MouseLeave,
    MouseEvent(yew::events::MouseEvent),
    MouseWheel(yew::events::WheelEvent),
    ToolSelected(ToolMode),
}

impl App {
    fn mouse_event(&mut self, ev: yew::events::MouseEvent) {
        let elem = self
            .node_ref
            .cast::<web_sys::Element>()
            .unwrap()
            .get_bounding_client_rect();

        let x = ((ev.x() - (elem.x() as i32)) / (self.grid_size as i32)) as usize;
        let y = ((ev.y() - (elem.y() as i32)) / (self.grid_size as i32)) as usize;

        self.cursor_position = Some((x, y));

        if (ev.buttons() & 1) == 0 {
            return;
        }

        let tool = if let Some(t) = self.tool_mode {
            t
        } else {
            return;
        };

        if let Some(cell) = self.cells.get_mut(y, x) {
            *cell = match tool {
                ToolMode::Brush => Some(Cell {}),
                ToolMode::Erasor => None,
            };
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            node_ref: NodeRef::default(),
            grid_size: 16,
            cells: Grid::new(8, 8),
            cursor_position: None,
            tool_mode: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MouseLeave => {
                self.cursor_position = None;
                true
            }
            Msg::MouseEvent(ev) => {
                self.mouse_event(ev);
                true
            }
            Msg::MouseWheel(ev) => {
                ev.prevent_default();
                self.grid_size =
                    usize::try_from(self.grid_size as i32 - ev.delta_y() as i32).unwrap_or(0);
                true
            }
            Msg::ToolSelected(tool) => {
                self.tool_mode = Some(tool);
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let width = self.grid_size * self.cells.cols();
        let height = self.grid_size * self.cells.rows();

        html!(
            <>
                <fieldset id="tools">
                    <legend>{"Tools"}</legend>
                    <button onclick=self.link.callback(|_| Msg::ToolSelected(ToolMode::Brush))>{"Brush"}</button>
                    <button onclick=self.link.callback(|_| Msg::ToolSelected(ToolMode::Erasor))>{"Erasor"}</button>
                </fieldset>
                <svg width=width height=height ref=self.node_ref.clone()
                    onmouseleave=self.link.callback(|_| Msg::MouseLeave)
                    onmousemove=self.link.callback(Msg::MouseEvent)
                    onmousewheel=self.link.callback(Msg::MouseWheel)
                    onmousedown=self.link.callback(Msg::MouseEvent)>
                    <g transform=format!("scale({})", self.grid_size)>
                        { for self.cells.iter().enumerate().map(|(i, cell)| {
                            let x = i % self.cells.cols();
                            let y = i / self.cells.cols();

                            if cell.is_some() {
                                html!(<rect width="1px" height="1px" x=x y=y style="fill:black" /> )
                            } else {
                                html!()
                            }
                        })}
                        {
                            if let Some((x, y)) = self.cursor_position {
                                html!( <rect width="1px" height="1px" x=x y=y style="fill:none;stroke:black;stroke-width:0.1" /> )
                            } else {
                                html!()
                            }
                        }
                    </g>
                </svg>
            </>
        )
    }
}

#[wasm_bindgen]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
