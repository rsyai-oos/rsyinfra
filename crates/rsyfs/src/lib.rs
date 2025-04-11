//! rsyfs is a wrapper for rust file system
pub use dirs::{audio_dir, home_dir};
pub use std::fs;
pub use tempfile;
pub use walkdir::{DirEntry, DirEntryExt, Error, FilterEntry, IntoIter, WalkDir};

mod tempfile_tests {
    use tempfile::tempfile;

    #[test]
    fn it_works() {
        let tmp_file = tempfile::Builder::new()
            .prefix("prefix")
            .rand_bytes(5)
            .keep(false)
            // .tempfile_in("./")
            .tempdir()
            .unwrap();
        println!("{:?}", tmp_file);
    }
}
