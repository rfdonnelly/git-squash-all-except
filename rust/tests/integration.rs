use git2::{Commit, Repository};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempdir::TempDir;
use testresult::{TestError, TestResult};

#[test]
fn integration() -> TestResult {
    let tmpdir = TempDir::new("test-")?;
    env::set_current_dir(&tmpdir.path())?;

    let repo = Repository::init(tmpdir.path())?;

    for i in 0..9 {
        let is = i.to_string();
        let mut file = File::create(&is)?;
        writeln!(file, "{i}")?;
        git_add(&repo, &is)?;
        git_commit(&repo, &is)?;
    }

    duct::cmd!("git", "log", "--format=%an,%ae,%s%d").run()?;
    duct::cmd!("git", "status").run()?;

    git_reduce::git_reduce(".", 3)?;

    duct::cmd!("git", "log", "--format=%an,%ae,%s%d").run()?;
    duct::cmd!("git", "status").run()?;

    Ok(())
}

fn git_add<P: AsRef<Path>>(repo: &Repository, path: P) -> TestResult {
    let mut index = repo.index()?;
    index.add_path(path.as_ref())?;
    index.write()?;

    Ok(())
}

fn git_commit(repo: &Repository, message: &str) -> Result<git2::Oid, TestError> {
    let mut index = repo.index()?;
    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;
    let signature = repo.signature()?;
    let maybe_parents = [get_head_commit(&repo)?];
    let parents: Vec<&Commit> = maybe_parents.iter().flatten().collect();

    let commit_oid = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        parents.as_slice(),
    )?;

    Ok(commit_oid)
}

fn get_head_commit(repo: &Repository) -> Result<Option<Commit>, TestError> {
    let head = match repo.head() {
        Err(_) => None,
        Ok(head) => {
            let commit = head.peel_to_commit()?;
            Some(commit)
        }
    };
    Ok(head)
}
