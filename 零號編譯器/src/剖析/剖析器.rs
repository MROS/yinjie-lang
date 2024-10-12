use super::調車場::調車場;
use crate::分詞器::{Ｏ分詞器, Ｏ詞, Ｏ運算子};
use std::collections::VecDeque;

pub struct Ｏ剖析器 {
    詞組: VecDeque<Ｏ詞>,
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

#[derive(Debug)]
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

impl Ｏ剖析器 {
    pub fn new(詞流: VecDeque<Ｏ詞>) -> Self {
        Ｏ剖析器 { 詞組: 詞流 }
    }

    pub fn 剖析(&mut self) -> Option<Ｏ語法樹> {
        self.剖析咒(0).map(|剖析結果| 剖析結果.0)
    }

    fn 剖析咒(&mut self, 游標: usize) -> Option<(Ｏ咒, usize)> {
        println!("剖析咒 {}", 游標);
        let mut 咒 = Ｏ咒 { 句: Vec::new() };
        let mut 游標 = 游標;

        while self.詞組.len() > 游標 {
            let (句, 新游標) = self.剖析句(游標)?;
            游標 = 新游標;
            咒.句.push(句);
        }
        Some((咒, 游標))
    }

    fn 剖析句(&self, 游標: usize) -> Option<(Ｏ句, usize)> {
        println!("剖析句 {}", 游標);
        if let Some((變數宣告, mut 游標)) = self.剖析變數宣告(游標) {
            // 忽略換行
            while let Some(新游標) = self.消耗(游標, Ｏ詞::換行) {
                游標 = 新游標;
            }

            return Some((Ｏ句::變數宣告(變數宣告), 游標));
        }
        if let Some((算式, mut 游標)) = self.剖析算式(游標) {
            // 忽略換行
            while let Some(新游標) = self.消耗(游標, Ｏ詞::換行) {
                游標 = 新游標;
            }

            return Some((Ｏ句::算式(算式), 游標));
        }

        None
    }

    fn 剖析變數宣告(&self, 游標: usize) -> Option<(Ｏ變數宣告, usize)> {
        let 游標 = self.消耗(游標, Ｏ詞::元)?;
        let 游標 = self.消耗(游標, Ｏ詞::音界)?;
        let (變數名, 游標) = self.剖析變數(游標)?;
        let 游標 = self.消耗(游標, Ｏ詞::賦值)?;
        let (算式, 游標) = self.剖析算式(游標)?;

        Some((Ｏ變數宣告 { 算式, 變數名 }, 游標))
    }

    fn 剖析算式(&self, 游標: usize) -> Option<(Ｏ算式, usize)> {
        let (原子式, mut 游標) = self.剖析原子式(游標)?;

        let mut 調車場 = 調車場::new(原子式);

        while let Some((新算子, 新游標)) = self.消耗運算子(游標) {
            let (新算元, 新游標) = self.剖析原子式(新游標)?;

            調車場.讀取(新算子, 新算元);

            游標 = 新游標
        }

        Some((調車場.結束(), 游標))
    }

    fn 剖析原子式(&self, 游標: usize) -> Option<(Ｏ算式, usize)> {
        // 原子式 = 數字
        if let Some((數字, 游標)) = self.剖析數字(游標) {
            return Some((Ｏ算式::數字(數字), 游標));
        }
        // 原子式 = 變數
        if let Some((變數, 游標)) = self.剖析變數(游標) {
            return Some((Ｏ算式::變數(變數), 游標));
        }
        // 原子式 = （算式）
        if let Some(結果) = (|| -> Option<(Ｏ算式, usize)> {
            let 游標 = self.消耗(游標, Ｏ詞::左圓括號)?;
            let (算式, 游標) = self.剖析算式(游標)?;
            let 游標 = self.消耗(游標, Ｏ詞::右圓括號)?;
            Some((算式, 游標))
        })() {
            return Some(結果);
        }
        None
    }

    fn 剖析數字(&self, 游標: usize) -> Option<(i64, usize)> {
        match self.詞組.get(游標) {
            Some(Ｏ詞::數字(數字)) => Some((數字.clone(), 游標 + 1)),
            Some(_) => None,
            _ => None,
        }
    }

    fn 剖析變數(&self, 游標: usize) -> Option<(String, usize)> {
        match self.詞組.get(游標) {
            Some(Ｏ詞::識別子(變數名)) => Some((變數名.clone(), 游標 + 1)),
            Some(_) => None,
            _ => None,
        }
    }

    fn 消耗運算子(&self, 游標: usize) -> Option<(Ｏ運算子, usize)> {
        match self.詞組.get(游標) {
            Some(Ｏ詞::運算子(運算子)) => Some((運算子.clone(), 游標 + 1)),
            _ => None,
        }
    }

    fn 消耗(&self, 游標: usize, 詞: Ｏ詞) -> Option<usize> {
        match self.詞組.get(游標) {
            Some(當前詞) => {
                if 當前詞 == &詞 {
                    Some(游標 + 1)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod 測試 {
    use super::*;
    fn 源碼剖析器(源碼: &'static str) -> Ｏ剖析器 {
        let 詞組 = Ｏ分詞器::new(源碼.to_owned()).分詞();
        Ｏ剖析器::new(詞組)
    }

    #[test]
    fn 測試運算子優先級() {
        let 剖析器 = 源碼剖析器("１＝＝５－３％２＊４－１");
        let (算式, _) = 剖析器.剖析算式(0).unwrap();
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
        let 剖析器 = 源碼剖析器("（（１＝＝５）－３）％２");
        let (算式, _) = 剖析器.剖析算式(0).unwrap();
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
}
