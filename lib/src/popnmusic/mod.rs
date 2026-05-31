pub mod models;

use std::borrow::Cow;

use crate::shared::traits::Otoge;
use models::Category;

pub struct PopNMusic;

impl Otoge for PopNMusic {
    type DataStore = models::DataStore;
    type Song = models::Song;

    fn name() -> &'static str {
        "popnmusic"
    }

    fn image_url(image_id: &str) -> String {
        format!("https://p.eagate.573.jp{}", image_id)
    }
}

pub fn get_all_versions() -> Vec<Category> {
    vec![
        Category {
            id: Cow::Borrowed("0"),
            name: Cow::Borrowed("pop'n 家庭用"),
        },
        Category {
            id: Cow::Borrowed("1"),
            name: Cow::Borrowed("pop'n music"),
        },
        Category {
            id: Cow::Borrowed("2"),
            name: Cow::Borrowed("pop'n music 2"),
        },
        Category {
            id: Cow::Borrowed("3"),
            name: Cow::Borrowed("pop'n music 3"),
        },
        Category {
            id: Cow::Borrowed("4"),
            name: Cow::Borrowed("pop'n music 4"),
        },
        Category {
            id: Cow::Borrowed("5"),
            name: Cow::Borrowed("pop'n music 5"),
        },
        Category {
            id: Cow::Borrowed("6"),
            name: Cow::Borrowed("pop'n music 6"),
        },
        Category {
            id: Cow::Borrowed("7"),
            name: Cow::Borrowed("pop'n music 7"),
        },
        Category {
            id: Cow::Borrowed("8"),
            name: Cow::Borrowed("pop'n music 8"),
        },
        Category {
            id: Cow::Borrowed("9"),
            name: Cow::Borrowed("pop'n music 9"),
        },
        Category {
            id: Cow::Borrowed("10"),
            name: Cow::Borrowed("pop'n music 10"),
        },
        Category {
            id: Cow::Borrowed("11"),
            name: Cow::Borrowed("pop'n music 11"),
        },
        Category {
            id: Cow::Borrowed("12"),
            name: Cow::Borrowed("pop'n music 12 いろは"),
        },
        Category {
            id: Cow::Borrowed("13"),
            name: Cow::Borrowed("pop'n music 13 カーニバル"),
        },
        Category {
            id: Cow::Borrowed("14"),
            name: Cow::Borrowed("pop'n music 14 FEVER！"),
        },
        Category {
            id: Cow::Borrowed("15"),
            name: Cow::Borrowed("pop'n music 15 ADVENTURE"),
        },
        Category {
            id: Cow::Borrowed("16"),
            name: Cow::Borrowed("pop'n music 16 PARTY♪"),
        },
        Category {
            id: Cow::Borrowed("17"),
            name: Cow::Borrowed("pop'n music 17 THE MOVIE"),
        },
        Category {
            id: Cow::Borrowed("18"),
            name: Cow::Borrowed("pop'n music 18 せんごく列伝"),
        },
        Category {
            id: Cow::Borrowed("19"),
            name: Cow::Borrowed("pop'n music 19 TUNE STREET"),
        },
        Category {
            id: Cow::Borrowed("20"),
            name: Cow::Borrowed("pop'n music 20 fantasia"),
        },
        Category {
            id: Cow::Borrowed("21"),
            name: Cow::Borrowed("pop'n music Sunny Park"),
        },
        Category {
            id: Cow::Borrowed("22"),
            name: Cow::Borrowed("pop'n music ラピストリア"),
        },
        Category {
            id: Cow::Borrowed("23"),
            name: Cow::Borrowed("pop'n music éclale"),
        },
        Category {
            id: Cow::Borrowed("24"),
            name: Cow::Borrowed("pop'n music うさぎと猫と少年の夢"),
        },
        Category {
            id: Cow::Borrowed("25"),
            name: Cow::Borrowed("pop'n music peace"),
        },
        Category {
            id: Cow::Borrowed("26"),
            name: Cow::Borrowed("pop'n music 解明リドルズ"),
        },
        Category {
            id: Cow::Borrowed("27"),
            name: Cow::Borrowed("pop'n music UniLab"),
        },
        Category {
            id: Cow::Borrowed("28"),
            name: Cow::Borrowed("pop'n music Jam&Fizz"),
        },
        Category {
            id: Cow::Borrowed("29"),
            name: Cow::Borrowed("pop'n music High☆Cheers!!"),
        },
    ]
}

pub fn get_all_bemani() -> Vec<Category> {
    vec![
        Category {
            id: Cow::Borrowed("1"),
            name: Cow::Borrowed("beatmania IIDX"),
        },
        Category {
            id: Cow::Borrowed("2"),
            name: Cow::Borrowed("DanceDanceRevolution"),
        },
        Category {
            id: Cow::Borrowed("3"),
            name: Cow::Borrowed("GITADORA"),
        },
        Category {
            id: Cow::Borrowed("4"),
            name: Cow::Borrowed("jubeat"),
        },
        Category {
            id: Cow::Borrowed("5"),
            name: Cow::Borrowed("REFLEC BEAT"),
        },
        Category {
            id: Cow::Borrowed("6"),
            name: Cow::Borrowed("SOUND VOLTEX"),
        },
        Category {
            id: Cow::Borrowed("7"),
            name: Cow::Borrowed("BeatStream"),
        },
        Category {
            id: Cow::Borrowed("8"),
            name: Cow::Borrowed("MUSECA"),
        },
        Category {
            id: Cow::Borrowed("9"),
            name: Cow::Borrowed("ノスタルジア"),
        },
        Category {
            id: Cow::Borrowed("10"),
            name: Cow::Borrowed("BEMANI"),
        },
    ]
}

pub fn get_all_categories() -> Vec<Category> {
    vec![
        Category {
            id: Cow::Borrowed("1"),
            name: Cow::Borrowed("オススメ"),
        },
        Category {
            id: Cow::Borrowed("2"),
            name: Cow::Borrowed("東方アレンジ"),
        },
        Category {
            id: Cow::Borrowed("3"),
            name: Cow::Borrowed("ひなビタ♪"),
        },
        Category {
            id: Cow::Borrowed("4"),
            name: Cow::Borrowed("バンめし♪"),
        },
        Category {
            id: Cow::Borrowed("5"),
            name: Cow::Borrowed("TV・アニメ"),
        },
        Category {
            id: Cow::Borrowed("6"),
            name: Cow::Borrowed("J-POP"),
        },
        Category {
            id: Cow::Borrowed("7"),
            name: Cow::Borrowed("NET MUSIC・VOCALOID"),
        },
        Category {
            id: Cow::Borrowed("8"),
            name: Cow::Borrowed("クラシック"),
        },
        Category {
            id: Cow::Borrowed("9"),
            name: Cow::Borrowed("GAME MUSIC"),
        },
    ]
}
