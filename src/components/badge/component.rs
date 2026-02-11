use dioxus::prelude::*;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum BadgeVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Outline,
}

impl BadgeVariant {
    pub fn class(&self) -> &'static str {
        match self {
            BadgeVariant::Primary => "border-white/20 bg-indigo-600 text-white",
            BadgeVariant::Secondary => "bg-white text-indigo-600",
            BadgeVariant::Destructive => "border-white/20 bg-red-500 text-white",
            BadgeVariant::Outline => "border-slate-500 bg-transparent text-slate-500",
        }
    }
}

/// The props for the [`Badge`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    #[props(default)]
    pub variant: BadgeVariant,

    /// Additional attributes to extend the badge element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the badge element
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    rsx! {
        span {
            class: format!(
                "inline-flex h-7 min-w-5 items-center justify-center gap-1 rounded-full border-2 border-indigo-200/60 px-3 text-[0.78rem] font-extrabold tracking-[0.02em] shadow-[2px_2px_4px_rgba(0,0,0,0.05)] {}",
                props.variant.class()
            ),
            ..props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn VerifiedIcon() -> Element {
    rsx! {
        // Badge icon from lucide https://lucide.dev/icons/badge
        svg {
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            width: "12",
            height: "12",
            fill: "none",
            stroke: "var(--secondary-color-4)",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: 2,
            path { d: "M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" }
            path { d: "m9 12 2 2 4-4" }
        }
    }
}
