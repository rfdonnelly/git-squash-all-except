use tempdir::TempDir;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use git2::{Commit, Repository};

#[test]
fn integration() {
    let tmpdir = TempDir::new("test-").unwrap();
    env::set_current_dir(&tmpdir.path()).unwrap();

    let repo = Repository::init(tmpdir.path()).unwrap();

    for i in 0..9 {
        let is = i.to_string();
        let mut file = File::create(&is)
            .unwrap();
        writeln!(file, "{i}")
            .unwrap();
        git_add(&repo, &is);
        git_commit(&repo, &is);
    }

    duct::cmd!("git", "log", "--format=%an,%ae,%s%d").run().unwrap();
    duct::cmd!("git", "status").run().unwrap();

    git_reduce::git_reduce(".", 3)
        .unwrap();

    duct::cmd!("git", "log", "--format=%an,%ae,%s%d").run().unwrap();
    duct::cmd!("git", "status").run().unwrap();
}

fn git_add<P: AsRef<Path>>(repo: &Repository, path: P) {
    let mut index = repo.index().unwrap();
    index.add_path(path.as_ref()).unwrap();
    index.write().unwrap();
}

fn git_commit(repo: &Repository, message: &str) -> git2::Oid {
    let mut index = repo.index().unwrap();
    let tree_oid = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_oid).unwrap();
    let signature = repo.signature().unwrap();
    let maybe_parents = [get_head_commit(&repo)];
    let parents: Vec<&Commit> = maybe_parents.iter().flatten().collect();

    let commit_oid = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        parents.as_slice(),
    ).unwrap();

    commit_oid
}

fn get_head_commit(repo: &Repository) -> Option<Commit> {
    match repo.head() {
        Err(_) => None,
        Ok(head) => {
            let commit = head
                .peel_to_commit()
                .unwrap();
            Some(commit)
        }
    }
}
