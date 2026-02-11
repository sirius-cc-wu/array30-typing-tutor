use dioxus::prelude::*;
use dioxus_primitives::tabs::{self, TabContentProps, TabListProps, TabTriggerProps};

/// The props for the [`Tabs`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// The class of the tabs component.
    #[props(default)]
    pub class: String,

    /// The controlled value of the active tab.
    pub value: ReadSignal<Option<String>>,

    /// The default active tab value when uncontrolled.
    #[props(default)]
    pub default_value: String,

    /// Callback fired when the active tab changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Whether the tabs are disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the tabs are horizontal.
    #[props(default)]
    pub horizontal: ReadSignal<bool>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// The variant of the tabs component.
    #[props(default)]
    pub variant: TabsVariant,

    /// Additional attributes to apply to the tabs element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tabs component.
    pub children: Element,
}

/// The variant of the tabs component.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum TabsVariant {
    /// The default variant.
    #[default]
    Default,
    /// The ghost variant.
    Ghost,
}

impl TabsVariant {
    /// Convert the variant to a string for use in class names
    fn root_class(self) -> &'static str {
        match self {
            TabsVariant::Default => "tabs-default",
            TabsVariant::Ghost => "tabs-ghost",
        }
    }
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let class = if props.class.trim().is_empty() {
        format!("tabs flex w-full flex-col gap-3 {}", props.variant.root_class())
    } else {
        format!(
            "{} tabs flex w-full flex-col gap-3 {}",
            props.class,
            props.variant.root_class()
        )
    };

    rsx! {
        tabs::Tabs {
            class: class,
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            horizontal: props.horizontal,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TabList(props: TabListProps) -> Element {
    rsx! {
        tabs::TabList {
            class: "tabs-list flex w-fit flex-row gap-2 rounded-full p-2",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TabTrigger(props: TabTriggerProps) -> Element {
    rsx! {
        tabs::TabTrigger {
            class: "tabs-trigger cursor-pointer rounded-full border-none bg-transparent px-5 py-2 text-sm font-semibold text-slate-600 transition-[background-color,color,box-shadow,transform] duration-150 hover:bg-white/65 hover:text-slate-900 focus-visible:outline-none focus-visible:shadow-[0_0_0_3px_rgba(37,99,235,0.2)] data-[state=active]:bg-white data-[state=active]:text-slate-900 data-[state=active]:shadow-[0_1px_2px_rgba(15,23,42,0.12),inset_0_0_0_1px_rgba(148,163,184,0.25)] data-[disabled=true]:cursor-not-allowed data-[disabled=true]:text-slate-400",
            id: props.id,
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TabContent(props: TabContentProps) -> Element {
    rsx! {
        tabs::TabContent {
            class: format!(
                "{} tabs-content w-full box-border p-1 data-[state=inactive]:hidden",
                props.class.unwrap_or_default()
            ),
            value: props.value,
            id: props.id,
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}
