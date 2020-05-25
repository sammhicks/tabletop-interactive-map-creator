use yew::prelude::*;

use crate::tile::Materials;

pub struct TilePatterns {
    props: Props,
}

pub enum Msg {}

#[derive(Clone, Properties)]
pub struct Props {
    pub tiles: Materials,
}

impl Component for TilePatterns {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html!(
            <>
                { for self.props.tiles.iter().map(|material| {
                    let size = material.size;
                    html!(
                        <pattern id=material.get_name() patternUnits="userSpaceOnUse" height=size width=size>
                            <image href=material.href.clone() width=size height=size />
                        </pattern>
                    )
                }) }
            </>
        )
    }
}
