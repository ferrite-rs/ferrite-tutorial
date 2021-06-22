/*
 Exercise 0: Hello World

 This exercise helps make sure your environment is setup correctly
 and that you are able to build Ferrite programs with Rust.

 Try building this file by running the following command:

 $ cargo run --bin 00_hello
 Hello World!

 If you encounter any error here, do ask for help during the tutorial!
*/

use ferrite_session::prelude::*;

fn hello() -> Session<SendValue<String, End>> {
  send_value("Hello World!".to_string(), terminate())
}

#[tokio::main]
pub async fn main() {
  env_logger::init();

  let result = run_session_with_result(hello()).await;

  println!("{}", result);
}
