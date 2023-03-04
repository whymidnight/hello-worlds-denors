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

## Example

### `deno task e2e scripts/vm_example.ts`

```
$ deno task e2e scripts/vm_example.ts
Task e2e deno task build && deno task vm-exec "scripts/vm_example.ts"
Task build deno task app-codegen && deno task build-vm
Task app-codegen cargo build -p app_rs --features codegen
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
Task build-vm cargo build -p vm
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
Task vm-exec cargo run --bin vm "scripts/vm_example.ts"
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/vm scripts/vm_example.ts`
[out]: "Hello, alice! You've been greeted from Rust!"
[out]: "Hello, alice! You've been greeted by bob!"
```
