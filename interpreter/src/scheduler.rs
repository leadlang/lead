use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{parallel_ipreter::Wrapped, Application};

pub struct Scheduler {
  pub(crate) tasks: Arc<Mutex<Vec<Task>>>,
  app: Wrapped<Application<'static>>,
}

impl Scheduler {
  pub fn new(app: Wrapped<Application<'static>>) -> Self {
    let mut def_tasks = Vec::with_capacity(10);

    Self {
      tasks: Arc::new(Mutex::new(def_tasks)),
      app,
    }
  }

  pub async fn add_task(&mut self, task: Task) {
    self.tasks.lock().await.push(task);
  }

  pub async fn manage(&mut self) {
    for task in self.tasks.lock().await.drain(..) {
      task.spawn().await;
    }
  }
}

pub struct Task {
  pub(crate) app: Wrapped<Application<'static>>,
  pub references: &'static str,
}

impl Task {
  pub async fn spawn(self) {
    tokio::task::spawn(async move {});
  }
}
