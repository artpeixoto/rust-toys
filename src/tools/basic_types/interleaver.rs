
struct Interleaver<T: Iterator>{
    current: usize,
    iters:   Option<Vec<T>>
}

impl<T:Iterator> Iterator for Interleaver<T>{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let true_iters = self.iters.as_mut()?;
        let res = true_iters.get_mut(self.current).and_then(|x| x.next());

        self.current = (self.current + 1) % true_iters.len();

        return res;
    }
}



impl<T:Iterator> Default for Interleaver<T> {
    fn default() -> Self {
        Self { current: 0, iters: Option::None }
    }
}

impl<T:Iterator> Interleaver<T>{
    fn new() -> Self {Default::default()}
    fn from<U: IntoIterator<IntoIter=T>, V: IntoIterator<Item=U>>(sources: V) -> Self {
        let terators  = sources.into_iter().map(|x| x.into_iter()).collect::<Vec<_>>()	;
        Interleaver { current: 0, iters: Some(terators) }
    }
}
