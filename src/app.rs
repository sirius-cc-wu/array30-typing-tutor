use dioxus::prelude::*;
use crate::components::{PracticeInterface, StatisticsDisplay};
use crate::storage::HistoryManager;
use crate::logic::PracticeSession;

pub fn App() -> Element {
    let mut current_tab = use_signal(|| "practice");
    let mut session = use_signal(|| PracticeSession::new());

    rsx! {
        style { {include_str!("../assets/styles.css")} }
        
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
                        button {
                            class: if *current_tab.read() == "practice" { "btn-primary" } else { "btn-ghost" },
                            onclick: move |_| current_tab.set("practice"),
                            "📝 Practice"
                        }
                        button {
                            class: if *current_tab.read() == "statistics" { "btn-primary" } else { "btn-ghost" },
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
                            
                            div {
                                class: "flex justify-center",
                                button {
                                    class: "btn-premium bg-rose-50 text-rose-600 hover:bg-rose-100 border border-rose-200 px-8",
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

fn confirm_clear() -> bool {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        fn confirm(message: &str) -> bool;
    }

    confirm("Clear all statistics? This cannot be undone.")
}
