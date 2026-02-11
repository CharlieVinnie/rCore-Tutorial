### 实现功能

1. 增加 `SyscallTrace` 结构体，用于记录每个任务每种 syscall 的次数
2. 在 `TaskControlBlock` 中增加一个 `SyscallTrace` 的字段，使任务之间的 syscall 记录相互独立
3. 增加 `TaskManager` 返回当前任务的 `SyscallTrace` 的接口
4. 实现所要求的 `sys_trace` 函数，可以读取、修改给定地址和输出当前任务的每种 syscall 次数

### 简答作业

1. 
    + `ch2b_bad_address.rs` 会输出 `[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.` 并立即退出
    + `ch2b_bad_instructions.rs` 和 `ch2b_bad_register.rs` 会输出 `[kernel] IllegalInstruction in application, kernel killed it.` 并立即退出
    + SBI 版本：`RustSBI version 0.3.0-alpha.2`
2.  1. 刚进入 `__restore` 时 `sp` 为内核栈的栈顶。当用户程序调用 `ecall` 时，操作系统处理完服务后会通过 `__restore` 返回原用户程序；当因 `yield` 或 timer 或其它异常而需要切换任务时，通过 `__restore` 切换到新任务的上下文继续执行
    2. 特殊处理了 `status` `sepc` 和 `sp` 寄存器。
        + `sstatus` 控制了 OS 会处理用户态发生的哪些异常
        + `sepc` 为 `__restore` 结束后所返回的用户程序地址，让用户程序继续执行
        + `sp` 为用户栈的栈顶，将已分配好的栈空间给用户态使用
    3. `x2` 为 `sp` 寄存器，影响恢复过程，必须最后赋值；`x4` 为 `tp` 寄存器，用户态程序不使用。
    4. L60 的指令之后，`sp` 为用户栈栈顶，`sscratch` 为内核栈栈顶。
    5. L61 的 `sret` 处发生状态切换；因为 `sstatus` 中的 `SPP` 位被设置为用户态，`sret` 会使得当前特权级被置为 `SPP`，即变为用户态。
    6. L13 的指令之后，`sp` 为内核栈栈顶，`sscratch` 为用户栈栈顶。
    7. 从 U 态进入 S 态在用户程序执行 `ecall` 时发生。