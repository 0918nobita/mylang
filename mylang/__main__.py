with open('output.s', 'w') as file:
    file.write(""".data
msg: .asciiz "Hello, world!\\n"
.text
main:
    li $v0, 4
    la $a0, msg
    syscall
    li $v0, 10
    syscall
""")
