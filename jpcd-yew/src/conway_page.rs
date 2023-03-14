
use gloo::timers::callback::Interval;
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html};
use jpcd_game_of_life::{cell::Cell, universe::Universe};

pub enum Msg {
    Start,
    Stop,
    Reset,
    ToggleCell((usize, usize)),
    Tick
}

pub struct Conway {
    universe: Universe,
    active: bool,
    _interval: Option<Interval>,
}

impl Conway {
    fn new(interval: Option<Interval>) -> Self {
        let width = 53;
        let height = 40;
        let mut universe = Universe::new(width, height);

        (0..(width * height))
            .filter(|i| {
                i % 9 == 0 || i % 7 == 0
            })
            .for_each(|id| {
                universe.toggle(id % width, id / width)
            });

        Conway {
            universe,
            active: false,
            _interval: interval
        }
    }

    fn view_cell(&self, width: usize, height: usize, cell: Cell, link: &Scope<Self>) -> Html {
        match cell {
            Cell::Dead => html! {
            <div key={width * height} class={classes!("game-cell", "dead-cell")}
                onclick={link.callback(move |_| Msg::ToggleCell((width, height)))}>
            </div>
            },
            Cell::Alive => html! {
            <div key={width * height} class={classes!("game-cell", "alive-cell")}
                onclick={link.callback(move |_| Msg::ToggleCell((width, height)))}>
            </div>
            }
        }
    }
}

impl Component for Conway {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(1, move || callback.emit(()));

        Conway::new(Some(interval))
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                self.active = true;
                log::info!("Start");
                false
            }
            Msg::Reset => {
                self.universe.reset();
                log::info!("Reset");
                true
            }
            Msg::Stop => {
                self.active = false;
                log::info!("Stop");
                false
            }
            Msg::ToggleCell(idx) => {
                self.universe.toggle(idx.0, idx.1);
                true
            }
            Msg::Tick => {
                if self.active {
                    self.universe.tick();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows = self.universe.enumerate_rows()
            .map(|(n, cells)| {
                let cells = cells
                    .iter()
                    .enumerate()
                    .map(|(x, cell)| self.view_cell(x, n, *cell, ctx.link()));
                html! {
                        <div key={n} class="game-row">
                            { for cells }
                        </div>
                    }

            }).collect::<Vec<Html>>();

        html!{
            <div>
                <section class="game-container">
                    <header class="app-header">
                        <h1 class="app-title">{ "Game of Life" }</h1>
                    </header>
                    <section class="game-area">
                        <div class="game-of-life">
                            { for cell_rows }
                        </div>
                        <div class="game-buttons">
                            <button class="game-button"
                                onclick={ctx.link().callback(|_| Msg::Start)}>{ "Start" }</button>
                            <button class="game-button"
                                onclick={ctx.link().callback(|_| Msg::Stop)}>{ "Stop" }</button>
                            <button class="game-button"
                                onclick={ctx.link().callback(|_| Msg::Reset)}>{ "Reset" }</button>
                        </div>
                    </section>
                </section>
                <footer class="app-footer">
                    <strong class="footer-text">
                      { "Game of Life - a yew experiment " }
                    </strong>
                </footer>
            </div>
        }
    }
}