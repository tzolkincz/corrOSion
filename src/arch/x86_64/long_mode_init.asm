; Copyright 2015 Philipp Oppermann. See the README.md
; file at the top-level directory of this distribution.
;
; Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
; http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
; <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
; option. This file may not be copied, modified, or distributed
; except according to those terms.

global long_mode_start

extern kentry
extern kmain
extern kstack

extern task_state_segment

section .text
bits 64
long_mode_start:

    ; set TSS
    ;cli ; disable interrupts
    ;mov ecx, [rust_main]
    ;mov [task_state_segment + 0x04], ecx  ;set esp0 (ring0 stack pointer)
    ;mov ax, 0x2B
    ;ltr ax

    ; Set model specific registers for sysenter/sysexit
    mov ecx, 0x174 ; writes SS to model specific registers
    mov edx, 0
    mov eax, 0x08 ;skip null segment
    wrmsr

    mov ecx, 0x175 ; writes kernel ESP to model specific registers
    mov edx, 0
    mov eax, kstack ; resets kernel stack
    wrmsr

    mov ecx, 0x176 ; writes kernel EIP to model specific registers
    mov edx, 0
    mov eax, kentry
    wrmsr

    ;  sti ; enable interrupts
    call kmain
.os_returned:
    ; rust main returned, print `OS returned!`
    mov rax, 0x4f724f204f534f4f
    mov [0xb8000], rax
    mov rax, 0x4f724f754f744f65
    mov [0xb8008], rax
    mov rax, 0x4f214f644f654f6e
    mov [0xb8010], rax
    hlt
