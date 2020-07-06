use std::path::Path;

extern crate pacman_repo_tools as prt;
extern crate slice_tracker;

use prt::srcinfo::parse_srcinfo_dir;

use slice_tracker::{SliceTracker, Source};
type SourceTracker<'a> = SliceTracker<String, Source<str>>;

fn main() {
	let args: Vec<_> = std::env::args().collect();

	let pool = SourceTracker::default();

	for (name, (path, package)) in parse_srcinfo_dir(&pool, Path::new(&args[1])).unwrap().into_iter() {
		println!("{}: {}-{}", path.display(), name, package.version());
	}
}