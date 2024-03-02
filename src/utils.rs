use std::process::Command;
use std::sync::OnceLock;

/// The default target to pass to cargo, to workaround issue #11.
pub fn default_target() -> &'static str {
    static DEFAULT_TARGET: OnceLock<String> = OnceLock::new();
    DEFAULT_TARGET.get_or_init(|| {
        // See https://github.com/rust-fuzz/cargo-fuzz/issues/355#issuecomment-1974748776
        // for why we got host triple from the cargo instead of using current_platform crate.
        let mut cmd = Command::new("cargo");
        cmd.args(["--version", "--verbose"]);
        let output = cmd
            .output()
            .expect("failed to execute `cargo --version --verbose`");
        let verbose_version = String::from_utf8_lossy(&output.stdout);
        let host = verbose_version
            .lines()
            .find_map(|line| line.strip_prefix("host: "))
            .expect("unexpected version output from `cargo --version --verbose`")
            .to_owned();
        assert!(output.status.success());
        host
    })
}
