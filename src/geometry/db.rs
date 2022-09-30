use crate::geometry::bh_tree::BHTree;
use log::{info, trace, warn};
use sled::{Db, Error};

pub struct DbHandle {
    db: Db,
}

impl DbHandle {
    fn new(path: String) -> Result<DbHandle, sled::Error> {
        info!("creating dbhandle with path {}", path);
        let dbhandle = DbHandle {
            db: sled::open(path)?,
        };
        info!("dbhandle recovered={}", dbhandle.db.was_recovered(),);

        return Ok(dbhandle);
    }

    fn persist(&mut self, time: f64, tree: BHTree) -> Result<(), sled::Error> {
        let result = self.db.insert(time.to_be_bytes(), vec![1, 2, 3]);
        info!(time = time.to_string(); "persisting tree state");
        return Ok(());
    }
}
