use dioxus::prelude::*;
use crate::logic::PracticeSession;
use crate::components::StatsDisplay;
use crate::storage::{HistoryManager, SessionRecord};
use js_sys;

#[component]
pub fn PracticeInterface(mut session: Signal<PracticeSession>) -> Element {
    let mut user_input = use_signal(|| String::new());
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

        // Check completion on every input
        let sess = session.read();
        if value.len() > 0 && sess.target_text.len() > 0 {
            let match_count = value.chars().zip(sess.target_text.chars())
                .filter(|(a, b)| a == b)
                .count();
            
            if match_count >= sess.target_text.len() * 95 / 100 {
                show_completion.set(true);
            } else {
                show_completion.set(false);
            }
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

    rsx! {
        div {
            class: "container mx-auto px-4 py-8 max-w-2xl",
            div {
                class: "mb-8",
                h1 {
                    class: "text-4xl font-bold text-gray-800 mb-2",
                    "Array30 Typing Tutor"
                }
                p {
                    class: "text-gray-600",
                    "Practice typing with the Array30 input method"
                }
            }

            {
                let stats = session.read().stats.clone();
                rsx! {
                    StatsDisplay {
                        stats: stats
                    }
                }
            }

            div {
                class: "bg-white rounded-lg shadow-lg p-8 mb-6",
                {
                    let session_read = session.read();
                    let target_text = session_read.target_text.clone();
                    let input_text = user_input.read().clone();
                    let input_len = input_text.len();
                    let target_len = target_text.len();
                    let progress = if target_len == 0 {
                        0.0
                    } else {
                        (input_len as f64 / target_len as f64) * 100.0
                    };

                    rsx! {
                        div {
                            class: "mb-6",
                            p {
                                class: "text-xl text-gray-800 font-mono mb-4 leading-relaxed",
                                "{target_text}"
                            }
                            div {
                                class: "w-full bg-gray-200 rounded-full h-2",
                                div {
                                    class: "bg-blue-600 h-2 rounded-full transition-all",
                                    style: "width: {progress}%"
                                }
                            }
                        }
                    }
                }

                div {
                    class: "mb-6",
                    textarea {
                        class: "w-full h-32 p-4 border-2 border-gray-300 rounded-lg focus:outline-none focus:border-blue-500 font-mono",
                        placeholder: "Start typing here...",
                        value: "{user_input}",
                        oninput: handle_input,
                        autofocus: true
                    }
                }

                div {
                    class: "flex gap-4",
                    button {
                        class: if *show_completion.read() {
                            "flex-1 bg-green-600 hover:bg-green-700 text-white font-bold py-3 px-6 rounded-lg transition"
                        } else {
                            "flex-1 bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-6 rounded-lg transition"
                        },
                        onclick: handle_next,
                        if *show_completion.read() { "✅ Save & Next" } else { "Next Exercise" }
                    }
                    button {
                        class: "flex-1 bg-gray-400 hover:bg-gray-500 text-white font-bold py-3 px-6 rounded-lg transition",
                        onclick: handle_reset,
                        "Reset"
                    }
                }
            }

            if session.read().started {
                div {
                    class: "bg-blue-50 border-l-4 border-blue-500 p-4 rounded",
                    p {
                        class: "text-blue-700",
                        "Session in progress... Type to practice!"
                    }
                }
            }

            if *show_completion.read() {
                div {
                    class: "bg-green-50 border-l-4 border-green-500 p-4 rounded mt-4",
                    p {
                        class: "text-green-700 font-bold",
                        "🎉 Great job! Click 'Save & Next' to record this session and move to the next exercise."
                    }
                }
            }
        }
    }
}

fn save_current_session(session: &crate::logic::PracticeSession) {
    let wpm = if session.stats.elapsed_seconds > 0 {
        (session.stats.characters_typed as f64 / 5.0) / (session.stats.elapsed_seconds as f64 / 60.0)
    } else {
        0.0
    };

    let accuracy = if session.stats.total_typed > 0 {
        ((session.stats.total_typed - session.stats.errors) as f64 / session.stats.total_typed as f64) * 100.0
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
