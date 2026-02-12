use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[component]
pub fn Card(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "flex flex-col gap-5 rounded-[2.5rem] border-4 border-indigo-950 bg-white px-0 py-6 text-indigo-950 shadow-[6px_6px_0px_rgba(72,72,229,0.15)]",
        "data-slot": "card",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}

#[component]
pub fn CardHeader(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "grid auto-rows-min items-start gap-2 px-6",
        "data-slot": "card-header",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}

#[component]
pub fn CardTitle(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "text-xl font-extrabold leading-tight tracking-[-0.02em]",
        "data-slot": "card-title",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}

#[component]
pub fn CardDescription(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "text-[0.95rem] font-semibold leading-[1.35] text-slate-500",
        "data-slot": "card-description",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}

#[component]
pub fn CardAction(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "col-start-2 row-span-2 place-self-start-end",
        "data-slot": "card-action",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}

#[component]
pub fn CardContent(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "px-6",
        "data-slot": "card-content",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}

#[component]
pub fn CardFooter(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "flex items-center px-6",
        "data-slot": "card-footer",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}
