use git_reduce::git_reduce;

const N: usize = 5;

fn main() -> anyhow::Result<()> {
    git_reduce(".", N)?;

    Ok(())
}
