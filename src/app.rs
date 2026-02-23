use crate::components::alert_dialog::{
    AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogContent,
    AlertDialogDescription, AlertDialogRoot, AlertDialogTitle,
};
use crate::components::button::{Button, ButtonVariant};
use crate::components::separator::Separator;
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};
use crate::components::toast::ToastProvider;
use crate::components::{PracticeInterface, StatisticsDisplay};
use crate::logic::PracticeSession;
use crate::storage::HistoryManager;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

#[derive(Clone, Copy, PartialEq, Eq)]
enum AppTab {
    Practice,
    Statistics,
}

impl std::fmt::Display for AppTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppTab::Practice => write!(f, "practice"),
            AppTab::Statistics => write!(f, "statistics"),
        }
    }
}

pub fn app() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("../assets/tailwind.css") }
        ToastProvider {
            AppContent {}
        }
    }
}

#[component]
fn AppContent() -> Element {
    let toast_api = use_toast();
    let mut current_tab = use_signal(|| Some(AppTab::Practice.to_string()));
    let mut show_reset_dialog = use_signal(|| false);
    let session = use_signal(PracticeSession::new);
    let stats = session.read().stats.clone();
    let wpm = stats.wpm();
    let accuracy = stats.accuracy();
    let wpm_text = format!("{wpm:.0}");
    let accuracy_text = format!("{accuracy:.0}%");

    rsx! {
        main {
            class: "app-shell text-base-content",

            div { class: "app-bg" }

            div {
                class: "app-frame mx-auto w-full max-w-6xl p-4 md:p-8",
                div {
                    class: "app-panel bg-base-100/80 backdrop-blur-xl border border-white/40 shadow-xl rounded-[2rem] p-6 md:p-8 space-y-8",
                    Tabs {
                        class: "app-tabs w-full",
                        value: current_tab,
                        on_value_change: move |value| current_tab.set(Some(value)),

                        // Header + Tabs
                        div {
                            class: "flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between",
                            div {
                                class: "space-y-2",
                                p { class: "text-xs font-bold uppercase tracking-[0.3em] text-primary/70", "Lesson 4" }
                                h1 { class: "text-4xl md:text-5xl font-extrabold tracking-tight", "Array30 Practice" }
                                p { class: "text-base text-base-content/70", "Master the art of typing with playful momentum." }
                            }
                            TabList {
                                class: "tabs-boxed w-fit bg-white/80 shadow-md p-1 rounded-full",
                                TabTrigger {
                                    class: "tab px-6 py-2 font-semibold",
                                    index: 0usize,
                                    value: AppTab::Practice.to_string(),
                                    "Practice"
                                }
                                TabTrigger {
                                    class: "tab px-6 py-2 font-semibold",
                                    index: 1usize,
                                    value: AppTab::Statistics.to_string(),
                                    "Statistics"
                                }
                            }
                        }

                        // Top Stats
                        div {
                            class: "grid gap-4 md:grid-cols-3",
                            div {
                                class: "bg-white rounded-2xl shadow-sm border-b-4 border-primary/30 p-4 text-center",
                                p { class: "text-xs font-bold uppercase text-primary/80", "WPM" }
                                p { class: "text-3xl font-black text-base-content", "{wpm_text}" }
                            }
                            div {
                                class: "bg-white rounded-2xl shadow-sm border-b-4 border-accent/60 p-4 text-center",
                                p { class: "text-xs font-bold uppercase text-accent/90", "Accuracy" }
                                p { class: "text-3xl font-black text-primary", "{accuracy_text}" }
                            }
                            div {
                                class: "bg-white rounded-2xl shadow-sm border-b-4 border-primary/30 p-4 text-center",
                                p { class: "text-xs font-bold uppercase text-primary/80", "Level" }
                                p { class: "text-3xl font-black text-base-content", "4/10" }
                            }
                        }

                        TabContent {
                            class: "app-tab-content mt-2",
                            index: 0usize,
                            value: AppTab::Practice.to_string(),
                            PracticeInterface { session: session }
                        }

                        TabContent {
                            class: "app-tab-content mt-2",
                            index: 1usize,
                            value: AppTab::Statistics.to_string(),
                            div {
                                class: "space-y-6",
                                StatisticsDisplay {
                                    stats: HistoryManager::get_statistics()
                                }

                                div {
                                    class: "flex flex-wrap gap-3",
                                    Button {
                                        variant: ButtonVariant::Ghost,
                                        onclick: move |_| current_tab.set(Some(AppTab::Practice.to_string())),
                                        "Back to Practice"
                                    }
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        onclick: move |_| show_reset_dialog.set(true),
                                        "Reset All Progress"
                                    }
                                }
                            }
                        }
                    }

                    AlertDialogRoot {
                        open: show_reset_dialog(),
                        on_open_change: move |open| show_reset_dialog.set(open),
                        AlertDialogContent {
                            AlertDialogTitle { class: "text-lg font-semibold", "Reset all progress?" }
                            AlertDialogDescription {
                                class: "text-sm text-base-content/70",
                                "This will permanently remove all saved practice sessions and statistics."
                            }
                            AlertDialogActions {
                                AlertDialogCancel { "Cancel" }
                                AlertDialogAction {
                                    on_click: move |_| {
                                        HistoryManager::clear_history();
                                        current_tab.set(Some(AppTab::Practice.to_string()));
                                        toast_api.warning(
                                            "All progress has been reset.".to_string(),
                                            ToastOptions::new().description("Your local history was cleared.")
                                        );
                                        show_reset_dialog.set(false);
                                    },
                                    "Reset Everything"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
