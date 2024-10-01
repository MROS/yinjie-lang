use crate::分詞器::{Ｏ分詞器, Ｏ詞, Ｏ運算子};
use std::collections::VecDeque;

pub struct Ｏ剖析器 {
    詞組: VecDeque<Ｏ詞>,
}

pub type Ｏ語法樹 = Ｏ咒;

#[derive(Debug)]
pub struct Ｏ咒 {
    句: Vec<Ｏ句>,
}

#[derive(Debug)]
enum Ｏ句 {
    變數宣告(Ｏ變數宣告),
    算式(Ｏ算式),
}

#[derive(Debug)]
struct Ｏ變數宣告 {
    變數名: String,
    算式: Ｏ算式,
}

#[derive(Debug)]
enum Ｏ算式 {
    變數(String),
    數字(i64),
    二元運算(Ｏ二元運算),
}

#[derive(Debug)]
struct Ｏ二元運算 {
    運算子: Ｏ運算子,
    左: Box<Ｏ算式>,
    右: Box<Ｏ算式>,
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
        let 游標 = self.消耗(游標, Ｏ詞::等號)?;
        let (算式, 游標) = self.剖析算式(游標)?;

        Some((Ｏ變數宣告 { 算式, 變數名 }, 游標))
    }

    fn 剖析算式(&self, 游標: usize) -> Option<(Ｏ算式, usize)> {
        let (mut 算式, mut 游標) = self.剖析乘除式(游標)?;
        while let Some((運算子, 新游標)) = self.消耗加減(游標) {
            let (右算元, 新游標) = self.剖析乘除式(新游標)?;

            算式 = Ｏ算式::二元運算(Ｏ二元運算 {
                左: Box::new(算式),
                右: Box::new(右算元),
                運算子,
            });
            游標 = 新游標
        }
        Some((算式, 游標))
    }

    fn 剖析乘除式(&self, 游標: usize) -> Option<(Ｏ算式, usize)> {
        let (mut 算式, mut 游標) = self.剖析原子式(游標)?;
        while let Some((運算子, 新游標)) = self.消耗乘除(游標) {
            let (右算元, 新游標) = self.剖析原子式(新游標)?;

            算式 = Ｏ算式::二元運算(Ｏ二元運算 {
                左: Box::new(算式),
                右: Box::new(右算元),
                運算子,
            });
            游標 = 新游標
        }
        Some((算式, 游標))
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
            let 游標 = self.消耗(游標, Ｏ詞::左括號)?;
            let (算式, 游標) = self.剖析算式(游標)?;
            let 游標 = self.消耗(游標, Ｏ詞::右括號)?;
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
            Some(Ｏ詞::變數(變數名)) => Some((變數名.clone(), 游標 + 1)),
            Some(_) => None,
            _ => None,
        }
    }

    fn 消耗乘除(&self, 游標: usize) -> Option<(Ｏ運算子, usize)> {
        if let Some(游標) = self.消耗(游標, Ｏ詞::運算子(Ｏ運算子::乘)) {
            Some((Ｏ運算子::乘, 游標))
        } else if let Some(游標) = self.消耗(游標, Ｏ詞::運算子(Ｏ運算子::除)) {
            Some((Ｏ運算子::除, 游標))
        } else {
            None
        }
    }

    fn 消耗加減(&self, 游標: usize) -> Option<(Ｏ運算子, usize)> {
        if let Some(游標) = self.消耗(游標, Ｏ詞::運算子(Ｏ運算子::加)) {
            Some((Ｏ運算子::加, 游標))
        } else if let Some(游標) = self.消耗(游標, Ｏ詞::運算子(Ｏ運算子::減)) {
            Some((Ｏ運算子::減, 游標))
        } else {
            None
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
