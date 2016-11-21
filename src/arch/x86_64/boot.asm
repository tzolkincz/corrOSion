; Copyright 2015 Philipp Oppermann. See the README.md
; file at the top-level directory of this distribution.
;
; Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
; http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
; <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
; option. This file may not be copied, modified, or distributed
; except according to those terms.

global start
extern rust_main
extern long_mode_start

section .text
bits 32
start:
    mov esp, stack_top

    call check_multiboot
    call check_cpuid
    call check_long_mode

    call set_up_page_tables
    call enable_paging
    call set_up_SSE

    ; load the 64-bit GDT
    lgdt [gdt64.pointer]

    ; update selectors
    mov ax, gdt64.data
    mov ss, ax
    mov ds, ax
    mov es, ax

    jmp gdt64.code:long_mode_start

    ; set TSS
    cli ; disable interrupts
    mov ecx, [rust_main]
    mov [task_state_segment + 0x04], ecx  ;set esp0 (ring0 stack pointer)
    ltr [0x28]

    ; Set model specific registers for sysenter/sysexit
    mov ecx, 0x174 ; writes SS to model specific registers
    mov edx, 0
    mov eax, 0
    wrmsr

    mov ecx, 0x175 ; writes kernel ESP to model specific registers
    mov edx, 0
    mov eax, stack_top ; resets kernel stack
    wrmsr

    mov ecx, 0x176 ; writes kernel EIP to model specific registers
    mov edx, 0
    mov eax, [rust_main]
    wrmsr

  ;  sti ; enable interrupts


set_up_page_tables:
    ; map first P4 entry to P3 table
    mov eax, p3_table
    or eax, 0b11 ; present + writable
    mov [p4_table], eax

    ; map first P3 entry to P2 table
    mov eax, p2_table
    or eax, 0b11 ; present + writable
    mov [p3_table], eax

    ; map each P2 entry to a huge 2MiB page
    mov ecx, 0 ; counter variable
.map_p2_table:
    ; map ecx-th P2 entry to a huge page that starts at address (2MiB * ecx)
    mov eax, 0x200000  ; 2MiB
    mul ecx            ; start address of ecx-th page
    or eax, 0b10000011 ; present + writable + huge
    mov [p2_table + ecx * 8], eax ; map ecx-th entry

    inc ecx            ; increase counter
    cmp ecx, 512       ; if counter == 512, the whole P2 table is mapped
    jne .map_p2_table  ; else map the next entry

    ret

enable_paging:
    ; load P4 to cr3 register (cpu uses this to access the P4 table)
    mov eax, p4_table
    mov cr3, eax

    ; enable PAE-flag in cr4 (Physical Address Extension)
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; set the long mode bit in the EFER MSR (model specific register)
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret

; Prints `ERR: ` and the given error code to screen and hangs.
; parameter: error code (in ascii) in al
error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al
    hlt

; Throw error 0 if eax doesn't contain the Multiboot 2 magic value (0x36d76289).
check_multiboot:
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
.no_multiboot:
    mov al, "0"
    jmp error

; Throw error 1 if the CPU doesn't support the CPUID command.
check_cpuid:
    ; Check if CPUID is supported by attempting to flip the ID bit (bit 21) in
    ; the FLAGS register. If we can flip it, CPUID is available.

    ; Copy FLAGS in to EAX via stack
    pushfd
    pop eax

    ; Copy to ECX as well for comparing later on
    mov ecx, eax

    ; Flip the ID bit
    xor eax, 1 << 21

    ; Copy EAX to FLAGS via the stack
    push eax
    popfd

    ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax

    ; Restore FLAGS from the old version stored in ECX (i.e. flipping the ID bit
    ; back if it was ever flipped).
    push ecx
    popfd

    ; Compare EAX and ECX. If they are equal then that means the bit wasn't
    ; flipped, and CPUID isn't supported.
    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov al, "1"
    jmp error

; Throw error 2 if the CPU doesn't support Long Mode.
check_long_mode:
    ; test if extended processor info in available
    mov eax, 0x80000000    ; implicit argument for cpuid
    cpuid                  ; get highest supported argument
    cmp eax, 0x80000001    ; it needs to be at least 0x80000001
    jb .no_long_mode       ; if it's less, the CPU is too old for long mode

    ; use extended info to test if long mode is available
    mov eax, 0x80000001    ; argument for extended processor info
    cpuid                  ; returns various feature bits in ecx and edx
    test edx, 1 << 29      ; test if the LM-bit is set in the D-register
    jz .no_long_mode       ; If it's not set, there is no long mode
    ret
.no_long_mode:
    mov al, "2"
    jmp error

; Check for SSE and enable it. If it's not supported throw error "a".
set_up_SSE:
    ; check for SSE
    mov eax, 0x1
    cpuid
    test edx, 1<<25
    jz .no_SSE

    ; enable SSE
    mov eax, cr0
    and ax, 0xFFFB      ; clear coprocessor emulation CR0.EM
    or ax, 0x2          ; set coprocessor monitoring  CR0.MP
    mov cr0, eax
    mov eax, cr4
    or ax, 3 << 9       ; set CR4.OSFXSR and CR4.OSXMMEXCPT at the same time
    mov cr4, eax

    ret
.no_SSE:
    mov al, "a"
    jmp error

section .data
; warning - zeroing data of this segment
align 4096
task_state_segment:
    resb 104 ; size of TSS


section .bss
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
    resb 4096 * 32
stack_top:


; http://cathyreisenwitz.com/wp-content/uploads/2016/01/no.jpg
section .rodata
gdt64: ; Global Descriptor Table (64-bit).
    .null: equ $ - gdt64         ; The null descriptor. Kernel parameters
    dw 0                         ; Limit (low).
    dw 0                         ; Base (low).
    db 0                         ; Base (middle)
    db 0                         ; Access.
    db 0                         ; Granularity.
    db 0                         ; Base (high).
    .code: equ $ - gdt64         ; The code descriptor. Ring0 info (aka DPL)
    dw 0                         ; Limit (low).         Descriptor Privilege Level
    dw 0                         ; Base (low).
    db 0                         ; Base (middle)
    db 10011010b                 ; Access (exec/read).
    db 00100000b                 ; Granularity.
    db 0                         ; Base (high).
    .data: equ $ - gdt64         ; The data descriptor.
    dw 0                         ; Limit (low).
    dw 0                         ; Base (low).
    db 0                         ; Base (middle)
    db 10010010b                 ; Access (read/write).
    db 00000000b                 ; Granularity.
    db 0                         ; Base (high).
    .code_dpl3: equ $ - gdt64         ; The code descriptor. Ring3 info
    dw 0                         ; Limit (low).
    dw 0                         ; Base (low).
    db 0                         ; Base (middle)
    db 10011010b                 ; Access (exec/read).
    db 00101000b                 ; Granularity and Limit (hi)
    db 00000001b                         ; Base (high).
    .data_dpl3: equ $ - gdt64         ; The data descriptor.
    dw 0                         ; Limit (low).
    dw 0                         ; Base (low).
    db 0                         ; Base (middle)
    db 10010010b                 ; Access (read/write).
    db 00001000b                 ; Granularity  and Limit (hi)
    db 00000010b                         ; Base (high).
    .tss:
    dq task_state_segment ; set task_state_segment (the only one)
    dw 0x89 ; limit
    dw 0x40 ; access
    dq 0
    dq 0
    dq 0
    .pointer:                    ; The GDT-pointer.
    dw $ - gdt64 - 1             ; Limit.
    dq gdt64                     ; Base.
