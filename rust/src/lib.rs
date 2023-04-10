use std::path::Path;

use git2::{
    AnnotatedCommit,
    Repository,
    ResetType,
};
use anyhow::anyhow;

pub fn git_reduce<P: AsRef<Path>>(path: P, num_keep: usize) -> anyhow::Result<()> {
    git_reduce_path(path.as_ref(), num_keep)
}

fn git_reduce_path(path: &Path, num_keep: usize) -> anyhow::Result<()> {
    let repo = Repository::open(path)?;

    let root_oid = {
        let mut walk = repo.revwalk()?;
        walk.simplify_first_parent()?;
        walk.push_head()?;
        walk.last().ok_or(anyhow!("no root"))??
    };

    let total_commits = {
        let mut walk = repo.revwalk()?;
        walk.simplify_first_parent()?;
        walk.push_head()?;
        walk.count()
    };

    if num_keep > total_commits {
        println!("Preserving all commits");
    } else {
        let mid_oid = repo.revparse_single(&format!("HEAD~{num_keep}"))?.id();
        let head_oid = repo.head()?.peel_to_commit()?.id();

        // repo.set_head_detached(root_oid)?;
        // repo.checkout_head(None)?;
        let root_commit = repo.find_commit(root_oid)?;
        let target = root_commit.as_object();
        repo.reset(
            target,
            ResetType::Hard,
            None,
        )?;

        // Merge
        let merge_opts = None;
        let checkout_opts = None;
        let annotated_commits: Vec<AnnotatedCommit> = [mid_oid]
            .into_iter()
            .map(|oid| repo.find_annotated_commit(oid))
            .collect::<Result<Vec<_>, _>>()?;
        let annotated_commit_refs: Vec<&AnnotatedCommit> = annotated_commits.iter().collect();
        repo.merge(
            annotated_commit_refs.as_slice(),
            merge_opts,
            checkout_opts,
        )?;

        // Amend root commit w/ merge result
        let mut index = repo.index()?;
        let tree_oid = index.write_tree()?;
        let tree = repo.find_tree(tree_oid)?;
        let root_commit = repo.find_commit(root_oid)?;
        let update_ref = Some("HEAD");
        let author = None;
        let committer = None;
        let message_encoding = None;
        let message = Some("Squashed");
        let tree = Some(&tree);
        root_commit.amend(
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
            walk.hide(mid_oid)?;
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
            let _commit_oid = repo.commit(
                update_ref,
                &author,
                &committer,
                message,
                &tree,
                &[&parent],
            )?;
        }
        repo.cleanup_state()?;
    }

    Ok(())
}
