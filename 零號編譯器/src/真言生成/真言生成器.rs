use crate::分詞器::Ｏ運算子;
use crate::剖析::抽象語法樹節點::*;
use std::{
    fs::File,
    io::{self, Write},
};

pub struct Ｏ真言生成器 {
    真言檔: File,
    語法樹: Ｏ語法樹,
}

impl Ｏ真言生成器 {
    pub fn new(真言檔: File, 語法樹: Ｏ語法樹) -> Self {
        Ｏ真言生成器 {
            真言檔, 語法樹
        }
    }

    pub fn 生成(&mut self) -> io::Result<()> {
        self.生成數據段()?;
        self.生成代碼段()
    }

    // 好用函式
    fn 換行(&mut self) -> io::Result<()> {
        writeln!(self.真言檔, "")
    }

    // 數據段
    fn 生成數據段(&mut self) -> io::Result<()> {
        writeln!(self.真言檔, ".section .data")?;
        self.生成全域變數標籤()
    }
    fn 生成全域變數標籤(&mut self) -> io::Result<()> {
        for 頂層宣告 in &self.語法樹.頂層宣告 {
            match 頂層宣告 {
                Ｏ頂層宣告::變數宣告(變數宣告) => {
                    writeln!(self.真言檔, "{}:", 變數宣告.變數名)?;
                    match 變數宣告.算式 {
                        Ｏ算式::數字(數) => {
                            writeln!(self.真言檔, "\t.quad {}", 數)?;
                        }
                        _ => {
                            panic!("頂層變數宣告僅支援常數")
                        }
                    }
                    writeln!(self.真言檔, "\t.quad 0")?;
                }
                _ => {}
            }
        }
        self.換行()
    }

    // 代碼段
    fn 生成代碼段(&mut self) -> io::Result<()> {
        writeln!(self.真言檔, ".section .text")?;
        // 編譯器會將某些 .data 段的變數存放到 .sdata 段
        // .sdata 段的數據可以直接用 gp 暫存器的相對位址得到
        // 會快一個指令，但 gp 初始化需要導引
        // 因此此處採用 main 而非 _start
        // gcc 編譯時不加 -nostdlib 參數，讓 gcc 生成 _start 協助引導
        writeln!(self.真言檔, ".global main")?;
        self.換行()?;
        writeln!(self.真言檔, "main:")?;
        let 語法樹 = &self.語法樹;

        for 頂層宣告 in &語法樹.頂層宣告 {
            match 頂層宣告 {
                // Ｏ句::術宣告(_) => unimplemented!(),
                // Ｏ句::變數宣告(變數宣告) => Self::賦值(&mut self.真言檔, &變數宣告)?,
                // Ｏ句::算式(算式) => Self::計算(&mut self.真言檔, &算式)?,
                _ => unimplemented!(),
            }
        }
        writeln!(self.真言檔, "# 結束")?;
        writeln!(self.真言檔, "\tli a7, 93")?; // RISCV Linux 中 exit 系統呼叫編號是 93
        writeln!(self.真言檔, "\tmv a0, t0")?; // a0 = t0
        writeln!(self.真言檔, "\tecall")?; // 執行系統呼叫 exit(t0)
        Ok(())
    }

    fn 賦值(真言檔: &mut File, 變數宣告: &Ｏ變數宣告) -> io::Result<()> {
        Self::計算(真言檔, &變數宣告.算式)?;
        writeln!(真言檔, "# 賦值給 {}", &變數宣告.變數名)?;
        writeln!(真言檔, "\tsd t0, {}, s1", &變數宣告.變數名) // 存入變數所在記憶體
    }

    // 計算結束後，結果置於 t0
    fn 計算(真言檔: &mut File, 算式: &Ｏ算式) -> io::Result<()> {
        match 算式 {
            Ｏ算式::二元運算(二元運算) => {
                Self::計算(真言檔, 二元運算.左.as_ref())?;
                Self::計算(真言檔, 二元運算.右.as_ref())?;
                Self::二元運算(真言檔, &二元運算.運算子)
            }
            Ｏ算式::數字(數) => Self::數字入棧(真言檔, 數),
            Ｏ算式::變數(變數) => Self::變數入棧(真言檔, 變數),
            Ｏ算式::施術(施術) => unimplemented!(),
        }
    }
    // 結束時，t0 = 數
    fn 數字入棧(真言檔: &mut File, 數: &i64) -> io::Result<()> {
        writeln!(真言檔, "# {} 入棧", 數)?;

        writeln!(真言檔, "\taddi sp, sp, -8")?; // 增加棧 64 位元的空間
        writeln!(真言檔, "\tli t0, {}", 數)?; // 將 t0 設為「數」
        writeln!(真言檔, "\tsd t0, 0(sp)") // t0 放入棧頂
    }
    // 結束時，t0 = 變數
    fn 變數入棧(真言檔: &mut File, 變數: &String) -> io::Result<()> {
        writeln!(真言檔, "# 變數「{}」入棧", 變數)?;

        writeln!(真言檔, "\taddi sp, sp, -8")?; // 增加棧 64 位元的空間
        writeln!(真言檔, "\tld t0, {}", 變數)?; // t0 = *(i64*)變數
        writeln!(真言檔, "\tsd t0, 0(sp)") // t0 放入棧頂
    }
    // 結束時，t0 = 二元運算結果
    fn 二元運算(真言檔: &mut File, 運算子: &Ｏ運算子) -> io::Result<()> {
        writeln!(真言檔, "# {:?}", 運算子)?;

        writeln!(真言檔, "\tld t1, 0(sp)")?; // t1 = 棧頂
        writeln!(真言檔, "\taddi sp, sp, 8")?; // 縮小棧
        writeln!(真言檔, "\tld t0, 0(sp)")?; // t0 = 棧頂

        match 運算子 {
            Ｏ運算子::加 => {
                writeln!(真言檔, "\tadd t0, t0, t1")?;
            }
            Ｏ運算子::減 => {
                writeln!(真言檔, "\tsub t0, t0, t1")?;
            }
            Ｏ運算子::乘 => {
                writeln!(真言檔, "\tmul t0, t0, t1")?;
            }
            Ｏ運算子::除 => {
                writeln!(真言檔, "\tdiv t0, t0, t1")?;
            }
            _ => {
                unimplemented!()
            }
        }

        writeln!(真言檔, "\tsd t0, 0(sp)") // t0 放入棧頂
    }
}
