use std::mem;

static _TABLE_SIZE : usize = 8 << 21;

pub struct TranspositionTable {
    pub table :  Box <[Transposition]>,
    pub stat_hint : i32,
    pub mask : usize,
}
#[derive(Debug, Clone, Copy)]
pub enum NodeType {
    PV,
    CUT,
    ALL,
}
#[derive(Debug, Clone, Copy)]
pub struct Transposition {
    pub hash : u64,
    pub depth : u8,
    pub eval : i32,
    pub bestmove : u64,
    pub node_type : NodeType
}
impl Transposition {
    pub fn new(hash : u64, depth : u8, eval : i32, bestmove : u64, node_type : NodeType) -> Transposition {
        Transposition {
            hash, depth, eval, bestmove, node_type,
        }
    }
}

impl TranspositionTable {
    pub fn with_memory(memory : usize) -> TranspositionTable {
        let capacity = memory / mem::size_of::<Transposition>();
        TranspositionTable::with_capacity(capacity)
    }
    pub fn with_capacity(capacity : usize) -> TranspositionTable {
        let n = if capacity.is_power_of_two() {
            capacity
        } else {
            capacity.next_power_of_two()
        };
        TranspositionTable {
            /*table : unsafe { mem::transmute::<Box<[u128]>, Box<[Transposition]>>(
            vec![0u128; n].into_boxed_slice()
            )},*/
            table : { vec![Transposition::new(0,0,0,0,NodeType::PV); n].into_boxed_slice()
            },
            stat_hint : 0,
            mask : n-1
        }
    }
    pub fn set(&mut self , hash : u64, depth : u8, eval : i32, bestmove : u64, node_type: NodeType) {
        let t = Transposition::new(hash, depth, eval, bestmove, node_type);
        let k = hash as usize % self.table.len();
        self.table[k] = t;
    }
    pub fn _set_tt(&mut self, tt : Transposition) {
        let k = tt.hash as usize % self.table.len();
        //let k = tt.hash as usize & self.mask;
        self.table[k] = tt;
    }
    pub fn get(&mut self, hash : u64) -> &Transposition {
        //let a = &self.table[hash as usize & self.mask];
        //println!("{:#034b}", self.mask);
        let a = &self.table[hash as usize % self.table.len()];
        if hash == a.hash && a.bestmove != 0 {
            self.stat_hint +=1;
        }
        a
    }
}