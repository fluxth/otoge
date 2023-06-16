mod deserializers;
pub mod models;

use crate::shared::traits::Otoge;
use models::Category;

use std::borrow::Cow;

pub struct MaimaiJP;
impl Otoge for MaimaiJP {
    type DataStore = models::DataStore;
    type Song = models::Song;

    fn name() -> &'static str {
        "maimai_jp"
    }
}

pub struct MaimaiIntl;
impl Otoge for MaimaiIntl {
    type DataStore = models::DataStore;
    type Song = models::Song;

    fn name() -> &'static str {
        "maimai_intl"
    }
}

pub fn get_all_jp_categories() -> Vec<Category> {
    vec![
        Category {
            slug: Cow::Borrowed("pops_anime"),
            name: Cow::Borrowed("POPS＆アニメ"),
        },
        Category {
            slug: Cow::Borrowed("niconico"),
            name: Cow::Borrowed("niconico＆ボーカロイド"),
        },
        Category {
            slug: Cow::Borrowed("toho"),
            name: Cow::Borrowed("東方Project"),
        },
        Category {
            slug: Cow::Borrowed("variety"),
            name: Cow::Borrowed("ゲーム＆バラエティ"),
        },
        Category {
            slug: Cow::Borrowed("maimai"),
            name: Cow::Borrowed("maimai"),
        },
        Category {
            slug: Cow::Borrowed("gekichu"),
            name: Cow::Borrowed("オンゲキ＆CHUNITHM"),
        },
    ]
}

pub fn get_all_intl_categories() -> Vec<Category> {
    vec![
        Category {
            slug: Cow::Borrowed("pops_anime"),
            name: Cow::Borrowed("POPS＆ANIME"),
        },
        Category {
            slug: Cow::Borrowed("niconico"),
            name: Cow::Borrowed("niconico＆VOCALOID™"),
        },
        Category {
            slug: Cow::Borrowed("toho"),
            name: Cow::Borrowed("東方Project"),
        },
        Category {
            slug: Cow::Borrowed("variety"),
            name: Cow::Borrowed("GAME＆VARIETY"),
        },
        Category {
            slug: Cow::Borrowed("maimai"),
            name: Cow::Borrowed("maimai"),
        },
        Category {
            slug: Cow::Borrowed("gekichu"),
            name: Cow::Borrowed("オンゲキ＆CHUNITHM"),
        },
    ]
}
