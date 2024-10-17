use super::區域變數計數::術內區域變數數量;
use crate::分詞器::Ｏ運算子;
use crate::剖析::抽象語法樹節點::*;
use derive_more::derive::Display;
use rpds::HashTrieMap;
use std::{
    fs::File,
    io::{self, Write},
};

// 編譯目標為 64 位元系統
const 字長: usize = 8;

#[derive(Clone, Copy, Display)]
enum Ｏ棧中類型 {
    區域變數,
    實參,
}
use Ｏ棧中類型::*;

#[derive(Clone, Copy)]
enum Ｏ變數位址 {
    全域,

    棧中(usize, Ｏ棧中類型),
}

impl Ｏ變數位址 {
    // 從記憶體載入暫存器中
    fn 載入(&self, 真言檔: &mut File, 暫存器名: &str, 變數名: &str) -> io::Result<()> {
        match self {
            Ｏ變數位址::全域 => {
                writeln!(真言檔, "# 載入全域變數「{}」", 變數名)?;
                writeln!(真言檔, "\tld {}, {}", 暫存器名, 變數名)
            }
            Ｏ變數位址::棧中(偏移, 棧中類型) => {
                writeln!(真言檔, "# 載入{}「{}」", 棧中類型, 變數名)?;
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
            Ｏ變數位址::棧中(偏移, 棧中類型) => {
                writeln!(真言檔, "# 寫出{}「{}」", 棧中類型, 變數名)?;
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
    fn 錄入全域變數(&mut self, 變數名: &String) {
        self.變數表.insert_mut(變數名.clone(), Ｏ變數位址::全域);
    }
    fn 錄入棧中變數(&mut self, 變數名: &String, 棧中類型: Ｏ棧中類型) {
        self.變數表.insert_mut(
            變數名.clone(),
            Ｏ變數位址::棧中(字長 * (self.計數 + 2), 棧中類型),
        );
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

    分支標籤計數: u64,
    區塊群結尾標籤計數: u64,
}

impl Ｏ真言生成器 {
    pub fn new(真言檔: File) -> Self {
        Ｏ真言生成器 {
            真言檔,
            分支標籤計數: 0,
            區塊群結尾標籤計數: 0,
        }
    }

    pub fn 生成(&mut self, 語法樹: Ｏ語法樹) -> io::Result<()> {
        let 符號表 = self.生成數據段(&語法樹)?;
        self.生成代碼段(&語法樹, 符號表)
    }

    /// 好用函式
    fn 換行(&mut self) -> io::Result<()> {
        writeln!(self.真言檔, "")
    }
    fn 彈出(&mut self) -> io::Result<()> {
        writeln!(self.真言檔, "\taddi sp, sp, 8")
    }

    /// 數據段
    fn 生成數據段(&mut self, 語法樹: &Ｏ語法樹) -> io::Result<Ｏ符號表> {
        writeln!(self.真言檔, ".section .data")?;
        self.生成全域變數標籤(語法樹)
    }
    // 順便回傳全域變數表給代碼段使用
    fn 生成全域變數標籤(&mut self, 語法樹: &Ｏ語法樹) -> io::Result<Ｏ符號表> {
        let mut 符號表 = Ｏ符號表::new();

        for 頂層宣告 in &語法樹.頂層宣告 {
            match 頂層宣告 {
                Ｏ頂層宣告::變數宣告(變數宣告) => {
                    // 將全域變數錄入符號表
                    符號表.錄入全域變數(&變數宣告.變數名);

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
    fn 生成代碼段(
        &mut self, 語法樹: &Ｏ語法樹, 符號表: Ｏ符號表
    ) -> io::Result<()> {
        writeln!(self.真言檔, ".section .text")?;
        self.生成main()?;

        for 頂層宣告 in &語法樹.頂層宣告 {
            match 頂層宣告 {
                Ｏ頂層宣告::術宣告(術) => {
                    self.生成術(術, 符號表.clone())?;
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

    fn 術開頭(&mut self, 術: &Ｏ術宣告) -> io::Result<()> {
        writeln!(self.真言檔, "\n")?;
        writeln!(self.真言檔, "{}:", 術.術名)?;

        let 區域變數數量 = 術內區域變數數量(&術);

        let 棧初始大小 = (術.形參.len() + 區域變數數量 + 2) * 字長;
        writeln!(self.真言檔, "\t# 區域變數數量 = {}", 區域變數數量)?;
        writeln!(self.真言檔, "\t# 參數數量 = {}", 術.形參.len())?;
        writeln!(
            self.真言檔,
            "\t# 棧大小 = (參數數量 + 區域變數數量 + 2) * 字長({}) = {}",
            字長, 棧初始大小
        )?;
        // 增長棧
        writeln!(self.真言檔, "\taddi sp, sp, -{}", 棧初始大小)?;
        // 儲存返回地址
        writeln!(self.真言檔, "\tsd ra, {}(sp)", 棧初始大小 - 字長)?;
        // 儲存舊棧底（fp）
        writeln!(self.真言檔, "\tsd s0, {}(sp)", 棧初始大小 - 字長 * 2)?;
        // 更新 s0 為現在的棧底（s0 就是 fp）
        writeln!(self.真言檔, "\taddi s0, sp, {}", 棧初始大小)
    }

    fn 生成句(&mut self, 句: &Ｏ句, 符號表: &mut Ｏ符號表) -> io::Result<()> {
        let mut 符號表 = 符號表;
        match 句 {
            Ｏ句::變數宣告(變數宣告) => self.賦值(變數宣告, &mut 符號表),
            // 算式可能含有副作用，如「曰」會打印數字
            // 不可為了優化直接省略掉
            Ｏ句::算式(算式) => {
                self.計算(算式, &符號表)?;
                self.彈出()
            }
            Ｏ句::歸(歸) => self.歸(&歸.0, &符號表),
            Ｏ句::若(若) => self.生成若(&若, &符號表),
        }
    }

    fn 生成術(&mut self, 術: &Ｏ術宣告, 符號表: Ｏ符號表) -> io::Result<()> {
        self.術開頭(術)?;

        let mut 符號表 = 符號表;
        // 將術的參數加入符號表
        // 並將參數從 a0~a7 寫入棧中
        for (編號, 參名) in 術.形參.iter().enumerate() {
            符號表.錄入棧中變數(參名, 實參);
            符號表
                .取得變數位址(參名)
                .寫出(&mut self.真言檔, &format!("a{}", 編號), 參名)?;
        }

        for 句 in &術.術體 {
            self.生成句(句, &mut 符號表)?;
        }

        writeln!(self.真言檔, "# 結束")?;

        // TODO: 若最後一句本就是歸語句，就不必再收尾一次了
        self.術收尾()
    }

    fn 賦值(
        &mut self, 變數宣告: &Ｏ變數宣告, 符號表: &mut Ｏ符號表
    ) -> io::Result<()> {
        符號表.錄入棧中變數(&變數宣告.變數名, 區域變數);

        self.計算(&變數宣告.算式, 符號表)?;
        self.彈出()?;

        符號表
            .取得變數位址(&變數宣告.變數名)
            .寫出(&mut self.真言檔, "t0", &變數宣告.變數名)
    }

    // 計算結束時，棧頂 = t0 = 計算結果
    fn 計算(&mut self, 算式: &Ｏ算式, 符號表: &Ｏ符號表) -> io::Result<()> {
        match 算式 {
            Ｏ算式::二元運算(二元運算) => {
                self.計算(二元運算.左.as_ref(), 符號表)?;
                self.計算(二元運算.右.as_ref(), 符號表)?;
                self.二元運算(&二元運算.運算子)
            }
            Ｏ算式::數字(數) => self.數字入棧(數),
            Ｏ算式::變數(變數) => self.變數入棧(變數, 符號表),
            Ｏ算式::施術(施術) => self.施術(施術, 符號表),
        }
    }
    // 結束時，棧頂 = t0 = 數
    fn 數字入棧(&mut self, 數: &i64) -> io::Result<()> {
        writeln!(self.真言檔, "# {} 入棧", 數)?;

        writeln!(self.真言檔, "\taddi sp, sp, -8")?; // 增加棧 64 位元的空間
        writeln!(self.真言檔, "\tli t0, {}", 數)?; // 將 t0 設為「數」
        writeln!(self.真言檔, "\tsd t0, 0(sp)") // t0 放入棧頂
    }
    // 結束時，棧頂 = t0 = 變數
    fn 變數入棧(&mut self, 變數名: &String, 符號表: &Ｏ符號表) -> io::Result<()> {
        writeln!(self.真言檔, "# 變數「{}」入棧", 變數名)?;

        writeln!(self.真言檔, "\taddi sp, sp, -8")?; // 增加棧 64 位元的空間
        符號表
            .取得變數位址(變數名)
            .載入(&mut self.真言檔, "t0", 變數名)?;
        writeln!(self.真言檔, "\tsd t0, 0(sp)") // t0 放入棧頂
    }
    // 結束時，棧頂 = t0 = 二元運算結果
    fn 二元運算(&mut self, 運算子: &Ｏ運算子) -> io::Result<()> {
        writeln!(self.真言檔, "# {:?}", 運算子)?;

        writeln!(self.真言檔, "\tld t1, 0(sp)")?; // t1 = 棧頂
        writeln!(self.真言檔, "\taddi sp, sp, 8")?; // 縮小棧
        writeln!(self.真言檔, "\tld t0, 0(sp)")?; // t0 = 棧頂

        match 運算子 {
            Ｏ運算子::加 => {
                writeln!(self.真言檔, "\tadd t0, t0, t1")?;
            }
            Ｏ運算子::減 => {
                writeln!(self.真言檔, "\tsub t0, t0, t1")?;
            }
            Ｏ運算子::乘 => {
                writeln!(self.真言檔, "\tmul t0, t0, t1")?;
            }
            Ｏ運算子::除 => {
                writeln!(self.真言檔, "\tdiv t0, t0, t1")?;
            }
            Ｏ運算子::餘 => {
                // rem 指令是有號整數取餘
                // 尚有 urem 指令，乃無號整數取餘，但音界咒並不支援
                writeln!(self.真言檔, "\trem t0, t0, t1")?;
            }
            Ｏ運算子::等於 => {
                writeln!(self.真言檔, "\txor t2, t0, t1")?; // t2 = t0 ^ t1
                writeln!(self.真言檔, "\tseqz t0, t2")?; // t0 = (t2 == 0) ? 1 : 0
            }
            Ｏ運算子::異於 => {
                writeln!(self.真言檔, "\txor t2, t0, t1")?; // t2 = t0 ^ t1
                writeln!(self.真言檔, "\tsnez t0, t2")?; // t0 = (t2 != 0) ? 1 : 0
            }
            // 以下比較運算僅 slt 為精五真言
            Ｏ運算子::小於 => {
                writeln!(self.真言檔, "\tslt t0, t0, t1")?; // t0 = (t0 < t1)
            }
            Ｏ運算子::大於 => {
                // 組譯為 slt t0, t1, t0
                writeln!(self.真言檔, "\tsgt t0, t0, t1")?; // t0 = (t0 > t1)
            }
            Ｏ運算子::小於等於 => {
                // 甲＜＝乙，即 ！（甲＞乙）
                writeln!(self.真言檔, "\tsgt t0, t0, t1")?; // t0 = (t0 > t0)
                writeln!(self.真言檔, "\txori t0, t0, 1")?; // t0 = t0 ^ 1
            }
            Ｏ運算子::大於等於 => {
                // 甲＞＝乙，即 ！（甲＜乙）
                writeln!(self.真言檔, "\tslt t0, t0, t1")?; // t0 = (t0 < t1)
                writeln!(self.真言檔, "\txori t0, t0, 1")?; // t0 = t0 ^ 1
            }
        }

        writeln!(self.真言檔, "\tsd t0, 0(sp)") // t0 放入棧頂
    }
    // 結束時，棧頂 = t0 = 術的歸值
    fn 施術(&mut self, 術: &Ｏ施術, 符號表: &Ｏ符號表) -> io::Result<()> {
        writeln!(self.真言檔, "# 施展「{}」", 術.術名)?;

        assert!(術.實參.len() <= 8, "音界咒暫不支援超過八個術參");

        // 將參數計算結果逐一推入棧中
        // NOTE: 不可將計算結果直接賦值給暫存器
        // 因其下個參數的計算過程中，可能又會汙染掉參數暫存器
        for 參數 in &術.實參 {
            self.計算(參數, 符號表)?;
        }
        // 將參數從棧中逐一載入參數暫存器
        for 編號 in (0..術.實參.len()).rev() {
            writeln!(self.真言檔, "\tld a{}, 0(sp)", 編號)?;
            self.彈出()?;
        }

        writeln!(self.真言檔, "\tcall {}", 術.術名)?;

        // 歸值放回 t0
        writeln!(self.真言檔, "\tmv t0, a0")?;
        writeln!(self.真言檔, "\taddi sp, sp, -8")?; // 增加棧 64 位元的空間
        writeln!(self.真言檔, "\tsd t0, 0(sp)") // t0 放入棧頂
    }
    fn 歸(&mut self, 算式: &Ｏ算式, 符號表: &Ｏ符號表) -> io::Result<()> {
        writeln!(self.真言檔, "# 歸")?;
        self.計算(算式, 符號表)?;
        self.彈出()?;
        writeln!(self.真言檔, "\tmv a0, t0")?; // 計算結果放進 a0 ，結束術
        self.術收尾()
    }
    fn 分支標籤名(&self) -> String {
        format!("分支標籤——{}", self.分支標籤計數)
    }
    fn 寫入分支標籤(&mut self) -> io::Result<()> {
        writeln!(self.真言檔, "{}:", self.分支標籤名())?;
        self.分支標籤計數 += 1;
        Ok(())
    }
    fn t0為0則跳至下個分支標籤(&mut self) -> io::Result<()> {
        writeln!(self.真言檔, "\tbeq t0, x0, {}", self.分支標籤名())
    }
    fn 區塊群結尾標籤名(&self) -> String {
        format!("區塊群結尾標籤——{}", self.區塊群結尾標籤計數)
    }
    fn 寫入區塊群結尾標籤(&mut self) -> io::Result<()> {
        writeln!(self.真言檔, "{}:", self.區塊群結尾標籤名())?;
        self.區塊群結尾標籤計數 += 1;
        Ok(())
    }
    fn 跳至區塊群結尾(&mut self) -> io::Result<()> {
        writeln!(self.真言檔, "j {}", self.區塊群結尾標籤名())
    }
    fn 生成若(&mut self, 若: &Ｏ若, 符號表: &Ｏ符號表) -> io::Result<()> {
        // 若
        self.計算(&若.條件, 符號表)?;
        self.彈出()?;
        self.t0為0則跳至下個分支標籤()?;
        // 區塊內的新增區域變數，在區塊外不應被擷取
        // 故需複製符號表，以免原符號表被影響
        self.生成區塊(&若.區塊, 符號表.clone())?;
        writeln!(self.真言檔, "\t")?;
        if 若.或若列表.len() > 0 || 若.不然.is_some() {
            self.跳至區塊群結尾()?;
        }

        // 或若
        for 或若 in &若.或若列表 {
            self.寫入分支標籤()?;
            self.計算(&或若.條件, 符號表)?;
            self.彈出()?;
            self.t0為0則跳至下個分支標籤()?;
            self.生成區塊(&或若.區塊, 符號表.clone())?;
            self.跳至區塊群結尾()?;
        }

        // 不然
        self.寫入分支標籤()?;
        if let Some(不然) = &若.不然 {
            self.生成區塊(&不然.區塊, 符號表.clone())?;
        }

        self.寫入區塊群結尾標籤()
    }
    fn 生成區塊(
        &mut self, 區塊: &Vec<Ｏ句>, mut 符號表: Ｏ符號表
    ) -> io::Result<()> {
        for 句 in 區塊 {
            self.生成句(句, &mut 符號表)?;
        }
        Ok(())
    }
    fn 術收尾(&mut self) -> io::Result<()> {
        writeln!(self.真言檔, "# 收尾")?;
        writeln!(self.真言檔, "\tld ra, -8(s0)")?; // 計算結果放進 a0 ，結束術
        writeln!(self.真言檔, "\tmv sp, s0")?; // 歸還棧空間
        writeln!(self.真言檔, "\tld s0, -16(s0)")?; // 恢復 fp
        writeln!(self.真言檔, "\tret")
    }
}
