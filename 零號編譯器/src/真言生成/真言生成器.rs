use super::變數計數::術內區域變數數量;
use crate::分詞器::Ｏ運算子;
use crate::剖析::抽象語法樹節點::*;
use rpds::HashTrieMap;
use std::{
    fs::File,
    io::{self, Write},
};

// 編譯目標為 64 位元系統
const 字長: usize = 8;

#[derive(Clone, Copy)]
enum Ｏ變數位址 {
    全域,

    // usize 為其偏移量
    區域(usize),

    // 實參與區域變數一樣都存在棧上，但先放實參再放區域變數
    // usize 為其在棧上偏移量
    實參(usize),
}

impl Ｏ變數位址 {
    // 從記憶體載入暫存器中
    fn 載入(&self, 真言檔: &mut File, 暫存器名: &str, 變數名: &str) -> io::Result<()> {
        match self {
            Ｏ變數位址::全域 => {
                writeln!(真言檔, "# 載入全域變數「{}」", 變數名)?;
                writeln!(真言檔, "\tld {}, {}", 暫存器名, 變數名)
            }
            Ｏ變數位址::區域(偏移) => {
                writeln!(真言檔, "# 載入區域變數「{}」", 變數名)?;
                writeln!(真言檔, "\tld {}, -{}(s0)", 暫存器名, 偏移)
            }
            Ｏ變數位址::實參(偏移) => {
                writeln!(真言檔, "# 載入實參「{}」", 變數名)?;
                writeln!(真言檔, "\tld {}, -{}(s0)", 暫存器名, 偏移)
            }
        }
    }

    // 從暫存器寫到記憶體
    fn 寫出(&self, 真言檔: &mut File, 暫存器名: &str, 變數名: &str) -> io::Result<()> {
        match self {
            Ｏ變數位址::全域 => {
                panic!("目前語法不會使全域變數的值被更改");
            }
            Ｏ變數位址::區域(偏移) => {
                writeln!(真言檔, "# 寫出區域變數「{}」", 變數名)?;
                writeln!(真言檔, "\tsd {}, -{}(s0)", 暫存器名, 偏移)
            }
            Ｏ變數位址::實參(偏移) => {
                // 僅用於在術啟動時，將參數從 a0 ~ a7 寫入棧中
                writeln!(真言檔, "# 寫出參數「{}」", 變數名)?;
                writeln!(真言檔, "\tsd {}, -{}(s0)", 暫存器名, 偏移)
            }
        }
    }
}

// 目前不支援術中術
// 符號檢查有通過的話，術一定都存在的，不需要記錄
#[derive(Clone)]
struct Ｏ符號表 {
    變數表: rpds::HashTrieMap<String, Ｏ變數位址>,
    計數: usize, // 當下術內有幾個實參跟區域變數
}

impl Ｏ符號表 {
    fn new() -> Self {
        Self {
            變數表: HashTrieMap::new(),
            計數: 1,
        }
    }
    fn 加入實參(&mut self, 參名: &String) {
        self.變數表
            .insert_mut(參名.clone(), Ｏ變數位址::實參(字長 * (self.計數 + 2)));
        self.計數 += 1;
    }
    fn 加入區域變數(&mut self, 變數名: &String) {
        self.變數表
            .insert_mut(變數名.clone(), Ｏ變數位址::區域(字長 * (self.計數 + 2)));
        self.計數 += 1;
    }
    fn 取得變數位址(&self, 變數名: &String) -> Ｏ變數位址 {
        match self.變數表.get(變數名) {
            Some(變數位址) => *變數位址,
            None => {
                panic!(
                    "編譯器內部錯誤：未在符號檢查階段檢查到未宣告變數「{}」",
                    變數名
                )
            }
        }
    }
}

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
        let 符號表 = self.生成數據段()?;
        self.生成代碼段(符號表)
    }

    /// 好用函式
    fn 換行(&mut self) -> io::Result<()> {
        writeln!(self.真言檔, "")
    }

    /// 數據段
    fn 生成數據段(&mut self) -> io::Result<Ｏ符號表> {
        writeln!(self.真言檔, ".section .data")?;
        self.生成全域變數標籤()
    }
    // 順便回傳全域變數表給代碼段使用
    fn 生成全域變數標籤(&mut self) -> io::Result<Ｏ符號表> {
        let mut 符號表 = Ｏ符號表::new();

        for 頂層宣告 in &self.語法樹.頂層宣告 {
            match 頂層宣告 {
                Ｏ頂層宣告::變數宣告(變數宣告) => {
                    // 將全域變數錄入符號表
                    符號表
                        .變數表
                        .insert_mut(變數宣告.變數名.clone(), Ｏ變數位址::全域);

                    // 寫入真言檔
                    writeln!(self.真言檔, "{}:", 變數宣告.變數名)?;
                    match 變數宣告.算式 {
                        Ｏ算式::數字(數) => {
                            writeln!(self.真言檔, "\t.quad {}", 數)?;
                        }
                        _ => {
                            panic!("頂層變數宣告僅支援常數")
                        }
                    }
                }
                _ => {}
            }
        }
        self.換行()?;

        Ok(符號表)
    }

    /// 代碼段
    fn 生成代碼段(&mut self, 符號表: Ｏ符號表) -> io::Result<()> {
        writeln!(self.真言檔, ".section .text")?;
        self.生成main()?;

        let 語法樹 = &self.語法樹;

        for 頂層宣告 in &語法樹.頂層宣告 {
            match 頂層宣告 {
                Ｏ頂層宣告::術宣告(術) => {
                    Self::生成術(&mut self.真言檔, 術, 符號表.clone())?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn 生成main(&mut self) -> io::Result<()> {
        // 編譯器會將某些 .data 段的變數存放到 .sdata 段
        // .sdata 段的數據可以直接用 gp 暫存器的相對位址得到
        // 會快一個指令，但 gp 初始化需要導引
        // 因此此處採用 main 而非 _start
        // gcc 編譯時不加 -nostdlib 參數，讓 gcc 生成 _start 協助引導
        writeln!(self.真言檔, ".global main")?;
        self.換行()?;
        writeln!(self.真言檔, "main:")?;
        writeln!(self.真言檔, "# 呼叫初術")?;
        writeln!(self.真言檔, "\tcall 初")?;

        writeln!(self.真言檔, "# 結束")?;
        writeln!(self.真言檔, "\tli a7, 93")?; // RISCV Linux 中 exit 系統呼叫編號是 93
        writeln!(self.真言檔, "\tecall")?; // a0 未動，exit 的返回值與「初」的返回值相同
        Ok(())
    }

    fn 生成術(
        真言檔: &mut File, 術: &Ｏ術宣告, 符號表: Ｏ符號表
    ) -> io::Result<()> {
        let mut 符號表 = 符號表;

        writeln!(真言檔, "\n")?;
        writeln!(真言檔, "{}:", 術.術名)?;

        let 區域變數數量 = 術內區域變數數量(&術);

        let 棧初始大小 = (術.形參.len() + 區域變數數量 + 2) * 字長;
        writeln!(真言檔, "\t# 區域變數數量 = {}", 區域變數數量)?;
        writeln!(真言檔, "\t# 參數數量 = {}", 術.形參.len())?;
        writeln!(
            真言檔,
            "\t# 棧大小 = (參數數量 + 區域變數數量 + 2) * 字長({}) = {}",
            字長, 棧初始大小
        )?;
        writeln!(真言檔, "\taddi sp, sp, -{}", 棧初始大小)?;
        // 儲存返回地址
        writeln!(真言檔, "\tsd ra, {}(sp)", 棧初始大小 - 字長)?;
        // 儲存舊棧底（fp）
        writeln!(真言檔, "\tsd s0, {}(sp)", 棧初始大小 - 字長 * 2)?;
        // 更新 s0 為現在的棧底，s0 就是 fp
        writeln!(真言檔, "\taddi s0, sp, {}", 棧初始大小)?;

        // 將術的參數加入符號表
        // 並將參數從 a0~a7 寫入棧中
        for (編號, 參名) in 術.形參.iter().enumerate() {
            符號表.加入實參(參名);
            符號表
                .取得變數位址(參名)
                .寫出(真言檔, &format!("a{}", 編號), 參名)?;
        }

        for 句 in &術.術體 {
            match 句 {
                Ｏ句::變數宣告(變數宣告) => {
                    Self::賦值(真言檔, 變數宣告, &mut 符號表)?;
                }
                // 算式可能含有副作用，如「曰」會打印數字
                // 不可為了優化直接省略掉
                Ｏ句::算式(算式) => {
                    Self::計算(真言檔, 算式, &符號表)?;
                    writeln!(真言檔, "\taddi sp, sp, 8")?; // 將計算結果彈出
                }
                Ｏ句::歸(歸) => {
                    Self::歸(真言檔, &歸.0, &符號表)?;
                }
                Ｏ句::若(若) => {
                    unimplemented!()
                }
            }
        }

        writeln!(真言檔, "# 結束")?;

        // TODO: 若最後一句本就是歸語句，就不必再收尾一次了
        Self::收尾(真言檔)
    }

    fn 賦值(
        真言檔: &mut File, 變數宣告: &Ｏ變數宣告, 符號表: &mut Ｏ符號表
    ) -> io::Result<()> {
        符號表.加入區域變數(&變數宣告.變數名);

        Self::計算(真言檔, &變數宣告.算式, 符號表)?;
        writeln!(真言檔, "\taddi sp, sp, 8")?; // 將計算結果彈出

        符號表
            .取得變數位址(&變數宣告.變數名)
            .寫出(真言檔, "t0", &變數宣告.變數名)
    }

    // 計算結束時，棧頂 = t0 = 計算結果
    fn 計算(
        真言檔: &mut File, 算式: &Ｏ算式, 符號表: &Ｏ符號表
    ) -> io::Result<()> {
        match 算式 {
            Ｏ算式::二元運算(二元運算) => {
                Self::計算(真言檔, 二元運算.左.as_ref(), 符號表)?;
                Self::計算(真言檔, 二元運算.右.as_ref(), 符號表)?;
                Self::二元運算(真言檔, &二元運算.運算子)
            }
            Ｏ算式::數字(數) => Self::數字入棧(真言檔, 數),
            Ｏ算式::變數(變數) => Self::變數入棧(真言檔, 變數, 符號表),
            Ｏ算式::施術(施術) => Self::施術(真言檔, 施術, 符號表),
        }
    }
    // 結束時，棧頂 = t0 = 數
    fn 數字入棧(真言檔: &mut File, 數: &i64) -> io::Result<()> {
        writeln!(真言檔, "# {} 入棧", 數)?;

        writeln!(真言檔, "\taddi sp, sp, -8")?; // 增加棧 64 位元的空間
        writeln!(真言檔, "\tli t0, {}", 數)?; // 將 t0 設為「數」
        writeln!(真言檔, "\tsd t0, 0(sp)") // t0 放入棧頂
    }
    // 結束時，棧頂 = t0 = 變數
    fn 變數入棧(
        真言檔: &mut File, 變數名: &String, 符號表: &Ｏ符號表
    ) -> io::Result<()> {
        writeln!(真言檔, "# 變數「{}」入棧", 變數名)?;

        writeln!(真言檔, "\taddi sp, sp, -8")?; // 增加棧 64 位元的空間
        符號表.取得變數位址(變數名).載入(真言檔, "t0", 變數名)?;
        writeln!(真言檔, "\tsd t0, 0(sp)") // t0 放入棧頂
    }
    // 結束時，棧頂 = t0 = 二元運算結果
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
    // 結束時，棧頂 = t0 = 術的歸值
    fn 施術(真言檔: &mut File, 術: &Ｏ施術, 符號表: &Ｏ符號表) -> io::Result<()> {
        writeln!(真言檔, "# 施展「{}」", 術.術名)?;

        assert!(術.實參.len() <= 8, "音界咒暫不支援超過八個術參");

        // 計算參數
        for (編號, 參數) in 術.實參.iter().enumerate() {
            Self::計算(真言檔, 參數, 符號表)?;
            writeln!(真言檔, "\taddi sp, sp, 8")?; // 將參數計算結果彈出
            writeln!(真言檔, "\tmv a{}, t0", 編號)?;
        }

        writeln!(真言檔, "\tcall {}", 術.術名)?;

        // 歸值放回 t0
        writeln!(真言檔, "\tmv t0, a0")?;
        writeln!(真言檔, "\taddi sp, sp, -8")?; // 增加棧 64 位元的空間
        writeln!(真言檔, "\tsd t0, 0(sp)") // t0 放入棧頂
    }
    fn 歸(真言檔: &mut File, 算式: &Ｏ算式, 符號表: &Ｏ符號表) -> io::Result<()> {
        writeln!(真言檔, "# 歸")?;
        Self::計算(真言檔, 算式, 符號表)?;
        writeln!(真言檔, "\taddi sp, sp, 8")?; // 將計算結果彈出
        writeln!(真言檔, "\tmv a0, t0")?; // 計算結果放進 a0 ，結束術
        Self::收尾(真言檔)
    }
    fn 收尾(真言檔: &mut File) -> io::Result<()> {
        writeln!(真言檔, "# 收尾")?;
        writeln!(真言檔, "\tld ra, -8(s0)")?; // 計算結果放進 a0 ，結束術
        writeln!(真言檔, "\tmv sp, s0")?; // 歸還棧空間
        writeln!(真言檔, "\tld s0, -16(s0)")?; // 恢復 fp
        writeln!(真言檔, "\tret")
    }
}
