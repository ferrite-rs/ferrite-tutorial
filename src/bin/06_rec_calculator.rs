use ferrite_session::prelude::*;

/*
  # Excercise 6: Recursive Calculator

  - Implement a stateful `calculator_provider` that keeps a number as
    current state and provides the following operations:

    - Add: new state = current state + received value

    - Mult: new state = current state * received value

    - Div:
      - If received value is not 0, offer DivOk and continue with
        new state = current state / received value
      - Otherwise, offer DivZero and terminate

    - Done: return current state and terminate

  - Implement a `calculator_client` that is given 3 parameters x, y, z,
    and calculate (((calculator_state * x) / y) + z) and send the
    result back

    - If the whole calculation succeeds, send Some(result)

    - If the provider offers DivZero, send None

  The main program will call your calculator provider and client with
  different parameters.

  After completing your solution, you should get the following result
  running the program:

  ```
  $ cargo run --bin 06_rec_calculator
  result of calculating ((2 * 3) / 4) + 5: 6.5
  error calculating ((4 * 2) / 0) + -2: divide by zero
  ```
*/
type Calculator = Rec<ExternalChoice<CalculatorOps>>;

define_choice! {DivResult;
  DivOk:
    Z,
  DivZero:
    End
}

define_choice! { CalculatorOps;
  Add:
    ReceiveValue<f64, Z>,
  Mult:
    ReceiveValue<f64, Z>,
  Div:
    ReceiveValue<f64,
      InternalChoice<DivResult>>,
  Done:
    SendValue<f64, End>,
}

fn calculator_provider(current: f64) -> Session<Calculator> {
  todo!("Implement calculator provider here");
}

fn calculator_client(
  x: f64,
  y: f64,
  z: f64,
) -> Session<ReceiveChannel<Calculator, SendValue<Option<f64>, End>>>
{
  todo!("implement calculator client here");
}

async fn calculate(init: f64, x: f64, y: f64, z: f64) {
  let res = run_session_with_result(apply_channel(
    calculator_client(x, y, z),
    calculator_provider(init),
  ))
  .await;

  match res {
    Some(res) => {
      println!(
        "result of calculating (({} * {}) / {}) + {}: {}",
        init, x, y, z, res
      );
    }
    None => {
      println!(
        "error calculating (({} * {}) / {}) + {}: divide by zero",
        init, x, y, z
      );
    }
  }
}

#[tokio::main]
pub async fn main() {
  env_logger::init();

  calculate(2.0, 3.0, 4.0, 5.0).await;

  calculate(4.0, 2.0, 0.0, -2.0).await;
}
