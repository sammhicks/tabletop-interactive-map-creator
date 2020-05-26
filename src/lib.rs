#![recursion_limit = "512"]

use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod grid;
mod list;
mod tile;
mod tile_chooser;
mod tile_patterns;

use grid::Grid;
use tile_chooser::TileChooser;
use tile_patterns::TilePatterns;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ToolMode {
    Brush,
    Erasor,
    Fill,
}

type Cells = Grid<Option<tile::Material>>;

pub struct App {
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    grid_size: usize,
    cells: Cells,
    undo: Vec<Cells>,
    redo: Vec<Cells>,
    cursor_position: Option<(usize, usize)>,
    current_tool: ToolMode,
    current_tile: Option<tile::Material>,
    tile_materials: tile::Materials,
}

#[derive(Debug)]
pub enum Msg {
    MouseLeave,
    MouseEvent(yew::events::MouseEvent),
    MouseWheel(yew::events::WheelEvent),
    Clear,
    ToolSelected(ToolMode),
    NewTiles(tile::Materials),
    TileSelected(tile::Material),
    Undo,
    Redo,
}

impl App {
    fn set_cell(&mut self, row: usize, col: usize, cell: Option<tile::Material>) {
        if let Some(new_cells) = self.cells.set(row, col, cell) {
            if self.cells != new_cells {
                self.redo = Vec::new();
                self.undo
                    .push(std::mem::replace(&mut self.cells, new_cells));
            }
        }
    }

    fn mouse_event(&mut self, ev: yew::events::MouseEvent) {
        let elem = self
            .node_ref
            .cast::<web_sys::Element>()
            .unwrap()
            .get_bounding_client_rect();

        let row = ((ev.y() - (elem.y() as i32)) / (self.grid_size as i32)) as usize;
        let col = ((ev.x() - (elem.x() as i32)) / (self.grid_size as i32)) as usize;

        self.cursor_position = Some((row, col));

        if (ev.buttons() & 1) == 0 {
            return;
        }

        match self.current_tool {
            ToolMode::Brush => self.set_cell(row, col, self.current_tile.clone()),
            ToolMode::Erasor => self.set_cell(row, col, None),
            ToolMode::Fill => {
                if let Some(first_cell) = self.cells.get(row, col) {
                    let mut new_cells = self.cells.clone();

                    let mut indices = vec![(row, col)];

                    while let Some((row, col)) = indices.pop() {
                        if Some(first_cell) != self.cells.get(row, col) {
                            continue;
                        }

                        if let Some(next_cells) = new_cells.set(row, col, self.current_tile.clone())
                        {
                            if new_cells != next_cells {
                                let left = col.checked_sub(1);
                                let up = row.checked_sub(1);
                                let right = col.checked_add(1);
                                let down = row.checked_add(1);

                                for coords in &[
                                    (Some(row), left),
                                    (Some(row), right),
                                    (up, Some(col)),
                                    (down, Some(col)),
                                ] {
                                    if let (Some(row), Some(col)) = coords {
                                        indices.push((*row, *col));
                                    }
                                }
                            }
                            new_cells = next_cells;
                        }
                    }

                    if new_cells != self.cells {
                        self.undo
                            .push(std::mem::replace(&mut self.cells, new_cells))
                    }
                }
            }
        }
    }

    fn cursor(&self) -> Html {
        if let Some((row, col)) = self.cursor_position {
            if self.current_tool != ToolMode::Erasor {
                if let Some(tile) = &self.current_tile {
                    return html!(<rect width="1" height="1" x=col y=row style=format!("fill:{}", tile.url_reference()) />);
                }
            }

            html!(<rect width="1" height="1" x=col y=row style="fill:none;stroke:black;stroke-width:0.1" />)
        } else {
            html!()
        }
    }

    fn button_class(&self, tool: ToolMode) -> Option<&'static str> {
        if self.current_tool == tool {
            Some("selected")
        } else {
            None
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
            cells: Grid::with_dimensions(16, 16),
            undo: Vec::new(),
            redo: Vec::new(),
            cursor_position: None,
            current_tool: ToolMode::Brush,
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
            Msg::Clear => {
                if self.cells.iter().any(|(_, _, cell)| cell.is_some()) {
                    let new_cells = Grid::with_dimensions(self.cells.rows(), self.cells.cols());
                    self.undo
                        .push(std::mem::replace(&mut self.cells, new_cells));
                    true
                } else {
                    false
                }
            }
            Msg::ToolSelected(tool) => {
                self.current_tool = tool;
                true
            }
            Msg::NewTiles(tiles) => {
                self.tile_materials = tiles;
                true
            }
            Msg::TileSelected(tile) => {
                self.current_tile = Some(tile);
                false
            }
            Msg::Undo => {
                if let Some(cells) = self.undo.pop() {
                    self.redo.push(std::mem::replace(&mut self.cells, cells));
                }
                true
            }
            Msg::Redo => {
                if let Some(cells) = self.redo.pop() {
                    self.undo.push(std::mem::replace(&mut self.cells, cells));
                }
                true
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
                    <button onclick=self.link.callback(|_| Msg::Clear)>{"Clear"}</button>
                    <button class=self.button_class(ToolMode::Brush) onclick=self.link.callback(|_| Msg::ToolSelected(ToolMode::Brush))>{"Brush"}</button>
                    <button class=self.button_class(ToolMode::Erasor) onclick=self.link.callback(|_| Msg::ToolSelected(ToolMode::Erasor))>{"Erasor"}</button>
                    <button class=self.button_class(ToolMode::Fill) onclick=self.link.callback(|_| Msg::ToolSelected(ToolMode::Fill))>{"Fill"}</button>
                    <TileChooser url="/tiles.json" tiles_changed=self.link.callback(Msg::NewTiles) tile_changed=self.link.callback(Msg::TileSelected)/>
                    <button onclick=self.link.callback(|_| Msg::Undo)>{"Undo"}</button>
                    <button onclick=self.link.callback(|_| Msg::Redo)>{"Redo"}</button>
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
                        { for self.cells.iter().map(|(row, col, cell)| {
                            if let Some(tile) = cell {
                                html!(<rect width="1" height="1" x=col y=row style=format!("fill:{}", tile.url_reference()) /> )
                            } else {
                                html!()
                            }
                        })}
                        { self.cursor() }
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
