# MARS MIPS での実行を想定

.text
main:
    # 横一列にピクセルを描画する処理を繰り返し、全体に描画する
    li $s1, 0 # y
loopY:
    li $s0, 0 # x
loopX:
    # t0: 色情報の書き込み先アドレス
    # t0 = (y * 256 + x) * 4 + startAddr
    mul $t0, $s1, 256
    add $t0, $t0, $s0
    mul $t0, $t0, 4
    addi $t0, $t0, 0x10010000

    # t1: 色情報
    # t1 = y * 0x100 + x
    move $t1, $s1
    mul $t1, $t1, 0x100
    add $t1, $t1, $s0

    sw $t1, 0($t0) # 描画の実行

    addi $s0, $s0, 1
    blt $s0, 256, loopX

    addi $s1, $s1, 1
    blt $s1, 256, loopY

    # 正常終了する
    li $v0, 10
    syscall
