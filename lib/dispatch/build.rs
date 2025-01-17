//! Automatically enables the `dispatch_once_inline_fastpath` feature if supported by the target.
//!
//! **Important:** `lib/dispatch-sys/build.rs` and `lib/dispatch/build.rs` should be identical.

use std::ffi::OsStr;
use std::process::ExitCode;

fn main() -> ExitCode {
    println!("cargo:rerun-if-changed=build.rs");

    if input_env_var("CARGO_FEATURE_DISPATCH_ONCE_INLINE_FASTPATH").is_some() {
        eprintln!("Do not manually enable the dispatch_once_inline_fastpath feature.");
        eprintln!("This feature is automatically enabled if supported by the target.");
        return ExitCode::FAILURE;
    }

    if !is_truthy(input_env_var("NO_DISPATCH_ONCE_INLINE_FASTPATH")) {
        let target_arch_is_x86_or_x86_64 = input_env_var("CARGO_CFG_TARGET_ARCH")
            .is_some_and(|arch| arch == "x86_64" || arch == "x86");
        let target_vendor_is_apple =
            input_env_var("CARGO_CFG_TARGET_VENDOR").is_some_and(|vendor| vendor == "apple");

        if target_arch_is_x86_or_x86_64 || target_vendor_is_apple {
            println!("cargo:rustc-cfg=feature=\"dispatch_once_inline_fastpath\"");
        }
    }

    ExitCode::SUCCESS
}

fn input_env_var<K>(key: K) -> Option<String>
where
    K: core::fmt::Display + AsRef<OsStr>,
{
    println!("cargo:rerun-if-env-changed={}", &key);
    std::env::var(key).ok()
}

fn is_truthy<T>(value: Option<T>) -> bool
where
    T: AsRef<str>,
{
    value.is_some_and(|value| {
        let value = value.as_ref();
        value == "1" || value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("yes")
    })
}
