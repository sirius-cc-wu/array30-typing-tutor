use dioxus::prelude::*;
use crate::components::PracticeInterface;

pub fn App() -> Element {
    rsx! {
        style {
            {include_str!("../assets/styles.css")}
        }
        div {
            class: "min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100",
            PracticeInterface {}
        }
    }
}
