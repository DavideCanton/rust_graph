pub trait Algorithm<'a, N> {
    fn run(&'a self, from: &'a N, to: &'a N) -> Option<Vec<&'a N>>;
}
