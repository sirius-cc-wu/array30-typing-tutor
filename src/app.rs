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
    let stats = session.read().stats.clone();
    let wpm = stats.wpm();
    let accuracy = stats.accuracy();
    let wpm_text = format!("{wpm:.0}");
    let accuracy_text = format!("{accuracy:.0}%");

    rsx! {
        main {
            class: "app-shell",

            div {
                class: "app-shell-card",
                div {
                    class: "app-shell-content",

                    Separator { horizontal: true }

                    // Top Stats
                    div {
                        class: "practice-metrics app-top-metrics",
                        div {
                            class: "metric-card",
                            "data-type": "wpm",
                            span { class: "metric-card-label", "WPM" }
                            span { class: "metric-card-value", "{wpm_text}" }
                        }
                        div {
                            class: "metric-card",
                            "data-type": "accuracy",
                            span { class: "metric-card-label", "Accuracy" }
                            span { class: "metric-card-value", "{accuracy_text}" }
                        }
                        div {
                            class: "metric-card",
                            "data-type": "progress",
                            span { class: "metric-card-label", "Level" }
                            span { class: "metric-card-value", "4/10" }
                        }
                    }

                    // Main Content Area
                    Tabs {
                        class: "app-tabs",
                        value: current_tab,
                        on_value_change: move |value| current_tab.set(Some(value)),

                        div {
                            class: "mode-switch-shell",
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
                        }

                        // Title under mode switch
                        header {
                            class: "app-header",
                            div {
                                h1 { class: "app-title text-gradient", "Array30" }
                                p { class: "app-subtitle", "Master the art of typing" }
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
                                class: "stats-tab-content",
                                StatisticsDisplay {
                                    stats: HistoryManager::get_statistics()
                                }

                                div {
                                    class: "stats-tab-actions",
                                    Button {
                                        variant: ButtonVariant::Ghost,
                                        onclick: move |_| current_tab.set(Some(AppTab::Practice.to_string())),
                                        "Back to Practice"
                                    }
                                    Button {
                                        class: "reset-progress-action",
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
