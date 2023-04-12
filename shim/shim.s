
.globl _write_memory
.globl _read_memory

.text

_write_memory:
    rts
    mov.l r5,@r4

_read_memory:
    rts
    mov.l @r4,r0
