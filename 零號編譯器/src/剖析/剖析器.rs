use super::抽象語法樹節點::*;
use super::組合子::{組合子, 自動映射組合子};
use super::調車場::Ｏ調車場;
use crate::分詞器::{
    Ｏ分詞器,
    Ｏ詞::{self, *},
    Ｏ運算子,
};

fn 剖析咒(詞組: &[Ｏ詞]) -> Option<(Ｏ咒, &[Ｏ詞])> {
    let mut 咒 = Ｏ咒 {
        頂層宣告: Vec::new(),
    };
    let mut 詞組 = 詞組;

    while 詞組.len() > 0 {
        println!("{:?}\n", 詞組);
        let (頂層宣告, 新詞組) = 剖析頂層宣告(詞組)?;
        println!("{:?}\n", 新詞組);

        詞組 = 新詞組;
        while let Some((_, 新詞組)) = 消耗(換行)(詞組) {
            詞組 = 新詞組;
        }

        println!("{:?}\n", 頂層宣告);

        咒.頂層宣告.push(頂層宣告);
    }
    Some((咒, 詞組))
}

fn 剖析頂層宣告(詞組: &[Ｏ詞]) -> Option<(Ｏ頂層宣告, &[Ｏ詞])> {
    剖析變數宣告(詞組)
        .自動映射()
        .or_else(|| 剖析術宣告(詞組).自動映射())
}

fn 剖析句(詞組: &[Ｏ詞]) -> Option<(Ｏ句, &[Ｏ詞])> {
    剖析變數宣告(詞組)
        .自動映射()
        .or_else(|| 剖析算式(詞組).自動映射())
        .or_else(|| 剖析歸(詞組).自動映射())
        .or_else(|| 剖析若(詞組).自動映射())
}

fn 剖析歸(詞組: &[Ｏ詞]) -> Option<(Ｏ歸, &[Ｏ詞])> {
    消耗(歸)(詞組)
        .且棄(消耗(音界))
        .且(剖析算式)
        .映射(|(_, 算式)| 算式)
        .自動映射()
}

fn 剖析變數宣告(詞組: &[Ｏ詞]) -> Option<(Ｏ變數宣告, &[Ｏ詞])> {
    let (((_, 變數名), 算式), 詞組) = 消耗(元)(詞組)
        .且棄(消耗(音界))
        .且(剖析識別子)
        .且棄(消耗(賦值))
        .且(剖析算式)?;

    Some((Ｏ變數宣告 { 算式, 變數名 }, 詞組))
}

fn 剖析若(詞組: &[Ｏ詞]) -> Option<(Ｏ若, &[Ｏ詞])> {
    消耗(若)(詞組)
        .且棄(消耗(左圓括號))
        .且(剖析算式)
        .且棄(消耗(右圓括號))
        .且棄(消耗(左基括號))
        .重複(剖析句)
        .且棄(消耗(右基括號))
        .重複(剖析或若)
        .可選(剖析不然)
        .映射(|((((_, 條件), 區塊), 或若列表), _不然)| Ｏ若 {
            條件,
            區塊,
            或若列表,
            不然: _不然,
        })
}

fn 剖析或若(詞組: &[Ｏ詞]) -> Option<(Ｏ或若, &[Ｏ詞])> {
    消耗(或若)(詞組)
        .且棄(消耗(左圓括號))
        .且(剖析算式)
        .且棄(消耗(右圓括號))
        .且棄(消耗(左基括號))
        .重複(剖析句)
        .且棄(消耗(右基括號))
        .映射(|((_, 條件), 區塊)| Ｏ或若 { 條件, 區塊 })
}

fn 剖析不然(詞組: &[Ｏ詞]) -> Option<(Ｏ不然, &[Ｏ詞])> {
    消耗(不然)(詞組)
        .且棄(消耗(左基括號))
        .重複(剖析句)
        .且棄(消耗(右基括號))
        .映射(|(_, 區塊)| Ｏ不然 { 區塊 })
}

fn 合併<T>(首元素: T, 其餘元素: Vec<T>) -> Vec<T> {
    let mut 元素陣列 = vec![首元素];
    元素陣列.extend(其餘元素);
    元素陣列
}

fn 剖析施術(詞組: &[Ｏ詞]) -> Option<(Ｏ施術, &[Ｏ詞])> {
    剖析識別子(詞組)
        .且棄(消耗(左圓括號))
        .且(剖析術實參)
        .且棄(消耗(右圓括號))
        .映射(|(術名, 實參)| Ｏ施術 { 術名, 實參 })
}

fn 剖析術實參(詞組: &[Ｏ詞]) -> Option<(Vec<Ｏ算式>, &[Ｏ詞])> {
    // 術實參 = 算式(．頓號．算式)*
    剖析算式(詞組)
        .重複(|詞組| 消耗(頓號)(詞組).且(剖析算式).映射(|元組| 元組.1))
        .映射(|(首算式, 其餘算式)| 合併(首算式, 其餘算式))
        // 術實參 = e
        .or_else(|| 空(詞組).映射(|_| vec![]))
}

fn 剖析術形參(詞組: &[Ｏ詞]) -> Option<(Vec<String>, &[Ｏ詞])> {
    // 術形參 = 識別子(．頓號．識別子)*
    剖析識別子(詞組)
        .重複(|詞組| 消耗(頓號)(詞組).且(剖析識別子).映射(|元組| 元組.1))
        .映射(|(首識別子, 其餘識別子)| 合併(首識別子, 其餘識別子))
        // 術形參 = e
        .or_else(|| 空(詞組).映射(|_| vec![]))
}

fn 剖析術體(詞組: &[Ｏ詞]) -> Option<(Vec<Ｏ句>, &[Ｏ詞])> {
    空(詞組).重複(剖析句).映射(|(_, 句組)| 句組)
}

fn 剖析術宣告(詞組: &[Ｏ詞]) -> Option<(Ｏ術宣告, &[Ｏ詞])> {
    消耗(術)(詞組)
        .且棄(消耗(音界))
        .且(剖析識別子)
        .且棄(消耗(左圓括號))
        .且(剖析術形參)
        .且棄(消耗(右圓括號))
        .且棄(消耗(左基括號))
        .且(剖析術體)
        .且棄(消耗(右基括號))
        .映射(|(((_, 術名), 形參), 術體)| Ｏ術宣告 {
            術名, 形參, 術體
        })
}

fn 剖析算式(詞組: &[Ｏ詞]) -> Option<(Ｏ算式, &[Ｏ詞])> {
    let (原子式, mut 詞組) = 剖析原子式(詞組)?;

    let mut 調車場 = Ｏ調車場::new(原子式);

    while let Some((新算子, 新詞組)) = 剖析運算子(詞組) {
        let (新算元, 新詞組) = 剖析原子式(新詞組)?;

        調車場.讀取(新算子, 新算元);

        詞組 = 新詞組
    }

    Some((調車場.結束(), 詞組))
}

fn 剖析原子式(詞組: &[Ｏ詞]) -> Option<(Ｏ算式, &[Ｏ詞])> {
    // 原子式 = 數字
    剖析數字(詞組)
        .自動映射()
        // 原子式 = 施術
        .or_else(|| 剖析施術(詞組).自動映射())
        // 原子式 = 變數
        .or_else(|| 剖析識別子(詞組).自動映射())
        // 原子式 = （算式）
        .or_else(|| {
            消耗(左圓括號)(詞組)
                .且(剖析算式)
                .且(消耗(右圓括號))
                .映射(|((_, 算式), _)| 算式)
                .自動映射()
        })
}

fn 剖析數字(詞組: &[Ｏ詞]) -> Option<(i64, &[Ｏ詞])> {
    match 詞組.split_first() {
        Some((數字(數), 剩餘詞組)) => Some((數.clone(), 剩餘詞組)),
        Some(_) => None,
        _ => None,
    }
}

fn 剖析識別子(詞組: &[Ｏ詞]) -> Option<(String, &[Ｏ詞])> {
    match 詞組.split_first() {
        Some((Ｏ詞::識別子(識別子名), 剩餘詞組)) => Some((識別子名.clone(), 剩餘詞組)),
        Some(_) => None,
        _ => None,
    }
}

fn 剖析運算子(詞組: &[Ｏ詞]) -> Option<(Ｏ運算子, &[Ｏ詞])> {
    match 詞組.split_first() {
        Some((運算子(算子), 剩餘詞組)) => Some((算子.clone(), 剩餘詞組)),
        _ => None,
    }
}

fn 消耗(詞: Ｏ詞) -> impl Fn(&[Ｏ詞]) -> Option<((), &[Ｏ詞])> {
    move |詞組: &[Ｏ詞]| match 詞組.split_first() {
        Some((當前詞, 剩餘詞組)) => {
            if *當前詞 == 詞 {
                Some(((), 剩餘詞組))
            } else {
                None
            }
        }
        None => None,
    }
}

fn 空(詞組: &[Ｏ詞]) -> Option<((), &[Ｏ詞])> {
    Some(((), 詞組))
}

pub struct Ｏ剖析器 {
    詞組: Vec<Ｏ詞>,
}

impl Ｏ剖析器 {
    pub fn new(詞組: Vec<Ｏ詞>) -> Self {
        Ｏ剖析器 { 詞組 }
    }

    pub fn 剖析(&mut self) -> Option<Ｏ語法樹> {
        剖析咒(&self.詞組).map(|剖析結果| 剖析結果.0)
    }
}

#[cfg(test)]
mod 測試 {
    use super::*;
    fn 分詞(源碼: &'static str) -> Vec<Ｏ詞> {
        Ｏ分詞器::new(源碼.to_owned()).分詞()
    }

    #[test]
    fn 測試運算子優先級() {
        let 詞組 = 分詞("１＝＝５－３％２＊４－１");
        let (算式, _) = 剖析算式(&詞組).unwrap();
        use Ｏ算式::*;
        use Ｏ運算子::*;
        assert_eq!(
            算式,
            二元運算(Ｏ二元運算 {
                運算子: 等於,
                左: Box::new(數字(1)),
                右: Box::new(二元運算(Ｏ二元運算 {
                    運算子: 減,
                    左: Box::new(二元運算(Ｏ二元運算 {
                        運算子: 減,
                        左: Box::new(數字(5)),
                        右: Box::new(二元運算(Ｏ二元運算 {
                            運算子: 餘,
                            左: Box::new(數字(3)),
                            右: Box::new(二元運算(Ｏ二元運算 {
                                運算子: 乘,
                                左: Box::new(數字(2)),
                                右: Box::new(數字(4)),
                            })),
                        }))
                    })),
                    右: Box::new(數字(1)),
                }))
            })
        )
    }
    #[test]
    fn 測試括號優先級() {
        let 詞組 = 分詞("（（１＝＝５）－３）％２");
        let (算式, _) = 剖析算式(&詞組).unwrap();
        use Ｏ算式::*;
        use Ｏ運算子::*;
        assert_eq!(
            算式,
            二元運算(Ｏ二元運算 {
                運算子: 餘,
                左: Box::new(二元運算(Ｏ二元運算 {
                    運算子: 減,
                    左: Box::new(二元運算(Ｏ二元運算 {
                        運算子: 等於,
                        左: Box::new(數字(1)),
                        右: Box::new(數字(5))
                    })),
                    右: Box::new(數字(3))
                })),
                右: Box::new(數字(2))
            })
        )
    }
    #[test]
    fn 測試變數宣告() {
        let 詞組 = 分詞("元．甲＝１");
        let (變數宣告, _) = 剖析變數宣告(&詞組).unwrap();
        use Ｏ算式::*;
        assert_eq!(
            變數宣告,
            Ｏ變數宣告 {
                變數名: "甲".to_owned(),
                算式: 數字(1),
            }
        )
    }
    #[test]
    fn 測試空術宣告() {
        let 詞組 = 分詞("術．積（）【】");
        let (術宣告, _) = 剖析術宣告(&詞組).unwrap();
        assert_eq!(
            術宣告,
            Ｏ術宣告 {
                術名: "積".to_owned(),
                形參: vec![],
                術體: vec![],
            }
        )
    }
    #[test]
    fn 測試術宣告() {
        let 詞組 = 分詞("術．積（甲、乙）【歸．甲＋乙】");
        let (術宣告, _) = 剖析術宣告(&詞組).unwrap();
        use Ｏ算式::*;
        assert_eq!(
            術宣告,
            Ｏ術宣告 {
                術名: "積".to_owned(),
                形參: vec!["甲".to_owned(), "乙".to_owned()],
                術體: vec![Ｏ句::歸(Ｏ歸(二元運算(Ｏ二元運算 {
                    運算子: Ｏ運算子::加,
                    左: Box::new(變數("甲".to_owned())),
                    右: Box::new(變數("乙".to_owned()))
                })))],
            }
        )
    }
    #[test]
    fn 測試施術() {
        let 詞組 = 分詞("積（甲、乙）");
        let (施術, _) = 剖析施術(&詞組).unwrap();
        use Ｏ算式::變數;
        assert_eq!(
            施術,
            Ｏ施術 {
                術名: "積".to_owned(),
                實參: vec![變數("甲".to_owned()), 變數("乙".to_owned())],
            }
        )
    }
    #[test]
    fn 測試歸() {
        let 詞組 = 分詞("歸．１");
        let (歸語句, _) = 剖析歸(&詞組).unwrap();
        assert_eq!(歸語句, Ｏ歸(Ｏ算式::數字(1)))
    }
    #[test]
    fn 測試若_或若_不然() {
        let 詞組 = 分詞("若（１）【 】或若（２）【 】或若（３）【 】不然【】");
        let (若語句, _) = 剖析若(&詞組).unwrap();
        use Ｏ算式::數字;
        assert_eq!(
            若語句,
            Ｏ若 {
                條件: 數字(1),
                區塊: vec![],
                或若列表: vec![
                    Ｏ或若 {
                        條件: 數字(2),
                        區塊: vec![],
                    },
                    Ｏ或若 {
                        條件: 數字(3),
                        區塊: vec![],
                    }
                ],
                不然: Some(Ｏ不然 { 區塊: vec![] })
            }
        )
    }
    #[test]
    fn 測試若() {
        let 詞組 = 分詞("若（１）【 】");
        let (若語句, _) = 剖析若(&詞組).unwrap();
        use Ｏ算式::數字;
        assert_eq!(
            若語句,
            Ｏ若 {
                條件: 數字(1),
                區塊: vec![],
                或若列表: vec![],
                不然: None
            }
        )
    }
}
