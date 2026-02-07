mod app;
mod components;
mod logic;

use dioxus::prelude::*;

fn main() {
    launch(app::App);
}
