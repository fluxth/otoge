pub mod models;

use models::Category;
use std::borrow::Cow;

pub fn get_all_categories() -> Vec<Category> {
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
