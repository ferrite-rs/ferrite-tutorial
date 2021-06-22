use ferrite_session::prelude::*;

/*
  # Excercise 8: Shared Queue

  - Implement a shared queue provider with a `Vec<String>` internal state
    and provides the following operations:

    - Enqueue: Receive a string value, enqueue it to the back of of the queue
      and then release.

    - Dequeue:
      - If the queue is not empty, pop the front of the queue and send the value
        as `Some(res)`.
      - If the queue is empty, sends `None`.

  - Implement an `enqueue` function, which takes a `SharedChannel<Queue>`
    and a string value. The function would run a Ferrite session that
    acquires the shared proess, choose Enqueue, and sends the value to
    the shared queue process.

  - Implement a `dequeue` function, which takes a `SharedChannel<Queue>`
    and does the following:

    - Run a Ferrite session that acquires the shared proess
    - Choose Dequeue and receives the value.
    - If the result is `Some(val)`, print "Gotten dequeue value: {val}"
    - If the result is `None`, print "Dequeue returns None".

  The provided main function will spawn a shared queue, and call
  the `enqueue` and `dequeue` functions with different parameters.

  After completing your solution, you should get the following result
  running the program:

  ```
  $ cargo run --bin 08_shared_queue
  Gotten dequeue value: World
  Gotten dequeue value: Hello
  Dequeue returns None
  ```
*/

type Queue = LinearToShared<ExternalChoice<QueueOps>>;

define_choice! { QueueOps;
  Enqueue: ReceiveValue<String, Release>,
  Dequeue: SendValue<Option<String>, Release>
}

fn shared_queue(mut queue: Vec<String>) -> SharedSession<Queue> {
  todo!("Implement shared queue here");
}

fn create_shared_queue() -> SharedChannel<Queue> {
  run_shared_session(shared_queue(vec![]))
}

async fn enqueue(queue: SharedChannel<Queue>, val: String) {
  todo!("Implement enqueue client here");
}

async fn dequeue_and_print(queue: SharedChannel<Queue>) {
  todo!("Implement dequeue client here");
}

#[tokio::main]
pub async fn main() {
  env_logger::init();

  let queue = create_shared_queue();

  enqueue(queue.clone(), "Hello".to_string()).await;
  enqueue(queue.clone(), "World".to_string()).await;

  dequeue_and_print(queue.clone()).await;
  dequeue_and_print(queue.clone()).await;
  dequeue_and_print(queue.clone()).await;
}
