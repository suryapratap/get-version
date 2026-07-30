# get-version

A small, self-contained CLI that reads git metadata from the current repository and fills a text template. Built for **CI/CD pipelines** where you need a simple, repeatable way to map commit details into a version file, build report, artifact label, or release note.

No shell scripting of `git rev-parse` / `git log` is required. Output goes to **stdout**, so you can pipe or redirect it wherever your pipeline needs it.

## Why this exists

In pipelines you often need to stamp builds with:

- which branch produced the artifact  
- which commit was built  
- whether a tag is present  
- when that commit was made  

`get-version` does that in one command, with an optional template so the same binary can drive different report formats across jobs.

## Placeholders

| Placeholder | Meaning |
|-------------|---------|
| `[BRANCH]`  | Current branch name (or `HEAD` if detached) |
| `[COMMIT]`  | Short commit SHA (7 characters) |
| `[TAG]`     | Nearest tag reachable from HEAD (`git describe --tags`), or empty if none |
| `[DATE]`    | HEAD **commit** date (committer time), UTC, `YYYY-MM-DD HH:mm` |

A default template is **embedded in the binary**. You do not need a template file on disk unless you want a custom layout.

Default template content:

```text
[BRANCH]
[COMMIT]
[TAG]
[DATE]
```

## Quick start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)  
- A git repository as the working directory when you run the tool  

### Build

```bash
cargo build --release
```

Binary:

- Windows: `target/release/get-version.exe`  
- Unix: `target/release/get-version`  

### Run (from a repo root)

```bash
# Print version info using the embedded template
./target/release/get-version

# Write version info to a file
./target/release/get-version > version.txt

# Emit the embedded template (for inspection or to seed a custom file)
./target/release/get-version write
./target/release/get-version write > template.txt

# Use a custom template
./target/release/get-version -t template.txt
./target/release/get-version -t template.txt > version.txt
```

Example output:

```text
main
a638e56

2026-07-24 10:30
```

## Usage

```text
get-version                 Print version info using the embedded template
get-version -t <file>       Print version info using a custom template file
get-version write           Write the embedded template to stdout

Options:
  -t, --template <file>   Template file path (default: embedded template)
  -h, --help              Show help
```

Notes:

- Always specify a custom template with **`-t`** / **`--template`**. A bare path is rejected on purpose so intent is clear.  
- **`write`** always prints the embedded template to stdout. Redirect or pipe it if you want a file.  
- The tool discovers the git repo from the **current working directory** (walks up like git).

## Custom templates

Any text file is valid. Placeholders are replaced literally; all other text is left as-is.

```text
Build report
------------
Branch:  [BRANCH]
Commit:  [COMMIT]
Tag:     [TAG]
Date:    [DATE] UTC
```

```bash
get-version -t report.tmpl > build-report.txt
```

## CI/CD examples

### Bash / Linux / macOS agents

```bash
get-version > version.txt
# or with a custom report layout
get-version -t ./ci/version.tmpl > artifacts/VERSION
```

### PowerShell agents

```powershell
./get-version.exe > version.txt
./get-version.exe -t .\ci\version.tmpl | Out-File -Encoding utf8 artifacts\VERSION
```

### GitHub Actions

```yaml
- name: Write version file
  run: |
    cargo build --release
    ./target/release/get-version > version.txt
    cat version.txt
```

### Azure Pipelines

```yaml
- script: |
    cargo build --release
    ./target/release/get-version > $(Build.ArtifactStagingDirectory)/version.txt
  displayName: Generate version metadata
```

Tip: check the binary into your pipeline tool cache, or publish a release binary so jobs do not need a Rust toolchain.

## Behavior details

| Situation | Result |
|-----------|--------|
| Not inside a git repo | `no-repo` / `unknown` / empty tag / `unknown` |
| Detached HEAD | Branch becomes `HEAD` |
| No reachable tag | Tag is an empty string |
| Tag selection | Nearest ancestor tag of HEAD (like `git describe --tags`), not an arbitrary tag name |
| Commit date | Committer timestamp of HEAD, formatted in **UTC** |

## Development

```bash
cargo build          # debug
cargo build --release
cargo run --         # run with embedded template
cargo run -- write
cargo run -- -t template.txt
```

Release builds are size-optimized (`opt-level = "z"`, LTO, strip).

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
