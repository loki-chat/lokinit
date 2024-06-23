#[macro_export]
macro_rules! repr_enums {
    (
        $repr:ty,
        $($(#[doc = $doc:literal])* pub enum $name:ident {$($(#[doc = $variant_doc:literal])* $variant:ident = $val:literal,)*})*
    ) => {
        $(
            $(#[doc = $doc])*
            #[repr($repr)]
            #[derive(Debug)]
            pub enum $name {
                $(
                    $(#[doc = $variant_doc])*
                    $variant = $val,
                )*
            }

            impl From<$name> for $repr {
                fn from(val: $name) -> Self {
                    val as $repr
                }
            }

            impl TryFrom<$repr> for $name {
                type Error = $repr;

                fn try_from(val: $repr) -> Result<Self, $repr> {
                    Ok(match val {
                        $($val => Self::$variant),*,
                        _ => return Err(val)
                    })
                }
            }
        )*
    };
}
pub use repr_enums;
