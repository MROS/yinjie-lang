use super::組合子::{組合子, 自動映射組合子};
use super::調車場::Ｏ調車場;
use crate::分詞器::{Ｏ分詞器, Ｏ詞, Ｏ運算子};

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
impl Into<Ｏ句> for Ｏ變數宣告 {
    fn into(self) -> Ｏ句 {
        Ｏ句::變數宣告(self)
    }
}
impl Into<Ｏ句> for Ｏ算式 {
    fn into(self) -> Ｏ句 {
        Ｏ句::算式(self)
    }
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
impl Into<Ｏ算式> for String {
    fn into(self) -> Ｏ算式 {
        Ｏ算式::變數(self)
    }
}
impl Into<Ｏ算式> for i64 {
    fn into(self) -> Ｏ算式 {
        Ｏ算式::數字(self)
    }
}
impl Into<Ｏ算式> for Ｏ二元運算 {
    fn into(self) -> Ｏ算式 {
        Ｏ算式::二元運算(self)
    }
}

#[derive(Debug, PartialEq)]
pub struct Ｏ二元運算 {
    pub 運算子: Ｏ運算子,
    pub 左: Box<Ｏ算式>,
    pub 右: Box<Ｏ算式>,
}
