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
                    
                        <H3 class="text-left font-code p-12">
                            {"hey, my name is "}
                            <span class="inline-flex animate-ping px-1">
                                <svg height="4" width="4">
                                    <circle  cx="2" cy="2" r="2" stroke-width="0" fill="white" />
                                </svg>
                                
                            </span>
                            <span class="inline-flex animate-ping animation-delay-500 px-1">
                                <svg height="4" width="4">
                                    <circle  cx="2" cy="2" r="2" stroke-width="0" fill="white" />
                                </svg>
                            </span>
                            <span class="inline-flex animate-ping animation-delay-1000 px-1">
                                <svg height="4" width="4">
                                    <circle  cx="2" cy="2" r="2" stroke-width="0" fill="white" />
                                </svg>
                            </span>
                        </H3>
                        <H1 class="text-left font-bold p-6" color={HeaderColor::Primary}>
                            {"JORGE LEWIS"}
                        </H1>
                        <H2 class="text-left p-12">
                            {"i develop "} <span class="text-primary-light dark:text-primary-dark">{"webapps"} </span>
                        </H2>
                    </div>
                </div>
            </>
        }
    }
}
