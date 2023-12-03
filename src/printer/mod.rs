pub mod commands;
pub mod commands_extend;

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

/// Only allow printable characters through
fn sanitize<'a>(input: &'a str) -> Vec<u8> {
    input
        .as_bytes()
        .iter()
        .filter(|c| **c >= 0x20u8 && **c != 0x7F) // 0x20 = Space, 0x7F = DEL
        .copied()
        .collect()
}

/// Split a u16 into two u8s
fn split(value: u16) -> [u8; 2] {
    [(value >> 8) as u8, (value & 0xff) as u8]
}
