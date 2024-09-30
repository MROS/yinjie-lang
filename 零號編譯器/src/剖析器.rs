use crate::分詞器::{Ｏ分詞器, Ｏ詞, Ｏ運算子};
use std::collections::VecDeque;

pub struct Ｏ剖析器 {
    詞流: VecDeque<Ｏ詞>,
}

pub type Ｏ語法樹 = Ｏ咒;

#[derive(Debug)]
pub struct Ｏ咒 {
    句: Vec<Ｏ句>,
}

#[derive(Debug)]
enum Ｏ句 {
    變數宣告(Ｏ變數宣告),
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
    算式(Ｏ二元運算),
}

#[derive(Debug)]
struct Ｏ二元運算 {
    運算子: Ｏ運算子,
    左: Box<Ｏ算式>,
    右: Box<Ｏ算式>,
}

impl Ｏ剖析器 {
    pub fn new(詞流: VecDeque<Ｏ詞>) -> Self {
        Ｏ剖析器 { 詞流 }
    }

    pub fn 剖析(mut self) -> Option<Ｏ語法樹> {
        self.剖析咒()
    }

    fn 剖析咒(mut self) -> Option<Ｏ咒> {
        let mut 咒 = Ｏ咒 { 句: Vec::new() };
        Some(咒)
    }
}
