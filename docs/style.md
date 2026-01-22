# Style Guide
Describes code style, module layout, naming conventions, and linting/formatting rules for Aqevia repositories.

## Formatting and linting

Whitespace, line endings, and indentation defaults are driven by `.editorconfig`; treat that file as the source of truth for things like UTF-8/LF output, final newlines, trailing-whitespace trimming (with Markdown opting out), and the per-language indent policies (2 spaces by default, 4 spaces for Rust/TOML, tabs for Makefiles).

Rust formatting is enforced via `cargo fmt --check`, while Clippy (`cargo clippy --all-targets --all-features -D warnings`) catches lint regressions and policy violations. The preferred local/CI entry point for those commands (plus `cargo test`) is `./scripts/test.sh`, which prints clear banners, runs fmt, runs clippy, and then executes the full test suite in order.
