.data
startAddr: .word 0x10010000

.text
main:
    la $t0, startAddr
    li $s0, 0

loop:
    mul $t1, $s0, 4
    add $t1, $t1, $t0
    sw $s0, 0($t1)
    addi $s0, $s0, 1
    blt $s0, 0x100, loop

    li $v0, 10
    syscall
