pub mod commands;

pub trait PrinterCommand<'a> {
    fn encode(&self) -> Vec<u8>;
}

/// Combines three arrays into one, used for composing an encoded command
fn compose<T>(prefix: &[T], middle: &[T], suffix: &[T], middle_limit: usize) -> Vec<T>
where
    T: Clone,
{
    let mut params = middle.to_vec();
    params.truncate(middle_limit);
    [prefix.to_vec(), params, suffix.to_vec()].concat()
}
