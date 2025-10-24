use super::{adjust_workers, handle_worker_task};
use crate::structs::WorkerTask;
use tokio::{spawn, sync::mpsc::Receiver, task::JoinHandle};

// * >>> *

pub async fn worker_pool(
  mut rx: Receiver<WorkerTask>,
  max_workers: usize,
  max_messages: usize,
  debug: bool,
) {
  let mut workers: Vec<JoinHandle<()>> = Vec::new();
  loop {
    if let Some(task) = rx.recv().await {
      if let Some(worker) = workers.iter_mut().find(|worker| worker.is_finished()) {
        worker.abort();
        let new_worker: JoinHandle<()> = spawn(handle_worker_task(task, debug));
        workers.push(new_worker);
      } else if workers.len() < max_workers {
        let new_worker: JoinHandle<()> = spawn(handle_worker_task(task, debug));
        workers.push(new_worker);
      } else if debug {
        println!("[DEBUG]: Maximum number of workers reached. Waiting...");
      }
    }

    adjust_workers(&mut workers, max_workers, max_messages, &rx, debug).await
  }
}
