use git_squash_range::git_squash_range;

use anyhow::anyhow;

use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = {
        let mut args = env::args();
        args.next().unwrap();
        args.collect()
    };

    if args.len() == 0 || args.len() >= 3 {
        return Err(anyhow!("usage: git-squash-range [<commit>] <commit>"));
    }

    let start =
        if args.len() == 1 {
            "ROOT"
        } else {
            args.first().unwrap()
        };

    let end = args.last().unwrap();

    git_squash_range(&start, &end)
}
