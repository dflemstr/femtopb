//! A oneof made up solely of scalar variants has no reason to borrow, so it can be declared
//! without a lifetime parameter. This used to be impossible: the `Oneof` derive required a
//! lifetime, but an unused lifetime on the enum is a hard error (E0392).

use femtopb::Message as _;

#[derive(Clone, Debug, PartialEq, femtopb::Oneof)]
pub enum ScalarOnly {
    #[femtopb(int32, tag = 1)]
    A(i32),
    #[femtopb(sint64, tag = 2)]
    B(i64),
}

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Wrapper<'a> {
    #[femtopb(oneof, tags = [1, 2])]
    pub choice: Option<ScalarOnly>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[test]
fn scalar_only_oneof_roundtrips() {
    for choice in [ScalarOnly::A(42), ScalarOnly::B(-5)] {
        let msg = Wrapper {
            choice: Some(choice.clone()),
            ..Default::default()
        };
        let mut buf = vec![0; msg.encoded_len()];
        msg.encode(&mut buf.as_mut_slice()).unwrap();
        let decoded = Wrapper::decode(buf.as_slice()).unwrap();
        assert_eq!(decoded.choice, Some(choice));
    }
}

// A oneof that *does* borrow keeps working with its declared lifetime.
#[derive(Clone, Debug, PartialEq, femtopb::Oneof)]
pub enum Borrowing<'a> {
    #[femtopb(int32, tag = 1)]
    Number(i32),
    #[femtopb(string, tag = 2)]
    Text(&'a str),
}

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct BorrowingWrapper<'a> {
    #[femtopb(oneof, tags = [1, 2])]
    pub choice: Option<Borrowing<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[test]
fn borrowing_oneof_roundtrips() {
    let msg = BorrowingWrapper {
        choice: Some(Borrowing::Text("hi")),
        ..Default::default()
    };
    let mut buf = vec![0; msg.encoded_len()];
    msg.encode(&mut buf.as_mut_slice()).unwrap();
    let decoded = BorrowingWrapper::decode(buf.as_slice()).unwrap();
    assert_eq!(decoded.choice, Some(Borrowing::Text("hi")));
}
