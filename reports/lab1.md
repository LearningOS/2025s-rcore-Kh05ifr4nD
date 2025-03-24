# Lab 1
## 功能总结（编程作业）
本章实验在批处理操作系统的基础上添加了多道程序分时多任务功能，我实现了成功添加了要求的 sys_trace 系统调用，通过所有测试样例。现在其可以提取用户程序并将其全部载入内存，实现对用户进程透明的进程切换和时间片轮换，提供了打印、主动让出 CPU 和读写指定地址内存（不安全）的系统调用。

## 简答作业
1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。请同学们可以自行测试这些内容（运行三个 bad 测例 (ch2b_bad_*.rs) ），描述程序出错行为，同时注意注明你使用的 sbi 及其版本。
使用 RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0/RustSBI-QEMU Version 0.2.0-alpha.2。
```rust
// ch2b_bad_address.rs
    unsafe {
        #[allow(clippy::zero_ptr)]
        (0x0 as *mut u8).write_volatile(0);
    }
```
行为：触发了 `Trap::Exception(Exception::StoreFault)`
```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
```

```rust
    unsafe {
        core::arch::asm!("sret");
    }
    panic!("FAIL: T.T\n");
```
行为：触发了 `Trap::Exception(Exception::IllegalInstruction)`
```
[kernel] IllegalInstruction in application, kernel killed it.
```

```rust
    let mut sstatus: usize;
    unsafe {
        core::arch::asm!("csrr {}, sstatus", out(reg) sstatus);
    }
```
行为：触发了 `Trap::Exception(Exception::IllegalInstruction)`
```
[kernel] IllegalInstruction in application, kernel killed it.
```

2. 深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:
   1. L40：刚进入 __restore 时，sp 代表了什么值。请指出 __restore 的两种使用情景。
        1. sp 代表当前程序内核栈地址。
        2. 程序运行结束或出错后的切换/运行第一个程序前的Trap上下文压栈初始化。

   2. L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。
    > ld t0, 32\*8(sp) \
    ld t1, 33\*8(sp) \
    ld t2, 2\*8(sp) \
    csrw sstatus, t0 \
    csrw sepc, t1 \
    csrw sscratch, t2

        sstatus(t0)恢复原先特权级/sepc(t1)用于跳回原程序下一条指令地址继续执行/sscratch(t2)恢复用户栈指针。

   3. L50-L56：为何跳过了 x2 和 x4？
    > ld x1, 1\*8(sp) \
    ld x3, 3\*8(sp) \
    .set n, 5 \
    .rept 27 \
       LOAD_GP %n \
       .set n, n+1 \
    .endr

        x2 为 sp，之后再存储；应用程序不使用 x4。

   4. L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？
    > csrrw sp, sscratch, sp

        sp：用户栈指针；sscratch：内核栈指针。

   5. __restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

        sret；其会根据恢复的 sstatus 值，确保返回正确的特权级别。

   6. L13：该指令之后，sp 和 sscratch 中的值分别有什么意义？
    > csrrw sp, sscratch, sp

        sp：内核栈指针；sscratch：用户栈指针。

   7. 从 U 态进入 S 态是哪一条指令发生的？
        ecall。
## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

    > 无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

    > 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。