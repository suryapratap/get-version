use std::env;
use std::fs;
use std::process;

/// Default template embedded into the binary at compile time.
const DEFAULT_TEMPLATE: &str = include_str!("../template.txt");

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match parse_args(&args) {
        Ok(Command::WriteTemplate) => {
            // Emit embedded template to stdout so callers can pipe or redirect.
            print!("{}", DEFAULT_TEMPLATE);
        }
        Ok(Command::Version { template_path }) => {
            let template_content = load_template(template_path.as_deref());
            print!("{}", render_version(&template_content));
        }
        Ok(Command::Help) => {
            print_usage();
        }
        Err(msg) => {
            eprintln!("Error: {}", msg);
            print_usage();
            process::exit(1);
        }
    }
}

enum Command {
    /// Fill placeholders and print version info.
    Version {
        template_path: Option<String>,
    },
    /// Dump the embedded default template to stdout.
    WriteTemplate,
    Help,
}

fn parse_args(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Ok(Command::Version {
            template_path: None,
        });
    }

    let mut template_path: Option<String> = None;
    let mut write = false;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => return Ok(Command::Help),
            "write" => {
                if write {
                    return Err("duplicate 'write' command".into());
                }
                write = true;
                i += 1;
            }
            "-t" | "--template" => {
                let path = args
                    .get(i + 1)
                    .ok_or_else(|| "option -t requires a template file path".to_string())?;
                if template_path.is_some() {
                    return Err("template path specified more than once".into());
                }
                template_path = Some(path.clone());
                i += 2;
            }
            other if other.starts_with('-') => {
                return Err(format!("unknown option '{}'", other));
            }
            other => {
                return Err(format!(
                    "unexpected argument '{}'; use -t <path> for a template",
                    other
                ));
            }
        }
    }

    if write {
        if template_path.is_some() {
            return Err("'write' does not take -t; it always emits the embedded template".into());
        }
        return Ok(Command::WriteTemplate);
    }

    Ok(Command::Version { template_path })
}

fn load_template(path: Option<&str>) -> String {
    match path {
        Some(template_path) => match fs::read_to_string(template_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading template file '{}': {}", template_path, e);
                process::exit(1);
            }
        },
        None => DEFAULT_TEMPLATE.to_string(),
    }
}

fn render_version(template_content: &str) -> String {
    let (branch, commit, tag, date) = match gix::discover(".") {
        Ok(repo) => {
            let branch_name = repo
                .head_name()
                .ok()
                .flatten()
                .map(|name| name.shorten().to_string())
                .unwrap_or_else(|| "HEAD".to_string());

            let commit_sha = repo
                .head_id()
                .map(|id| id.to_string()[..7].to_string())
                .unwrap_or_else(|_| "unknown".to_string());

            // Commit date = committer time of HEAD (UTC).
            let date = repo
                .head_commit()
                .ok()
                .and_then(|c| c.time().ok())
                .map(|t| format_utc_secs(t.seconds))
                .unwrap_or_else(|| "unknown".to_string());

            let mut latest_tag = String::new();
            if let Ok(references) = repo.references() {
                if let Ok(tags) = references.tags() {
                    for tag_ref in tags.flatten() {
                        if let Some(name) = tag_ref.name().shorten().to_owned().to_string().into() {
                            latest_tag = name;
                            break;
                        }
                    }
                }
            }
            (branch_name, commit_sha, latest_tag, date)
        }
        Err(_) => (
            "no-repo".to_string(),
            "unknown".to_string(),
            String::new(),
            "unknown".to_string(),
        ),
    };

    template_content
        .replace("[BRANCH]", &branch)
        .replace("[COMMIT]", &commit)
        .replace("[TAG]", &tag)
        .replace("[DATE]", &date)
}

/// Format Unix epoch seconds as `YYYY-MM-DD HH:mm` in UTC.
fn format_utc_secs(secs: i64) -> String {
    // Handle negative timestamps (pre-1970) by clamping to epoch.
    let secs = secs.max(0) as u64;

    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    let hours = rem / 3_600;
    let minutes = (rem % 3_600) / 60;

    // Civil date from days since Unix epoch (Howard Hinnant algorithm).
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    format!("{:04}-{:02}-{:02} {:02}:{:02}", y, m, d, hours, minutes)
}

fn print_usage() {
    eprintln!(
        "\
Usage:
  get-version                 Print version info using the embedded template
  get-version -t <file>       Print version info using a custom template file
  get-version write           Write the embedded template to stdout

Options:
  -t, --template <file>   Template file path (default: embedded template)
  -h, --help              Show this help

Examples:
  get-version
  get-version -t ./my-template.txt
  get-version write > template.txt
  get-version write | Out-File -Encoding utf8 template.txt"
    );
}
