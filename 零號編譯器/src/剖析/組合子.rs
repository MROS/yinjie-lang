use crate::分詞器::Ｏ詞;

pub trait 組合子<'a, T, U> {
    // 接受一個返回 (剖析結果, &[Ｏ詞]) 的函式
    // 把剖析結果包進 tuple 裡傳遞給下個組合子
    fn 且<F>(self, f: F) -> Option<((T, U), &'a [Ｏ詞])>
    where
        F: FnOnce(&'a [Ｏ詞]) -> Option<(U, &'a [Ｏ詞])>;

    // 接受一個只返回 (剖析結果, &[Ｏ詞]) 的函式
    // 但會把剖析結果直接拋棄
    // 以免浪費空間、增加 tuple 層級
    fn 且棄<F>(self, f: F) -> Option<(T, &'a [Ｏ詞])>
    where
        F: FnOnce(&'a [Ｏ詞]) -> Option<(U, &'a [Ｏ詞])>;

    // 重複 0 到多次
    fn 重複<F>(self, f: F) -> Option<((T, Vec<U>), &'a [Ｏ詞])>
    where
        F: Fn(&'a [Ｏ詞]) -> Option<(U, &'a [Ｏ詞])>;

    fn 映射<F>(self, f: F) -> Option<(U, &'a [Ｏ詞])>
    where
        F: FnOnce(T) -> U;
}

impl<'a, T, U> 組合子<'a, T, U> for Option<(T, &'a [Ｏ詞])> {
    fn 且<F>(self, f: F) -> Option<((T, U), &'a [Ｏ詞])>
    where
        F: FnOnce(&'a [Ｏ詞]) -> Option<(U, &'a [Ｏ詞])>,
    {
        self.and_then(|(剖析結果, 游標)| {
            // 將之前的剖析結果與新剖析結果以 tuple 包裝
            f(游標).map(|(新剖析結果, 游標)| ((剖析結果, 新剖析結果), 游標))
        })
    }

    fn 且棄<F>(self, f: F) -> Option<(T, &'a [Ｏ詞])>
    where
        F: FnOnce(&'a [Ｏ詞]) -> Option<(U, &'a [Ｏ詞])>,
    {
        self.and_then(|(剖析結果, 游標)| f(游標).map(|(_, 游標)| (剖析結果, 游標)))
    }

    fn 重複<F>(self, f: F) -> Option<((T, Vec<U>), &'a [Ｏ詞])>
    where
        F: Fn(&'a [Ｏ詞]) -> Option<(U, &'a [Ｏ詞])>,
    {
        match self {
            Some((t, 詞組)) => {
                let mut 當前詞組 = 詞組;
                let mut 剖析結果陣列 = Vec::new();
                while let Some((剖析結果, 詞組)) = f(當前詞組) {
                    當前詞組 = 詞組;
                    剖析結果陣列.push(剖析結果);
                }
                Some(((t, 剖析結果陣列), 當前詞組))
            }
            None => None,
        }
    }

    fn 映射<F>(self, f: F) -> Option<(U, &'a [Ｏ詞])>
    where
        F: FnOnce(T) -> U,
    {
        self.map(|(剖析結果, 游標)| (f(剖析結果), 游標))
    }
}

pub trait 自動映射組合子<'a, T, U>
where
    T: Into<U>,
{
    fn 自動映射(self) -> Option<(U, &'a [Ｏ詞])>;
}

impl<'a, T, U> 自動映射組合子<'a, T, U> for Option<(T, &'a [Ｏ詞])>
where
    T: Into<U>,
{
    fn 自動映射(self) -> Option<(U, &'a [Ｏ詞])> {
        self.map(|(剖析結果, 游標)| (剖析結果.into(), 游標))
    }
}
