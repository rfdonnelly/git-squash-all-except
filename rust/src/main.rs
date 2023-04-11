use git_squash_range::git_squash_range;

use anyhow::anyhow;

#[cfg(feature = "git2")]
fn main() -> anyhow::Result<()> {
    use git2::Repository;

    let repo = Repository::discover(".")?;

    let root_oid = {
        let mut revwalk = repo.revwalk()?;
        revwalk.simplify_first_parent()?;
        revwalk.push_head()?;
        revwalk.last().ok_or(anyhow!("no commits"))??
    };
    let num_keep = 3;

    let start_oid = root_oid;
    let end_oid = repo.revparse_single(&format!("HEAD~{num_keep}"))?.id();
    eprintln!("{start_oid} {end_oid}");
    // git_squash_range(&repo, start_oid, end_oid)?;

    Ok(())
}

#[cfg(feature = "gix")]
fn main() -> anyhow::Result<()> {
    let repo = gix::discover(".")?;

    let root_id = {
        let head_id = repo.head_id()?;
        repo.rev_walk(Some(head_id))
            .first_parent_only()
            .all()?
            .last()
            .ok_or(anyhow!("no commits"))??
    };
    let num_keep = 3;

    let start_id = root_id;
    let end_id = repo.rev_parse_single(format!("HEAD~{num_keep}").as_bytes())?;
    eprintln!("{start_id} {end_id}");

    Ok(())
}
