# 本文件僅為測試用途
# 用以測試「曰.o」中外術鏈結
    .section .text
    .global 甲
    .global 乙
    .global main

main:
    call 甲

    # 結束程序，返回0
    li a7, 93           # ecall號碼93表示退出
    li a0, 0
    ecall

甲:
    # 四個區域變數＋返回位址＋舊 fp
    # 總共 6 * 8 位元組 （64 位元系統）
    addi sp, sp, -48
    # 儲存返回地址
    sd ra, 40(sp)      # ra = return address = 返回地址
    # 儲存舊棧底（fp）
    sd s0, 32(sp)      # s0 就是 fp
    # 更新 s0(fp) 為現在的棧底
    addi s0, sp, 48

    // 初始化區域變數
    li t0, 1             # t0 = 1
    sd t0, -24(s0)       # 元．天 = 1
    sd t0, -32(s0)       # 元．地 = 1
    sd t0, -40(s0)       # 元．玄 = 1
    sd t0, -48(s0)       # 元．黃 = 1

    call 乙              # jal ra, 術乙

    # 收尾
    ld ra, -8(s0)        # 恢復返回位址給暫存器 ra
    mv sp, s0            # 歸還棧空間
    ld s0, -16(s0)        # 恢復 fp
    ret                  # jr ra


乙:
    # 四個區域變數＋返回位址＋舊 fp
    # 總共 6 * 8 位元組 （64 位元系統）
    addi sp, sp, -48
    # 儲存返回地址
    sd ra, 40(sp)      # ra = return address = 返回地址
    # 儲存舊棧底（fp）
    sd s0, 32(sp)      # s0 就是 fp
    # 更新 s0(fp) 為現在的棧底
    addi s0, sp, 48

    // 初始化區域變數
    li t0, 1             # t0 = 1
    sd t0, -24(s0)        # 元．宇 = 1
    sd t0, -32(s0)        # 元．宙 = 1
    sd t0, -40(s0)        # 元．洪 = 1
    sd t0, -48(s0)        # 元．荒 = 1

    # 同零．一版用堆疊機來計算宇＋宙＋洪＋荒

    # 宇入棧
    addi sp, sp, -8
    ld t0, -24(s0)
    sd t0, 0(sp)

    # 宙入棧
    addi sp, sp, -8
    ld t0, -32(s0)
    sd t0, 0(sp)

    # 加
    ld t1, 0(sp)
	addi sp, sp, 8
	ld t0, 0(sp)
	add t0, t0, t1
	sd t0, 0(sp)

    # 洪入棧
    addi sp, sp, -8
    ld t0, -40(s0)
    sd t0, 0(sp)

    # 加
    ld t1, 0(sp)
	addi sp, sp, 8
	ld t0, 0(sp)
	add t0, t0, t1
	sd t0, 0(sp)

    # 荒入棧
    addi sp, sp, -8
    ld t0, -48(s0)
    sd t0, 0(sp)

    # 加
    ld t1, 0(sp)
	addi sp, sp, 8
	ld t0, 0(sp)
	add t0, t0, t1
	sd t0, 0(sp)

    # 答案在棧頂，將其載到 t1 暫存器
    ld t1, 0(sp)
    # 彈出計算結果後 sp 又回到術開始時的原本位置，棧大小恢復如初
	addi sp, sp, 8

    # 準備參數，施展外術「曰」
    mv a0, t1
    call 曰

    # 收尾
    ld ra, -8(s0)        # 恢復返回位址給暫存器 ra
    mv sp, s0            # 歸還棧空間
    ld s0, -16(s0)       # 恢復 fp
    ret                  # jr ra


