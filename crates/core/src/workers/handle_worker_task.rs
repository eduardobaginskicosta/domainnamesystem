use crate::{actions::handle_query, structs::WorkerTask};

// * >>> *

pub async fn handle_worker_task(task: WorkerTask, debug: bool) {
  if let Err(e) = handle_query(
    &task.config,
    &task.lookup,
    &task.socket,
    task.data,
    task.src,
    task.debug,
  ) {
    if debug {
      println!("[DEBUG]: Error processing query for {}: {}", task.src, e);
    }
  }
}
