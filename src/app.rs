use dioxus::prelude::*;
use crate::components::{PracticeInterface, StatisticsDisplay};
use crate::storage::HistoryManager;
use crate::logic::PracticeSession;

pub fn App() -> Element {
    let mut current_tab = use_signal(|| "practice");
    let mut session = use_signal(|| PracticeSession::new());

    rsx! {
        style {
            {include_str!("../assets/styles.css")}
        }
        div {
            class: "min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100",
            div {
                class: "container mx-auto px-4 py-8 max-w-2xl",
                div {
                    class: "mb-6",
                    div {
                        class: "flex gap-4 border-b border-gray-300",
                        button {
                            class: if *current_tab.read() == "practice" {
                                "px-6 py-3 font-bold text-blue-600 border-b-2 border-blue-600 transition"
                            } else {
                                "px-6 py-3 font-bold text-gray-600 hover:text-gray-800 transition"
                            },
                            onclick: move |_| current_tab.set("practice"),
                            "📝 Practice"
                        }
                        button {
                            class: if *current_tab.read() == "statistics" {
                                "px-6 py-3 font-bold text-blue-600 border-b-2 border-blue-600 transition"
                            } else {
                                "px-6 py-3 font-bold text-gray-600 hover:text-gray-800 transition"
                            },
                            onclick: move |_| current_tab.set("statistics"),
                            "📊 Statistics"
                        }
                    }
                }

                if *current_tab.read() == "practice" {
                    PracticeInterface {
                        session: session,
                    }
                } else {
                    div {
                        StatisticsDisplay {
                            stats: HistoryManager::get_statistics()
                        }
                        
                        div {
                            class: "mt-6 text-center",
                            button {
                                class: "px-6 py-2 bg-red-500 hover:bg-red-600 text-white font-bold rounded-lg transition",
                                onclick: move |_| {
                                    if confirm_clear() {
                                        HistoryManager::clear_history();
                                        current_tab.set("practice");
                                    }
                                },
                                "🗑️ Clear All Statistics"
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
