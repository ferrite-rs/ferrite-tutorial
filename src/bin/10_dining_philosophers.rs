/*
 Exercise 10: Dining Philosophers

 In this exercise you will implement the dining philosophers
 problem using session type.

   - Implement the fork as shared process.

     - Each fork is identified by a unique ID represented as u8.

     - When the fork is acquired, prints
       "fork {id} has been acquired"

   - Implement the philosopher as a linear process.

     - Each philosopher is identified by a unique ID represented as u8

     - Each philosopher is given two shared channels, representing the
       left and right forks.

     - The philosopher starts thinking by printing
       "philosopher {id} is thinking",
       then pauses for 1 second.

     - After finished thinking, print
       "philosopher {id} is going to eat"

     - Try to acquire the left fork, then prints
       "philosopher {id} has acquired the left fork"

     - Try to acquire the right fork, then prints
       "philosopher {id} has acquired the right fork"

     - Print "philosopher {id} is eating", then pause for 1 second.

     - After finished eating, print
       "philosopher {} has finished eating and is releasing the forks",

     - Release the right fork, followed by the left fork.

     - Start from the beginning again.

 - The main_session function has been provided to spawn up five philosophers and forks
   and run the session.

     - If your program is written correctly, the initial program *will* deadlock.

     - Fix the deadlock in main_session by reordering one of the fork pairs
       assigned to the philosophers.

 - Tips:

   - Use `sleep(Duration::from_secs(1)).await;` to pause for 1 second.

   - Use `step` to execute async code around Ferrite expressions. For example:

     let pause: Session<End> = step(async move {
       sleep(Duration::from_secs(1)).await;
       terminate()
     })
*/

use ferrite_session::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

type Fork = LinearToShared<Release>;

fn fork(id: u8) -> SharedSession<Fork> {
  todo!("Implement fork here");
}

fn run_fork(id: u8) -> SharedChannel<Fork> {
  run_shared_session(fork(id))
}

fn philosopher(
  id: u8,
  left: SharedChannel<Fork>,
  right: SharedChannel<Fork>,
) -> Session<End> {
  todo!("Implement philosopher here");
}

fn main_session() -> Session<End> {
  let f0 = run_fork(0);
  let f1 = run_fork(1);
  let f2 = run_fork(2);
  let f3 = run_fork(3);

  let p0 = philosopher(0, f0.clone(), f1.clone());
  let p1 = philosopher(1, f1.clone(), f2.clone());
  let p2 = philosopher(2, f2.clone(), f3.clone());
  let p3 = philosopher(3, f3.clone(), f0.clone());

  include_session(p0, move |c0| {
    include_session(p1, move |c1| {
      include_session(p2, move |c2| {
        include_session(p3, move |c3| {
          wait_all!([c0, c1, c2, c3], terminate())
        })
      })
    })
  })
}

#[tokio::main]
pub async fn main() {
  env_logger::init();

  run_session(main_session()).await;
}
