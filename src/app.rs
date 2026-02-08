use crate::components::alert_dialog::{
    AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogContent,
    AlertDialogDescription, AlertDialogRoot, AlertDialogTitle,
};
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{Card, CardContent};
use crate::components::separator::Separator;
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs, TabsVariant};
use crate::components::toast::ToastProvider;
use crate::components::{PracticeInterface, StatisticsDisplay};
use crate::logic::PracticeSession;
use crate::storage::HistoryManager;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

pub fn app() -> Element {
    rsx! {
        ToastProvider {
            AppContent {}
        }
    }
}

#[component]
fn AppContent() -> Element {
    let toast_api = use_toast();
    let mut current_tab = use_signal(|| Some("practice".to_string()));
    let mut show_reset_dialog = use_signal(|| false);
    let session = use_signal(PracticeSession::new);

    rsx! {
        main {
            class: "min-h-screen py-12 px-4 flex flex-col items-center",

            Card {
                class: "w-full max-w-4xl app-shell-card",
                CardContent {
                    class: "space-y-8 py-8 md:py-10",

                    // Header
                    header {
                        class: "flex flex-col md:flex-row md:items-center justify-between gap-6 px-2 md:px-4",

                        div {
                            h1 { class: "text-4xl font-extrabold text-gradient tracking-tight", "Array30" }
                            p { class: "text-slate-500 font-medium", "Master the art of typing" }
                        }
                    }

                    Separator { horizontal: true }

                    // Main Content Area
                    Tabs {
                        class: "px-2 md:px-4 app-tabs",
                        value: current_tab,
                        on_value_change: move |value| current_tab.set(Some(value)),
                        variant: TabsVariant::Ghost,

                        TabList {
                            class: "app-tablist",
                            TabTrigger {
                                index: 0usize,
                                value: "practice",
                                class: "app-tab-trigger",
                                "📝 Practice"
                            }
                            TabTrigger {
                                index: 1usize,
                                value: "statistics",
                                class: "app-tab-trigger",
                                "📊 Statistics"
                            }
                        }

                        TabContent {
                            index: 0usize,
                            value: "practice",
                            class: "app-tab-content",
                            PracticeInterface { session: session }
                        }

                        TabContent {
                            index: 1usize,
                            value: "statistics",
                            class: "app-tab-content",
                            div {
                                class: "space-y-8",
                                StatisticsDisplay {
                                    stats: HistoryManager::get_statistics()
                                }

                                Card {
                                    class: "glass-card border-rose-100",
                                    CardContent {
                                        class: "flex justify-center gap-3 pt-6",
                                        Button {
                                            class: "btn-ghost border border-slate-200",
                                            variant: ButtonVariant::Ghost,
                                            onclick: move |_| current_tab.set(Some("practice".to_string())),
                                            "Back to Practice"
                                        }
                                        Button {
                                            class: "btn-premium bg-rose-50 text-rose-600 hover:bg-rose-100 border border-rose-200 px-8",
                                            variant: ButtonVariant::Destructive,
                                            onclick: move |_| show_reset_dialog.set(true),
                                            "🗑️ Reset All Progress"
                                        }
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
                                        current_tab.set(Some("practice".to_string()));
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
