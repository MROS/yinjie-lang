# 音界咒

關鍵字全為全形文字的法咒（編程語言），拒絕使用空白。

[離塵指引．卷之一．試結丹](https://github.com/MROS/compiler_tutorial)的姐妹專案，該書為一初階編譯器教材，也可視為本法咒的設計文件及部分源碼解析。

## 範例

### 曰一二三

```音界
術．初（）【
    曰（１）
    曰（２）
    曰（３）
】
```

### 費氏數列
```
術．費氏數（項）【
    若（項＜＝１）【
        歸．１
    】
    歸．費氏數（項－２）＋費氏數（項－１）
】

術．打印數列（項）【
    若（項＞１）【
        打印數列（項－１）
    】
    曰（費氏數（項））
】

術．初（）【
    打印數列（１０）
    歸．０
】
```

## 編譯

需先安裝

- [rust 工具鏈](https://rust-lang.org/zh-TW/tools/install)
- [just](https://github.com/casey/just)
- riscv64 交叉編譯工具鏈 `riscv64-linux-gnu-gcc` ，用於組譯及鏈結外術。
    - `sudo apt install riscv64-linux-gnu-gcc`
- `qemu-riscv64` 虛擬用戶態執行環境。
    - `sudo apt install qemu-user`

```
git clone https://github.com/MROS/yinjie-lang 音界咒
cd 音界咒/零號編譯器
just pre-build               # 編譯外術
just build 範例/曰一二三.音界 # 範例資料夾有更多例子
```

可在 `範例/曰一二三.音界.S` 找到編譯後的真言文件。

## 執行
在 `零號編譯器` 資料夾下執行
```
just run 範例/曰一二三.音界 # 範例資料夾有更多例子
```
可編譯並以 qemu-riscv64 執行生成的 `a.out` 檔案

## 編譯目標
僅支援 qemu-riscv64 ，於 linux 環境下執行。

