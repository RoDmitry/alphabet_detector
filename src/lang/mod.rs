macro_rules! impl_try_from {
    ($enum:ident, $tc:ty, $($t:ty)*) => {$(
        impl TryFrom<$t> for $enum {
            type Error = &'static str;

            #[inline]
            fn try_from(v: $t) -> Result<Self, Self::Error> {
                Self::from_code(v as $tc).ok_or("No such variant code")
            }
        }
    )*};
}

macro_rules! impl_serde {
    ($enum:ident, $name:literal) => {
        impl ::serde::Serialize for $enum {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                ::serde::Serializer::serialize_unit_variant(
                    serializer,
                    $name,
                    self.into_code() as u32,
                    self.into_str(),
                )
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $enum {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct EnumValueVisitor;

                impl<'de> ::serde::de::Visitor<'de> for EnumValueVisitor {
                    type Value = $enum;

                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "variant identifier")
                    }

                    #[inline]
                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error,
                    {
                        Self::Value::try_from(v).map_err(|_| {
                            ::serde::de::Error::invalid_value(
                                ::serde::de::Unexpected::Unsigned(v),
                                &"No such variant code",
                            )
                        })
                    }

                    #[inline]
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error,
                    {
                        Self::Value::from_str(v).ok_or_else(|| {
                            ::serde::de::Error::unknown_variant(v, Self::Value::VARIANTS)
                        })
                    }

                    #[inline]
                    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error,
                    {
                        Self::Value::from_bytes(v).ok_or_else(|| {
                            ::serde::de::Error::unknown_variant(
                                &::serde::__private::from_utf8_lossy(v),
                                Self::Value::VARIANTS,
                            )
                        })
                    }
                }

                ::serde::Deserializer::deserialize_identifier(deserializer, EnumValueVisitor)
            }
        }
    };
}

mod alphabets;
mod language;
mod script;
mod script_language;

pub mod ucd;

pub use alphabets::*;
pub use language::*;
pub use script::*;
pub use script_language::*;
pub use ucd::UcdScript;
