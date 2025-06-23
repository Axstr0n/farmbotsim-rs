use std::collections::VecDeque;

use crate::task_module::task::{Intent, Task};


/// Represents a schedule of tasks assigned to an agent.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct WorkSchedule {
    /// Queue of tasks to be performed.
    pub tasks: VecDeque<Task>,
}


impl WorkSchedule {
    /// Adds a task to the end of the schedule.
    pub fn push_back(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    /// Removes and returns the first task in the schedule.
    pub fn pop_front(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }

    /// Checks if the schedule contains no tasks.
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Removes all tasks from the schedule.
    pub fn clear(&mut self) {
        self.tasks.clear();
    }

    /// Returns the number of tasks in the schedule.
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Checks if the schedule contains any charging-related tasks.
    pub fn has_charging(&self) -> bool {
        self.tasks.iter().any(|task| 
            matches!(task,
                Task::WaitDuration { intent: Intent::Charge, .. } |
                Task::WaitDuration { intent: Intent::Queue, .. } |
                Task::WaitInfinite { intent: Intent::Charge, .. } |
                Task::WaitInfinite { intent: Intent::Queue, .. } |
                Task::Travel { intent: Intent::Charge, .. } |
                Task::Travel { intent: Intent::Queue, .. }
            ))
    }
}
