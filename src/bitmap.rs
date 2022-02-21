use futures::future::Select;

pub struct Bitmap {
    len: usize,
    bits: [bool; 1024],
}

impl Bitmap {
    fn New(mut self, n: usize) -> Self {
        self.len = n;
        self.bits = [false; 1024];
        self
    }

    fn Set() {}
    fn Get() {}
}
