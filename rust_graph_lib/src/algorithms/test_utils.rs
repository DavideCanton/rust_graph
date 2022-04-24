use std::borrow::Borrow;

pub fn slice_equal<T: Eq, B1: Borrow<T>, B2: Borrow<T>>(s1: &[B1], s2: &[B2]) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    s1.iter()
        .zip(s2.iter())
        .all(|(a, b)| a.borrow() == b.borrow())
}
