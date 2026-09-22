use std::{env, path::PathBuf, process::Command};
fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../..");
    println!("cargo:rerun-if-changed=../../ui");
    println!("cargo:rerun-if-changed=../../scripts/build.ts");
    println!("cargo:rerun-if-env-changed=BUN");
    let runtime = env::var("BUN").unwrap_or_else(|_| "bun".into());
    let status=Command::new(runtime).current_dir(&root).arg("scripts/build.ts").arg("--outdir").arg(env::var("OUT_DIR").unwrap()).status().expect("Bun 1.3.14+ is required to bundle Noctave's TypeScript UI. Install Bun and restart your terminal.");
    assert!(
        status.success(),
        "Bun could not build the Noctave interface"
    );
}
