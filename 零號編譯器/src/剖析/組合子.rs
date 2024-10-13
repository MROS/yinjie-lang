pub trait 組合子<T, U> {
    fn 且<F>(self, f: F) -> Option<((T, U), usize)>
    where
        F: FnOnce(usize) -> Option<(U, usize)>;
}

impl<T, U> 組合子<T, U> for Option<(T, usize)> {
    fn 且<F>(self, f: F) -> Option<((T, U), usize)>
    where
        F: FnOnce(usize) -> Option<(U, usize)>,
    {
        self.and_then(|(剖析結果, 游標)| {
            // 將之前的剖析結果與新剖析結果以 tuple 包裝
            f(游標).map(|(新剖析結果, 游標)| ((剖析結果, 新剖析結果), 游標))
        })
    }
}
