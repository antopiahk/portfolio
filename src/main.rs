use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::*;

mod lorc;
use lorc::generic::templates::Page;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(NotFound)]
fn not_found() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/> },
        Route::About => html! { <About/> },
        Route::Contact => html! { <Contact/> },
        Route::NotFound => html! {
            <NotFound/>
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Page>
                <Switch<Route> render={switch}/> // <- must be child of <BrowserRouter>
            </Page>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
