#[cfg(feature = "codegen")]
use specta::specta;

#[allow(unused)]
#[cfg_attr(feature = "deno_op", deno_core::op)]
#[cfg_attr(feature = "codegen", specta)]
pub fn hello_world(my_name: String) -> String {
    format!("Hello, {my_name}! You've been greeted from Rust!")
}

#[allow(unused)]
#[cfg_attr(feature = "deno_op", deno_core::op)]
#[cfg_attr(feature = "codegen", specta)]
pub async fn second_hello_world(
    my_name: String,
    your_name: String,
) -> Result<String, deno_core::anyhow::Error> {
    Ok(format!(
        "Hello, {my_name}! You've been greeted by {your_name}!"
    ))
}
