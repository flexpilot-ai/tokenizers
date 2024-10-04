import test from "ava";

import { Tokenizer } from "../index.js";
import { readFileSync } from "fs";

test("Tokenizer from native", (t) => {
  const fileBuffer = readFileSync("./__test__/tokenizer.json");
  const byteArray = Array.from(fileBuffer);
  const tokenizer = new Tokenizer(byteArray);
  const originalString = "Hello, y'all! How are you 😁 ?";
  const encoded = tokenizer.encode(originalString, false);
  const decodedString = tokenizer.decode(encoded, false);
  t.is(originalString, decodedString);
});
