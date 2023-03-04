#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[allow(unused_imports)]
use deno_specta::{abi, runtime};
#[allow(unused_imports)]
use specta::*;

#[path = "src/actions/mod.rs"]
mod actions;

fn main() {
    /*
    #[cfg(feature = "codegen")]
    abi::export(
        // TODO: dynamically compile array of `actions::*`
        collect_types![actions::hello_world, actions::second_hello_world],
        "../ts/bindings.ts",
    )
    .unwrap();
    */

    #[cfg(feature = "codegen")]
    runtime::export(
        // TODO: dynamically compile array of `actions::*`
        collect_types![actions::hello_world, actions::second_hello_world],
        "../vm/src/runtime.js",
    )
    .unwrap();
}
