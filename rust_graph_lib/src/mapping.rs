use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

pub struct DoubleMapping<N> {
    to_index: HashMap<Rc<N>, usize>,
    from_index: Vec<Option<Rc<N>>>,
    autodecrement: bool,
}

impl<N: Hash + Eq + Debug> DoubleMapping<N> {
    pub fn new(autodecrement: bool) -> DoubleMapping<N> {
        DoubleMapping {
            to_index: HashMap::new(),
            from_index: Vec::with_capacity(50),
            autodecrement,
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
                if self.autodecrement {
                    self.from_index.remove(id);
                    for v in self.to_index.values_mut() {
                        if *v > id {
                            *v -= 1;
                        }
                    }
                } else {
                    self.from_index[id] = None;
                }
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

    pub fn iter_obj(&self) -> impl Iterator<Item = &N> {
        self.to_index.keys().map(|v| v.as_ref())
    }
}

impl<N: Hash + Eq + Debug> Default for DoubleMapping<N> {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod test_mapping {
    use std::collections::HashSet;

    use super::*;

    #[derive(Eq, PartialEq, Debug, Hash)]
    struct S(u32);

    #[test]
    fn test_insert_get_remove() {
        let mut d = DoubleMapping::default();
        let id1 = d.insert(S(1)).unwrap();
        let id2 = d.insert(S(2)).unwrap();
        let id3 = id2 + 1;

        assert_eq!(d.contains_id(id1), true);
        assert_eq!(d.contains_id(id2), true);
        assert_eq!(d.contains_id(id3), false);

        assert_eq!(d.iter_obj().collect::<HashSet<_>>(), HashSet::from_iter(vec![&S(1), &S(2)].into_iter()));

        assert_eq!(d.get_by_id(id1), Some(&S(1)));
        assert_eq!(d.get_by_id(id2), Some(&S(2)));
        assert_eq!(d.get_by_id(id3), None);

        assert_eq!(d.get_by_obj(&S(1)), Some(id1));
        assert_eq!(d.get_by_obj(&S(2)), Some(id2));
        assert_eq!(d.get_by_obj(&S(3)), None);

        assert_eq!(d.remove(id1), Some(S(1)));

        assert_eq!(d.iter_obj().collect::<HashSet<_>>(), HashSet::from_iter(vec![&S(2)].into_iter()));

        assert_eq!(d.get_by_id(id1), None);
        assert_eq!(d.get_by_id(id2), Some(&S(2)));
        assert_eq!(d.get_by_id(id3), None);

        assert_eq!(d.get_by_obj(&S(1)), None);
        assert_eq!(d.get_by_obj(&S(2)), Some(id2));
        assert_eq!(d.get_by_obj(&S(3)), None);

        assert_eq!(d.remove(id2), Some(S(2)));

        assert!(d.iter_obj().collect::<Vec<_>>().is_empty());

        assert_eq!(d.get_by_id(id1), None);
        assert_eq!(d.get_by_id(id2), None);
        assert_eq!(d.get_by_id(id3), None);

        assert_eq!(d.get_by_obj(&S(1)), None);
        assert_eq!(d.get_by_obj(&S(2)), None);
        assert_eq!(d.get_by_obj(&S(3)), None);

        assert_eq!(d.remove(id2), None);
        assert_eq!(d.remove(id3), None);
    }

    #[test]
    fn test_autodecrement() {
        let mut d = DoubleMapping::new(true);
        
        assert_eq!(d.insert(S(1)).unwrap(), 1);
        assert_eq!(d.insert(S(2)).unwrap(), 2);

        assert_eq!(d.contains_id(1), true);
        assert_eq!(d.contains_id(2), true);
        assert_eq!(d.contains_id(3), false);

        assert_eq!(d.iter_obj().collect::<HashSet<_>>(), HashSet::from_iter(vec![&S(1), &S(2)].into_iter()));

        assert_eq!(d.get_by_id(1), Some(&S(1)));
        assert_eq!(d.get_by_id(2), Some(&S(2)));
        assert_eq!(d.get_by_id(3), None);

        assert_eq!(d.get_by_obj(&S(1)), Some(1));
        assert_eq!(d.get_by_obj(&S(2)), Some(2));
        assert_eq!(d.get_by_obj(&S(3)), None);

        assert_eq!(d.remove(1), Some(S(1)));

        assert_eq!(d.iter_obj().collect::<HashSet<_>>(), HashSet::from_iter(vec![&S(2)].into_iter()));

        assert_eq!(d.get_by_id(1), Some(&S(2)));
        assert_eq!(d.get_by_id(2), None);
        assert_eq!(d.get_by_id(3), None);

        assert_eq!(d.get_by_obj(&S(1)), None);
        assert_eq!(d.get_by_obj(&S(2)), Some(1));
        assert_eq!(d.get_by_obj(&S(3)), None);

        assert_eq!(d.remove(1), Some(S(2)));

        assert!(d.iter_obj().collect::<Vec<_>>().is_empty());

        assert_eq!(d.get_by_id(1), None);
        assert_eq!(d.get_by_id(2), None);
        assert_eq!(d.get_by_id(3), None);

        assert_eq!(d.get_by_obj(&S(1)), None);
        assert_eq!(d.get_by_obj(&S(2)), None);
        assert_eq!(d.get_by_obj(&S(3)), None);

        assert_eq!(d.remove(2), None);
        assert_eq!(d.remove(3), None);
    }
}
