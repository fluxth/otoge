pub mod models;

use crate::shared::traits::Otoge;

use models::Category;
use std::borrow::Cow;

pub struct ChunithmJP;
impl Otoge for ChunithmJP {
    type DataStore = models::DataStore;
    type Song = models::Song;

    fn name() -> &'static str {
        "chunithm_jp"
    }
}

pub struct ChunithmIntl;
impl Otoge for ChunithmIntl {
    type DataStore = models::DataStore;
    type Song = models::Song;

    fn name() -> &'static str {
        "chunithm_intl"
    }
}

pub(crate) fn get_all_categories() -> Vec<Category> {
    vec![
        Category {
            slug: Cow::Borrowed("pops_anime"),
            name: Cow::Borrowed("POPS & ANIME"),
        },
        Category {
            slug: Cow::Borrowed("niconico"),
            name: Cow::Borrowed("niconico"),
        },
        Category {
            slug: Cow::Borrowed("toho"),
            name: Cow::Borrowed("東方Project"),
        },
        Category {
            slug: Cow::Borrowed("variety"),
            name: Cow::Borrowed("VARIETY"),
        },
        Category {
            slug: Cow::Borrowed("irodorimidori"),
            name: Cow::Borrowed("イロドリミドリ"),
        },
        Category {
            slug: Cow::Borrowed("gekimai"),
            name: Cow::Borrowed("ゲキマイ"),
        },
        Category {
            slug: Cow::Borrowed("original"),
            name: Cow::Borrowed("ORIGINAL"),
        },
    ]
}
