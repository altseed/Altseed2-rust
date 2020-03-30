pub trait RetainMutResult<T> {
    fn retain_mut_result<E, F: FnMut(&mut T) -> Result<bool, E>>(&mut self, f: F) -> Result<(), E>;
}

impl<T> RetainMutResult<T> for Vec<T> {
    // https://github.com/rust-lang/rust/blob/0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb/src/liballoc/vec.rs#L1072-L1093
    fn retain_mut_result<E, F: FnMut(&mut T) -> Result<bool, E>>(
        &mut self,
        mut f: F,
    ) -> Result<(), E> {
        let len = self.len();
        let mut del = 0;
        {
            let v = &mut **self;

            for i in 0..len {
                if !(f(&mut v[i])?) {
                    del += 1;
                } else if del > 0 {
                    v.swap(i - del, i);
                }
            }
        }
        if del > 0 {
            self.truncate(len - del);
        }

        Ok(())
    }
}
