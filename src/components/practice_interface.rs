use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::{Button, ButtonVariant};

use crate::array30_data;
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
            start_time_ms.set(chrono::Utc::now().timestamp_millis() as u64);
        }

        let elapsed =
            (chrono::Utc::now().timestamp_millis() as u64).saturating_sub(*start_time_ms.read());
        session.write().update_input(&value, elapsed);

        let target_text = session.read().target_text.clone();
        let target_char_count = target_text.chars().count();
        let input_char_count = value.chars().count();
        if target_char_count > 0 {
            let matches_target = input_char_count == target_char_count
                && value
                    .chars()
                    .zip(target_text.chars())
                    .all(|(a, b)| a == b);
            if matches_target {
                show_completion.set(true);
                save_current_session(&session.read());
                toast_api.success(
                    "Session saved".to_string(),
                    ToastOptions::new().description("Progress recorded. Loading next challenge."),
                );

                let mut new_session = session.read().clone();
                new_session.next_exercise();
                session.set(new_session);

                user_input.set(String::new());
                start_time_ms.set(0);
                show_completion.set(false);
            } else {
                show_completion.set(false);
            }
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

    let (next_char, next_char_hint) = {
        let input = user_input.read().clone();
        let target = session.read().target_text.clone();
        let matched_prefix_count = input
            .chars()
            .zip(target.chars())
            .take_while(|(typed, expected)| typed == expected)
            .count();

        let next_char = target.chars().nth(matched_prefix_count);
        let next_char_hint = next_char.and_then(|c| {
            // Keep hint anchored on the next expected character while input is composing/mismatched.
            array30_data::get_array30_code(c).map(|code| (c, code))
        });

        (next_char, next_char_hint)
    };

    rsx! {
        div {
            class: "space-y-6 practice-root",

            // Typing Exercise Area
            div {
                class: "grid gap-6 lg:grid-cols-[minmax(0,1fr)_320px] practice-main-grid",

                // Typing Area Wrapper
                div {
                    class: "bg-white rounded-3xl shadow-lg border border-primary/10 p-6 relative practice-card",
                    div {
                        class: "space-y-6 practice-card-body",
                        div {
                            class: "flex items-center justify-between",
                            div {
                                class: "flex items-center gap-2 rounded-full bg-primary/10 px-3 py-1 text-xs font-bold uppercase tracking-widest text-primary",
                                span { class: "inline-flex h-2 w-2 rounded-full bg-primary" }
                                "Live Practice"
                            }
                            if session.read().started && !*show_completion.read() {
                                Badge {
                                    variant: BadgeVariant::Secondary,
                                    "Recording session..."
                                }
                            }
                        }
                        div {
                            class: "typing-area",
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
                                            rsx! { span { key: "{i}", class: "{class}", "{c}" } }
                                        }
                                    }
                                }
                            }
                        }

                        // Hidden but functional textarea
                        div {
                            class: "form-control pb-2",
                            textarea {
                                class: "textarea textarea-bordered h-32 w-full typing-input text-base resize-none",
                                placeholder: "Focus here and start typing...",
                                value: "{user_input}",
                                oninput: handle_input,
                                autofocus: true
                            }
                        }
                    }
                }

                // Hint Box
                div {
                    class: "bg-accent/15 border-2 border-dashed border-accent/60 rounded-3xl p-6 flex flex-col justify-between gap-6",
                    div {
                        class: "space-y-2",
                        h3 { class: "text-xs font-bold uppercase tracking-[0.25em] text-base-content/60", "Next Key Hint" }
                        if let Some((c, code)) = next_char_hint {
                            div {
                                class: "code-hint-row",
                                span { class: "code-hint-char", "{c}" }
                                span { class: "code-hint-arrow", "→" }
                                CodeDisplay { code: code }
                            }
                        } else if let Some(c) = next_char {
                            div {
                                class: "code-hint-row",
                                span { class: "code-hint-char", "{c}" }
                            }
                        } else {
                            // Completed or empty
                            div {
                                class: "code-hint-complete",
                                "Exercise Complete"
                            }
                        }
                    }
                    div {
                        class: "rounded-2xl bg-white/80 border border-accent/40 px-4 py-3 text-xs font-bold uppercase tracking-widest text-base-content/70 text-center",
                        "Array30 Code"
                    }
                }
            }

            // Action Footer (Outside the white card)
            div {
                class: "flex flex-wrap items-center gap-3 pt-1",
                Button {
                    class: "btn-lg shadow-lg shadow-primary/30 text-lg px-8",
                    variant: ButtonVariant::Primary,
                    onclick: handle_next,
                    span {
                        class: "flex items-center gap-4",
                        svg {
                            width: "24",
                            height: "24",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            path { d: "M8 5v14l11-7z" }
                        }
                        if *show_completion.read() { "Save & Next Lesson" } else { "Skip to Next Lesson" }
                    }
                }
                Button {
                    class: "btn-lg btn-circle shadow-md",
                    variant: ButtonVariant::Secondary,
                    onclick: handle_reset,
                    svg {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "3",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M23 4v6h-6" }
                        path { d: "M20.49 15a9 9 0 1 1-2.12-9.36L23 10" }
                    }
                }
            }


        }
    }
}

#[component]
fn CodeDisplay(code: &'static str) -> Element {
    // Split codes by pipe if multiple
    let codes: Vec<&str> = code.split('|').collect();

    rsx! {
        span {
            class: "code-hint-keycap",
            { codes.join(" / ") }
        }
    }
}

fn save_current_session(session: &crate::logic::PracticeSession) {
    let wpm = session.stats.wpm();
    let accuracy = session.stats.accuracy();

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
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
