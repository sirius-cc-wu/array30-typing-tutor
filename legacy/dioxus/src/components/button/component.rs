use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Outline,
    Ghost,
}

impl ButtonVariant {
    pub fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "bg-orange-500 text-white shadow-[6px_6px_0px_rgba(154,52,18,0.5)] hover:brightness-110 hover:shadow-[10px_10px_0px_rgba(154,52,18,0.55)]",
            ButtonVariant::Secondary => "bg-indigo-600 text-white shadow-[6px_6px_0px_rgba(30,27,75,0.5)] hover:brightness-110 hover:shadow-[10px_10px_0px_rgba(30,27,75,0.55)]",
            ButtonVariant::Destructive => "bg-red-600 text-white hover:bg-red-700",
            ButtonVariant::Outline => "border border-indigo-600 bg-transparent text-indigo-600 hover:bg-indigo-600/5",
            ButtonVariant::Ghost => "bg-transparent text-indigo-600 shadow-none hover:-translate-y-0.5 hover:bg-indigo-600/10",
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(extends=GlobalAttributes)]
    #[props(extends=button)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let base = attributes!(button {
        class: format!(
            "inline-flex min-h-11 select-none items-center justify-center rounded-full border-0 px-8 py-2 text-[1.1rem] font-extrabold uppercase leading-none tracking-[0.02em] transition-all duration-200 [transition-timing-function:cubic-bezier(0.175,0.885,0.32,1.275)] focus-visible:outline-none focus-visible:shadow-[10px_10px_0px_rgba(30,27,75,0.45),0_0_0_6px_rgba(72,72,229,0.2)] enabled:hover:-translate-x-0.5 enabled:hover:-translate-y-0.5 enabled:active:scale-[0.98] enabled:active:shadow-[2px_2px_0px_rgba(30,27,75,0.3)] disabled:cursor-not-allowed disabled:opacity-50 disabled:shadow-none {}",
            variant.class()
        ),
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        button {
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &onmousedown {
                    f.call(event);
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &onmouseup {
                    f.call(event);
                }
            },
            ..merged,
            {children}
        }
    }
}
