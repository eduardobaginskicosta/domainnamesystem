use crate::structs::WorkerTask;
use tokio::{sync::mpsc::Receiver, task::JoinHandle};

// * >>> *

pub async fn adjust_workers(
  workers: &mut Vec<JoinHandle<()>>,
  max_workers: usize,
  max_messages: usize,
  rx: &Receiver<WorkerTask>,
  debug: bool,
) {
  let workers_len: usize = workers.len();
  let messages_in_queue: usize = rx.len();

  if workers_len < max_workers && debug {
    println!("[DEBUG]: Increasing the number of workers!");
  }

  if debug {
    println!(
      "[INFO]: Workers {}/{} | Message Queue: {}/{}",
      if workers_len > max_workers {
        max_workers
      } else {
        workers_len
      },
      max_workers,
      if messages_in_queue > max_messages {
        max_messages
      } else {
        messages_in_queue
      },
      max_messages
    );
  }

  workers.retain(|worker| !worker.is_finished());
  if workers_len > max_workers && debug {
    println!("[DEBUG]: Reducting workers, removing inactive ones...");
  }
}
