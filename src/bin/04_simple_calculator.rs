use ferrite_session::prelude::*;

/*
  # Excercise 4: Simple Calculator

  - Implement `calculator_provider` to provide either addition or
    multiplication operations, depending on the choice selected by
    the client.

  - You are given two calculator channels in `main_session`. Use them to
    calculate 3 + 4 * 5 and print the result.

  After completing your solution, you should get the following result
  running the program:

  ```
  $ cargo run --bin 04_simple_calculator
  result of 3 + 4 x 5: 23
  ```
*/
type Calculator = ExternalChoice<CalculatorOps>;

define_choice! { CalculatorOps;
  Add: ReceiveValue<(i32, i32), SendValue<i32, End>>,
  Mult: ReceiveValue<(i32, i32), SendValue<i32, End>>,
}

fn calculator_provider() -> Session<Calculator> {
  todo!("Implement a calculator provider here");
}

// Challenge: Implement a main program that includes two instances
// of calculator_provider, calculate the result of 3 + 4 * 5,
// and prints the result.
fn main_session() -> Session<End> {
  include_session(calculator_provider(), move |c1| {
    include_session(calculator_provider(), move |c2| {
      todo!("Implement a calculator client here");
    })
  })
}

#[tokio::main]
pub async fn main() {
  env_logger::init();

  run_session(main_session()).await
}
