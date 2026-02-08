use crate::components::button::{Button, ButtonVariant};
use crate::logic::PracticeSession;
use crate::storage::{HistoryManager, SessionRecord};
use dioxus::prelude::*;

#[component]
pub fn PracticeInterface(mut session: Signal<PracticeSession>) -> Element {
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

    rsx! {
        div {
            class: "space-y-8",

            // Statistics Grid
            div {
                class: "grid grid-cols-3 gap-4",
                StatCard { label: "WPM", value: format!("{:.1}", wpm), color: "text-indigo-600" }
                StatCard { label: "Accuracy", value: format!("{:.1}%", accuracy), color: "text-emerald-600" }
                StatCard { label: "Time", value: format!("{}s", stats.elapsed_seconds), color: "text-purple-600" }
            }

            // Typing Exercise Card
            div {
                class: "glass-card p-8 space-y-8",

                div {
                    class: "space-y-4",
                    div { class: "flex justify-between items-end",
                        span { class: "text-xs font-bold uppercase tracking-widest text-slate-400", "Current Exercise" }
                        {
                            let input_char_count = user_input.read().chars().count();
                            let target_char_count = session.read().target_text.chars().count();
                            rsx! {
                                span { class: "text-xs font-bold text-indigo-600",
                                    "{input_char_count} / {target_char_count}"
                                }
                            }
                        }
                    }

                    // The "Alive" Typing Display
                    div {
                        class: "typing-area p-6 bg-slate-50/50 rounded-2xl border border-slate-100",
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

                    // Elegant Progress Bar
                    div {
                        class: "w-full bg-slate-100 rounded-full h-1.5 overflow-hidden",
                        div {
                            class: "bg-gradient-to-r from-indigo-500 to-purple-500 h-full transition-all duration-300 shadow-[0_0_8px_rgba(99,102,241,0.5)]",
                            style: {
                                let input_char_count = user_input.read().chars().count();
                                let target_char_count = session.read().target_text.chars().count().max(1);
                                let width = ((input_char_count as f64 / target_char_count as f64) * 100.0).min(100.0);
                                format!("width: {width}%")
                            }
                        }
                    }
                }

                // Hidden but functional textarea
                div {
                    class: "group relative",
                    textarea {
                        class: "w-full h-32 p-6 bg-transparent border-2 border-slate-100 rounded-2xl focus:border-indigo-500/50 focus:outline-none transition-all duration-300 font-mono text-lg placeholder:text-slate-300",
                        placeholder: "Focus here and start typing...",
                        value: "{user_input}",
                        oninput: handle_input,
                        autofocus: true
                    }
                    div { class: "absolute inset-0 pointer-events-none rounded-2xl ring-4 ring-indigo-500/0 group-focus-within:ring-indigo-500/5 transition-all duration-500" }
                }

                // Action Buttons
                div {
                    class: "flex gap-4",
                    Button {
                        class: if *show_completion.read() { "flex-[2] btn-primary bg-emerald-600 border-emerald-800 hover:bg-emerald-700" } else { "flex-[2] btn-primary" },
                        variant: if *show_completion.read() { ButtonVariant::Secondary } else { ButtonVariant::Primary },
                        onclick: handle_next,
                        if *show_completion.read() { "✅ Save & Next Challenge" } else { "Next Exercise" }
                    }
                    Button {
                        class: "flex-1 btn-ghost border border-slate-200",
                        variant: ButtonVariant::Outline,
                        onclick: handle_reset,
                        "Reset"
                    }
                }
            }

            if session.read().started && !*show_completion.read() {
                div {
                    class: "flex items-center justify-center gap-2 text-indigo-500 animate-pulse",
                    span { class: "w-2 h-2 bg-indigo-500 rounded-full" }
                    span { class: "text-sm font-semibold tracking-wide uppercase", "Recording Session..." }
                }
            }

            if *show_completion.read() {
                div {
                    class: "glass-card p-6 bg-emerald-50/50 border-emerald-100 flex items-center gap-4 animate-in fade-in slide-in-from-bottom-4 duration-500",
                    div { class: "text-3xl", "🎯" }
                    div {
                        h4 { class: "text-emerald-900 font-bold", "Excellent Accuracy!" }
                        p { class: "text-emerald-700 text-sm", "You've mastered this exercise. Save your progress to continue." }
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
            class: "glass-card p-4 flex flex-col items-center justify-center space-y-1 hover:scale-105 transition-transform duration-300",
            span { class: "text-[10px] font-bold uppercase tracking-widest text-slate-400", "{label}" }
            span { class: "text-2xl font-black {color}", "{value}" }
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
