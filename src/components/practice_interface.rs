use dioxus::prelude::*;
use crate::logic::PracticeSession;
use crate::components::StatsDisplay;
use js_sys;

#[component]
pub fn PracticeInterface() -> Element {
    let mut session = use_signal(|| PracticeSession::new());
    let mut user_input = use_signal(|| String::new());
    let mut start_time_ms = use_signal(|| 0u64);

    let handle_input = move |event: Event<FormData>| {
        let value = event.value();
        user_input.set(value.clone());
        
        if !session.read().started {
            session.write().start();
            start_time_ms.set(js_sys::Date::now() as u64);
        }

        let elapsed = (js_sys::Date::now() as u64).saturating_sub(*start_time_ms.read());
        session.write().update_input(&value, elapsed);
    };

    let handle_reset = move |_| {
        session.set(PracticeSession::new());
        user_input.set(String::new());
        start_time_ms.set(0);
    };

    let handle_next = move |_| {
        session.write().next_exercise();
        user_input.set(String::new());
        start_time_ms.set(0);
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
                        class: "flex-1 bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-6 rounded-lg transition",
                        onclick: handle_next,
                        "Next Exercise"
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
        }
    }
}
