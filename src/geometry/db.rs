use crate::geometry::bh_tree::BHTree;
use log::{info, trace, warn};
use sled::{Db, Error};

pub struct DbHandle {
    db: Db,
}

impl DbHandle {
    fn new(path: String) -> Result<DbHandle, sled::Error> {
        // TODO: Allow it to pick up where it left off somehow.
        info!("creating dbhandle with path {}", path);
        let dbhandle = DbHandle {
            db: sled::open(path)?,
        };
        info!("dbhandle recovered={}", dbhandle.db.was_recovered(),);

        return Ok(dbhandle);
    }

    fn persist(&mut self, time: f64, tree: BHTree) -> Result<Option<sled::IVec>, sled::Error> {
        info!(time = time.to_string(); "persisting tree state");
        let bhts = serde_json::to_vec(&tree).unwrap();
        return self.db.insert(time.to_be_bytes(), bhts);
    }
}
