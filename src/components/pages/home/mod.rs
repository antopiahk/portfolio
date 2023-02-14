use yew::prelude::*;
use yew_router::prelude::*;

use crate::lorc::generic::{Icon, IconType, *};
use crate::{atoms::*, Route};

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="py-40 px-4 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40">
                    <div class="mx-40 my-40">
                        <H3 class="text-left">
                        {"Hey, my name is ..."}
                        </H3>
                        <H1 class="text-left font-bold" color={HeaderColor::Primary}>
                        {"Jorge"}
                        </H1>
                        <H2 class="text-left">
                        {"I develop "} <span class="text-primary-light dark:text-primary-dark">{"Webapps"} </span>
                        </H2>
                    </div>
                </div>
            </>
        }
    }
}
