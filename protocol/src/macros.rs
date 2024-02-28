/// 仿造 https://stackoverflow.com/questions/32710187/how-do-i-get-an-enum-as-a-string 编写
/// 
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

    #[test]
    fn test_01() {
        assert_eq!("LengthNotEnough", ParseError::LengthNotEnough(String::from("123")).name());
    }
}
