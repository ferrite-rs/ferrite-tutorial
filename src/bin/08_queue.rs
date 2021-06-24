use ferrite_session::prelude::*;

/*
  # Excercise 8: Linear Queue

  - Implement a queue provider consist of chains of linear processes,
    providing the session type Queue with following operations:

    - Enqueue: Receive a string value, enqueue it to the back of of the queue
      and then release.

    - Dequeue:
      - If the queue is not empty:
        - Offer the branch `Elem`
        - Pop the front of the queue and send the value
        - Recurse back to the Queue session type

      - If the queue is empty:
        - Offer the branch `Empty`
        - terminate

  - Implement an empty queue provider.

  - Implement a elem queue provider which:
      - Takes a string value
      - Receives a tail channel offering `Queue`,
      - Offers a new `Queue` channel with the given string value as head,
        and tail operations delegated to the tail process.

  - Implement an `enqueue_hello_world` client which enqueues "Hello" and then
    enqueues "World" to the queue, then sends back the new queue.

  - Implement a `dequeue_all` client that dequeues all elements in the queue
    and then terminates.

    - If the current queue is non empty, prints
      "Dequeued value: {val}"
      and then continue dequeue the remaining elements

    - If the current queue is empty, prints
      "Queue is now empty" and then terminate

  The provided main function spawns the queues providers and clients,
  use the enqueue the strings, and then.

  After completing your solution, you should get the following result
  running the program:

  ```
  $ cargo run --bin 08_queue
  Dequeued value: World
  Dequeued value: Hello
  Queue is now empty
  ```
*/


type Queue = Rec<ExternalChoice<QueueOps>>;

define_choice! { QueueOps;
  Enqueue: ReceiveValue<String, Z>,
  Dequeue: InternalChoice<DequeueOps>
}

define_choice! { DequeueOps;
  Elem: SendValue<String, Z>,
  Empty: End,
}

fn empty() -> Session<Queue>
{
  todo!("Implement empty queue here");
}

fn elem(val: String) -> Session<ReceiveChannel<Queue, Queue>>
{
  todo!("Implement elem queue here");
}

fn dequeue_all() -> Session<ReceiveChannel<Queue, End>>
{
  todo!("Implement dequeue all here");
}

fn enqueue_hello_world() -> Session<ReceiveChannel<Queue, Queue>>
{
  todo!("Implement enqueue here");
}

fn main_session() -> Session<End>
{
  include_session(empty(), move |queue| {
    include_session(enqueue_hello_world(), move |c1| {
      include_session(dequeue_all(), move |c2| {
        send_channel_to(c1, queue,
          send_channel_to(c2, c1,
            wait(c2, terminate())))
      })
    })
  })
}

#[tokio::main]
pub async fn main()
{
  env_logger::init();

  run_session(main_session()).await;
}
