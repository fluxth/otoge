pub mod models;

use std::borrow::Cow;

use models::Category;

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
