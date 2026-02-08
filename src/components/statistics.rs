use crate::components::badge::{Badge, BadgeVariant};
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::separator::Separator;
use crate::storage::Statistics;
use dioxus::prelude::*;

#[component]
pub fn StatisticsDisplay(stats: Statistics) -> Element {
    rsx! {
        div {
            class: "space-y-8 animate-in fade-in duration-700",

            h2 { class: "text-2xl font-bold text-slate-800 tracking-tight flex items-center gap-2",
                span { "📈" }
                "Performance Overview"
            }

            if stats.total_sessions > 0 {
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",

                    PremiumStatCard {
                        label: "Total Sessions",
                        value: format!("{}", stats.total_sessions),
                        icon: "🔥",
                        subtext: "Sessions completed"
                    }

                    PremiumStatCard {
                        label: "Best Speed",
                        value: format!("{:.1} WPM", stats.best_wpm),
                        icon: "⚡",
                        subtext: "Your all-time peak"
                    }

                    PremiumStatCard {
                        label: "Avg Speed",
                        value: format!("{:.1} WPM", stats.average_wpm),
                        icon: "📊",
                        subtext: "Overall average"
                    }

                    PremiumStatCard {
                        label: "Max Accuracy",
                        value: format!("{:.1}%", stats.best_accuracy),
                        icon: "🎯",
                        subtext: "Highest precision"
                    }

                    PremiumStatCard {
                        label: "Avg Accuracy",
                        value: format!("{:.1}%", stats.average_accuracy),
                        icon: "✅",
                        subtext: "Consistency score"
                    }

                    PremiumStatCard {
                        label: "Total Practice",
                        value: format_time(stats.total_practice_time),
                        icon: "⌛",
                        subtext: "Time on keys"
                    }
                }
            } else {
                Card {
                    class: "glass-card",
                    CardHeader {
                        class: "text-center",
                        CardTitle { class: "text-xl font-bold text-slate-800", "No data yet" }
                        CardDescription {
                            class: "text-slate-500 max-w-sm mx-auto",
                            "Start your first practice session to see your typing statistics and track your progress over time."
                        }
                    }
                    CardContent {
                        class: "flex justify-center pb-8",
                        Badge {
                            variant: BadgeVariant::Outline,
                            "⌨️ Waiting for first session"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PremiumStatCard(label: String, value: String, icon: String, subtext: String) -> Element {
    rsx! {
        Card {
            class: "glass-card p-6 space-y-4 hover:scale-[1.02] transition-all duration-300",
            CardHeader {
                class: "space-y-3",
                div { class: "flex justify-between items-center",
                    div { class: "p-2 bg-indigo-50 rounded-xl text-xl", "{icon}" }
                    Badge {
                        variant: BadgeVariant::Secondary,
                        class: "uppercase tracking-widest text-[10px]",
                        "{label}"
                    }
                }
            }
            Separator { horizontal: true }
            CardContent {
                class: "space-y-1",
                h3 { class: "text-2xl font-black text-gradient", "{value}" }
                p { class: "text-xs text-slate-400 font-medium", "{subtext}" }
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
