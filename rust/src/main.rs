use git_squash_range::git_squash_range;

use anyhow::anyhow;
use git2::Repository;

fn main() -> anyhow::Result<()> {
    let repo = Repository::open(".")?;

    let root_oid = {
        let mut revwalk = repo.revwalk()?;
        revwalk.simplify_first_parent()?;
        revwalk.push_head()?;
        revwalk.last().ok_or(anyhow!("no commits"))??
    };
    let num_keep = 3;

    let start_oid = root_oid;
    let end_oid = repo.revparse_single(&format!("HEAD~{num_keep}"))?.id();
    git_squash_range(&repo, start_oid, end_oid)?;

    Ok(())
}
