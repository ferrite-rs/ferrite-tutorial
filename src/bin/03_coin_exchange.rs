use ferrite_session::prelude::*;

/**
  # Excercise 3: Coin Exchange

  You are given the following channels
    - `exchange` accepts 2 nickels and gives back one dime.
    - `vending_machine` accepts a dime and gives back a drink
    - 2 nickels represented as linear channels.

  - Implement `main_session` to exchange the 2 nickels for one dime,
    and buy a drink from the vending machine. After getting the drink
    value, print out the line "[Main] Gotten Drink drink".

  After completing your solution, you should get the following result
  running the program:

  ```
  $ cargo run --bin 03_coin_exchange
  [Coin Exchange] Received 2 nickels, dispensing 1 dime
  [Vending Machine] Received a dime, dispensing soft drink
  [Main] Gotten Drink drink
  ```
**/

// A drink is represented as a value
struct Drink;

// We wrap nickels and dimes as linear resources using SendValue.
// To keep the exercise simple, we do not implement any encapsulation
// to prevent anyone from forging the coins.
struct NickelVal;
struct DimeVal;

type Nickel = SendValue<NickelVal, End>;
type Dime = SendValue<DimeVal, End>;

// Helper functions to forge nickels and dimes. Do not use these
// in your solution!
fn forge_nickel() -> Session<Nickel> {
  send_value(NickelVal, terminate())
}

fn forge_dime() -> Session<Dime> {
  send_value(DimeVal, terminate())
}

fn vending_machine(
) -> Session<ReceiveChannel<Dime, SendValue<Drink, End>>> {
  receive_channel(move |dime| {
    println!(
      "[Vending Machine] Received a dime, dispensing soft drink"
    );
    receive_value_from(dime, move |_| {
      send_value(Drink, wait(dime, terminate()))
    })
  })
}

fn coin_exchange() -> Session<
  ReceiveChannel<
    Nickel,
    ReceiveChannel<Nickel, SendChannel<Dime, End>>,
  >,
> {
  receive_channel(move |nickel1| {
    receive_channel(move |nickel2| {
      println!(
        "[Coin Exchange] Received 2 nickels, dispensing 1 dime"
      );
      receive_value_from(nickel1, move |_| {
        receive_value_from(nickel2, move |_| {
          include_session(forge_dime(), move |dime| {
            send_channel_from(
              dime,
              wait_all!([nickel1, nickel2], terminate()),
            )
          })
        })
      })
    })
  })
}

fn main_session() -> Session<End> {
  include_session(coin_exchange(), move |coin_exchange| {
    include_session(vending_machine(), move |vending_machine| {
      include_session(forge_nickel(), move |nickel1| {
        include_session(forge_nickel(), move |nickel2| {
          todo!("Exchange nickel for dime, adn then get soft drink from vending machine");
        })
      })
    })
  })
}

#[tokio::main]
async fn main() {
  env_logger::init();

  run_session(main_session()).await
}
