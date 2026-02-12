use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::{
    self, AlertDialogActionProps, AlertDialogActionsProps, AlertDialogCancelProps,
    AlertDialogContentProps, AlertDialogDescriptionProps, AlertDialogRootProps,
    AlertDialogTitleProps,
};

#[component]
pub fn AlertDialogRoot(props: AlertDialogRootProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogRoot {
            class: "alert-dialog-backdrop fixed inset-0 z-[1000] bg-black/30 transition-opacity data-[state=closed]:opacity-0 data-[state=open]:opacity-100",
            id: props.id,
            default_open: props.default_open,
            open: props.open,
            on_open_change: props.on_open_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogContent {
            id: props.id,
            class: format!(
                "{} alert-dialog fixed left-1/2 top-1/2 z-[1001] flex w-full max-w-[calc(100%-2rem)] -translate-x-1/2 -translate-y-1/2 flex-col gap-4 rounded-[2.5rem] border-4 border-indigo-200/70 bg-white px-8 py-8 text-center text-slate-800 shadow-[0_2px_10px_rgba(0,0,0,0.18)] sm:max-w-[32rem] sm:text-left",
                props.class.unwrap_or_default()
            ),
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogTitle {
            class: "m-0 text-xl font-bold text-slate-900",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogDescription {
            class: "m-0 text-base text-slate-500",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogActions(props: AlertDialogActionsProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogActions {
            class: "alert-dialog-actions flex flex-col-reverse gap-3 sm:flex-row sm:justify-end",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogCancel {
            on_click: props.on_click,
            class: "alert-dialog-cancel rounded-full border-2 border-indigo-200 bg-transparent px-[18px] py-2 text-base text-indigo-600 transition-colors hover:bg-indigo-50 focus-visible:outline-none focus-visible:shadow-[0_0_0_2px_rgb(43,127,255)]",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogAction {
            class: "alert-dialog-action rounded-full border-none bg-red-500 px-6 py-2.5 text-base text-white transition-colors hover:bg-red-600 focus-visible:outline-none focus-visible:shadow-[0_0_0_2px_rgb(43,127,255)]",
            on_click: props.on_click,
            attributes: props.attributes,
            {props.children}
        }
    }
}
