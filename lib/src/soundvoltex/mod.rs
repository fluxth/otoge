pub mod models;

use std::borrow::Cow;

use crate::shared::traits::Otoge;
use models::Category;

pub struct SoundVoltex;

impl Otoge for SoundVoltex {
    type DataStore = models::DataStore;
    type Song = models::Song;

    fn name() -> &'static str {
        "soundvoltex"
    }
}

pub(crate) fn get_all_categories() -> Vec<Category> {
    vec![
        Category {
            id: Cow::Borrowed("pops"),
            name: Cow::Borrowed("POPS&アニメ"),
        },
        Category {
            id: Cow::Borrowed("toho"),
            name: Cow::Borrowed("東方アレンジ"),
        },
        Category {
            id: Cow::Borrowed("vocaloid"),
            name: Cow::Borrowed("ボーカロイド"),
        },
        Category {
            id: Cow::Borrowed("bemani"),
            name: Cow::Borrowed("BEMANI"),
        },
        Category {
            id: Cow::Borrowed("hinabita"),
            name: Cow::Borrowed("ひなビタ♪/バンめし♪"),
        },
        Category {
            id: Cow::Borrowed("floor"),
            name: Cow::Borrowed("FLOOR"),
        },
        Category {
            id: Cow::Borrowed("sdvx"),
            name: Cow::Borrowed("SDVXオリジナル"),
        },
        Category {
            id: Cow::Borrowed("others"),
            name: Cow::Borrowed("その他"),
        },
    ]
}
