use gloo::timers::callback::Interval;
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

pub enum Msg {
    Start,
    Stop,
    Reset,
    ToggleCell(usize),
    Tick
}

pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    active: bool,
    _interval: Interval,
}

impl Universe {

    fn set_width(&mut self, size: usize) {
        self.width = size;
    }

    fn set_height(&mut self, size: usize) {
        self.height = size;
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    fn set_cells(&mut self, cells: &[(usize, usize)]) {
        self.reset();
        for (row, col) in cells {
            if *row < self.height && *col < self.width {
                let idx = self.get_index(*row, *col);
                self.cells[idx] = Cell::Alive
            }
        }
    }

    fn reset(&mut self) {
        let width = 53;
        let height = 40;

        self.cells = (0..width * height)
            .map(|i| {
                if i % 9 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        let mut diff = Vec::new();
        for i in 0..next.len() {
            if next[i] != self.cells[i] {
                diff.push(i);
            }
        }
        // log!{"Changed cells are {:?}", diff}
        self.cells = next;
    }

    pub fn new(int: Interval) -> Universe {
        let width = 53;
        let height = 40;

        let cells = (0..width * height)
            .map(|i| {
                if i % 9 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
            active: false,
            _interval: int
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: usize, column: usize) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    fn view_cell(&self, offset: usize, cell: Cell, link: &Scope<Self>) -> Html {
        match cell {
            Cell::Dead => html! {
            <div key={offset} class={classes!("game-cell", "dead-cell")}
                onclick={link.callback(move |_| Msg::ToggleCell(offset))}>
            </div>
            },
            Cell::Alive => html! {
            <div key={offset} class={classes!("game-cell", "alive-cell")}
                onclick={link.callback(move |_| Msg::ToggleCell(offset))}>
            </div>
            }
        }
    }
}

impl Component for Universe {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(1, move || callback.emit(()));

        Universe::new(interval)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows = self.cells
            .chunks(self.width as usize)
            .enumerate()
            .map(|(n, cells)| {
                let idx_offset = n * self.width;

                let cells = cells
                    .iter()
                    .enumerate()
                    .map(|(x, cell)| self.view_cell(idx_offset + x, *cell, ctx.link()));
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
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Start)}>{ "Start" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Stop)}>{ "Stop" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Reset)}>{ "Reset" }</button>
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                self.active = true;
                log::info!("Start");
                false
            }
            Msg::Reset => {
                self.reset();
                log::info!("Reset");
                true
            }
            Msg::Stop => {
                self.active = false;
                log::info!("Stop");
                false
            }
            Msg::ToggleCell(idx) => {
                let cell = self.cells.get_mut(idx).unwrap();
                cell.toggle();
                true
            }
            Msg::Tick => {
                if self.active {
                    self.tick();
                    true
                } else {
                    false
                }
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::Renderer::<Universe>::new().render();
}