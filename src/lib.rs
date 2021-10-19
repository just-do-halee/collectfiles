// Copyright 2021 Hwakyeom Kim(=just-do-halee)

//! `collectfiles`
//! ## Example
//! ```ignore
//! use collectfiles::*;
//!
//! let vec = CollectFiles("/Users/hwakyeom/programs/")
//!         .with_depth(1)
//!         .with_target_regex(".md$")
//!         .with_hook(|path| path.with_extension("mutated"))
//!         .collect();
//!
//! println!("{:#?}", vec);
//! ```

use std::{
    fs,
    path::{Path, PathBuf},
};

use rayon::prelude::*;
use regex::Regex;

pub trait CollectFilesPrelude {
    fn as_root_dir(&self) -> &Path;
    fn as_target_regex(&self) -> Option<&str>;
    fn as_hook(&self) -> Option<fn(PathBuf) -> PathBuf>;
    fn as_depth(&self) -> Option<usize>;
    fn with_hook(self, hook_fn: fn(PathBuf) -> PathBuf) -> CollectFilesConfigured;
    fn with_depth(self, level: usize) -> CollectFilesConfigured;
    fn with_target_regex(self, regex: &str) -> CollectFilesConfigured;
    fn collect(&self) -> Vec<PathBuf>;
}
use private::*;
pub mod private {
    use super::*;
    #[derive(Debug, Default, Clone)]
    pub struct CollectFilesConfigured {
        root_dir: PathBuf,
        depth: Option<usize>,
        hook_fn: Option<fn(PathBuf) -> PathBuf>,
        target_regex: Option<Regex>,
    }
    impl CollectFilesConfigured {
        pub fn new(root_dir: PathBuf) -> Self {
            Self {
                root_dir,
                ..Default::default()
            }
        }
    }

    impl CollectFilesPrelude for CollectFilesConfigured {
        #[inline]
        fn as_target_regex(&self) -> Option<&str> {
            if let Some(regex) = &self.target_regex {
                Some(regex.as_str())
            } else {
                None
            }
        }
        #[inline]
        fn as_root_dir(&self) -> &Path {
            self.root_dir.as_ref()
        }
        #[inline]
        fn as_hook(&self) -> Option<fn(PathBuf) -> PathBuf> {
            self.hook_fn
        }
        #[inline]
        fn as_depth(&self) -> Option<usize> {
            self.depth
        }
        #[inline]
        fn with_hook(mut self, hook_fn: fn(PathBuf) -> PathBuf) -> Self {
            self.hook_fn = Some(hook_fn);
            self
        }
        #[inline]
        fn with_depth(mut self, level: usize) -> Self {
            self.depth = Some(level);
            self
        }
        #[inline]
        fn with_target_regex(mut self, regex: &str) -> CollectFilesConfigured {
            self.target_regex = Some(
                Regex::new(regex).unwrap_or_else(|_| panic!("* Regular Expression: {}", regex)),
            );
            self
        }
        #[inline]
        fn collect(&self) -> Vec<PathBuf> {
            collect_files(
                self.root_dir.clone(),
                self.depth,
                self.hook_fn,
                self.target_regex.clone(),
            )
        }
    }
}

/// CollectFiles(`entry_dir`)
///
/// ## Example
/// ```ignore
/// use collectfiles::*;
///
/// let vec = CollectFiles("/Users/hwakyeom/programs/")
///         .with_depth(1)
///         .with_target_regex(".md$")
///         .with_hook(|path| path.with_extension("mutated"))
///         .collect();
///
/// println!("{:#?}", vec);
/// ```
#[derive(Debug)]
pub struct CollectFiles<T>(pub T)
where
    T: AsRef<Path> + Clone + Send + Sync;

impl<T> CollectFiles<T>
where
    T: AsRef<Path> + Clone + Send + Sync,
{
    fn clone(&self) -> CollectFilesConfigured {
        CollectFilesConfigured::new(self.0.as_ref().to_path_buf())
    }
}

impl<T> CollectFilesPrelude for CollectFiles<T>
where
    T: AsRef<Path> + Clone + Send + Sync,
{
    #[inline]
    fn as_target_regex(&self) -> Option<&str> {
        None
    }
    #[inline]
    fn as_root_dir(&self) -> &Path {
        self.0.as_ref()
    }
    #[inline]
    fn as_hook(&self) -> Option<fn(PathBuf) -> PathBuf> {
        None
    }
    #[inline]
    fn as_depth(&self) -> Option<usize> {
        None
    }
    #[inline]
    fn with_hook(self, hook_fn: fn(PathBuf) -> PathBuf) -> CollectFilesConfigured {
        self.clone().with_hook(hook_fn)
    }
    #[inline]
    fn with_depth(self, level: usize) -> CollectFilesConfigured {
        self.clone().with_depth(level)
    }
    #[inline]
    fn with_target_regex(self, regex: &str) -> CollectFilesConfigured {
        self.clone().with_target_regex(regex)
    }
    #[inline]
    fn collect(&self) -> Vec<PathBuf> {
        collect_files(self.0.as_ref().to_path_buf(), None, None, None)
    }
}
#[inline]
fn collect_files(
    dir_path: PathBuf,
    depth: Option<usize>,
    hook_fn: Option<fn(PathBuf) -> PathBuf>,
    target_regex: Option<Regex>,
) -> Vec<PathBuf> {
    let paths = fs::read_dir(dir_path).unwrap().par_bridge();

    paths
        .flat_map(|p| {
            let path = p.unwrap().path();
            if path.is_dir() {
                match depth {
                    Some(dep) if dep > 0 => {
                        collect_files(path, Some(dep - 1), hook_fn, target_regex.clone())
                    }
                    Some(_) => vec![PathBuf::default()],
                    None => collect_files(path, depth, hook_fn, target_regex.clone()),
                }
            } else {
                match &target_regex {
                    Some(r)
                        if r.is_match(path.to_str().unwrap_or_else(|| {
                            panic!("* not a valid unicode extension: {}", path.display())
                        })) =>
                    {
                        if let Some(hook) = hook_fn {
                            vec![hook(path)]
                        } else {
                            vec![path]
                        }
                    }
                    Some(_) => vec![PathBuf::default()],
                    None => vec![path],
                }
            }
        })
        .filter(|p| p.as_os_str() != "")
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let c = CollectFiles("/Users/hwakyeom/programs/")
            .with_depth(1)
            .with_target_regex(".md$")
            .with_hook(|path| path.with_extension("mutated"));
        println!("{:#?}", c.collect());
        println!(
            "entry: {:?}\ndepth: {:?}\nhook: {:?}\n regex: {:?}",
            c.as_root_dir(),
            c.as_depth(),
            c.as_hook(),
            c.as_target_regex()
        )
    }
}
