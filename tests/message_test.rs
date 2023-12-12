use totoro::{message::{ClientType, Message}, config::DEFAULT_MAX_BUFFER_SIZE};

#[test]
pub fn test_message_to_buffer_conversion() {
    let sub_registration = Message::Registration(ClientType::Subscriber).to_buffer(&Default::default());
    assert_eq!(sub_registration[0], 0); // package type
    assert_eq!(sub_registration[1], 0); // client type

    let pub_registration = Message::Registration(ClientType::Publisher).to_buffer(&Default::default());
    assert_eq!(pub_registration[0], 0); // package type
    assert_eq!(pub_registration[1], 1); // client type
    
    let ack_registration = Message::RegistrationAck.to_buffer(&Default::default());
    assert_eq!(ack_registration[0], 1); // package type
    
    let message_str = "hello world".to_string();
    // substract one, because package will have first byte reserved to indicate package type
    let mut test_buffer: [u8; DEFAULT_MAX_BUFFER_SIZE - 1] = [0; DEFAULT_MAX_BUFFER_SIZE - 1];
    test_buffer[..message_str.len()].copy_from_slice(message_str.as_bytes()); 
    let data = Message::Data(message_str.clone()).to_buffer(&Default::default());
    assert_eq!(data[0], 2); // package type
    assert_eq!(data[1..], test_buffer);
}
