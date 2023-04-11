
#[cfg(feature = "git2")]
mod git2_impl;
#[cfg(feature = "git2")]
pub use git2_impl::git_squash_range;

#[cfg(feature = "gix")]
mod gix_impl;
#[cfg(feature = "gix")]
pub use gix_impl::git_squash_range;
