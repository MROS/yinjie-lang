use rpds::HashTrieMap;

use crate::分詞器::Ｏ運算子;
use crate::剖析::抽象語法樹節點::*;
use crate::外術列表::外術列表;

enum Ｏ類型 {
    變數,
    術 {
        // TODO: 未來要讓回傳值也是函式簽名的一部分
        形參數量: usize,
    },
}

type Ｏ環境 = HashTrieMap<String, Ｏ類型>;

pub fn 檢查語法樹(語法樹: &Ｏ語法樹) -> bool {
    let mut 全部通過 = true;

    let mut 環境 = Ｏ環境::new();

    for (術名, 形參數量) in 外術列表() {
        環境.insert_mut(術名, Ｏ類型::術 { 形參數量 });
    }

    let mut 存在初術 = false;

    for 頂層宣告 in &語法樹.頂層宣告 {
        let (新環境, 通過) = match 頂層宣告 {
            Ｏ頂層宣告::術宣告(宣告) => {
                if 宣告.術名 == "初" {
                    存在初術 = true;
                }
                檢查術宣告(&環境, 宣告)
            }
            Ｏ頂層宣告::變數宣告(宣告) => 檢查變數宣告(&環境, 宣告),
        };
        環境 = 新環境;
        全部通過 = 全部通過 && 通過;
    }

    if !存在初術 {
        panic!("必須定義音界咒入口「初」術")
    }

    全部通過
}

// 會增加定義（改變環境）的語法結構需要返回環境
fn 檢查術宣告(環境: &Ｏ環境, 術宣告: &Ｏ術宣告) -> (Ｏ環境, bool) {
    let mut 新環境 = 環境.clone();

    // 檢查形參並將其加入新的環境
    for 形參 in &術宣告.形參 {
        新環境 = 新環境.insert(形參.clone(), Ｏ類型::變數);
    }

    // 檢查術體的每一句
    let 通過 = 檢查區塊(&新環境, &術宣告.術體);

    // 將術名加進原環境中
    let 最終環境 = 環境.insert(
        術宣告.術名.clone(),
        Ｏ類型::術 {
            形參數量: 術宣告.形參.len(),
        },
    );

    (最終環境, 通過)
}

fn 檢查句(環境: &Ｏ環境, 句: &Ｏ句) -> (Ｏ環境, bool) {
    match 句 {
        Ｏ句::變數宣告(變數宣告) => 檢查變數宣告(環境, 變數宣告),
        Ｏ句::算式(算式) => (環境.clone(), 檢查算式(環境, 算式)),
        Ｏ句::歸(歸) => (環境.clone(), 檢查算式(環境, &歸.0)),
        Ｏ句::若(若) => (環境.clone(), 檢查若(環境, &若)),
    }
}

fn 檢查若(環境: &Ｏ環境, 若: &Ｏ若) -> bool {
    let 條件通過 = 檢查算式(環境, &若.條件);

    let 區塊通過 = 檢查區塊(環境, &若.區塊);

    let mut 或若通過 = true;
    for 或若 in &若.或若列表 {
        let 條件通過 = 檢查算式(環境, &或若.條件);
        let 或若區塊通過 = 檢查區塊(環境, &或若.區塊);
        或若通過 = 或若通過 && 條件通過 && 或若區塊通過;
    }

    let 不然通過 = match &若.不然 {
        Some(不然區塊) => 檢查區塊(環境, &不然區塊.區塊),
        None => true,
    };

    條件通過 && 區塊通過 && 或若通過 && 不然通過
}

fn 檢查變數宣告(環境: &Ｏ環境, 變數宣告: &Ｏ變數宣告) -> (Ｏ環境, bool) {
    let 通過 = 檢查算式(環境, &變數宣告.算式);
    let 新環境 = 環境.insert(變數宣告.變數名.clone(), Ｏ類型::變數);
    (新環境, 通過)
}

// 不會增加定義（改變環境）的語法結構返回檢查是否成功即可
fn 檢查算式(環境: &Ｏ環境, 算式: &Ｏ算式) -> bool {
    match 算式 {
        Ｏ算式::變數(變數名) => {
            if 環境.contains_key(變數名) {
                // println!("找到「{}」", 變數名);
                true
            } else {
                println!("「{}」未宣告", 變數名);
                false
            }
        }
        Ｏ算式::數字(_) => true,
        Ｏ算式::二元運算(二元運算) => {
            let 左通過 = 檢查算式(環境, 二元運算.左.as_ref());
            let 右通過 = 檢查算式(環境, 二元運算.右.as_ref());
            左通過 && 右通過
        }
        Ｏ算式::施術(施術) => match 環境.get(&施術.術名) {
            Some(Ｏ類型::術 { 形參數量 }) => *形參數量 == 施術.實參.len(),
            _ => {
                println!("{} 形參實參數量不同", 施術.術名);
                false
            }
        },
    }
}

fn 檢查區塊(環境: &Ｏ環境, 句組: &[Ｏ句]) -> bool {
    let mut 新環境 = 環境.clone();
    let mut 全部通過 = true;

    for 句 in 句組 {
        let (當前環境, 通過) = 檢查句(&新環境, 句);
        新環境 = 當前環境;
        全部通過 = 全部通過 && 通過;
    }

    全部通過
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 測試變數宣告_數字() {
        let 變數宣告 = Ｏ變數宣告 {
            變數名: "x".to_owned(),
            算式: Ｏ算式::數字(42),
        };

        let 環境 = Ｏ環境::new();
        let (新環境, 通過) = 檢查變數宣告(&環境, &變數宣告);

        assert!(通過);
        assert!(新環境.contains_key("x"));
    }

    #[test]
    fn 測試變數宣告_變數未宣告() {
        let 變數宣告 = Ｏ變數宣告 {
            變數名: "x".to_owned(),
            算式: Ｏ算式::變數("y".to_owned()),
        };

        let 環境 = Ｏ環境::new();
        let (新環境, 通過) = 檢查變數宣告(&環境, &變數宣告);

        assert!(!通過);
        assert!(新環境.contains_key("x"));
    }

    #[test]
    fn 測試術宣告_將術加入環境() {
        let 術宣告 = Ｏ術宣告 {
            術名: "和".to_owned(),
            形參: vec!["甲".to_owned(), "乙".to_owned()],
            術體: vec![Ｏ句::歸(Ｏ歸(Ｏ算式::二元運算(Ｏ二元運算 {
                運算子: Ｏ運算子::加,
                左: Box::new(Ｏ算式::變數("甲".to_owned())),
                右: Box::new(Ｏ算式::變數("乙".to_owned())),
            })))],
        };

        let 環境 = Ｏ環境::new();
        let (新環境, 通過) = 檢查術宣告(&環境, &術宣告);

        assert!(通過);
        assert!(新環境.contains_key("和"));
        match 新環境.get("和") {
            Some(Ｏ類型::術 { 形參數量 }) => assert_eq!(*形參數量, 2),
            _ => panic!("「和」術未正確宣告"),
        }
    }

    #[test]
    fn 測試語法樹_術宣告與施術_形參實參量一致() {
        // 建立語法樹，包含術宣告與施術
        let 語法樹 = Ｏ語法樹 {
            頂層宣告: vec![
                // 宣告術
                Ｏ頂層宣告::術宣告(Ｏ術宣告 {
                    術名: "和".to_owned(),
                    形參: vec!["甲".to_owned(), "乙".to_owned()],
                    術體: vec![Ｏ句::歸(Ｏ歸(Ｏ算式::二元運算(Ｏ二元運算 {
                        運算子: Ｏ運算子::加,
                        左: Box::new(Ｏ算式::變數("甲".to_owned())),
                        右: Box::new(Ｏ算式::變數("乙".to_owned())),
                    })))],
                }),
                // 變數宣告，施術包含在裡面
                Ｏ頂層宣告::變數宣告(Ｏ變數宣告 {
                    變數名: "結果".to_owned(),
                    算式: Ｏ算式::施術(Ｏ施術 {
                        術名: "和".to_owned(),
                        實參: vec![Ｏ算式::數字(10), Ｏ算式::數字(20)],
                    }),
                }),
            ],
        };

        // 檢查語法樹
        let 檢查結果 = 檢查語法樹(&語法樹);

        // 檢查結果
        assert!(檢查結果); // 應該通過
    }

    #[test]
    fn 測試語法樹_術宣告與施術_形參實參量不一致() {
        // 建立語法樹，包含術宣告與施術
        let 語法樹 = Ｏ語法樹 {
            頂層宣告: vec![
                // 宣告術
                Ｏ頂層宣告::術宣告(Ｏ術宣告 {
                    術名: "和".to_owned(),
                    形參: vec!["甲".to_owned(), "乙".to_owned()],
                    術體: vec![Ｏ句::歸(Ｏ歸(Ｏ算式::二元運算(Ｏ二元運算 {
                        運算子: Ｏ運算子::加,
                        左: Box::new(Ｏ算式::變數("甲".to_owned())),
                        右: Box::new(Ｏ算式::變數("乙".to_owned())),
                    })))],
                }),
                // 變數宣告，施術包含在裡面
                Ｏ頂層宣告::變數宣告(Ｏ變數宣告 {
                    變數名: "結果".to_owned(),
                    算式: Ｏ算式::施術(Ｏ施術 {
                        術名: "和".to_owned(),
                        實參: vec![
                            Ｏ算式::數字(10), // 只提供一個實參
                        ],
                    }),
                }),
            ],
        };

        // 檢查語法樹
        let 檢查結果 = 檢查語法樹(&語法樹);

        // 檢查結果
        assert!(!檢查結果); // 應該失敗
    }

    #[test]
    fn 測試若語句_條件正確() {
        let 若語句 = Ｏ若 {
            條件: Ｏ算式::變數("甲".to_owned()),
            區塊: vec![Ｏ句::算式(Ｏ算式::數字(1))],
            或若列表: vec![],
            不然: None,
        };

        let mut 環境 = Ｏ環境::new();
        環境 = 環境.insert("甲".to_owned(), Ｏ類型::變數);

        let 通過 = 檢查若(&環境, &若語句);

        assert!(通過);
    }

    #[test]
    fn 測試若語句_條件未定義() {
        let 若語句 = Ｏ若 {
            條件: Ｏ算式::變數("甲".to_string()),
            區塊: vec![Ｏ句::算式(Ｏ算式::數字(1))],
            或若列表: vec![],
            不然: None,
        };

        let 環境 = Ｏ環境::new(); // y 未宣告

        let 通過 = 檢查若(&環境, &若語句);

        assert!(!通過);
    }

    #[test]
    fn 測試句組_多個句子() {
        let 句組 = vec![
            Ｏ句::變數宣告(Ｏ變數宣告 {
                變數名: "甲".to_string(),
                算式: Ｏ算式::數字(1),
            }),
            Ｏ句::變數宣告(Ｏ變數宣告 {
                變數名: "乙".to_string(),
                算式: Ｏ算式::二元運算(Ｏ二元運算 {
                    運算子: Ｏ運算子::加,
                    左: Box::new(Ｏ算式::變數("甲".to_string())),
                    右: Box::new(Ｏ算式::數字(1)),
                }),
            }),
        ];

        let 環境 = Ｏ環境::new();
        let 通過 = 檢查區塊(&環境, &句組);

        assert!(通過);
    }
}
