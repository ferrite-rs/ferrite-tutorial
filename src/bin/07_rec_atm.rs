use ferrite_session::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

/*
  # Excercise 7: Recursive ATM

  - Implement an `atm_provider` that dispense cash if the client provides
    the correct pin and the requested amount is lower than available balance.

    - If incorrect pin is given, offer the WrongPin branch and allow the client
      to retry up to MAX_RETY times. Sleeps for 1 second before responding
      to the client.

    - When max retry is exceeded, offer the MaxRetry branch and terminates.


  - Implement an `atm_client` that attempts to withdraw a given amount of
    cash from the ATM with the given pin.

    - If the ATM returns the WrongPin branch, retry again with the current pin + 1.

  - Print the following messages when the provider is in the following steps:

    - Received pin value:
      "[Provider] Authenticating ATM withdrawal with given pin {pin}"

    - Given pin matches the actual pin:
      "[Provider] Provided pin is correct. Asking for withdrawal amount""

    - Incorrect pin provider, with retry remaining:
      "[Provider] Provided pin is incorrect. Remaining attempts: {remaining}"

    - Incorrect pin provider, with maximum retry exceeded:
      "[Provider] Maximum attempts exceeded. Denying withdrawal access"

    - Withdraw amount <= account balance:
      "[Provider] Withdrawing ${withdraw_amount} from account. Remaining balance: ${balance}"

    - Withdraw amount > account balance:
      "[Provider] Insufficient fund: requested amount ${} is more than available balance ${}"

  - Print the following messages when the client is in the following steps:

    - Withdraw successful and cash spent
      "[Client] Spent cash of amount ${amount}"

    - Withdraw failed with incorrect pin:
      "[Client] Failed with incorrect pin. Trying again with pin {}"

    - Withdraw failed with maximum retry:
      "[Client] Failed to withdraw from ATM: Maximum retry attempted."

    - Withdraw failed with insufficient funds:
      "[Client] Failed to withdraw from ATM: Account has insufficient fund"

  The given main program will try to run your program with different
  parameters.

  Tips:

    - Use `include_session(atm_client(...))` and `forward` inside
      atm_client itself to recurse and then forward to the new client.

  After completing your solution, you should get the following result
  running the program:

  ```
  $ cargo run --bin 07_rec_atm
  *** Running new ATM session with actual pin: 1024, try pin: 1022, balance: 1000, withdrawal amount: 900 ***
  [Provider] Authenticating ATM withdrawal with given pin 1022
  [Provider] Provided pin is incorrect. Remaining attempts: 3
  [Client] Failed with incorrect pin. Trying again with pin 1023
  [Provider] Authenticating ATM withdrawal with given pin 1023
  [Provider] Provided pin is incorrect. Remaining attempts: 2
  [Client] Failed with incorrect pin. Trying again with pin 1024
  [Provider] Authenticating ATM withdrawal with given pin 1024
  [Provider] Provided pin is correct. Asking for withdrawal amount
  [Provider] Withdrawing $900 from account. Remaining balance: $100
  [Client] Spent cash of amount $900

  *** Running new ATM session with actual pin: 1024, try pin: 1010, balance: 1000, withdrawal amount: 900 ***
  [Provider] Authenticating ATM withdrawal with given pin 1010
  [Provider] Provided pin is incorrect. Remaining attempts: 3
  [Client] Failed with incorrect pin. Trying again with pin 1011
  [Provider] Authenticating ATM withdrawal with given pin 1011
  [Provider] Provided pin is incorrect. Remaining attempts: 2
  [Client] Failed with incorrect pin. Trying again with pin 1012
  [Provider] Authenticating ATM withdrawal with given pin 1012
  [Provider] Provided pin is incorrect. Remaining attempts: 1
  [Client] Failed with incorrect pin. Trying again with pin 1013
  [Provider] Authenticating ATM withdrawal with given pin 1013
  [Provider] Maximum attempts exceeded. Denying withdrawal access
  [Client] Failed to withdraw from ATM: Maximum retry attempted.

  *** Running new ATM session with actual pin: 1024, try pin: 1023, balance: 1000, withdrawal amount: 2000 ***
  [Provider] Authenticating ATM withdrawal with given pin 1023
  [Provider] Provided pin is incorrect. Remaining attempts: 3
  [Client] Failed with incorrect pin. Trying again with pin 1024
  [Provider] Authenticating ATM withdrawal with given pin 1024
  [Provider] Provided pin is correct. Asking for withdrawal amount
  [Provider] Insufficient fund: requested amount $2000 is more than available balance $1000
  [Client] Failed to withdraw from ATM: Account has insufficient fund.
  ```
*/
type Pin = u32;
type CashAmount = u64;

const MAX_RETRY: u8 = 3;

define_choice! { PinResult;
  PinOk:
    ReceiveValue<CashAmount, InternalChoice<WithdrawResult>>,
  WrongPin: Z,
  MaxRetry: End,
}

define_choice! { WithdrawResult;
  WithdrawOk: SendChannel<Cash, End>,
  InsufficientFund: End,
}

type Atm = Rec<ReceiveValue<Pin, InternalChoice<PinResult>>>;

pub struct CashVal {
  amount: CashAmount,
}

type Cash = SendValue<CashVal, End>;

fn forge_cash(amount: u64) -> Session<Cash> {
  send_value(CashVal { amount }, terminate())
}

/*
  A helper to spend the linear cash channel. You can use this like:
   include_session(spend_cash(), move |spend| {
     send_channel_to(spend, cash, ...) })
*/
fn spend_cash() -> Session<ReceiveChannel<Cash, End>> {
  receive_channel(move |cash| {
    receive_value_from(cash, move |cash_val: CashVal| {
      println!("[Client] Spent cash of amount ${}", cash_val.amount);
      wait(cash, terminate())
    })
  })
}

fn atm_provider(
  actual_pin: Pin,
  attempts: u8,
  balance: u64,
) -> Session<Atm> {
  todo!("Implement ATM provider here");
}

fn atm_client(
  pin: Pin,
  withdraw_amount: CashAmount,
) -> Session<ReceiveChannel<Atm, End>> {
  todo!("Implement ATM client here");
}

async fn run_atm_session(
  actual_pin: Pin,
  try_pin: Pin,
  balance: CashAmount,
  withdraw_amount: CashAmount,
) {
  println!("*** Running new ATM session with actual pin: {}, try pin: {}, balance: {}, withdrawal amount: {} ***",
    actual_pin, try_pin, balance, withdraw_amount);

  run_session(apply_channel(
    atm_client(try_pin, withdraw_amount),
    atm_provider(actual_pin, 0, balance),
  ))
  .await
}

#[tokio::main]
pub async fn main() {
  env_logger::init();

  run_atm_session(1024, 1022, 1000, 900).await;

  sleep(Duration::from_secs(2)).await;
  run_atm_session(1024, 1010, 1000, 900).await;

  sleep(Duration::from_secs(2)).await;
  run_atm_session(1024, 1023, 1000, 2000).await;
}
