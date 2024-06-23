use serde_derive::Deserialize;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

struct Token {
    kind: String,
}

struct TokenInfo {
    file: PathBuf,
    tokens: Vec<Token>,
    id: usize,
}

#[derive(Deserialize)]
struct Repository {
    name: String,
    url: String,
    branch: Option<String>,
}

#[derive(Deserialize)]
struct Manifest {
    repositories: Vec<Repository>,
}

fn main() {
    // Read the manifest file
    // In the end, the manifest file will not exist in the Rust project
    // ...instead, it will live in the central swa project/repo (I think)
    let manifest_content =
        fs::read_to_string("./manifest.yaml").expect("Failed to read manifest.yml");
    let manifest: Manifest =
        serde_yaml::from_str(&manifest_content).expect("Failed to parse manifest.yml");

    // Checkout each repository
    for repo in &manifest.repositories {
        let branch = repo.branch.clone().unwrap_or_else(|| "main".to_string());
        println!("Checking out {} from {}...", repo.name, repo.url);

        // Create directory for the repository
        fs::create_dir_all(format!("src/{}", repo.name)).expect("Failed to create directory");

        // Now, create directory for documentation -- this will be moved closer to doc generation invocation
        // fs::create_dir_all(path)

        // Clone the repository
        let status = Command::new("git")
            .args(&[
                "clone",
                "--branch",
                &branch,
                "--single-branch",
                &repo.url,
                &format!("src/{}", repo.name),
            ])
            .status()
            .expect("Failed to execute git clone");

        if !status.success() {
            eprintln!("Failed to checkout {}", repo.name);
        }
    }

    // Combine repositories
    combine_repositories(&manifest).expect("Failed to combine repositories");

    // Update SUMMARY.md
    update_summary().expect("Failed to update SUMMARY.md");
}

fn combine_repositories(manifest: &Manifest) -> std::io::Result<()> {
    let book_src_dir = Path::new("book/src");
    fs::create_dir_all(book_src_dir)?;

    for repo in &manifest.repositories {
        let formatted_dir_path = &format!("src/{}/src", repo.name);
        let repo_src_dir = Path::new(formatted_dir_path);
        match repo_src_dir.exists() {
            true => {
                for entry in fs::read_dir(repo_src_dir)? {
                    let entry = entry?;
                    let dest_path = book_src_dir.join(entry.file_name());
                    if entry.file_type()?.is_file() {
                        fs::copy(entry.path(), dest_path)?;
                    } else if entry.file_type()?.is_dir() {
                        fs::create_dir_all(&dest_path)?;
                        copy_dir_all(entry.path(), dest_path)?;
                    }
                }
            }
            false => (),
        }
    }

    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dst.as_ref().join(entry.file_name());

        if entry.file_type()?.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir_all(entry_path, dest_path)?;
        } else {
            fs::copy(entry_path, dest_path)?;
        }
    }
    Ok(())
}

fn update_summary() -> io::Result<()> {
    let summary_path = "book/src/SUMMARY.md";
    let mut summary_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(summary_path)?;

    for entry in fs::read_dir("src")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let repo_name = entry.file_name().into_string().unwrap();
            let repo_summary_path = format!("src/{}/SUMMARY.md", repo_name);

            if fs::metadata(&repo_summary_path).is_ok() {
                writeln!(summary_file, "\n# {}\n", capitalize(&repo_name))?;
                let mut repo_summary_file = fs::File::open(&repo_summary_path)?;
                let mut contents = String::new();
                repo_summary_file.read_to_string(&mut contents)?;
                writeln!(summary_file, "{}", adjust_paths(&repo_name, &contents))?;
            }
        }
    }

    Ok(())
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn adjust_paths(repo_name: &str, contents: &str) -> String {
    contents
        .lines()
        .map(|line| {
            if line.trim().starts_with("-") && line.contains("[") && line.contains("](") {
                let mut parts = line.split("](");
                let text_part = parts.next().unwrap_or("");
                let path_part = parts.next().unwrap_or("").strip_suffix(")").unwrap_or("");
                format!("{}]({}/{})", text_part, repo_name, path_part)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}
