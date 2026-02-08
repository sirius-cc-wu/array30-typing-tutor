use crate::components::badge::{Badge, BadgeVariant};
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::separator::Separator;
use crate::storage::Statistics;
use dioxus::prelude::*;

#[component]
pub fn StatisticsDisplay(stats: Statistics) -> Element {
    rsx! {
        div {
            class: "statistics-layout",

            h2 { class: "statistics-title", "Performance Overview" }

            if stats.total_sessions > 0 {
                div {
                    class: "statistics-grid",

                    PremiumStatCard {
                        label: "Total Sessions",
                        value: format!("{}", stats.total_sessions),
                        subtext: "Sessions completed"
                    }

                    PremiumStatCard {
                        label: "Best Speed",
                        value: format!("{:.1} WPM", stats.best_wpm),
                        subtext: "Your all-time peak"
                    }

                    PremiumStatCard {
                        label: "Avg Speed",
                        value: format!("{:.1} WPM", stats.average_wpm),
                        subtext: "Overall average"
                    }

                    PremiumStatCard {
                        label: "Max Accuracy",
                        value: format!("{:.1}%", stats.best_accuracy),
                        subtext: "Highest precision"
                    }

                    PremiumStatCard {
                        label: "Avg Accuracy",
                        value: format!("{:.1}%", stats.average_accuracy),
                        subtext: "Consistency score"
                    }

                    PremiumStatCard {
                        label: "Total Practice",
                        value: format_time(stats.total_practice_time),
                        subtext: "Time on keys"
                    }
                }
            } else {
                Card {
                    class: "statistics-empty",
                    CardHeader {
                        class: "statistics-empty-header",
                        CardTitle { class: "statistics-empty-title", "No data yet" }
                        CardDescription {
                            class: "statistics-empty-description",
                            "Start your first practice session to see your typing statistics and track your progress over time."
                        }
                    }
                    CardContent {
                        class: "statistics-empty-badge",
                        Badge {
                            variant: BadgeVariant::Outline,
                            "Waiting for first session"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PremiumStatCard(label: &'static str, value: String, subtext: &'static str) -> Element {
    rsx! {
        Card {
            class: "statistics-card",
            CardHeader {
                class: "statistics-card-header",
                div { class: "statistics-card-header-row",
                    Badge {
                        variant: BadgeVariant::Secondary,
                        class: "statistics-card-label",
                        "{label}"
                    }
                }
            }
            Separator { horizontal: true }
            CardContent {
                class: "statistics-card-content",
                h3 { class: "statistics-card-value", "{value}" }
                p { class: "statistics-card-subtext", "{subtext}" }
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
