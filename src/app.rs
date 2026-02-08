use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{Card, CardContent};
use crate::components::{PracticeInterface, StatisticsDisplay};
use crate::logic::PracticeSession;
use crate::storage::HistoryManager;
use dioxus::prelude::*;

pub fn app() -> Element {
    let mut current_tab = use_signal(|| "practice");
    let session = use_signal(PracticeSession::new);

    rsx! {
        main {
            class: "min-h-screen py-12 px-4 flex flex-col items-center",

            div {
                class: "w-full max-w-3xl space-y-8",

                // Header & Navigation
                header {
                    class: "flex flex-col md:flex-row md:items-center justify-between gap-6",

                    div {
                        h1 { class: "text-4xl font-extrabold text-gradient tracking-tight", "Array30" }
                        p { class: "text-slate-500 font-medium", "Master the art of typing" }
                    }

                    nav {
                        class: "glass p-1.5 rounded-2xl flex gap-1",
                        Button {
                            class: if *current_tab.read() == "practice" { "btn-primary" } else { "btn-ghost" },
                            variant: if *current_tab.read() == "practice" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                            onclick: move |_| current_tab.set("practice"),
                            "📝 Practice"
                        }
                        Button {
                            class: if *current_tab.read() == "statistics" { "btn-primary" } else { "btn-ghost" },
                            variant: if *current_tab.read() == "statistics" { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                            onclick: move |_| current_tab.set("statistics"),
                            "📊 Statistics"
                        }
                    }
                }

                // Main Content Area
                div {
                    class: "transition-all duration-500",
                    if *current_tab.read() == "practice" {
                        PracticeInterface { session: session }
                    } else {
                        div {
                            class: "space-y-8",
                            StatisticsDisplay {
                                stats: HistoryManager::get_statistics()
                            }

                            Card {
                                class: "glass-card border-rose-100",
                                CardContent {
                                    class: "flex justify-center pt-6",
                                    Button {
                                        class: "btn-premium bg-rose-50 text-rose-600 hover:bg-rose-100 border border-rose-200 px-8",
                                        variant: ButtonVariant::Destructive,
                                        onclick: move |_| {
                                            if confirm_clear() {
                                                HistoryManager::clear_history();
                                                current_tab.set("practice");
                                            }
                                        },
                                        "🗑️ Reset All Progress"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn confirm_clear() -> bool {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        fn confirm(message: &str) -> bool;
    }

    confirm("Clear all statistics? This cannot be undone.")
}
