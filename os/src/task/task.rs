//! Types related to task management

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskCtrlBlk {
    /// The task context
    pub ctx: super::TaskCtx,
    /// The task status in it's lifecycle
    pub stat: TaskStat,
    /// The array for counting the time of each invoked syscall
    pub syscall_time: [usize; crate::config::SYSCALL_NUM],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStat {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
