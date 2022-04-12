use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

pub struct DoubleMapping<N> {
    to_index: HashMap<Rc<N>, usize>,
    from_index: Vec<Option<Rc<N>>>,
}

impl<N: Hash + Eq + Debug> DoubleMapping<N> {
    pub fn new() -> DoubleMapping<N> {
        DoubleMapping {
            to_index: HashMap::new(),
            from_index: Vec::with_capacity(50),
        }
    }

    pub fn contains_id(&self, id: usize) -> bool {
        self.get_by_id(id).is_some()
    }

    pub fn contains_obj(&self, obj: &N) -> bool {
        self.get_by_obj(obj).is_some()
    }

    pub fn insert(&mut self, obj: N) -> Option<usize> {
        match self.get_by_obj(&obj) {
            Some(_) => None,
            None => {
                let id = self.from_index.len() + 1;
                let obj = Rc::new(obj);
                self.to_index.insert(obj.clone(), id);
                self.from_index.push(Some(obj));
                Some(id)
            }
        }
    }

    pub fn remove(&mut self, id: usize) -> Option<N> {
        if self.contains_id(id) {
            let id = id - 1;
            let obj = self.from_index[id].take();
            if let Some(obj) = obj {
                self.to_index.remove(&obj);
                self.from_index[id] = None;
                return Some(Rc::try_unwrap(obj).unwrap());
            }
        }
        None
    }

    pub fn get_by_id(&self, id: usize) -> Option<&N> {
        self.from_index
            .get(id - 1)
            .and_then(|v| v.as_ref())
            .map(|r| r.as_ref())
    }

    pub fn get_by_obj(&self, obj: &N) -> Option<usize> {
        self.to_index.get(obj).copied()
    }
}

#[cfg(test)]
mod test_mapping {
    use super::*;

    #[derive(Eq, PartialEq, Debug, Hash)]
    struct S(u32);

    #[test]
    fn test_insert_get_remove() {
        let mut d = DoubleMapping::new();
        let id1 = d.insert(S(1)).unwrap();
        let id2 = d.insert(S(2)).unwrap();
        let id3 = id2 + 1;

        assert_eq!(d.contains_id(id1), true);
        assert_eq!(d.contains_id(id2), true);
        assert_eq!(d.contains_id(id3), false);

        assert_eq!(d.get_by_id(id1), Some(&S(1)));
        assert_eq!(d.get_by_id(id2), Some(&S(2)));
        assert_eq!(d.get_by_id(id3), None);

        assert_eq!(d.get_by_obj(&S(1)), Some(id1));
        assert_eq!(d.get_by_obj(&S(2)), Some(id2));
        assert_eq!(d.get_by_obj(&S(3)), None);

        assert_eq!(d.remove(id1), Some(S(1)));

        assert_eq!(d.get_by_id(id1), None);
        assert_eq!(d.get_by_id(id2), Some(&S(2)));
        assert_eq!(d.get_by_id(id3), None);

        assert_eq!(d.get_by_obj(&S(1)), None);
        assert_eq!(d.get_by_obj(&S(2)), Some(id2));
        assert_eq!(d.get_by_obj(&S(3)), None);

        assert_eq!(d.remove(id2), Some(S(2)));

        assert_eq!(d.get_by_id(id1), None);
        assert_eq!(d.get_by_id(id2), None);
        assert_eq!(d.get_by_id(id3), None);

        assert_eq!(d.get_by_obj(&S(1)), None);
        assert_eq!(d.get_by_obj(&S(2)), None);
        assert_eq!(d.get_by_obj(&S(3)), None);

        assert_eq!(d.remove(id2), None);
        assert_eq!(d.remove(id3), None);
    }
}
