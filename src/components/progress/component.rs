use dioxus::prelude::*;
use dioxus_primitives::progress::{self, ProgressIndicatorProps, ProgressProps};

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    rsx! {
        progress::Progress {
            class: "progress relative h-[1.15rem] w-full overflow-hidden rounded-full border-[3px] border-indigo-200/40 bg-indigo-50 shadow-[inset_2px_2px_4px_rgba(0,0,0,0.1)]",
            value: props.value,
            max: props.max,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ProgressIndicator(props: ProgressIndicatorProps) -> Element {
    rsx! {
        progress::ProgressIndicator {
            class: "progress-indicator h-full bg-gradient-to-r from-indigo-600 to-indigo-400 shadow-[inset_0_2px_4px_rgba(255,255,255,0.3)] transition-[width] duration-300 [transition-timing-function:cubic-bezier(0.34,1.56,0.64,1)]",
            attributes: props.attributes,
            {props.children}
        }
    }
}
