use std::collections::VecDeque;

// Rust 慣以駝峰式命名類型
// 漢語無大小寫，本作慣例以全形英文字母Ｏ來當類型的開頭
// Rust 管制識別符的字元組成，不允許 ◉、⦿、☯︎ 等等萬國碼，故採用常見的全形Ｏ來代替。
#[derive(Debug, PartialEq, Clone)]
pub enum Ｏ運算子 {
    // 四則運算
    加, // ＋
    減, // －
    乘, // ＊
    除, // ／

    // 取餘數
    餘, // ％

    // 比較
    等於,     // ＝＝
    異於,     // ！＝
    小於,     // ＜
    小於等於, // ＜＝
    大於,     // ＞
    大於等於, // ＞＝
}

#[derive(Debug, PartialEq)]
pub enum Ｏ詞 {
    // 運算子
    運算子(Ｏ運算子),

    // 關鍵字
    元,
    術,
    若,
    或若,
    不然,
    歸,

    // 括號
    左圓括號,
    右圓括號,
    左基括號,
    右基括號,

    // 特殊符號
    賦值, // ＝
    音界,
    頓號,
    換行,
    空白,

    數字(i64),
    識別子(String),
}

enum Ｏ字類 {
    特殊符號,
    數字,
    其他,
}

fn 字類(字: &char) -> Ｏ字類 {
    match 字 {
        '＋' | '－' | '＊' | '／' | '％' | '＝' | '＞' | '＜' | '！' | '（' | '）' | '【'
        | '】' | '．' | '、' | '\n' | '\t' | ' ' | '　' => Ｏ字類::特殊符號,
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
        let mut 字流: VecDeque<char> = 源碼.chars().collect();
        字流.push_back('\n'); // 檔尾硬加一個換行來方便變數、數字分詞

        Ｏ分詞器 { 字流 }
    }

    pub fn 分詞(mut self) -> VecDeque<Ｏ詞> {
        let mut 詞列: VecDeque<Ｏ詞> = VecDeque::new();
        while self.字流.front().is_some() {
            match self.起點態() {
                Some(詞) => {
                    // println!("{:?}", 詞);
                    詞列.push_back(詞);
                }
                None => {
                    panic!("分詞錯誤");
                }
            }
        }
        詞列.into_iter().filter(|詞| *詞 != Ｏ詞::空白).collect()
    }

    fn 起點態(&mut self) -> Option<Ｏ詞> {
        let 字 = self.字流.pop_front()?;
        match 字 {
            '＋' => Some(Ｏ詞::運算子(Ｏ運算子::加)),
            '－' => Some(Ｏ詞::運算子(Ｏ運算子::減)),
            '＊' => Some(Ｏ詞::運算子(Ｏ運算子::乘)),
            '／' => Some(Ｏ詞::運算子(Ｏ運算子::除)),
            '％' => Some(Ｏ詞::運算子(Ｏ運算子::餘)),
            '＝' => match self.字流.front() {
                Some('＝') => {
                    self.字流.pop_front()?;
                    Some(Ｏ詞::運算子(Ｏ運算子::等於))
                }
                _ => Some(Ｏ詞::賦值),
            },
            '！' => match self.字流.front() {
                Some('＝') => {
                    self.字流.pop_front()?;
                    Some(Ｏ詞::運算子(Ｏ運算子::異於))
                }
                _ => panic!("！後必接＝"),
            },
            '＜' => match self.字流.front() {
                Some('＝') => {
                    self.字流.pop_front()?;
                    Some(Ｏ詞::運算子(Ｏ運算子::小於等於))
                }
                _ => Some(Ｏ詞::運算子(Ｏ運算子::小於)),
            },
            '＞' => match self.字流.front() {
                Some('＝') => {
                    self.字流.pop_front()?;
                    Some(Ｏ詞::運算子(Ｏ運算子::大於等於))
                }
                _ => Some(Ｏ詞::運算子(Ｏ運算子::大於)),
            },
            '（' => Some(Ｏ詞::左圓括號),
            '）' => Some(Ｏ詞::右圓括號),
            '【' => Some(Ｏ詞::左基括號),
            '】' => Some(Ｏ詞::右基括號),
            '．' => Some(Ｏ詞::音界),
            '、' => Some(Ｏ詞::頓號),
            '\n' => Some(Ｏ詞::換行),
            '\t' | ' ' | '　' => Some(Ｏ詞::空白),
            '１' | '２' | '３' | '４' | '５' | '６' | '７' | '８' | '９' | '０' => {
                self.數字態(字.to_string())
            }
            _ => self.識別子態(字.to_string()),
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
                self.識別子態(前綴)
            }
            _ => {
                let 數 = crate::全形處理::數字::字串轉整數(&前綴);
                Some(Ｏ詞::數字(數))
            }
        }
    }
    fn 識別子態(&mut self, mut 前綴: String) -> Option<Ｏ詞> {
        let 字 = self.字流.front()?;
        match 字類(字) {
            Ｏ字類::數字 | Ｏ字類::其他 => {
                前綴.push(self.字流.pop_front()?);
                self.識別子態(前綴)
            }
            Ｏ字類::特殊符號 => {
                // 遇到特殊符號，識別子截止
                // 判定是否是關鍵字
                match 前綴.as_str() {
                    "元" => Some(Ｏ詞::元),
                    "術" => Some(Ｏ詞::術),
                    "歸" => Some(Ｏ詞::歸),
                    "若" => Some(Ｏ詞::若),
                    "或若" => Some(Ｏ詞::或若),
                    "不然" => Some(Ｏ詞::不然),
                    _ => Some(Ｏ詞::識別子(前綴)),
                }
            }
        }
    }
}
