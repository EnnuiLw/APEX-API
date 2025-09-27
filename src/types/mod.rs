pub mod legends;
pub mod platform;

pub use legends::Character;
pub use platform::Platform;

#[macro_export]
macro_rules! to_string {
    (
        $(
            $(#[$outer:meta])*
            $vis:vis enum $Enum:ident {
                $(
                    $(#[doc = $doc:literal])*
                    $(#[cfg $($cfg:tt)*])?
                    $(#[default $($dummy:tt)?])?
                    $Variant:ident,
                )*
            }
        )*
    ) => {
        $(
            $(#[$outer])*
            $vis enum $Enum {
                $(

                    $(#[doc = $doc])*
                    $(#[cfg $($cfg)*])?
                    $(#[default $($dummy:tt)?])?
                    $Variant,
                )*
            }

            impl ToString for $Enum {
                fn to_string(&self) -> String {
                    match self {
                        $(Self::$Variant => stringify!($Variant).to_string()),*
                    }
                }
            }
        )*

    };
}
