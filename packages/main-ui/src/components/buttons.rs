#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::icons::{heart::HeartIcon, send::SendIcon};

#[component]
pub fn HoverPrimaryButton(
    children: Element,
    onchangehover: EventHandler<bool>,
    onclick: EventHandler<()>,
) -> Element {
    rsx! {
        button {
            class: "flex flex-row items-center justify-center gap-[10px] hover:bg-[#D4EED4] px-[10px] py-[4] rounded-[12px] text-[#5B5B5B] hover:text-[#16775D] font-semibold",
            onmouseenter: move |_| onchangehover(true),
            onmouseleave: move |_| onchangehover(false),
            onclick: move |_| {
                onclick(());
            },
            {children}
        }
    }
}

#[component]
pub fn ShareButton(onclick: EventHandler<()>) -> Element {
    let mut hover = use_signal(|| false);
    let color = if hover() { "#16775D" } else { "#979797" };

    rsx! {
        HoverPrimaryButton {
            onclick: move |_| {
                onclick(());
            },
            onchangehover: move |e| {
                hover.set(e);
            },
            SendIcon { color: "{color}" }
            "Share"
        }
    }
}

#[component]
pub fn HeartButton(onclick: EventHandler<()>, liked: bool, children: Element) -> Element {
    let mut hover = use_signal(|| false);
    let color = if hover() { "#16775D" } else { "#979797" };

    rsx! {
        HoverPrimaryButton {
            onclick: move |_| {
                onclick(());
            },
            onchangehover: move |e| {
                hover.set(e);
            },
            HeartIcon { color: "{color}", fill: if liked { "{color}" } else { "none" } }
            {children}
        }
    }
}
