/* MIT-licensed x86_64 Linux syscall-only Flatpak fixture; no runtime libraries. */
.section .rodata
message:
    .ascii "ForgeStore Flatpak fixture @VERSION@\n"
message_end:
.section .text
.global _start
_start:
    mov $1, %rax
    mov $1, %rdi
    lea message(%rip), %rsi
    mov $(message_end-message), %rdx
    syscall
    mov $60, %rax
    xor %rdi, %rdi
    syscall
