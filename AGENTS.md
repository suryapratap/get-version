# AGENTS.md

## Project overview

`get-version` is a small Rust CLI for CI/CD pipelines. It discovers the current
Git repository, renders version metadata into a template, and writes the result
to standard output.

## Repository layout

- `src/main.rs` contains the CLI, Git metadata lookup, template rendering, and
  UTC date formatting.
- `template.txt` is the default template embedded at compile time.
- `build.rs` ensures changes to `template.txt` rebuild the binary.
- `README.md` is the public CLI and behavior reference; keep it aligned with
  user-visible changes.

## Development commands

Run from the repository root:

```powershell
cargo fmt --check
cargo check
cargo test
cargo build --release
cargo run --
cargo run -- write
cargo run -- -t template.txt
```

There is currently no dedicated test module. Add focused unit tests alongside
the code when changing argument parsing, placeholder rendering, or date logic.

## Implementation conventions

- Keep the binary self-contained and avoid adding dependencies unless they are
  clearly justified.
- Preserve the CLI contract documented in `README.md`, especially stdout for
  successful output and stderr plus a non-zero exit code for errors.
- Support the embedded template as the default; `template.txt` changes must stay
  compatible with `include_str!("../template.txt")`.
- Use the `gix` crate for Git interactions rather than invoking the `git` CLI.
- Keep output deterministic: commit dates are formatted in UTC as
  `YYYY-MM-DD HH:mm`.
- Retain graceful behavior outside a Git repository (`no-repo`, `unknown`, and
  an empty tag) unless intentionally changing the documented behavior.

## Change checklist

- Run formatting and relevant Cargo checks.
- Update `README.md` for any CLI, placeholder, template, or behavior change.
- Do not commit `target/` build artifacts.
