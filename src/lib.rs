#![recursion_limit = "512"]

use grid::Grid;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod tile;
mod tile_chooser;
mod tile_patterns;

use tile_chooser::TileChooser;
use tile_patterns::TilePatterns;

#[derive(Clone)]
pub struct Cell(tile::Material);

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
    current_tile: Option<tile::Material>,
    tile_materials: tile::Materials,
}

#[derive(Debug)]
pub enum Msg {
    MouseLeave,
    MouseEvent(yew::events::MouseEvent),
    MouseWheel(yew::events::WheelEvent),
    ToolSelected(ToolMode),
    NewTiles(tile::Materials),
    TileSelected(tile::Material),
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
                ToolMode::Brush => self.current_tile.clone().map(Cell),
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
            cells: Grid::new(16, 16),
            cursor_position: None,
            tool_mode: None,
            current_tile: None,
            tile_materials: tile::Materials::default(),
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
            Msg::NewTiles(tiles) => {
                self.tile_materials = tiles;
                true
            }
            Msg::TileSelected(tile) => {
                self.current_tile = Some(tile);
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let width = self.cells.cols();
        let height = self.cells.rows();

        html!(
            <>
                <fieldset id="tools">
                    <legend>{"Tools"}</legend>
                    <button onclick=self.link.callback(|_| Msg::ToolSelected(ToolMode::Brush))>{"Brush"}</button>
                    <button onclick=self.link.callback(|_| Msg::ToolSelected(ToolMode::Erasor))>{"Erasor"}</button>
                    <TileChooser url="/tiles.json" tiles_changed=self.link.callback(Msg::NewTiles) tile_changed=self.link.callback(Msg::TileSelected)/>
                </fieldset>
                <svg width={self.grid_size * width} height={self.grid_size * height} ref=self.node_ref.clone()
                    onmouseleave=self.link.callback(|_| Msg::MouseLeave)
                    onmousemove=self.link.callback(Msg::MouseEvent)
                    onmousewheel=self.link.callback(Msg::MouseWheel)
                    onmousedown=self.link.callback(Msg::MouseEvent)>
                    <defs>
                        <TilePatterns tiles=self.tile_materials.clone() />
                    </defs>
                    <g transform=format!("scale({})", self.grid_size)>
                        { for self.cells.iter().enumerate().map(|(i, cell)| {
                            let x = i % self.cells.cols();
                            let y = i / self.cells.cols();

                            if let Some(Cell(tile)) = cell {
                                html!(<rect width="1" height="1" x=x y=y style=format!("fill:{}", tile.as_url()) /> )
                            } else {
                                html!()
                            }
                        })}
                        {
                            if let Some((x, y)) = self.cursor_position {
                                html!( <rect width="1" height="1" x=x y=y style="fill:none;stroke:black;stroke-width:0.1" /> )
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
