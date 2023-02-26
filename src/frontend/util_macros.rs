macro_rules! create_parser {
    {
        $class_name:ident,
        $(
            $instruction:ident, $serialize_name: expr
        );+
    } => {
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq, EnumString)]
        pub enum $class_name {
            $(#[strum(serialize = $serialize_name, ascii_case_insensitive)]
            $instruction),+
        }
    }
}

pub(crate) use create_parser;
