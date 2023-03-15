use gloo::timers::callback::Interval;
use rand::Rng;
use yew::{Component, Context, Html, html};
use crate::pages::sorting_algos::Algorithm::Insertion;

pub enum Msg {
    Start,
    Stop,
    Reset,
    ShowStep,
    Tick,
    Change
}

pub enum Algorithm {
    Insertion,
    Selection
}

pub struct Sorting {
    alg: Algorithm,
    active: bool,
    coll: Vec<usize>,
    _interval: Interval
}

impl Sorting {

    pub fn new(a: Algorithm, size: usize, interval: Interval) -> Sorting {
        Self {
            alg: a,
            active: false,
            coll: generate_random(size),
            _interval: interval
        }
    }
}

impl Component for Sorting {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(100, move || callback.emit(()));

        Sorting::new(Insertion, 200, interval)
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cols = self.coll.iter()
            .map(|c| {
                html! {
                    <div class="cell" value={c} style.height={c}></div>
                }
            });
        html! {
            <div>
                { for cols }
            </div>
        }
    }
}

fn generate_random(size: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    (0..size).into_iter()
        .map(|i| rng.gen_range(0..2*size))
        .collect()
}