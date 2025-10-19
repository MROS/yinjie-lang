#!/usr/bin/env bash

function green_print() {
  green='\033[0;32m'
  reset='\033[0m'
  echo -e "${green}$1${reset}"
}

function red_print() {
  red='\033[0;31m'
  reset='\033[0m'
  echo -e "${red}$1${reset}"
}

function integration_test() {
  rm 範例/*.S
  cases=($(find "範例" -type f -name "*.音界"))
  passed_count=0
  failed_count=0

  for case in "${cases[@]}"; do
    filename=$(basename "$case")
    echo ">>> 測試範例: $filename"

    cargo run "$case" 1> /tmp/null 2> /dev/null
    riscv64-unknown-elf-gcc "${case}.S" 外術/曰.o -o /tmp/音界咒執行檔

    # qemu 可能不會直接輸出到 stdout
    # 使用 script 創建偽終端以捕捉 qemu 的輸出
    script -q -c "qemu-riscv64 /tmp/音界咒執行檔" > /tmp/答案

    expected="範例/${filename%.音界}.解"
    # NOTE: diff -w 忽略空白、換行差異，可能導致潛在錯誤
    if diff -w /tmp/答案 "$expected" > /dev/null; then
      green_print "通過"
      ((passed_count++))
    else
      red_print "失敗"
      ((failed_count++))
    fi
  done

  echo "================================"
  echo "通過: $passed_count"
  echo "失敗: $failed_count"

  # Exit with status 1 if there are failed cases
  if [[ $failed_count -gt 0 ]]; then
    exit 1
  else
    exit 0
  fi
}

integration_test

