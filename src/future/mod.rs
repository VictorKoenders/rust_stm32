pub trait Future {
    type Item;
    fn poll(&mut self) -> Async<Self::Item>;

    fn wait(mut self) -> Self::Item 
        where Self: Sized {
        loop {
            if let Async::Ready(item) = self::poll() {
                return item;
            }
        }
    }
}

pub enum Async<T> {
    Ready(T),
    NotReady,
}
