# Custom Rust Deno Runtime Stack

## Intuition

Existing Rust applications would benefit from higher level interfaces using Typescript/Javascript to reduce crust writing tests, tighter data modeling, profiling, etc.

## Requirements

requires Rust - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.

requires Deno - `curl -fsSL https://deno.land/x/install/install.sh | sh`.

## Commands

### `deno task build`

Generates a `runtime.js` for Deno V8 runtime from `app/rs` and compiles `app/rs` and `app/vm`.

### `deno task vm-exec <script>.ts`

Runs `app/vm` V8 runtime with `<script>.ts`.

### `deno task e2e <script>.ts`

Codegens, compiles, and runs `app/vm` V8 runtime with `<script>.ts`.
