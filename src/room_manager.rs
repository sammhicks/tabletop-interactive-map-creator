use yew::prelude::*;
use yew_components::Select;

use crate::room::{Room, Rooms};
use crate::tile;

pub struct RoomManager {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    NewRoom,
    RoomMaterialChanged(usize, tile::Material),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub tile_materials: tile::Materials,
    pub rooms: Rooms,
    pub rooms_changed: Callback<Rooms>,
    pub selected_room: usize,
    pub selected_room_changed: Callback<usize>,
}

impl Component for RoomManager {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewRoom => self
                .props
                .rooms_changed
                .emit(self.props.rooms.push_back(Room {
                    tile_material: self.props.tile_materials.as_ref()[0].clone(),
                })),
            Msg::RoomMaterialChanged(index, material) => {
                if let Some(mut room) = self.props.rooms.get(index).cloned() {
                    room.tile_material = material;
                    if let Some(new_rooms) = self.props.rooms.set(index, room) {
                        self.props.rooms_changed.emit(new_rooms);
                    }
                }
            }
        }

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html!(
            <fieldset>
                <legend>{"Rooms"}</legend>
                <table>
                    <thead>
                        <tr>
                            <th/>
                            <th>{"Tile Material"}</th>
                            <th/>
                        </tr>
                    </thead>
                    <tbody>
                        { for self.props.rooms.iter().enumerate().map(|(index, room)| {
                            let class = if index == self.props.selected_room {
                                "selected"
                            } else {
                                ""
                            };
                            html!(
                                <tr class=class>
                                    <td>{ index + 1 }</td>
                                    <td><Select<tile::Material> selected=room.tile_material.clone() options=self.props.tile_materials.as_vec() on_change=self.link.callback(move |material| Msg::RoomMaterialChanged(index, material))/></td>
                                    <td><button onclick=self.props.selected_room_changed.reform(move |_| index)>{"Select"}</button></td>
                                </tr>
                            )
                        }) }
                    </tbody>
                </table>
                <button onclick=self.link.callback(|_| Msg::NewRoom)>{"Add Room"}</button>
            </fieldset>
        )
    }
}
