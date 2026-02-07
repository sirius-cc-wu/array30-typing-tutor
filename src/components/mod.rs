use dioxus::prelude::*;
use crate::logic::TypingStats;

mod practice_interface;
mod statistics;
pub use practice_interface::PracticeInterface;
pub use statistics::StatisticsDisplay;

#[component]
pub fn StatsDisplay(stats: TypingStats) -> Element {
    let wpm = if stats.elapsed_seconds > 0 {
        (stats.characters_typed as f64 / 5.0) / (stats.elapsed_seconds as f64 / 60.0)
    } else {
        0.0
    };

    let accuracy = if stats.total_typed > 0 {
        ((stats.total_typed - stats.errors) as f64 / stats.total_typed as f64) * 100.0
    } else {
        100.0
    };

    rsx! {
        div {
            class: "bg-white rounded-lg shadow-md p-6 mb-6",
            div {
                class: "grid grid-cols-3 gap-4",
                div {
                    class: "text-center",
                    div {
                        class: "text-4xl font-bold text-blue-600",
                        "{wpm:.1}"
                    }
                    div {
                        class: "text-gray-600 text-sm mt-1",
                        "WPM"
                    }
                }
                div {
                    class: "text-center",
                    div {
                        class: "text-4xl font-bold text-green-600",
                        "{accuracy:.1}%"
                    }
                    div {
                        class: "text-gray-600 text-sm mt-1",
                        "Accuracy"
                    }
                }
                div {
                    class: "text-center",
                    div {
                        class: "text-4xl font-bold text-indigo-600",
                        "{stats.elapsed_seconds}"
                    }
                    div {
                        class: "text-gray-600 text-sm mt-1",
                        "Seconds"
                    }
                }
            }
        }
    }
}
