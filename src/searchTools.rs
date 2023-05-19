use crate::table_transposition::TranspositionTable;
use std::sync::Arc;
pub struct SearchTools {
    pub tt : TranspositionTable,
    pub timeover : Arc<bool>,
}

impl SearchTools {
    pub fn new(tt : TranspositionTable, timeover : Arc<bool>) -> SearchTools {
        SearchTools {
            tt,
            timeover
        }
    }
}