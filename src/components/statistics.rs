use crate::components::badge::{Badge, BadgeVariant};
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::separator::Separator;
use crate::storage::Statistics;
use dioxus::prelude::*;

#[component]
pub fn StatisticsDisplay(stats: Statistics) -> Element {
    rsx! {
        div {
            class: "grid gap-10 px-0 md:px-2",

            h2 { class: "m-0 text-[2.5rem] font-bold tracking-[-0.05em]", "Performance Overview" }

            if stats.total_sessions > 0 {
                div {
                    class: "grid gap-8 md:grid-cols-3",

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
                    class: "rounded-[2rem] p-16",
                    CardHeader {
                        class: "text-center",
                        CardTitle { class: "m-0 text-[2rem] font-bold", "No data yet" }
                        CardDescription {
                            class: "mx-auto mt-6 max-w-[45ch] text-[1.25rem] text-slate-500",
                            "Start your first practice session to see your typing statistics and track your progress over time."
                        }
                    }
                    CardContent {
                        class: "flex justify-center pt-8",
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
            class: "rounded-[2rem] transition-all duration-300 hover:-translate-y-2 hover:scale-[1.02]",
            CardHeader {
                class: "grid gap-4",
                div { class: "flex items-center justify-between",
                    Badge {
                        variant: BadgeVariant::Secondary,
                        class: "text-[0.85rem] uppercase tracking-[0.15em]",
                        "{label}"
                    }
                }
            }
            Separator { horizontal: true }
            CardContent {
                class: "grid gap-3",
                h3 { class: "m-0 text-[2.25rem] font-bold leading-tight text-indigo-600", "{value}" }
                p { class: "m-0 text-base font-semibold text-slate-500", "{subtext}" }
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
