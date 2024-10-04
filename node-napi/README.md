<p align="center">
    <br>
    <img src="https://huggingface.co/landing/assets/tokenizers/tokenizers-logo.png" width="600"/>
    <br>
<p>
<div align="center">

[![NPM Version](https://img.shields.io/npm/v/@flexpilot-ai/tokenizers)](https://www.npmjs.com/package/@flexpilot-ai/tokenizers)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/flexpilot-ai/tokenizers/CI.yml)](https://github.com/flexpilot-ai/tokenizers/actions/workflows/CI.yml)
[![GitHub License](https://img.shields.io/github/license/flexpilot-ai/tokenizers)](https://github.com/flexpilot-ai/tokenizers/blob/main/LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)

</div>
A high-performance Node.js library for text tokenization, providing bindings to the Rust implementation of HuggingFace's Tokenizers.

## Main Features

- **Fast and Efficient**: Leverages Rust's performance for rapid tokenization.
- **Versatile**: Supports various tokenization models including BPE, WordPiece, and Unigram.
- **Easy Integration**: Seamlessly use pre-trained tokenizers in your Node.js projects.
- **Customizable**: Fine-tune tokenization parameters for your specific use case.
- **Production-Ready**: Designed for both research and production environments.

## Installation

Install the package using npm:

```bash
npm install @flexpilot-ai/tokenizers
```

## Usage Example

Here's an example demonstrating how to use the Tokenizer class:

```typescript
import { Tokenizer } from "@flexpilot-ai/tokenizers";
import fs from "fs";

// Read the tokenizer configuration file
const fileBuffer = fs.readFileSync("path/to/tokenizer.json");
const byteArray = Array.from(fileBuffer);

// Create a new Tokenizer instance
const tokenizer = new Tokenizer(byteArray);

// Encode a string
const text = "Hello, y'all! How are you 😁 ?";
const encoded = tokenizer.encode(text, true);
console.log("Encoded:", encoded);

// Decode the tokens
const decoded = tokenizer.decode(encoded, false);
console.log("Decoded:", decoded);

// Use the fast encoding method
const fastEncoded = tokenizer.encodeFast(text, true);
console.log("Fast Encoded:", fastEncoded);
```

## API Reference

### `Tokenizer`

The main class for handling tokenization.

#### Constructor

```typescript
constructor(bytes: Array<number>)
```

Creates a new `Tokenizer` instance from a configuration provided as an array of bytes.

- `bytes`: An array of numbers representing the tokenizer configuration.

#### Methods

##### `encode`

```typescript
encode(input: string, addSpecialTokens: boolean): Array<number>
```

Encodes the input text into token IDs.

- `input`: The text to tokenize.
- `addSpecialTokens`: Whether to add special tokens during encoding.
- Returns: An array of numbers representing the token IDs.

##### `decode`

```typescript
decode(ids: Array<number>, skipSpecialTokens: boolean): string
```

Decodes the token IDs back into text.

- `ids`: An array of numbers representing the token IDs.
- `skipSpecialTokens`: Whether to skip special tokens during decoding.
- Returns: The decoded text as a string.

##### `encodeFast`

```typescript
encodeFast(input: string, addSpecialTokens: boolean): Array<number>
```

A faster version of the `encode` method for tokenizing text.

- `input`: The text to tokenize.
- `addSpecialTokens`: Whether to add special tokens during encoding.
- Returns: An array of numbers representing the token IDs.

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for more details.

## License

This project is licensed under the Apache-2.0 License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- This library is based on the [HuggingFace Tokenizers](https://github.com/huggingface/tokenizers) Rust implementation.
- Special thanks to the Rust and Node.js communities for their invaluable resources and support.
