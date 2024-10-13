use crate::分詞器::Ｏ詞;

pub trait 組合子<'a, T, U> {
    fn 且<F>(self, f: F) -> Option<((T, U), &'a [Ｏ詞])>
    where
        F: FnOnce(&'a [Ｏ詞]) -> Option<(U, &'a [Ｏ詞])>;

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
