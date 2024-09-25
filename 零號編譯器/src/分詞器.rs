use std::collections::VecDeque;

// Rust 慣以駝峰式命名類型
// 漢語無大小寫，本作慣例以全形英文字母Ｏ來當類型的開頭
// Rust 管制識別符的字元組成，不允許 ◉、⦿、☯︎ 等等萬國碼，故採用常見的全形Ｏ來代替。
#[derive(Debug)]
enum Ｏ運算子 {
    加,
    減,
    乘,
    除,
}

#[derive(Debug)]
pub enum Ｏ詞 {
    元,
    左括號,
    右括號,
    運算子(Ｏ運算子),
    等,
    音界,
    換行,
    數字(i64),
    變數(String),
}

enum Ｏ字類 {
    特殊符號,
    數字,
    元,
    其他,
}

fn 字類(字: &char) -> Ｏ字類 {
    match 字 {
        '＋' | '－' | '＊' | '／' | '＝' | '（' | '）' | '・' | '\n' => {
            Ｏ字類::特殊符號
        }
        '元' => Ｏ字類::元,
        '１' | '２' | '３' | '４' | '５' | '６' | '７' | '８' | '９' | '０' => {
            Ｏ字類::數字
        }
        _ => Ｏ字類::其他,
    }
}

pub struct Ｏ分詞器 {
    字流: VecDeque<char>,
}

impl Ｏ分詞器 {
    pub fn new(源碼: String) -> Self {
        Ｏ分詞器 {
            字流: 源碼.chars().collect(),
        }
    }

    fn 起點態(&mut self) -> Option<Ｏ詞> {
        let 字 = self.字流.pop_front()?;
        match 字 {
            '＋' => Some(Ｏ詞::運算子(Ｏ運算子::加)),
            '－' => Some(Ｏ詞::運算子(Ｏ運算子::減)),
            '＊' => Some(Ｏ詞::運算子(Ｏ運算子::乘)),
            '／' => Some(Ｏ詞::運算子(Ｏ運算子::除)),
            '＝' => Some(Ｏ詞::等),
            '（' => Some(Ｏ詞::左括號),
            '）' => Some(Ｏ詞::右括號),
            '・' => Some(Ｏ詞::音界),
            '\n' => Some(Ｏ詞::換行),
            '元' => self.元態(),
            '１' | '２' | '３' | '４' | '５' | '６' | '７' | '８' | '９' | '０' => {
                self.數字態(字.to_string())
            }
            _ => self.變數態(字.to_string()),
        }
    }
    fn 元態(&mut self) -> Option<Ｏ詞> {
        let 字 = self.字流.front()?;
        match 字類(字) {
            Ｏ字類::元 | Ｏ字類::數字 | Ｏ字類::其他 => {
                self.變數態("元".to_string())
            }
            _ => Some(Ｏ詞::元),
        }
    }
    fn 數字態(&mut self, mut 前綴: String) -> Option<Ｏ詞> {
        let 字 = self.字流.front()?;
        match 字類(字) {
            Ｏ字類::數字 => {
                前綴.push(self.字流.pop_front()?);
                self.數字態(前綴)
            }
            Ｏ字類::其他 => {
                前綴.push(self.字流.pop_front()?);
                self.變數態(前綴)
            }
            _ => {
                let 數 = crate::全形處理::數字::字串轉整數(&前綴);
                Some(Ｏ詞::數字(數))
            }
        }
    }
    fn 變數態(&mut self, mut 前綴: String) -> Option<Ｏ詞> {
        let 字 = self.字流.front()?;
        match 字類(字) {
            Ｏ字類::元 | Ｏ字類::數字 | Ｏ字類::其他 => {
                前綴.push(self.字流.pop_front()?);
                self.變數態(前綴)
            }
            _ => Some(Ｏ詞::變數(前綴)),
        }
    }

    pub fn 分詞(mut self) -> Vec<Ｏ詞> {
        let mut 詞列: Vec<Ｏ詞> = Vec::new();
        while self.字流.front().is_some() {
            match self.起點態() {
                Some(詞) => {
                    詞列.push(詞);
                }
                None => {
                    panic!("分詞錯誤");
                }
            }
        }
        詞列
    }
}
