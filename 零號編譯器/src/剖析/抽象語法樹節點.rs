use crate::分詞器::Ｏ運算子;
use derive_more::From;

pub type Ｏ語法樹 = Ｏ咒;

#[derive(Debug)]
pub struct Ｏ咒 {
    pub 句: Vec<Ｏ句>,
}

#[derive(Debug, From)]
pub enum Ｏ句 {
    變數宣告(Ｏ變數宣告),
    算式(Ｏ算式),
}

#[derive(Debug, PartialEq)]
pub struct Ｏ變數宣告 {
    pub 變數名: String,
    pub 算式: Ｏ算式,
}

#[derive(Debug, PartialEq, From)]
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
