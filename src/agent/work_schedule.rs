use std::collections::VecDeque;

use crate::task::task::{Intent, Task};



#[derive(Clone, PartialEq, Debug)]
pub struct WorkSchedule {
    pub tasks: VecDeque<Task>,
}

impl WorkSchedule {
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }

    pub fn push_back(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    pub fn pop_front(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn has_charging(&self) -> bool {
        self.tasks.iter().any(|task| 
            matches!(task,
                Task::Wait { intent: Intent::Charge, .. } |
                Task::Wait { intent: Intent::Queue, .. } |
                Task::Travel { intent: Intent::Charge, .. } |
                Task::Travel { intent: Intent::Queue, .. }
            ))
    }
}
