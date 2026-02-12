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
            class: "tabs-list mb-10 flex w-fit flex-row gap-2 rounded-[2rem] border-[3px] border-indigo-200/60 bg-indigo-50/80 p-2 shadow-[inset_4px_4px_12px_rgba(0,0,0,0.02)]",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TabTrigger(props: TabTriggerProps) -> Element {
    rsx! {
        tabs::TabTrigger {
            class: "tabs-trigger cursor-pointer rounded-3xl border-none bg-transparent px-8 py-3 text-base font-extrabold text-slate-500 transition-[background-color,color,box-shadow,transform] duration-300 [transition-timing-function:cubic-bezier(0.34,1.56,0.64,1.0)] hover:-translate-y-0.5 hover:bg-white hover:text-indigo-600 hover:shadow-[0_4px_12px_rgba(79,70,229,0.1)] focus-visible:outline-none focus-visible:shadow-[0_0_0_3px_rgba(37,99,235,0.2)] data-[state=active]:-translate-y-0.5 data-[state=active]:scale-[1.06] data-[state=active]:bg-indigo-600 data-[state=active]:text-white data-[state=active]:shadow-[0_8px_24px_rgba(79,70,229,0.3)] data-[disabled=true]:cursor-not-allowed data-[disabled=true]:text-slate-400",
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
