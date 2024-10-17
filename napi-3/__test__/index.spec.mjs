import test from "ava";

import {
  sum,
  hello,
  concatStr,
  getOptions,
  asyncFib,
  callThreadsafeFunction,
} from "../index.js";

test("sum from native", (t) => {
  t.is(sum(1, 2), 3);
});

test("hello", (t) => {
  t.is(hello(), "Hello, world!");
});

test("concatStr", (t) => {
  t.is(concatStr("Hello", "World"), "Hello World");
});

test("getOptions from native", (t) => {
  const options = {
    id: 1,
    name: "napi-3",
  };
  t.deepEqual(getOptions(options), options);
});

test("asyncFib from native", async (t) => {
  t.is(await asyncFib(10), 55);
});

test("callThreadsafeFunction from native", async (t) => {
  t.is(
    callThreadsafeFunction((err, ...args) => {
      console.log("Get the result from rust", args);
    })
  );
});
