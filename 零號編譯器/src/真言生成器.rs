use crate::剖析器::Ｏ語法樹;
use std::{
    collections::HashSet,
    fs::File,
    io::{self, Write},
};

pub struct Ｏ真言生成器 {
    真言檔: File,
    語法樹: Ｏ語法樹,
    變數集: HashSet<String>,
}

impl Ｏ真言生成器 {
    pub fn new(真言檔: File, 語法樹: Ｏ語法樹, 變數集: HashSet<String>) -> Self {
        Ｏ真言生成器 {
            真言檔,
            語法樹,
            變數集,
        }
    }

    pub fn 生成(&mut self) -> io::Result<()> {
        self.生成數據段()?;
        self.生成代碼段()
    }

    fn 換行(&mut self) -> io::Result<()> {
        writeln!(self.真言檔, "")
    }

    fn 生成數據段(&mut self) -> io::Result<()> {
        writeln!(self.真言檔, ".section .data")?;
        self.生成變數標籤()
    }
    fn 生成變數標籤(&mut self) -> io::Result<()> {
        for 變數 in &self.變數集 {
            writeln!(self.真言檔, "{}:", 變數)?;
            // 初始值為 0
            // 其實初始值是多少不重要，通過符號檢查，代表每個變數使用前都會先賦值
            writeln!(self.真言檔, "\t.word 0:")?;
        }
        self.換行()
    }
    fn 生成代碼段(&mut self) -> io::Result<()> {
        writeln!(self.真言檔, ".section .text")?;
        writeln!(self.真言檔, ".global _start")?;
        self.換行()?;
        Ok(())
    }
}
