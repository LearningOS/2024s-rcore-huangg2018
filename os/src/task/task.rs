//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// the task infomarion
    pub task_info: TaskInfo,
    /// the task start time
    pub task_start_time: usize,
    /// the task end time
    pub task_end_time: usize
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}


#[derive(Copy, Clone)]
/// the information of task
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

impl TaskInfo{
    /// initialize task infomarion
    pub fn init() ->Self {
        TaskInfo {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
    /// increase the time of task
    pub fn task_run_time(&mut self, rt: usize) {
        self.time = rt;
    }
    /// increase the time of task
    pub fn inc_syscall_times(&mut self, syscall_id: usize) {
        self.syscall_times[syscall_id] += 1;
    }
    /// get current task status
    pub fn set_status(&mut self,status: TaskStatus) {
        self.status = status;
    }
}