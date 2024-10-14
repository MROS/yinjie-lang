    .section .text
    .global main

main:
    # 傳遞一個 64 位的數字作為參數，放在 a0 中
    li a0, 1234567890123456789  # 要打印的 64 位整數

    # 呼叫 print_int 函數
    call print_int

    # 結束程序，返回0
    li a7, 93           # ecall號碼93表示退出
    ecall

