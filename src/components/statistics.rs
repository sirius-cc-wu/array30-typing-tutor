use dioxus::prelude::*;
use crate::storage::Statistics;

#[component]
pub fn StatisticsDisplay(stats: Statistics) -> Element {
    rsx! {
        div {
            class: "bg-gradient-to-r from-purple-50 to-blue-50 rounded-lg shadow-md p-6 mb-6 border border-purple-200",
            div {
                class: "mb-4",
                h2 {
                    class: "text-2xl font-bold text-gray-800 mb-2",
                    "📊 Your Statistics"
                }
            }

            if stats.total_sessions > 0 {
                div {
                    class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                    
                    div {
                        class: "bg-white rounded p-3 shadow-sm",
                        div {
                            class: "text-sm text-gray-600",
                            "Total Sessions"
                        }
                        div {
                            class: "text-2xl font-bold text-purple-600",
                            "{stats.total_sessions}"
                        }
                    }
                    
                    div {
                        class: "bg-white rounded p-3 shadow-sm",
                        div {
                            class: "text-sm text-gray-600",
                            "Best WPM"
                        }
                        div {
                            class: "text-2xl font-bold text-blue-600",
                            "{stats.best_wpm:.1}"
                        }
                    }
                    
                    div {
                        class: "bg-white rounded p-3 shadow-sm",
                        div {
                            class: "text-sm text-gray-600",
                            "Average WPM"
                        }
                        div {
                            class: "text-2xl font-bold text-indigo-600",
                            "{stats.average_wpm:.1}"
                        }
                    }
                    
                    div {
                        class: "bg-white rounded p-3 shadow-sm",
                        div {
                            class: "text-sm text-gray-600",
                            "Best Accuracy"
                        }
                        div {
                            class: "text-2xl font-bold text-green-600",
                            "{stats.best_accuracy:.1}%"
                        }
                    }
                    
                    div {
                        class: "bg-white rounded p-3 shadow-sm",
                        div {
                            class: "text-sm text-gray-600",
                            "Avg Accuracy"
                        }
                        div {
                            class: "text-2xl font-bold text-emerald-600",
                            "{stats.average_accuracy:.1}%"
                        }
                    }
                    
                    div {
                        class: "bg-white rounded p-3 shadow-sm",
                        div {
                            class: "text-sm text-gray-600",
                            "Total Time"
                        }
                        div {
                            class: "text-2xl font-bold text-orange-600",
                            {format_time(stats.total_practice_time)}
                        }
                    }
                }
            } else {
                div {
                    class: "text-center py-8 text-gray-500",
                    p {
                        "No sessions yet. Start practicing to build your statistics!"
                    }
                }
            }
        }
    }
}

fn format_time(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}
