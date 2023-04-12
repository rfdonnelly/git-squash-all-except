use anyhow::anyhow;
use git2::{AnnotatedCommit, Oid, Repository, ResetType};

pub fn git_squash_range(start: &str, end: &str) -> anyhow::Result<()> {
    let repo = Repository.discover(".")?;

    let start_oid =
        if start == "ROOT" {
            let mut revwalk = repo.revwalk()?;
            revwalk.simplify_first_parent()?;
            revwalk.push_head()?;
            revwalk.last().ok_or(anyhow!("no commits"))??
        } else {
            repo.revparse_single(start)?.id()
        };
    let end_oid = repo.revparse_single(end)?.id();

    let commits_to_squash = {
        let mut walk = repo.revwalk()?;
        walk.simplify_first_parent()?;
        walk.push(end_oid)?;
        walk.hide(start_oid)?;
        walk.count()
    };

    eprintln!("{start_oid} {end_oid} {commits_to_squash}");

    if commits_to_squash == 0 {
        return Err(anyhow!("nothing to squash"));
    }

    let head_oid = repo.head()?.peel_to_commit()?.id();

    // repo.set_head_detached(start_oid)?;
    // repo.checkout_head(None)?;
    let start_commit = repo.find_commit(start_oid)?;
    let target = start_commit.as_object();
    repo.reset(target, ResetType::Hard, None)?;

    // Merge
    let merge_opts = None;
    let checkout_opts = None;
    let annotated_commits: Vec<AnnotatedCommit> = [end_oid]
        .into_iter()
        .map(|oid| repo.find_annotated_commit(oid))
        .collect::<Result<Vec<_>, _>>()?;
    let annotated_commit_refs: Vec<&AnnotatedCommit> = annotated_commits.iter().collect();
    repo.merge(annotated_commit_refs.as_slice(), merge_opts, checkout_opts)?;

    // Amend start commit w/ merge result
    let mut index = repo.index()?;
    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;
    let start_commit = repo.find_commit(start_oid)?;
    let update_ref = Some("HEAD");
    let author = None;
    let committer = None;
    let message_encoding = None;
    let message = Some("Squashed");
    let tree = Some(&tree);
    start_commit.amend(
        update_ref,
        author,
        committer,
        message_encoding,
        message,
        tree,
    )?;
    repo.cleanup_state()?;

    // Cherrypick most recent N commits
    let preserve_commit_oids = {
        let mut walk = repo.revwalk()?;
        walk.simplify_first_parent()?;
        walk.push(head_oid)?;
        walk.hide(end_oid)?;
        walk.collect::<Result<Vec<_>, _>>()?
    };
    for oid in preserve_commit_oids.into_iter().rev() {
        let commit = repo.find_commit(oid)?;
        repo.cherrypick(&commit, None)?;

        let tree_oid = repo.index()?.write_tree()?;
        let tree = repo.find_tree(tree_oid)?;
        let update_ref = Some("HEAD");
        let author = commit.author();
        let committer = commit.committer();
        let message = commit.message().unwrap_or("");
        let parent = repo.head()?.peel_to_commit()?;
        let _commit_oid =
            repo.commit(update_ref, &author, &committer, message, &tree, &[&parent])?;
    }
    repo.cleanup_state()?;

    Ok(())
}

