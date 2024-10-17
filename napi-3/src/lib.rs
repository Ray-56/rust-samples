#![deny(clippy::all)]

use std::thread;
use napi::{
  bindgen_prelude::*, Env, JsNumber, Result, Task, JsFunction,
 threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode}
};


#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn hello() -> String {
  "Hello, world!".to_string()
}

#[napi]
pub fn concat_str(a: String, b: String) -> String {
  format!("{} {}", a, b)
}

#[napi(object)]
pub struct ToolOptions {
  pub id: i32,
  pub name: String,
}

#[napi]
pub fn get_options(options: ToolOptions) -> ToolOptions {
  println!("id: {}, name: {}", options.id, options.name);
  options
}

struct AsyncFib {
  input: u32,
}

impl Task for AsyncFib {
  type Output = u32;
  type JsValue = JsNumber;

  fn compute(&mut self) -> Result<Self::Output> {
    Ok(fib(self.input))
  }

  fn resolve(&mut self, env: Env, output: u32) -> Result<Self::JsValue> {
    env.create_uint32(output)
  }
}

pub fn fib(n: u32) -> u32 {
  match n {
    0 | 1 => n,
    _ => fib(n - 1) + fib(n - 2),
  }
}

#[napi(ts_return_type = "Promise<string>")]
fn async_fib(input: u32) -> AsyncTask<AsyncFib> {
  AsyncTask::new(AsyncFib { input })
}


#[napi(ts_args_type = "callback: (err: null | Error, result: number) => void")]
pub fn call_threadsafe_function(callback: JsFunction) -> Result<()> {
  let tsfn: ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx| ctx.env.create_uint32(ctx.value).map(|v| vec![v]))?;

  for n in 0..100 {
    let tsfn = tsfn.clone();
    thread::spawn(move || {
      tsfn.call(Ok(n), ThreadsafeFunctionCallMode::Blocking);
    });
  }

  Ok(())
}