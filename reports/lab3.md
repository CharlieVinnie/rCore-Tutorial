### 实现功能

1. 封装了 `Priority` 类来记录进程的优先级，其中记录了 `stride` 和 `pass`，并支持带有溢出处理的比较
1. 修改了 `TaskManager`，使其支持按优先级调度，实现上是暴力扫描并取出优先级最高的进程
1. 实现了 `sys_spawn`，使用 `TaskControBlock::new` 直接新建进程

### 问答作业

+ 实际情况是继续轮到 p2 执行，因为 p2 的 pass 值溢出后为 `(255+10)%256=4` 小于 255。
+ 设当前进程最大最小优先级分别为 `MAX` 和 `MIN`，则执行一步之后 `MIN` 增加不超过 `BIG_STRIDE/2`，故新的 `MAX` 不超过 `max(MAX, MIN + BIG_STRIDE/2)`，新的 `MIN` 不小于 `MIN`，故若原来的 `MAX-MIN` 不超过 `BIG_STRIDE/2`，则新的 `MAX-MIN` 也不会超过。
+   ```rust
    let diff = other.pass.wrapping_sub(self.pass);
    if diff == 0 {
        core::cmp::Ordering::Equal
    } else if diff <= BIG_STRIDE / 2 {
        core::cmp::Ordering::Greater
    } else {
        core::cmp::Ordering::Less
    }
    ```
