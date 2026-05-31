pub mod models;

use std::borrow::Cow;

use crate::shared::traits::Otoge;
use models::Category;

pub struct PolarisChord;
impl Otoge for PolarisChord {
    type DataStore = models::DataStore;
    type Song = models::Song;

    fn name() -> &'static str {
        "polarischord"
    }

    fn image_url(image_id: &str) -> String {
        format!(
            "https://p.eagate.573.jp/game/polarischord/pc/img/music/jacket.html?c={}",
            image_id
        )
    }
}

pub(crate) fn get_all_categories() -> Vec<Category> {
    vec![
        Category {
            bitflag: 1,
            slug: Cow::Borrowed("virtual"),
            name: Cow::Borrowed("Virtual"),
        },
        Category {
            bitflag: 2,
            slug: Cow::Borrowed("social"),
            name: Cow::Borrowed("ソーシャルミュージック"),
        },
        Category {
            bitflag: 3,
            slug: Cow::Borrowed("pops&anime"),
            name: Cow::Borrowed("POPS&アニメ"),
        },
        Category {
            bitflag: 4,
            slug: Cow::Borrowed("touhou"),
            name: Cow::Borrowed("東方"),
        },
        Category {
            bitflag: 5,
            slug: Cow::Borrowed("variety"),
            name: Cow::Borrowed("バラエティ"),
        },
        Category {
            bitflag: 6,
            slug: Cow::Borrowed("original"),
            name: Cow::Borrowed("オリジナル"),
        },
    ]
}
