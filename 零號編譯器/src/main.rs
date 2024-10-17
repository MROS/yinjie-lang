#![allow(uncommon_codepoints)]

#[path = "全形處理/mod.rs"] // non-ASCII 的檔名必須顯式寫出路徑
mod 全形處理;
#[path = "分詞器.rs"]
mod 分詞器;
#[path = "剖析/mod.rs"]
mod 剖析;
#[path = "外術列表.rs"]
mod 外術列表;
#[path = "真言生成/mod.rs"]
mod 真言生成;
#[path = "符號檢查.rs"]
mod 符號檢查;
#[path = "通用優化/mod.rs"]
mod 通用優化;

use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // 從命令行參數獲取檔案名稱
    let 參數: Vec<String> = env::args().collect();
    if 參數.len() < 2 {
        eprintln!("用法：音界 源碼.音界 [-O2]");
        std::process::exit(1);
    }

    let 檔名 = &參數[1];
    let 啟用優化 = 參數.len() == 3 && 參數[2] == "-O2";

    // 打開檔案
    let mut 檔案 = File::open(檔名)?;

    // 讀取檔案內容
    let mut 音界咒源碼 = String::new();
    檔案.read_to_string(&mut 音界咒源碼)?;

    // 分詞
    let 詞列 = 分詞器::Ｏ分詞器::new(音界咒源碼).分詞();
    let mut i = 0;
    for 詞 in &詞列 {
        println!("{} {:?}", i, 詞);
        i += 1;
    }

    // 剖析語法
    let mut 語法樹 = match 剖析::Ｏ剖析器::new(詞列).剖析() {
        None => {
            println!("剖析失敗");
            return Ok(());
        }
        Some(語法樹) => {
            println!("{:#?}", 語法樹);
            語法樹
        }
    };

    // 檢查符號
    match 符號檢查::檢查語法樹(&語法樹) {
        true => {
            println!("符號檢查通過");
        }
        false => {
            println!("符號檢查失敗");
            return Ok(());
        }
    };

    // 優化
    if 啟用優化 {
        println!("啟用優化");
        語法樹 = 通用優化::優化(語法樹);
    }

    let 真言檔名 = format!("{}.S", 檔名);
    let 真言檔 = File::create(真言檔名)?;
    let mut 生成器 = 真言生成::Ｏ真言生成器::new(真言檔);
    生成器.生成(語法樹)?;

    Ok(())
}
