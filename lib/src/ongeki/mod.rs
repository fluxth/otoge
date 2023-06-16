pub mod models;

use std::borrow::Cow;

use crate::shared::traits::Otoge;
use models::Category;

mod deserializers;

pub struct Ongeki;
impl Otoge for Ongeki {
    type DataStore = models::DataStore;
    type Song = models::Song;

    fn name() -> &'static str {
        "ongeki"
    }
}

pub(crate) fn get_all_categories() -> Vec<Category> {
    vec![
        Category {
            id: Cow::Borrowed("06"),
            slug: Cow::Borrowed("ongeki"),
            name: Cow::Borrowed("オンゲキ"),
        },
        Category {
            id: Cow::Borrowed("01"),
            slug: Cow::Borrowed("pops_and_anime"),
            name: Cow::Borrowed("POPS & ANIME"),
        },
        Category {
            id: Cow::Borrowed("02"),
            slug: Cow::Borrowed("niconico"),
            name: Cow::Borrowed("niconico"),
        },
        Category {
            id: Cow::Borrowed("03"),
            slug: Cow::Borrowed("touhou"),
            name: Cow::Borrowed("東方Project"),
        },
        Category {
            id: Cow::Borrowed("04"),
            slug: Cow::Borrowed("variety"),
            name: Cow::Borrowed("VARIETY"),
        },
        Category {
            id: Cow::Borrowed("05"),
            slug: Cow::Borrowed("chumai"),
            name: Cow::Borrowed("チュウマイ"),
        },
    ]
}
