use clap::{Parser, Subcommand};
use git_core as git;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "foos")]
#[command(about = "A Git-like CLI tool powered by git-core", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to the repositories directory
    #[arg(short, long, default_value = "./repos")]
    repo_dir: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new repository
    Init {
        /// Name of the repository
        name: String,
    },

    /// Clone a repository
    Clone {
        /// Source repository path
        source: String,
        /// Name for the new repository
        name: Option<String>,
    },

    /// List all commits in a branch
    Log {
        /// Repository name
        repo: String,
        /// Branch name
        #[arg(default_value = "master")]
        branch: String,
        /// Maximum number of commits
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Show commit information
    Show {
        /// Repository name
        repo: String,
        /// Commit hash
        hash: String,
    },

    /// List branches
    Branch {
        /// Repository name
        repo: String,
        /// Create a new branch
        #[arg(short, long)]
        create: Option<String>,
        /// Delete a branch
        #[arg(short, long)]
        delete: Option<String>,
    },

    /// Show diff between two branches/commits
    Diff {
        /// Repository name
        repo: String,
        /// Base branch/commit
        base: String,
        /// Head branch/commit
        head: String,
    },

    /// List repositories
    Repos,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => match git::init_repo(&cli.repo_dir, &name) {
            Ok(info) => println!("Initialized empty repository: {}", info.name),
            Err(e) => eprintln!("Error: {}", e),
        },

        Commands::Clone { source, name } => {
            let dest_name = name.unwrap_or_else(|| {
                std::path::Path::new(&source)
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "repo".to_string())
            });
            match git::init_repo(&cli.repo_dir, &dest_name) {
                Ok(info) => println!("Cloned repository: {}", info.name),
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        Commands::Log {
            repo,
            branch,
            limit,
        } => match git::log(&cli.repo_dir, &repo, &branch, limit) {
            Ok(commits) => {
                for commit in commits {
                    println!(
                        "{} - {}",
                        &commit.hash[..7],
                        commit.message.lines().next().unwrap_or("")
                    );
                    println!("    Author: {} <{}>", commit.author.name, commit.author.email);
                    println!();
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },

        Commands::Show { repo, hash } => match git::get_commit(&cli.repo_dir, &repo, &hash) {
            Ok(commit) => {
                println!("commit {}", commit.hash);
                println!(
                    "Author: {} <{}>",
                    commit.author.name, commit.author.email
                );
                println!("Date:   {}", commit.author.timestamp);
                println!();
                println!("{}", commit.message);
            }
            Err(e) => eprintln!("Error: {}", e),
        },

        Commands::Branch {
            repo,
            create,
            delete,
        } => {
            if let Some(name) = create {
                match git::log(&cli.repo_dir, &repo, "master", 1) {
                    Ok(commits) => {
                        if let Some(commit) = commits.first() {
                            match git::create_branch(&cli.repo_dir, &repo, &name, &commit.hash) {
                                Ok(branch) => println!("Created branch: {}", branch.name),
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        } else {
                            eprintln!("No commits found");
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else if let Some(name) = delete {
                match git::delete_branch(&cli.repo_dir, &repo, &name) {
                    Ok(_) => println!("Deleted branch: {}", name),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                match git::list_branches(&cli.repo_dir, &repo) {
                    Ok(branches) => {
                        for branch in branches {
                            let prefix = if branch.is_head { "*" } else { " " };
                            println!("{} {}", prefix, branch.name);
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }

        Commands::Diff {
            repo,
            base,
            head,
        } => match git::diff_branch_from_base(&cli.repo_dir, &repo, &base, &head, true) {
            Ok(diff) => {
                println!("Files changed: {}", diff.files_changed.len());
                println!("Insertions: {}", diff.insertions);
                println!("Deletions: {}", diff.deletions);
                println!();
                for file in diff.files_changed {
                    println!(
                        "{}: {} -> {}",
                        format!("{:?}", file.status),
                        file.old_path.as_deref().unwrap_or("/dev/null"),
                        file.new_path.as_deref().unwrap_or("/dev/null")
                    );
                    if let Some(patch) = file.patch {
                        println!("{}", patch);
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },

        Commands::Repos => match git::list_repos(&cli.repo_dir) {
            Ok(repos) => {
                for repo in repos {
                    println!("{}", repo.name);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
    }
}
