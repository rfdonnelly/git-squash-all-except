use anyhow::anyhow;

pub fn git_squash_range(start: &str, end: &str) -> anyhow::Result<()> {
    let repo = gix::discover(".")?;

    let start_id =
        if start == "ROOT" {
            let head_id = repo.head_id()?;
            repo.rev_walk(Some(head_id))
                .first_parent_only()
                .all()?
                .last()
                .ok_or(anyhow!("no commits"))??
        } else {
            repo.rev_parse_single(start.as_bytes())?
        };
    let end_id = repo.rev_parse_single(end.as_bytes())?;

    let commits_to_squash = repo
        .rev_walk(Some(end_id))
        .first_parent_only()
        .all()?
        .take_while(|id| id.as_ref().unwrap_or(&start_id) != &start_id)
        .count();

    eprintln!("{start_id} {end_id} {commits_to_squash}");

    if commits_to_squash == 0 {
        return Err(anyhow!("nothing to squash"));
    }

    Ok(())
}
