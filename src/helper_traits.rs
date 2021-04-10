pub trait AsBytes{

    /// Gets the byte equivalent of a value
    fn as_bytes(&self) -> Vec<u8>;
    /// converts a value into it byte equivalent of a value
    fn into_bytes(self) -> Vec<u8> where Self: Sized {
        self.as_bytes()
    }
}


impl AsBytes for &str {
    fn as_bytes(&self) -> Vec<u8> {
        self.bytes().collect()
    }
}

impl AsBytes for String {
    fn as_bytes(&self) -> Vec<u8> {
        self.bytes().collect()
    }
}

macro_rules! primitive_as_bytes {
    ($($type:ty)*) => {
        $(primitive_as_bytes! { @ $type })*
    };
    (@ $my_type:ty) => {
        impl AsBytes for $my_type {

            fn as_bytes(&self) -> Vec<u8> {
                (*self).into_bytes()
            }

            fn into_bytes(self) -> Vec<u8> {
                Vec::from(self.to_be_bytes())
            }
        }
    }
}

primitive_as_bytes!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize f32 f64);


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn strings_to_bytes() {
        let string = "hello world";
        let expected: Vec<u8> = string.bytes().collect();
        assert_eq!(&AsBytes::into_bytes(string), &expected);
        assert_eq!(AsBytes::into_bytes(String::from(string)), expected);
    }

    #[test]
    fn integers_to_bytes() {
        assert_eq!(&*0u8.as_bytes(), &[0u8]);
        assert_eq!(&*0u16.as_bytes(), &[0u8; 2]);
        assert_eq!(&*0u32.as_bytes(), &[0u8; 4]);
        assert_eq!(&*0u64.as_bytes(), &[0u8; 8]);
        assert_eq!(&*0i8.as_bytes(), &[0u8]);
        assert_eq!(&*0i16.as_bytes(), &[0u8; 2]);
        assert_eq!(&*0i32.as_bytes(), &[0u8; 4]);
        assert_eq!(&*0i64.as_bytes(), &[0u8; 8]);
        assert_eq!(&*0.0f32.as_bytes(), &[0u8; 4]);
        assert_eq!(&*0.0f64.as_bytes(), &[0u8; 8]);
        assert_eq!(&*(-0.0f32).as_bytes(), &[0x80u8, 0u8, 0u8, 0u8]);
        assert_eq!(&*(-0.0f64).as_bytes(), &[0x80u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
    }
}