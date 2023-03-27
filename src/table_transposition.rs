use std::mem;

static TABLE_SIZE : usize = 8 << 21;

pub struct TranspositionTable {
    pub table :  Box <[Transposition]>,
    pub stat_hint : i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Transposition {
    hash : u64,
    depth : i8,
    eval : i32,
    bestmove : u64,
}
impl Transposition {
    pub fn new(hash : u64, depth : i8, eval : i32, bestmove : u64) -> Transposition {
        Transposition {
            hash, depth, eval, bestmove
        }
    }
}

impl TranspositionTable {
    pub fn with_memory(memory : usize) -> TranspositionTable {
        let capacity = memory / mem::size_of::<Transposition>();
        TranspositionTable::with_capacity(capacity)
    }
    pub fn with_capacity(capacity : usize) -> TranspositionTable {
        TranspositionTable {
            table : unsafe { mem::transmute::<Box<[u128]>, Box<[Transposition]>>(
            vec![0u128; capacity].into_boxed_slice()
            )},
            stat_hint : 0,
        }
    }
    pub fn set(&mut self , hash : u64, depth : i8, eval : i32, bestmove : u64 ) {
        let t = Transposition::new(hash, depth, eval, bestmove);
        let k = (hash % self.table.len() as u64 ) as usize;
        self.table[k] = t;
    }
    pub fn get(&mut self, hash : u64) -> Transposition {
        self.table[(hash % self.table.len() as u64) as usize]
    }
}