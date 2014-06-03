use std::slice::raw;
use std::mem;

/// Given a type, produce a raw representation of it in the format of &[u8]. This allows us
/// to efficiently encode and decode messages without analyzing the type further like JSON.
///
/// 1) Convert `x` to a pointer of u8. This requires a double cast.
/// 2) Get the size of type `T`
/// 3) buf_as_slice will return a new raw Slice where the data is the pointer of u8s with
///    the size of type `T`.
/// 4) Transmute the raw slice &[T] to &[u8] in the closure.
pub fn as_byte_slice<'a, T>(x: &'a T) -> &'a [u8] {
    unsafe {
        raw::buf_as_slice(
            x as *_ as *u8,
            mem::size_of::<T>(),
            |v| mem::transmute(v)
        )
    }
}

/// We basically need to do the opposite from the as_byte_slice function.
///
/// * We have: &[u8]
/// * We need: T
///
/// 1) Convert x to &[T]
/// 2) Get the raw pointer from the slice.
/// 3) Dereference the pointer and return a reference.
pub fn from_byte_slice<'a, T>(x: &'a [u8]) -> &'a T {
    unsafe {
        let slice: &[T] = mem::transmute(x);
        let ptr = slice.as_ptr() as *T;
        &*ptr
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_int() {
        let i = 5;
        let bytes = as_byte_slice(&i);

        let dec = from_byte_slice(bytes);
        assert_eq!(i, *dec);
    }

    #[test]
    fn decode_struct() {
        #[deriving(PartialEq, Show)]
        struct Foo {
            i: int
        }

        let foo = Foo { i : 5 };

        let bytes = as_byte_slice(&foo);
        let dec   = from_byte_slice(bytes);
        assert_eq!(foo, *dec);
        assert_eq!(dec.i, 5);
    }


    #[test]
    fn decode_boxed_struct() {
        #[deriving(PartialEq, Show)]
        struct Foo {
            i: int
        }

        let foo = box Foo { i : 5 };
        let bytes = as_byte_slice(&foo);
        let decoded = from_byte_slice(bytes);
        assert_eq!(foo, *decoded);
    }

    #[test]
    fn decode_recursive_struct() {
        #[deriving(PartialEq, Show)]
        struct Foo {
            rec: Option<Box<Foo>>
        }

        let foo = Foo { rec: Some(box Foo { rec: None }) };
        let bytes = as_byte_slice(&foo);
        let dec = from_byte_slice(bytes);
        assert_eq!(foo, *dec);
        assert!(dec.rec.is_some());
        assert!(dec.rec.get_ref().rec.is_none());
    }
}
