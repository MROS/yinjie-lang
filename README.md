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

## 執行

需先安裝

-riscv64 交叉編譯工具鏈，用於組譯即鏈結外術。
-`qemu-riscv64` 虛擬執行環境。

```
git clone https://github.com/MROS/yinjie-lang 音界咒
cd 音界咒/零號編譯器
just run 範例/曰一二三.音界 # 範例資料夾有更多例子
```

可在 `範例/曰一二三.音界.S` 找到編譯後的真言文件。

## 編譯目標
僅支援 qemu-riscv64

