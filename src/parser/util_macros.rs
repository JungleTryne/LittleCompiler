macro_rules! create_parser {
    {
        $class_name:ident,
        $(
            $instruction:ident, $serialize_name: expr, $address: expr
        );+
    } => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, EnumString)]
        pub enum $class_name {
            $(#[strum(serialize = $serialize_name, ascii_case_insensitive)]
            $instruction),+
        }

        impl $class_name {
            pub fn as_addr(&self) -> u32 {
                match self {
                    $($class_name::$instruction => $address),+
                }
            }
        }
    }
}

pub(crate) use create_parser;
