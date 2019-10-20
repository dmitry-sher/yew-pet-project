use yew::prelude::*;
use yew_router::{prelude::*, Switch};
use crate::{app::App};

#[derive(Switch, Debug)]
pub enum AppRoute {
    #[to = "/tag/{tag}"]
    Tag{tag: String},
    #[to = "/"]
    Index,
}

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu",>
                    <RouterButton: text=String::from("Todo"), link="/", />
                    <RouterLink: text=String::from("Tags"), link="/tag/test_tag", />
                    <RouterButton: text=String::from("Go to bad path"), link="/a_bad_path", />
                </nav>
                <div>
                    <Router<AppRoute, ()>
                        render = Router::render(|switch: Option<AppRoute>| {
                            match switch {
                                Some(AppRoute::Tag{tag}) => html!{<div>{ tag }</div>},
                                Some(AppRoute::Index) => html!{<App />},
                                None => html!{"404"}
                            }
                        })
                    />
                </div>
            </div>
        }
    }
}
