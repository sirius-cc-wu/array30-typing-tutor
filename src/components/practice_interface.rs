use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{
    Card, CardContent, CardDescription, CardHeader, CardTitle,
};
use crate::components::progress::{Progress, ProgressIndicator};
use crate::logic::PracticeSession;
use crate::storage::{HistoryManager, SessionRecord};
use crate::array30_data;
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

        let elapsed = (chrono::Utc::now().timestamp_millis() as u64).saturating_sub(*start_time_ms.read());
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
    let wpm = stats.wpm();
    let accuracy = stats.accuracy();
    let input_char_count = user_input.read().chars().count();
    let target_char_count = session.read().target_text.chars().count();
    let progress_value = if target_char_count == 0 {
        0.0
    } else {
        (input_char_count as f64 / target_char_count as f64 * 100.0).min(100.0)
    };

    let next_char = session.read().target_text.chars().nth(input_char_count);
    
    let next_char_hint = next_char.and_then(|c| {
        // Show hint for the NEXT character that needs to be typed
        array30_data::get_array30_code(c).map(|code| (c, code))
    });

    rsx! {
        div {
            class: "grid gap-8",

            // Statistics Grid
            div {
                class: "grid gap-8 md:grid-cols-3",
                StatCard { label: "WPM", value: format!("{:.0}", wpm), r#type: "wpm" }
                StatCard { label: "Accuracy", value: format!("{:.0}%", accuracy), r#type: "accuracy" }
                StatCard { label: "Level", value: "4/10", r#type: "progress" }
            }

            // Typing Exercise Card
            Card {
                class: "rounded-[2.5rem]",
                CardHeader {
                    class: "grid gap-4 px-6",
                    CardTitle {
                        class: "m-0 text-[1.1rem] font-black uppercase tracking-[0.25em] text-slate-500",
                        "Current Exercise"
                    }
                    div {
                        class: "flex items-center justify-between gap-6",
                        CardDescription {
                            class: "text-[1.35rem] font-bold text-indigo-600",
                            "{input_char_count} / {target_char_count}"
                        }
                        Badge {
                            variant: if *show_completion.read() { BadgeVariant::Primary } else { BadgeVariant::Secondary },
                            if *show_completion.read() { "Ready to save" } else { "In progress" }
                        }
                        if stats.errors > 0 {
                            Badge {
                                variant: BadgeVariant::Destructive,
                                {format!("{} errors", stats.errors)}
                            }
                        }
                    }
                }
                CardContent {
                    class: "grid gap-10",
                    div {
                        class: "min-h-[clamp(140px,20vh,220px)] rounded-[2.5rem] bg-white p-12 font-['Atkinson_Hyperlegible','Fira_Code',monospace] text-[clamp(1.75rem,4vw,2.5rem)] leading-[1.5] tracking-[0.04em] shadow-[2px_2px_0px_rgba(30,27,75,0.3)]",
                        {
                            let target = session.read().target_text.clone();
                            let input = user_input.read().clone();
                            let input_chars: Vec<char> = input.chars().collect();

                            rsx! {
                                for (i, c) in target.chars().enumerate() {
                                    {
                                        let class = if i < input_chars.len() {
                                            if input_chars[i] == c { "font-bold text-indigo-600 [text-shadow:0_0_20px_rgba(79,70,229,0.4)]" } else { "rounded-xl bg-red-500 px-1 text-white shadow-[0_6px_16px_rgba(239,68,68,0.4)]" }
                                        } else {
                                            "text-slate-500 opacity-30"
                                        };
                                        rsx! { span { key: "{i}", class: "{class}", "{c}" } }
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

                    div {
                        class: "mt-4 flex min-h-24 w-full items-center rounded-[2rem] border-4 border-dashed border-indigo-200/60 bg-indigo-600/[0.02] px-12 py-3 shadow-[inset_6px_6px_15px_rgba(0,0,0,0.015)]",
                        
                        if let Some((c, code)) = next_char_hint {
                            div {
                                class: "w-full flex items-center justify-between gap-4 overflow-hidden",
                                span {
                                    class: "font-bold text-indigo-900",
                                    "Next: Array30 Code for "
                                    span { class: "font-black mx-1", "{c}" }
                                }
                                div {
                                    class: "overflow-x-auto",
                                    CodeDisplay { code: code }
                                }
                            }
                        } else if let Some(c) = next_char {
                            div {
                                class: "w-full flex items-center justify-between gap-4 overflow-hidden",
                                span {
                                    class: "font-bold text-slate-500",
                                    "Next: "
                                    span { class: "font-black text-slate-900 mx-1", "{c}" }
                                }
                                Badge {
                                    variant: BadgeVariant::Outline,
                                    class: "font-mono text-xs opacity-50 flex-shrink-0",
                                    "No code hint"
                                }
                            }
                        } else {
                            // Completed or empty
                            div {
                                class: "w-full flex items-center justify-center text-slate-400 font-bold",
                                "Exercise Complete"
                            }
                        }
                    }

                    // Hidden but functional textarea
                    div {
                        class: "group relative",
                        textarea {
                            class: "block min-h-[clamp(120px,22vh,220px)] w-full resize-none appearance-none rounded-[2.5rem] border-none bg-white p-12 font-['Atkinson_Hyperlegible','Fira_Code',monospace] text-[clamp(1.75rem,4vw,2.5rem)] leading-[1.5] tracking-[0.04em] text-indigo-950 shadow-[6px_6px_0px_rgba(30,27,75,0.4)] transition-all duration-300 [transition-timing-function:cubic-bezier(0.34,1.56,0.64,1.0)] placeholder:text-[1.35rem] placeholder:font-medium placeholder:text-slate-400 focus:-translate-y-1 focus:scale-[1.02] focus:outline-none focus:shadow-[30px_30px_60px_rgba(79,70,229,0.12),0_0_0_6px_rgba(72,72,229,0.2)]",
                            placeholder: "Focus here and start typing...",
                            value: "{user_input}",
                            oninput: handle_input,
                            autofocus: true
                        }
                        div { class: "pointer-events-none absolute inset-0 rounded-[2.5rem] transition-shadow duration-300 group-focus-within:shadow-[0_0_0_6px_rgba(72,72,229,0.2)]" }
                    }
                }

            }

            // Action Footer (Outside the white card)
            div {
                class: "mt-8 flex flex-col items-center justify-center gap-6 bg-transparent px-4 py-6 sm:flex-row sm:px-8 md:px-16",
                Button {
                    class: "h-32 max-w-[800px] flex-[0_1_800px] rounded-[4rem] text-[2.5rem] shadow-[0_16px_0_rgba(154,52,18,1)] active:translate-y-2 active:shadow-[0_8px_0_rgba(154,52,18,1)]",
                    variant: ButtonVariant::Primary,
                    onclick: handle_next,
                    span {
                        style: "display: flex; align-items: center; gap: 1.5rem;",
                        svg {
                            width: "48",
                            height: "48",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            path { d: "M8 5v14l11-7z" }
                        }
                        if *show_completion.read() { "Save & Next Lesson" } else { "Skip to Next Lesson" }
                    }
                }
                Button {
                    class: "h-32 w-32 flex-[0_0_8rem] rounded-full p-0 shadow-[0_16px_0_rgba(30,27,75,1)] active:translate-y-2 active:shadow-[0_8px_0_rgba(30,27,75,1)]",
                    variant: ButtonVariant::Secondary,
                    onclick: handle_reset,
                    svg {
                        width: "48",
                        height: "48",
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

            if session.read().started && !*show_completion.read() {
                div {
                    class: "flex justify-center pb-6",
                    Badge {
                        variant: BadgeVariant::Secondary,
                        "Recording session..."
                    }
                }
            }

            if *show_completion.read() {
                div {
                    class: "mt-6 animate-[bounce-in_0.6s_cubic-bezier(0.34,1.56,0.64,1.0)] rounded-[2.5rem] border-[5px] border-teal-600 bg-teal-50 p-12 text-center shadow-[0_25px_50px_rgba(13,148,136,0.2)]",
                    div {
                        h4 { class: "m-0 text-[2rem] font-bold text-teal-700", "Excellent Accuracy!" }
                        p { class: "mt-3 text-[1.35rem] font-semibold text-teal-800", "You've mastered this exercise. Save your progress to continue." }
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
        div {
            class: "flex gap-2 whitespace-nowrap",
            for c in codes {
                Badge {
                    variant: BadgeVariant::Secondary,
                    class: "font-mono text-xs",
                    "{c}"
                }
            }
        }
    }
}

#[component]
fn StatCard(label: &'static str, value: String, r#type: &'static str) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center gap-3 rounded-[4rem] bg-white px-6 py-10 text-center shadow-[6px_6px_0px_rgba(72,72,229,0.15),inset_0_10px_20px_rgba(255,255,255,0.9),inset_0_-8px_15px_rgba(72,72,229,0.05)] transition-all duration-300 [transition-timing-function:cubic-bezier(0.34,1.56,0.64,1.0)] hover:-translate-y-3 hover:scale-[1.04] hover:shadow-[10px_10px_0px_rgba(30,27,75,0.45)]",
            "data-type": r#type,
            span { class: "text-[0.9rem] font-bold uppercase tracking-[0.1em] text-slate-500", "{label}" }
            if r#type == "accuracy" {
                span { class: "text-[2.25rem] font-extrabold leading-none text-emerald-500", "{value}" }
            } else if r#type == "progress" {
                span { class: "text-[2.25rem] font-extrabold leading-none text-orange-500", "{value}" }
            } else {
                span { class: "text-[2.25rem] font-extrabold leading-none text-indigo-600", "{value}" }
            }
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
