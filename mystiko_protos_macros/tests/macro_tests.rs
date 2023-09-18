use mystiko_protos_macros::ProtoBuilder;
use prost::Message;
use std::fmt::Debug;

#[test]
fn test_try_from_bytes() {
    let message = TestMessage {
        message: "Hello, world!".to_string(),
    };
    let bytes = message.encode_to_vec();
    assert_eq!(get_message(bytes.as_slice()), "Hello, world!".to_string());
    assert_eq!(get_message(&bytes), "Hello, world!".to_string());
    assert_eq!(get_message(bytes), "Hello, world!".to_string());
}

#[derive(Clone, PartialEq, Message, ProtoBuilder)]
struct TestMessage {
    #[prost(string, tag = "1")]
    pub message: String,
}

fn get_message<M>(message: M) -> String
where
    M: TryInto<TestMessage>,
    <M as TryInto<TestMessage>>::Error: Debug,
{
    let message: TestMessage = message.try_into().unwrap();
    message.message
}
