mod reader;
mod types;

pub use self::reader::ReadError;
pub use self::types::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Endianness {
    Little,
    Big,
    Native,
}

pub fn read<'a, T: serde::Deserialize<'a>>(
    b: &'a [u8],
    endian: Endianness,
) -> Result<T, ReadError> {
    T::deserialize(&mut self::reader::Reader::new(b, endian))
}

#[cfg(test)]
mod tests {
    use crate::{read, Endianness, PreeditDone, Feedback, FeedbackType};

    #[test]
    fn read_preedit_done() {
        let done: PreeditDone = read(b"\x00\x04\x01\x01", Endianness::Little).unwrap();
        assert_eq!(
            done,
            PreeditDone {
                method_id: 0x0400,
                context_id: 0x0101
            }
        );
    }

    #[test]
    fn read_feedback() {
        let feedback: Feedback = read(b"\x02\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00", Endianness::Little).unwrap();
        assert_eq!(feedback.arr, [FeedbackType::Reverse, FeedbackType::Highlight]);
    }
}