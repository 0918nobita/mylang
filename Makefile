hello: hello.o
	gcc hello.o -no-pie -o hello

hello.o: hello.s
	nasm -f elf64 hello.s -o hello.o

.PHONY: clean
clean:
	rm -f hello hello.o
