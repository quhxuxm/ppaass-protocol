pub mod error;
pub mod generator;
pub mod message;

#[macro_export]
macro_rules! make_as_bytes {
    (
        $(#[$meta:meta])*
        $struct_vis: vis struct $struct_name: ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis: vis $field_name: ident : $field_type: ty
            ),* $(,)*
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
                bincode::deserialize(&value).map_err(|e|ProtocolError::Other(format!("Fail to deserialize bytes to object because of error: {e:?}")))
            }
        }

        impl TryFrom<$struct_name> for Bytes {
            type Error = ProtocolError;

            fn try_from(value: $struct_name) -> Result<Self, Self::Error> {
                bincode::serialize(&value)
                    .map(Bytes::from)
                    .map_err(|e|ProtocolError::Other(format!("Fail to serialize object to bytes because of error: {e:?}")))
            }
        }
    };
    (
        $(#[$meta:meta])*
        $enum_vis: vis enum $enum_name: ident {
            $(
                $(#[$element_meta:meta])*
                $element_name: ident {
                    $(
                        $(#[$field_meta:meta])*
                        $field_name: ident : $field_type: ty
                    ),*$(,)*
                }
            ),* $(,)*
        }
    ) => {
        $(#[$meta])*
        pub enum $enum_name {
            $(
                $(#[$element_meta])*
                $element_name {
                    $(
                        $(#[$field_meta])*
                        $field_name: $field_type
                    ),*
                },
            )*
        }

        impl TryFrom<Bytes> for $enum_name {
            type Error = ProtocolError;

            fn try_from(value: Bytes) -> Result<Self, Self::Error> {
                bincode::deserialize(&value).map_err(|e|ProtocolError::Other(format!("Fail to deserialize bytes to object because of error: {e:?}")))
            }
        }

        impl TryFrom<$enum_name> for Bytes {
            type Error = ProtocolError;

            fn try_from(value: $enum_name) -> Result<Self, Self::Error> {
                bincode::serialize(&value)
                    .map(Bytes::from)
                    .map_err(|e|ProtocolError::Other(format!("Fail to serialize object to bytes because of error: {e:?}")))
            }
        }
    };
    (
        $(#[$meta:meta])*
        $enum_vis: vis enum $enum_name: ident {
            $(
                $(#[$element_meta:meta])*
                $element_name: ident (
                    $(
                        $(#[$field_meta:meta])*
                        $field_type: ty
                    ),*$(,)*
                )
            ),* $(,)*
        }
    ) => {
        $(#[$meta])*
        pub enum $enum_name {
            $(
                $(#[$element_meta])*
                $element_name (
                    $(
                        $(#[$field_meta])*
                        $field_type
                    ),*
                ),
            )*
        }

        impl TryFrom<Bytes> for $enum_name {
            type Error = ProtocolError;

            fn try_from(value: Bytes) -> Result<Self, Self::Error> {
                bincode::deserialize(&value).map_err(|e|ProtocolError::Other(format!("Fail to deserialize bytes to object because of error: {e:?}")))
            }
        }

        impl TryFrom<$enum_name> for Bytes {
            type Error = ProtocolError;

            fn try_from(value: $enum_name) -> Result<Self, Self::Error> {
                bincode::serialize(&value)
                    .map(Bytes::from)
                    .map_err(|e|ProtocolError::Other(format!("Fail to serialize object to bytes because of error: {e:?}")))
            }
        }
    };
}
