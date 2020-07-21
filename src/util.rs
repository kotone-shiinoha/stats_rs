use std::collections::BTreeMap;
use std::cmp::Ordering;

use crate::{
    Input,
    Num
};


pub mod sort {
    use super::*;

    pub fn partial_ord(input: &Input) -> Vec<Num>{
        let mut map = BTreeMap::new();
        input
            .iter()
            .for_each(|item| {
                let ord_f = OrdFloat{
                    item: *item
                };
                map.entry(ord_f).or_insert(0);
                *map.get_mut(&ord_f).unwrap() += 1;
            });
        
        let mut vec = Vec::with_capacity(input.len());
    
        map.iter().for_each(|(key, v)| {
            let range = 0..*v;
            for _i in range {
                vec.push(key.item)
            }
        });
    
        vec
    }
}


#[derive(Clone, Copy)]
pub(crate)struct OrdFloat {
    item: Num
}

impl PartialOrd for OrdFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.item.partial_cmp(&other.item)
    }
}

impl Ord for OrdFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.item.partial_cmp(&other.item) {
            Some(e) => e,
            None => {
                let check = other.item.is_normal() || other.item == 0 as Num;
                let (label, value) = if check {
                    ("other", other.item)
                } else {
                    ("self", self.item)
                };
                panic!("ordering error with {}: {}", label, &value);
            }
        }
    }
}


impl Eq for OrdFloat {}

impl PartialEq for OrdFloat {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}