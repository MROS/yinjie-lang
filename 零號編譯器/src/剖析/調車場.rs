use crate::分詞器::Ｏ運算子;
use crate::剖析::{Ｏ二元運算, Ｏ算式};
use std::collections::VecDeque;

fn 優先級(運算子: &Ｏ運算子) -> u64 {
    match 運算子 {
        Ｏ運算子::乘 => 4,
        Ｏ運算子::除 => 4,

        Ｏ運算子::餘 => 3,

        Ｏ運算子::加 => 2,
        Ｏ運算子::減 => 2,

        Ｏ運算子::等於 => 1,
        Ｏ運算子::異於 => 1,
        Ｏ運算子::小於 => 1,
        Ｏ運算子::小於等於 => 1,
        Ｏ運算子::大於 => 1,
        Ｏ運算子::大於等於 => 1,
    }
}

pub struct 調車場 {
    算元棧: VecDeque<Ｏ算式>,
    算子棧: VecDeque<Ｏ運算子>,
}

impl 調車場 {
    pub fn new(首個算元: Ｏ算式) -> Self {
        Self {
            算元棧: vec![首個算元].into(),
            算子棧: vec![].into(),
        }
    }
    fn 結合棧中算子(&mut self) {
        let 右算元 = self.算元棧.pop_back().unwrap();
        let 左算元 = self.算元棧.pop_back().unwrap();
        let 運算子 = self.算子棧.pop_back().unwrap();
        self.算元棧.push_back(Ｏ算式::二元運算(Ｏ二元運算 {
            運算子,
            左: Box::new(左算元),
            右: Box::new(右算元),
        }));
    }
    pub fn 讀取(&mut self, 新算子: Ｏ運算子, 新算元: Ｏ算式) {
        // 讀取到新算子，進行棧操作
        while !self.算子棧.is_empty() && 優先級(self.算子棧.back().unwrap()) >= 優先級(&新算子)
        {
            // 新算子優先級較低，代表棧中的算子算元可以先結合了。
            self.結合棧中算子();
        }
        // 棧中能結合的算子跟算元都結合了，推入新算子跟算元
        self.算子棧.push_back(新算子);
        self.算元棧.push_back(新算元);
    }

    pub fn 結束(&mut self) -> Ｏ算式 {
        while !self.算子棧.is_empty() {
            self.結合棧中算子();
        }
        assert_eq!(self.算子棧.len(), 0, "調車場算法結束時，算子棧不為空");
        assert_eq!(
            self.算元棧.len(),
            1,
            "調車場算法結束時，算元棧不恰有 1 元素"
        );

        self.算元棧.pop_back().unwrap()
    }
}
