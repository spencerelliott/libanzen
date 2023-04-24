shim:
	rm -f libs/libshim.a
	sh-elf-as -little -o libs/shim.o shim/shim.s
	sh-elf-ar rs libs/libshim.a libs/shim.o
	rm libs/shim.o

.PHONY: shim