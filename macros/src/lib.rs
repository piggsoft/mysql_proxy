#[macro_export]
macro_rules! enum_and_str {
    (enum $name:ident {
        $($variant:ident($val:ident)),*,
    }) => {
        enum $name {
            $($variant($val)),*,
        }

        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant($val) => stringify!($variant)),*
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    enum_and_str! {
        enum ParseError {
            LengthNotEnough(String),
            NotSupportHeader(String),
        }
    }

    crate::enum_and_str! {
         enum ParseError1 {
            LengthNotEnough(String),
            NotSupportHeader(String),
        }
    }

    #[test]
    fn test_01() {
        assert_eq!("LengthNotEnough", ParseError::LengthNotEnough(String::from("123")).name());
        assert_eq!("LengthNotEnough", ParseError1::LengthNotEnough(String::from("123")).name());
    }
}

