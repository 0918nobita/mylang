.data
startAddr: .word 0x10010000

.text
main:
    la $t0, startAddr
    li $s1, 0 # y

loopY:
    li $s0, 0 # x
loopX:
    mul $t1, $s1, 256
    add $t1, $t1, $s0
    mul $t1, $t1, 4
    add $t1, $t1, $t0
    move $t2, $s1
    mul $t2, $t2, 0x100
    add $t2, $t2, $s0
    sw $t2, 0($t1)
    addi $s0, $s0, 1
    blt $s0, 256, loopX
    addi $s1, $s1, 1
    blt $s1, 256, loopY

    li $v0, 10
    syscall
