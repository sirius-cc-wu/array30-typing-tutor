use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{
    Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::components::progress::{Progress, ProgressIndicator};
use crate::logic::PracticeSession;
use crate::storage::{HistoryManager, SessionRecord};
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

#[component]
pub fn PracticeInterface(mut session: Signal<PracticeSession>) -> Element {
    let toast_api = use_toast();
    let mut user_input = use_signal(String::new);
    let mut start_time_ms = use_signal(|| 0u64);
    let mut show_completion = use_signal(|| false);

    let handle_input = move |event: Event<FormData>| {
        let value = event.value();
        user_input.set(value.clone());

        if !session.read().started {
            session.write().start();
            start_time_ms.set(js_sys::Date::now() as u64);
        }

        let elapsed = (js_sys::Date::now() as u64).saturating_sub(*start_time_ms.read());
        session.write().update_input(&value, elapsed);

        let sess = session.read();
        let target_char_count = sess.target_text.chars().count();
        let input_char_count = value.chars().count();
        if target_char_count > 0 {
            let matches_target = input_char_count == target_char_count
                && value
                    .chars()
                    .zip(sess.target_text.chars())
                    .all(|(a, b)| a == b);
            show_completion.set(matches_target);
        } else {
            show_completion.set(false);
        }
    };

    let handle_reset = move |_| {
        session.set(PracticeSession::new());
        user_input.set(String::new());
        start_time_ms.set(0);
        show_completion.set(false);
    };

    let handle_next = move |_| {
        if *show_completion.read() {
            save_current_session(&session.read());
            toast_api.success(
                "Session saved".to_string(),
                ToastOptions::new().description("Progress recorded. Loading next challenge."),
            );
        }

        let mut new_session = session.read().clone();
        new_session.next_exercise();
        session.set(new_session);

        user_input.set(String::new());
        start_time_ms.set(0);
        show_completion.set(false);
    };

    let stats = session.read().stats.clone();
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
    let input_char_count = user_input.read().chars().count();
    let target_char_count = session.read().target_text.chars().count();
    let progress_value = if target_char_count == 0 {
        0.0
    } else {
        (input_char_count as f64 / target_char_count as f64 * 100.0).min(100.0)
    };

    rsx! {
        div {
            class: "practice-layout",

            // Statistics Grid
            div {
                class: "practice-metrics",
                StatCard { label: "WPM", value: format!("{:.1}", wpm), color: "metric-value-speed" }
                StatCard { label: "Accuracy", value: format!("{:.1}%", accuracy), color: "metric-value-accuracy" }
                StatCard { label: "Time", value: format!("{}s", stats.elapsed_seconds), color: "metric-value-time" }
            }

            // Typing Exercise Card
            Card {
                class: "exercise-card",
                CardHeader {
                    class: "exercise-header",
                    CardTitle {
                        class: "exercise-title",
                        "Current Exercise"
                    }
                    div {
                        class: "exercise-meta",
                        CardDescription {
                            class: "exercise-progress-count",
                            "{input_char_count} / {target_char_count}"
                        }
                        Badge {
                            variant: if *show_completion.read() { BadgeVariant::Primary } else { BadgeVariant::Outline },
                            if *show_completion.read() { "Ready to save" } else { "In progress" }
                        }
                    }
                }
                CardContent {
                    class: "exercise-content",
                    div {
                        class: "exercise-badges",
                        Badge {
                            variant: BadgeVariant::Secondary,
                            {format!("WPM {:.1}", wpm)}
                        }
                        Badge {
                            variant: BadgeVariant::Outline,
                            {format!("Accuracy {:.1}%", accuracy)}
                        }
                        if stats.errors > 0 {
                            Badge {
                                variant: BadgeVariant::Destructive,
                                {format!("{} errors", stats.errors)}
                            }
                        }
                    }
                    div {
                        class: "typing-area exercise-text",
                        {
                            let target = session.read().target_text.clone();
                            let input = user_input.read().clone();
                            let input_chars: Vec<char> = input.chars().collect();

                            rsx! {
                                for (i, c) in target.chars().enumerate() {
                                    {
                                        let class = if i < input_chars.len() {
                                            if input_chars[i] == c { "char-correct" } else { "char-incorrect" }
                                        } else {
                                            "char-untyped"
                                        };
                                        rsx! { span { class: "{class}", "{c}" } }
                                    }
                                }
                            }
                        }
                    }

                    Progress {
                        style: "width: 100%",
                        value: Some(progress_value),
                        ProgressIndicator {}
                    }
                }

                // Hidden but functional textarea
                div {
                    class: "typing-input-wrap",
                    textarea {
                        class: "typing-input",
                        placeholder: "Focus here and start typing...",
                        value: "{user_input}",
                        oninput: handle_input,
                        autofocus: true
                    }
                    div { class: "typing-input-ring" }
                }

                CardFooter {
                    class: "exercise-actions",
                    Button {
                        class: "exercise-primary-action",
                        variant: if *show_completion.read() {
                            ButtonVariant::Secondary
                        } else {
                            ButtonVariant::Ghost
                        },
                        onclick: handle_next,
                        if *show_completion.read() { "Save & Next Challenge" } else { "Skip to Next Exercise" }
                    }
                    Button {
                        class: "exercise-secondary-action",
                        variant: ButtonVariant::Outline,
                        onclick: handle_reset,
                        "Reset"
                    }
                }
            }

            if session.read().started && !*show_completion.read() {
                div {
                    class: "recording-status",
                    Badge {
                        variant: BadgeVariant::Secondary,
                        "Recording session..."
                    }
                }
            }

            if *show_completion.read() {
                div {
                    class: "completion-banner",
                    div {
                        h4 { class: "completion-title", "Excellent Accuracy!" }
                        p { class: "completion-text", "You've mastered this exercise. Save your progress to continue." }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(label: String, value: String, color: String) -> Element {
    rsx! {
        div {
            class: "metric-card",
            span { class: "metric-label", "{label}" }
            span { class: "metric-value {color}", "{value}" }
        }
    }
}

fn save_current_session(session: &crate::logic::PracticeSession) {
    let wpm = if session.stats.elapsed_seconds > 0 {
        (session.stats.characters_typed as f64 / 5.0)
            / (session.stats.elapsed_seconds as f64 / 60.0)
    } else {
        0.0
    };

    let accuracy = if session.stats.total_typed > 0 {
        ((session.stats.total_typed - session.stats.errors) as f64
            / session.stats.total_typed as f64)
            * 100.0
    } else {
        100.0
    };

    let record = SessionRecord {
        wpm,
        accuracy,
        timestamp: format_timestamp(),
        elapsed_seconds: session.stats.elapsed_seconds,
        exercise_text: session.target_text.clone(),
    };

    HistoryManager::save_session(record);
}

fn format_timestamp() -> String {
    use js_sys::Date;
    let date = Date::new_0();
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        date.get_full_year(),
        date.get_month() + 1,
        date.get_date(),
        date.get_hours(),
        date.get_minutes(),
        date.get_seconds()
    )
}
