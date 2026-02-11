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
        document::Link { rel: "stylesheet", href: asset!("../assets/styles.css") }
        document::Link { rel: "stylesheet", href: asset!("../assets/dx-components-theme.css") }
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

    rsx! {
        main {
            class: "flex h-screen w-screen flex-col overflow-hidden",

            div {
                class: "flex flex-1 flex-col overflow-x-hidden overflow-y-auto bg-indigo-50 [background-image:radial-gradient(circle_at_20%_20%,rgba(72,72,229,0.06),transparent_50%),radial-gradient(circle_at_80%_80%,rgba(249,115,22,0.05),transparent_50%)]",
                div {
                    class: "mx-auto grid w-full max-w-[1400px] gap-10 overflow-y-auto bg-transparent px-6 py-8 md:px-16",

                    // Header
                    header {
                        class: "flex flex-col items-start justify-between gap-4 pt-4 md:flex-row md:items-end",

                        div {
                            h1 {
                                class: "m-0 bg-gradient-to-br from-indigo-950 to-indigo-600 bg-clip-text text-[clamp(3rem,7vw,4rem)] font-bold leading-[0.9] tracking-[-0.06em] text-transparent",
                                "Array30"
                            }
                            p {
                                class: "mt-4 text-[1.35rem] font-semibold tracking-[-0.02em] text-slate-500",
                                "Master the art of typing"
                            }
                        }
                    }

                    Separator { horizontal: true }

                    // Main Content Area
                    Tabs {
                        class: "px-0 md:px-2",
                        value: current_tab,
                        on_value_change: move |value| current_tab.set(Some(value)),

                        TabList {
                            TabTrigger {
                                index: 0usize,
                                value: AppTab::Practice.to_string(),
                                "Practice"
                            }
                            TabTrigger {
                                index: 1usize,
                                value: AppTab::Statistics.to_string(),
                                "Statistics"
                            }
                        }

                        TabContent {
                            index: 0usize,
                            value: AppTab::Practice.to_string(),
                            PracticeInterface { session: session }
                        }

                        TabContent {
                            index: 1usize,
                            value: AppTab::Statistics.to_string(),
                            div {
                                class: "grid gap-10",
                                StatisticsDisplay {
                                    stats: HistoryManager::get_statistics()
                                }

                                div {
                                    class: "flex flex-col justify-center gap-4 pt-4 sm:flex-row",
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
                            AlertDialogTitle { "Reset all progress?" }
                            AlertDialogDescription {
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
