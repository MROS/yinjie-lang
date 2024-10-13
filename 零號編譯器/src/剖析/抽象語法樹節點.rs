use crate::分詞器::Ｏ運算子;
use derive_more::From;

pub type Ｏ語法樹 = Ｏ咒;

#[derive(Debug)]
pub struct Ｏ咒 {
    pub 頂層宣告: Vec<Ｏ頂層宣告>,
}

#[derive(Debug, PartialEq, From)]
pub enum Ｏ頂層宣告 {
    變數宣告(Ｏ變數宣告),
    術宣告(Ｏ術宣告),
}

#[derive(Debug, PartialEq, From)]
pub enum Ｏ句 {
    變數宣告(Ｏ變數宣告),
    算式(Ｏ算式),
    歸(Ｏ歸),
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
    施術(Ｏ施術),
}

#[derive(Debug, PartialEq, From)]
pub struct Ｏ歸(pub Ｏ算式);

#[derive(Debug, PartialEq, From)]
pub struct Ｏ若 {
    pub 條件: Ｏ算式,
    pub 區塊: Vec<Ｏ句>,
    pub 或若列表: Vec<Ｏ或若>,
    pub 不然: Option<Ｏ不然>,
}

#[derive(Debug, PartialEq, From)]
pub struct Ｏ或若 {
    pub 條件: Ｏ算式,
    pub 區塊: Vec<Ｏ句>,
}

#[derive(Debug, PartialEq, From)]
pub struct Ｏ不然 {
    pub 區塊: Vec<Ｏ句>,
}

#[derive(Debug, PartialEq)]
pub struct Ｏ術宣告 {
    pub 術名: String,
    pub 形參: Vec<String>,
    pub 術體: Vec<Ｏ句>,
}

#[derive(Debug, PartialEq)]
pub struct Ｏ施術 {
    pub 術名: String,
    pub 實參: Vec<Ｏ算式>,
}

#[derive(Debug, PartialEq)]
pub struct Ｏ二元運算 {
    pub 運算子: Ｏ運算子,
    pub 左: Box<Ｏ算式>,
    pub 右: Box<Ｏ算式>,
}
