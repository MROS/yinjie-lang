use std::collections::HashSet;

use crate::剖析::{Ｏ句, Ｏ算式, Ｏ語法樹};

pub fn 檢查語法樹(語法樹: &Ｏ語法樹) -> Option<HashSet<String>> {
    let mut 通過 = true;

    let mut 變數集 = HashSet::<String>::new();

    for 句 in &語法樹.句 {
        通過 = match 句 {
            Ｏ句::變數宣告(宣告) => {
                let 通過 = 檢查算式(&變數集, &宣告.算式);
                變數集.insert(宣告.變數名.clone());
                通過
            }
            Ｏ句::算式(算式) => 檢查算式(&變數集, &算式),
        } && 通過 // 「通過」寫在 && 後面，避免短路
    }

    if 通過 {
        Some(變數集)
    } else {
        None
    }
}

fn 檢查算式(變數集: &HashSet<String>, 算式: &Ｏ算式) -> bool {
    match 算式 {
        Ｏ算式::變數(變數名) => {
            if 變數集.contains(變數名) {
                // println!("找到「{}」", 變數名);
                true
            } else {
                println!("「{}」未宣告", 變數名);
                false
            }
        }
        Ｏ算式::數字(_) => true,
        Ｏ算式::二元運算(二元運算) => {
            let 左通過 = 檢查算式(變數集, 二元運算.左.as_ref());
            let 右通過 = 檢查算式(變數集, 二元運算.右.as_ref());
            左通過 && 右通過
        }
    }
}
