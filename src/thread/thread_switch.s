.intel_syntax noprefix
.global x86_64_thread_switch

/*
rsp: [rdi],
rbp: [rdi + 8],
rbx: [rdi + 16],
r12: [rdi + 24],
r13: [rdi + 32],
r14: [rdi + 40],
r15: [rdi + 48],
*/

x86_64_thread_switch:
    pushfq      // push RFLAGS register to stack

    mov qword ptr [rdi + 8], rbp
    mov qword ptr [rdi + 16], rbx
    mov qword ptr [rdi + 24], r12
    mov qword ptr [rdi + 32], r13
    mov qword ptr [rdi + 40], r14
    mov qword ptr [rdi + 48], r15

    mov qword ptr [rdi], rsp        // save old stack pointer in `rax` register
    mov rsp, qword ptr [rsi]        // load new stack pointer (given as argument)

    mov rbp, qword ptr [rsi + 8]
    mov rbx, qword ptr [rsi + 16]
    mov r12, qword ptr [rsi + 24]
    mov r13, qword ptr [rsi + 32]
    mov r14, qword ptr [rsi + 40]
    mov r15, qword ptr [rsi + 48]

    popfq       //; pop RFLAGS register to stack
    ret         //; pop return address from stack and jump to it
