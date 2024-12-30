// The build for pcre2-sys currently does roughly the following:
//
//   1. Use the PCRE2 system library as reported by pkg-config if it exists
//      and only if we don't explicitly want a static build.
//   2. Otherwise, statically build PCRE2 by hand.
//
// For step 1, we permit opting out of using the system library via either
// explicitly setting the PCRE2_SYS_STATIC environment variable or if we
// otherwise believe we want a static build (e.g., when building with MUSL).
//
// For step 2, we roughly follow the directions as laid out in
// pcre2/NON-AUTOTOOLS-BUILD. It's pretty straight-forward: copy a few files,
// set a few defines and then build it. We can get away with a pretty stripped
// down setup here since the PCRE2 build setup also handles various command
// line tools (like pcre2grep) and its test infrastructure, and we needn't
// concern ourselves with that.
//
// It is plausible that this build script will need to evolve to do better
// platform detection for the various PCRE2 settings, but this should work
// as-is on Windows, Linux and macOS.

fn main() {
    println!("cargo:rerun-if-env-changed=PCRE2_SYS_STATIC");

    let do_utf32 = feature_enabled("UTF32");

    if do_utf32 {
        println!("cargo:rustc-link-lib=pcre2-32");
    }
    println!("cargo:rustc-link-lib=pcre2-8");
}

// Return whether a given feature is enabled.
fn feature_enabled(feature: &str) -> bool {
    let env_var_name = format!("CARGO_FEATURE_{}", feature);
    match std::env::var(&env_var_name) {
        Ok(s) => s == "1",
        Err(_) => false,
    }
}
