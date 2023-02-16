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
                    <div class="flex flex-row my-40">
                        <div class="flex flex-col max-w-xs justify-center">
                            <H3 class="text-center -my-[24px] font-light opacity-50">
                                {"<>"}
                            </H3>
                            <Icon class="mx-auto opacity-50" icon={IconType::Line} height="300" width="2" color="white"/>
                            <H3 class="text-center -my-[24px] font-light opacity-50">
                                {"<>"}
                            </H3>
                        </div>
                        <div class="flex flex-col my-auto">
                            <H3 class="text-left font-code p-12">
                                {"hey, my name is "}
                                <span class="animate-blink">
                                    {"."}
                                </span>
                                <span class="animate-blink animation-delay-300">
                                    {"."}
                                </span>
                                <span class="animate-blink animation-delay-600">
                                    {"."}
                                </span>
                            </H3>
                            <H1 class="text-left font-bold p-6" color={HeaderColor::Primary}>
                                {"Jorge Lewis"}
                                <span class="inline-flex animate-blink duration-2000 align-middle">
                                    <Icon icon={IconType::Line} color="white" height="60" width="3"/>
                                </span>
                            </H1>
                            <H2 class="text-left p-12">
                                {"i develop "} <span class="text-primary-light dark:text-primary-dark">{"webapps"} </span>
                            </H2>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
