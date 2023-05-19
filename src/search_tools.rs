use crate::table_transposition::TranspositionTable;
use std::sync::{Arc, atomic::AtomicBool};
pub struct SearchTools {
    pub tt : TranspositionTable,
    pub timeover : Arc<AtomicBool>,
}

impl SearchTools {
    pub fn new(tt : TranspositionTable, timeover : Arc<AtomicBool>) -> SearchTools {
        SearchTools {
            tt,
            timeover
        }
    }
}