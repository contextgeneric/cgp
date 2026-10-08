//! An `Enum!` names the shape of an enum that derives the whole extensible
//! variant family with `#[derive(CgpData)]`, whose variants each carry one
//! positional payload. Generic code bounded on that shape converts a value
//! built in the shape into the enum.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/enum.md.

use cgp_fork::prelude::*;

#[derive(Debug, PartialEq, CgpData)]
pub enum Value {
    Int(u64),
    Text(String),
}

pub fn from_shape<T>(fields: Enum! { Int(u64), Text(String) }) -> T
where
    T: FromFields<Fields = Enum! { Int(u64), Text(String) }>,
{
    T::from_fields(fields)
}

#[test]
fn a_shape_value_converts_into_the_enum() {
    assert_eq!(
        from_shape::<Value>(Either::Left(Field::from(7))),
        Value::Int(7)
    );

    assert_eq!(
        from_shape::<Value>(Either::Right(Either::Left(Field::from("seven".to_owned())))),
        Value::Text("seven".to_owned())
    );
}

#[test]
fn the_variant_derives_agree_with_the_shape() {
    assert_eq!(
        Value::from_variant(PhantomData::<Symbol!("Text")>, "eight".to_owned()),
        from_shape::<Value>(Either::Right(Either::Left(Field::from("eight".to_owned())))),
    );
}
