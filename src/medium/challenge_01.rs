
use parity_scale_codec::Encode;
use parity_scale_codec::Decode;

#[derive(Encode, Decode, Debug, PartialEq)]
pub struct FixedTuple<T, U: Default + Copy, const N: usize>(T, [U; N]);

#[allow(dead_code)]
impl<T, U, const N: usize> FixedTuple<T, U, N>
    where U: Default + Copy,
{
    fn new(value: T, items: &[U]) -> Self {
        let mut array = [U::default(); N];
        for (i, &item) in items.iter().take(N).enumerate() {
                array[i] = item;
        }
        FixedTuple(value, array)
    }
}

mod tests{
    use parity_scale_codec::{Decode, Encode};
    use super::FixedTuple;
    #[test]
    fn test_1() {
        let value = 10u32;
        let n = 4;
        let items = &[1,2];
        let fixed = FixedTuple::<u32,u8, 4>::new(value, items);

        assert_eq!(fixed.0, value);
        assert_eq!(fixed.1, [1u8, 2, 0, 0]);

        println!("{:?}", fixed);
    }
    #[test]
    fn test_2() {
        let value: &'static str = "teste2";
        let n = 3;
        let items = &[5,6,7,8];
        let fixed = FixedTuple::<&'static str,u8, 3>::new(value, items);

        assert_eq!(fixed.0, value);
        assert_eq!(fixed.1, [5,6,7]);
    }

    #[test]
    fn test_3() {
        let original = FixedTuple::<u32, u8, 3>::new(42, &[9, 9]);
        let encoded = original.encode();
        // decodificando:
        let mut input = &encoded[..];
        let decoded = FixedTuple::<u32, u8, 3>::decode(&mut input).unwrap();
        assert_eq!(original, decoded);

    }
}

#[allow(dead_code)]
fn main() {
}