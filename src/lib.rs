pub mod error;
pub mod message;
pub mod values;

#[macro_export]
macro_rules! make_as_protocol_message {
    (
        $(#[$meta:meta])*
        $struct_vis: vis struct $struct_name: ident {
            $(
                 $(#[$field_meta:meta])*
                $field_vis: vis $field_name: ident : $field_type: ty
            ),* $(,)+
        }
    ) => {
        $(#[$meta])*
        pub struct $struct_name {
            $(
                $(#[$field_meta])*
                pub $field_name : $field_type,
            )*
        }

        impl TryFrom<Bytes> for $struct_name {
            type Error = ProtocolError;

            fn try_from(value: Bytes) -> Result<Self, Self::Error> {
                bincode::deserialize(&value).map_err(ProtocolError::Deserialize)
            }
        }

        impl TryFrom<$struct_name> for Bytes {
            type Error = ProtocolError;

            fn try_from(value: $struct_name) -> Result<Self, Self::Error> {
                bincode::serialize(&value)
                    .map(Bytes::from)
                    .map_err(ProtocolError::Serialize)
            }
        }
    };
}
