use ferrite_session::prelude::*;

/**
  # Excercise 1: Greet Provider

  Implement a greeter provider that receives the name as a string value and then
  print out the line "Hello, {name}!"

  After completing your solution, you should get the following result
  running the program:

  ```
  $ cargo run --bin 01_greet
  Hello, Alice!
  ```
**/

fn greeter() -> Session<ReceiveValue<String, End>> {
  todo!("implement greeter here");
}

fn main_session() -> Session<End> {
  include_session(greeter(), move |a| {
    send_value_to(a, "Alice".to_string(), wait(a, terminate()))
  })
}

#[tokio::main]
pub async fn main() {
  env_logger::init();

  run_session(main_session()).await
}
