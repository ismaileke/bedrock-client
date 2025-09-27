#[macro_export]
macro_rules! downcast_bedrock_packet {
    ($packet:ident, $ty:ty, $body:expr) => {
        if let Some(p) = $packet.as_any().downcast_ref::<$ty>() {
            $body(p)
        }
    };
}
