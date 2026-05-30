use alloc::vec::Vec;

pub struct Task {
    id: TaskId,
    future: alloc::boxed::Box<
        dyn core::future::Future<Output = ()> + Send + 'static,
    >,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

pub struct Executor {
    tasks: Vec<Task>,
    next_id: u64,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: Vec::new(),
            next_id: 0,
        }
    }

    pub fn spawn(&mut self, future: impl core::future::Future<Output = ()> + Send + 'static) {
        self.tasks.push(Task {
            id: TaskId(self.next_id),
            future: alloc::boxed::Box::new(future),
        });
        self.next_id += 1;
    }
}

#[allow(dead_code)]
mod alloc {
    pub use alloc::boxed;
    pub use alloc::vec;
}
