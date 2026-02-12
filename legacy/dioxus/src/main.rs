mod app;
mod array30_data;
mod components;
mod logic;
mod storage;

use dioxus::prelude::*;

fn main() {
    launch(app::app);
}
