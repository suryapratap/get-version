use std::env;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 1. Parse CLI arguments for template path
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Missing template file path argument.");
        std::process::exit(1);
    }
    let template_path = &args[1];

    // 2. Read template file
    let template_content = match fs::read_to_string(template_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading template file '{}': {}", template_path, e);
            std::process::exit(1);
        }
    };

    // 3. Gather Git Metadata natively using gix
    let (branch, commit, tag) = match gix::discover(".") {
        Ok(repo) => {
            // Get Branch Name
            let branch_name = repo.head_name()
                .ok()
                .flatten()
                .map(|name| name.shorten().to_string())
                .unwrap_or_else(|| "HEAD".to_string());

            // Get Commit Hash (Short SHA, first 7 chars)
            let commit_sha = repo.head_id()
                .map(|id| id.to_string()[..7].to_string())
                .unwrap_or_else(|_| "unknown".to_string());

            // Get Latest Tag (fallback if no tag is found)
            let mut latest_tag = "no-tag".to_string();
            if let Ok(references) = repo.references() {
                if let Ok(tags) = references.tags() {
                    // Try to find a tag pointing to HEAD or the latest chronological tag
                    for tag_ref in tags.flatten() {
                        if let Some(name) = tag_ref.name().shorten().to_owned().to_string().into() {
                            latest_tag = name;
                            break; // Simplification: takes the first resolved tag
                        }
                    }
                }
            }
            (branch_name, commit_sha, latest_tag)
        }
        Err(_) => {
            ("no-repo".to_string(), "unknown".to_string(), "no-tag".to_string())
        }
    };

    // 4. Generate current UTC timestamp
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // 5. Replace placeholders
    let output = template_content
        .replace("[BRANCH]", &branch)
        .replace("[COMMIT]", &commit)
        .replace("[TAG]", &tag)
        .replace("[DATE]", &now.to_string());

    // 6. Write directly to stdout (stdio)
    print!("{}", output);
}