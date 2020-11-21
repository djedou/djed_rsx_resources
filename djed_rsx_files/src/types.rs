
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::fs;
use std::hash::Hasher;
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;
use std::any::Any;

use fnv::{FnvHashMap, FnvHasher};
use djed_rsx_shared::traits::file_traits::TFileCache;

use error::{FileError, Result};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(u64);

impl FileId {
    fn new<P>(src: P) -> Result<Self>
    where
        P: AsRef<Path>
    {
        let mut hasher = FnvHasher::default();
        hasher.write(fs::canonicalize(src)?.to_str().unwrap().as_bytes());
        Ok(FileId(hasher.finish()))
    }
}

#[derive(Debug, PartialEq)]
pub struct SharedFiles(Rc<RefCell<FileCache>>);

impl From<FileCache> for SharedFiles {
    fn from(value: FileCache) -> Self {
        SharedFiles(Rc::new(RefCell::new(value)))
    }
}

impl Clone for SharedFiles {
    fn clone(&self) -> Self {
        SharedFiles(Rc::clone(&self.0))
    }
}

impl Deref for SharedFiles {
    type Target = RefCell<FileCache>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TFileCache for SharedFiles {
    type File = Rc<Vec<u8>>;
    type ResourceUpdates = Box<dyn Any>;

    fn add_file<P>(&mut self, src: P) -> Option<()>
    where
        P: AsRef<Path>
    {
        self.borrow_mut().add_file(src).ok()
    }

    fn get_file<P>(&self, src: P) -> Option<Self::File>
    where
        P: AsRef<Path>
    {
        self.borrow().get_file(src).ok()
    }

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates {
        unreachable!()
    }
}

#[derive(Debug, PartialEq)]
pub struct FileCache {
    files: FnvHashMap<FileId, Rc<Vec<u8>>>
}

impl FileCache {
    pub fn new() -> Result<Self> {
        Ok(FileCache {
            files: FnvHashMap::default()
        })
    }

    pub fn add_file<P>(&mut self, src: P) -> Result<()>
    where
        P: AsRef<Path>
    {
        match self.files.entry(FileId::new(&src)?) {
            Entry::Occupied(_) => {
                Err(FileError::FileAlreadyAdded)?;
            }
            Entry::Vacant(e) => {
                let bytes = super::util::load_bytes(src)?;
                e.insert(Rc::new(bytes));
            }
        }

        Ok(())
    }

    pub fn get_file<P>(&self, src: P) -> Result<Rc<Vec<u8>>>
    where
        P: AsRef<Path>
    {
        self.files
            .get(&FileId::new(src)?)
            .ok_or(FileError::FileNotFound)
            .map(Rc::clone)
    }
}
