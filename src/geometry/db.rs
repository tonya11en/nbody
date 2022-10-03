use crate::geometry::bh_tree::BHTree;
use log::{info, trace, warn};
use sled::{Db, Error};

pub struct DbHandle {
    db: Db,
}

impl DbHandle {
    pub fn new(path: String) -> Result<DbHandle, sled::Error> {
        // TODO: Allow it to pick up where it left off somehow.
        info!("creating dbhandle with path {}", path);
        let dbhandle = DbHandle {
            db: sled::open(path)?,
        };
        info!("dbhandle recovered={}", dbhandle.db.was_recovered(),);

        return Ok(dbhandle);
    }

    pub fn persist(&mut self, time: f64, tree: &BHTree) -> Result<Option<sled::IVec>, sled::Error> {
        info!("persisting tree state @ t={}", time);
        let bhts = serde_json::to_vec(&tree).unwrap();
        return self.db.insert(time.to_be_bytes(), bhts);
    }
}
