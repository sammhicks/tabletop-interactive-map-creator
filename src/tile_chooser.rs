use anyhow::Result;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew_components::Select;

use crate::tile::{Material, Materials};

pub struct TileChooser {
    props: Props,
    link: ComponentLink<Self>,
    tiles: Materials,
    service: FetchService,
    task: Option<FetchTask>,
}

pub enum Msg {
    FetchTiles,
    NewTiles(Response<Json<Result<Materials>>>),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub url: String,
    pub tile_changed: Callback<Material>,
    pub tiles_changed: Callback<Materials>,
}

fn fetch_tiles(
    url: String,
    service: &mut FetchService,
    link: &ComponentLink<TileChooser>,
) -> Option<FetchTask> {
    let request = Request::get(url)
        .body(Nothing)
        .map_err(|err| log::error!("Failed to create request: {:?}", err))
        .ok()?;

    service
        .fetch(request, link.callback(Msg::NewTiles))
        .map_err(|err| log::error!("Failed to send request: {:?}", err))
        .ok()
}

impl Component for TileChooser {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut service = FetchService::new();

        let task = fetch_tiles(props.url.clone(), &mut service, &link);

        Self {
            link,
            props,
            tiles: Materials::default(),
            service,
            task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchTiles => {
                self.task = fetch_tiles(self.props.url.clone(), &mut self.service, &self.link);
                false
            }
            Msg::NewTiles(response) => {
                if !response.status().is_success() {
                    log::error!("Failed to get materials: {:?}", response);
                    false
                } else {
                    match response.into_body().0 {
                        Ok(tiles) => {
                            self.tiles = tiles.clone();
                            self.props.tiles_changed.emit(tiles);
                            true
                        }
                        Err(err) => {
                            log::error!("{:?}", err.context("returned materials is not JSON"));
                            false
                        }
                    }
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        false
    }

    fn view(&self) -> Html {
        html!(
            <fieldset>
                <legend>{"Tile Chooser"}</legend>
                <Select<Material> options=self.tiles.as_vec() on_change=self.props.tile_changed.clone()/>
                <button onclick=self.link.callback(|_|Msg::FetchTiles)>{"Refresh"}</button>
            </fieldset>
        )
    }
}
