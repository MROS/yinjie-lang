use super::組合子::組合子;
use super::調車場::Ｏ調車場;
use crate::分詞器::{Ｏ分詞器, Ｏ詞, Ｏ運算子};

pub struct Ｏ剖析器 {
    詞組: Vec<Ｏ詞>,
}

pub type Ｏ語法樹 = Ｏ咒;

#[derive(Debug)]
pub struct Ｏ咒 {
    pub 句: Vec<Ｏ句>,
}

#[derive(Debug)]
pub enum Ｏ句 {
    變數宣告(Ｏ變數宣告),
    算式(Ｏ算式),
}

#[derive(Debug, PartialEq)]
pub struct Ｏ變數宣告 {
    pub 變數名: String,
    pub 算式: Ｏ算式,
}

#[derive(Debug, PartialEq)]
pub enum Ｏ算式 {
    變數(String),
    數字(i64),
    二元運算(Ｏ二元運算),
}

#[derive(Debug, PartialEq)]
pub struct Ｏ二元運算 {
    pub 運算子: Ｏ運算子,
    pub 左: Box<Ｏ算式>,
    pub 右: Box<Ｏ算式>,
}

fn 剖析咒(詞組: &[Ｏ詞]) -> Option<(Ｏ咒, &[Ｏ詞])> {
    let mut 咒 = Ｏ咒 { 句: Vec::new() };
    let mut 詞組 = 詞組;

    while 詞組.len() > 0 {
        let (句, 新詞組) = 剖析句(詞組)?;

        詞組 = 新詞組;
        while let Some((_, 新詞組)) = 消耗(Ｏ詞::換行, 詞組) {
            詞組 = 新詞組;
        }

        咒.句.push(句);
    }
    Some((咒, 詞組))
}

fn 剖析句(詞組: &[Ｏ詞]) -> Option<(Ｏ句, &[Ｏ詞])> {
    剖析變數宣告(詞組)
        .映射(|變數宣告| Ｏ句::變數宣告(變數宣告))
        .or_else(|| 剖析算式(詞組).映射(|算式| Ｏ句::算式(算式)))
}

fn 剖析變數宣告(詞組: &[Ｏ詞]) -> Option<(Ｏ變數宣告, &[Ｏ詞])> {
    let ((((_, 變數名), _), 算式), 詞組) = 消耗元(詞組)
        .且(|i| 消耗音界(i))
        .且(|i| 剖析變數(i))
        .且(|i| 消耗賦值(i))
        .且(|i| 剖析算式(i))?;

    Some((Ｏ變數宣告 { 算式, 變數名 }, 詞組))
}

fn 剖析算式(詞組: &[Ｏ詞]) -> Option<(Ｏ算式, &[Ｏ詞])> {
    let (原子式, mut 詞組) = 剖析原子式(詞組)?;

    let mut 調車場 = Ｏ調車場::new(原子式);

    while let Some((新算子, 新詞組)) = 消耗運算子(詞組) {
        let (新算元, 新詞組) = 剖析原子式(新詞組)?;

        調車場.讀取(新算子, 新算元);

        詞組 = 新詞組
    }

    Some((調車場.結束(), 詞組))
}

fn 剖析原子式(詞組: &[Ｏ詞]) -> Option<(Ｏ算式, &[Ｏ詞])> {
    // 原子式 = 數字
    if let Some((數字, 詞組)) = 剖析數字(詞組) {
        return Some((Ｏ算式::數字(數字), 詞組));
    }
    // 原子式 = 變數
    if let Some((變數, 詞組)) = 剖析變數(詞組) {
        return Some((Ｏ算式::變數(變數), 詞組));
    }
    // 原子式 = （算式）
    if let Some(結果) = (|| -> Option<(Ｏ算式, &[Ｏ詞])> {
        let (_, 詞組) = 消耗(Ｏ詞::左圓括號, 詞組)?;
        let (算式, 詞組) = 剖析算式(詞組)?;
        let (_, 詞組) = 消耗(Ｏ詞::右圓括號, 詞組)?;
        Some((算式, 詞組))
    })() {
        return Some(結果);
    }
    None
}

fn 剖析數字(詞組: &[Ｏ詞]) -> Option<(i64, &[Ｏ詞])> {
    match 詞組.split_first() {
        Some((Ｏ詞::數字(數字), 剩餘詞組)) => Some((數字.clone(), 剩餘詞組)),
        Some(_) => None,
        _ => None,
    }
}

fn 剖析變數(詞組: &[Ｏ詞]) -> Option<(String, &[Ｏ詞])> {
    match 詞組.split_first() {
        Some((Ｏ詞::識別子(變數名), 剩餘詞組)) => Some((變數名.clone(), 剩餘詞組)),
        Some(_) => None,
        _ => None,
    }
}

fn 消耗運算子(詞組: &[Ｏ詞]) -> Option<(Ｏ運算子, &[Ｏ詞])> {
    match 詞組.split_first() {
        Some((Ｏ詞::運算子(運算子), 剩餘詞組)) => Some((運算子.clone(), 剩餘詞組)),
        _ => None,
    }
}

fn 消耗元(詞組: &[Ｏ詞]) -> Option<((), &[Ｏ詞])> {
    消耗(Ｏ詞::元, 詞組)
}
fn 消耗音界(詞組: &[Ｏ詞]) -> Option<((), &[Ｏ詞])> {
    消耗(Ｏ詞::音界, 詞組)
}
fn 消耗賦值(詞組: &[Ｏ詞]) -> Option<((), &[Ｏ詞])> {
    消耗(Ｏ詞::賦值, 詞組)
}

fn 消耗(詞: Ｏ詞, 詞組: &[Ｏ詞]) -> Option<((), &[Ｏ詞])> {
    match 詞組.split_first() {
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

impl Ｏ剖析器 {
    pub fn new(詞流: Vec<Ｏ詞>) -> Self {
        Ｏ剖析器 { 詞組: 詞流 }
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
}
