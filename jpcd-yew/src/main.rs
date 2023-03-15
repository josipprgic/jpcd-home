use yew::{Component, Context, Html, html};
use yew::html::Scope;
use yew_router::prelude::*;

mod pages;
use pages::{conway::Conway, sorting_algos::Sorting};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/conway")]
    Conway,
    #[at("/sorting")]
    Sorting

}

struct Home {
    name: &'static str
}

impl Component for Home {

    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { name: "Visitor" }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let write =  "Hello ".to_string() + self.name;
        html! {
                <div class="home-greeting">{ write }</div>
        }
    }
}

impl App {
    fn view_nav(&self, _link: &Scope<Self>) -> Html {
        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item">{ "JPCD" }</h1>
                </div>
                <div class="navbar-menu">
                    <div class="navbar-start">
                        <Link<Route> classes="navbar-item" to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes="navbar-item" to={Route::Conway}>
                            { "Conway" }
                        </Link<Route>>
                        <Link<Route> classes="navbar-item" to={Route::Sorting}>
                            { "Sorting" }
                        </Link<Route>>
                    </div>
                </div>
            </nav>
        }
    }
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App{}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }
                <main>
                    <Switch<Route> render={switch} />
                </main>
                <footer class="app-footer">
                    <div class="content has-text-centered">
                        { "Created by " }
                        <a href="https://github.com/josipprgic">{ "Josip Prgic" }</a>
                    </div>
                </footer>
            </BrowserRouter>
        }
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home/> },
        Route::Conway => html! { <Conway/> },
        Route::Sorting => html! { <Sorting/> },
        _ => {html! {<div/>}}
    }
}

struct App {}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::Renderer::<App>::new().render();
}