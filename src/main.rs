mod app;
mod components;
mod logic;
mod storage;

use dioxus::prelude::*;

fn main() {
    launch(app::App);
}
