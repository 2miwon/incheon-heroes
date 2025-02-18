#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn SongsByIdPage(id: i64, lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: SongsByIdTranslate = translate(&lang);

    rsx! {
        div { id: "songs-by-id", "{tr.title} PAGE" }
    }
}
