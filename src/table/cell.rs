use crate::types::{EnumTag, Type, TypeId, TypeMap};
use bincode::deserialize;
use std::cmp::{Ord, Ordering, PartialOrd};

pub struct Cell<'ts, 'tb> {
    type_id: TypeId,
    types: &'ts TypeMap,
    pub data: &'tb [u8],
}

impl<'ts, 'tb> Cell<'ts, 'tb> {
    pub fn new(type_id: TypeId, data: &'tb [u8], types: &'ts TypeMap) -> Self {
        Cell {
            type_id,
            types,
            data,
        }
    }
}

impl PartialEq for Cell<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        debug_assert_eq!(self.type_id, other.type_id);
        self.data == other.data
    }
}

impl PartialOrd for Cell<'_, '_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        debug_assert_eq!(self.type_id, other.type_id);

        match &self.types[&self.type_id] {
            Type::Integer => deserialize::<i32>(self.data)
                .unwrap()
                .partial_cmp(&deserialize(other.data).unwrap()),
            Type::Bool => deserialize::<bool>(self.data)
                .unwrap()
                .partial_cmp(&deserialize(other.data).unwrap()),
            Type::Double => deserialize::<f64>(self.data)
                .unwrap()
                .partial_cmp(&deserialize(other.data).unwrap()),
            Type::Sum(variants) => {
                let mut data1 = self.data;
                let mut data2 = other.data;

                let tag_size = std::mem::size_of::<EnumTag>();
                let tag1: EnumTag = deserialize(&data1[..tag_size]).unwrap();
                let tag2: EnumTag = deserialize(&data2[..tag_size]).unwrap();

                data1 = &data1[tag_size..];
                data2 = &data2[tag_size..];

                match tag1.cmp(&tag2) {
                    Ordering::Equal => {
                        let (_name, members) = &variants[tag1];
                        for &type_id in members {
                            let t = &self.types[&type_id];
                            let t_size = t.size_of(self.types);
                            let member_cell1 = Cell {
                                types: self.types,
                                type_id,
                                data: &data1[..t_size],
                            };
                            let member_cell2 = Cell {
                                types: self.types,
                                type_id,
                                data: &data2[..t_size],
                            };

                            match member_cell1.partial_cmp(&member_cell2) {
                                Some(Ordering::Equal) => continue,
                                not_equal => return not_equal,
                            }
                        }
                        Some(Ordering::Equal)
                    }
                    not_equal => Some(not_equal),
                }
            }
        }
    }
}
