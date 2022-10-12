/*
use crate::geometry::bh_tree::BHTree;
use log::info;
use rayon::prelude::*;
use sled::Db;

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

    pub fn persist(&self, time: f64, tree: BHTree) {
        info!("persisting tree state @ t={}", time);
        tree.points().par_iter().for_each(|p| {
            self.db
                .insert(time.to_be_bytes(), p.to_string().as_bytes())
                .unwrap();
        });
    }
}
*/
