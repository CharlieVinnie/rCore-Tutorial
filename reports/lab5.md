### 实现功能

1. 封装 `Resources` 类来记录 mutex 和 semaphores 的占用情况和请求量
1. 封装 `ResourcesBank` 类来记录可用资源量，并在其基础上实现 deadlock 检测算法
1. 在 `wakeup_task` 中以及 lock 之后将资源请求量合并入占用量

完成时间：约 4 小时

### 问答作业

1. 需要回收的资源： mutex 和 semaphore 锁资源、内核栈空间、用户栈空间、trap context 页面
   其它线程的 `TaskControlBlock` 可能在调度器和 mutex、semaphore 的等待队列中被引用，它们都需要被回收。

2. 区别：`Mutex1` 在 `lock` 中会循环检测是否 locked，而 `Mutex2` 只检测一次；`Mutex1` 在 `unlock` 中总是会释放锁，而 `Mutex2` 若有等待该锁的线程则跳过释放锁的步骤。
    
   问题：`Mutex2` 的实现是错误的。由于 `Mutex2` 在 `unlock` 中只是将等待锁的 task 加入执行队列，因此 processor 下一个调度到的 task 不一定是等待该锁的 task，这意味着获得该锁的 task 可能不是期望的 task，导致错误。