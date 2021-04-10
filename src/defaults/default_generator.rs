use crate::generator::Generator;
use crate::helper_traits::AsBytes;

pub struct ChecksumGenerator<T : AsBytes>(T);

impl<T : AsBytes> Generator for ChecksumGenerator<T> {
    type Output = (u16, u16);

    fn generate_checksum(self) -> Self::Output {
        let bytes = self.0.as_bytes();
        let mut bytes = bytes.iter();
        let mut shorts: Vec<u16> = if bytes.len() % 2 == 0 {
            Vec::with_capacity(bytes.len() / 2)
        } else {
            Vec::with_capacity(bytes.len() / 2 + 1)
        };
        loop {
            let (front, back) = (bytes.next(), bytes.next());
            match (front, back) {
                (Some(front), Some(back)) => {
                    let short = ((*front as u16) << 8u16) | (*back as u16);
                    shorts.push(short);
                },
                (Some(front), None) => {
                    let short = (*front as u16) << 8u16;
                    shorts.push(short);
                },
                _ => break
            }
        }

        let mut xor = 0u16;
        let mut sum = 0u16;

        for short in shorts {
            let (new_sum, _) = sum.overflowing_add(short);
            sum = new_sum;
            xor ^= short;
        }

        (xor, sum)
    }
}

