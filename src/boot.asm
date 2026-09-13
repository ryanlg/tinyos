[BITS 32]
; @attrib: https://www.cs.vu.nl/~herbertb/misc/writingkernels.txt
; This part MUST be 4byte aligned, so we solve that issue using 'ALIGN 4'
ALIGN 4
mboot:
    ; Multiboot macros to make a few lines later more readable
    MULTIBOOT_PAGE_ALIGN	equ 1<<0
    MULTIBOOT_MEMORY_INFO	equ 1<<1
    MULTIBOOT_HEADER_MAGIC	equ 0x1BADB002
    MULTIBOOT_HEADER_FLAGS	equ MULTIBOOT_PAGE_ALIGN | MULTIBOOT_MEMORY_INFO
    MULTIBOOT_CHECKSUM	equ -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS)

    ; This is the GRUB Multiboot header. A boot signature
    dd MULTIBOOT_HEADER_MAGIC
    dd MULTIBOOT_HEADER_FLAGS
    dd MULTIBOOT_CHECKSUM

global start
extern _start
start:
    ; Technically we should verify to multiboot signature, but I'm lazy.

    ; Install GDT
    lgdt [gdt.pointer]

    ; At this point we are using segmente memory
    ; Set all the segment register to the data segment
    mov   ax, 0x10
    mov   ds, ax
    mov   es, ax
    mov   fs, ax
    mov   gs, ax
    mov   ss, ax

    ; Set up the stack
    mov esp, kernel_stack_top
    mov ebp, esp

    ; See you in Rust
    call _start

align 8
gdt:
    dq 0x0000000000000000 ; 0x00 | Null descriptor
    dq 0x00cf9a000000ffff ; 0x08 | 32-bit, present, code, base 0
    dq 0x00cf92000000ffff ; 0x10 | 32-bit, present, data, base 0
.pointer:
    dw .pointer - gdt - 1
    dd gdt

; Reserve space for kernel's stack
SECTION .bss
kernel_stack_bottom:
    resb 8192  ; 8KB
kernel_stack_top:
