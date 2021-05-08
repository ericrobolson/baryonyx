use crate::compileable::*;

/// The list of types that may be used
pub enum BType {
    i32,
    bool,
}

impl Compile for BType {
    fn compile(&self, language: Target) -> String {
        match language {
            Target::Rust => match self {
                BType::i32 => "i32".into(),
                BType::bool => "bool".into(),
            },
            Target::CSharp => match self {
                BType::i32 => "Int32".into(),
                BType::bool => "bool".into(),
            },
            Target::JavaScript => match self {
                BType::i32 => "Number".into(),
                BType::bool => "Boolean".into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i32_compiles_rust() {
        let expected: String = "i32".into();
        let actual = BType::i32.compile(Target::Rust);
        assert_eq!(expected, actual);
    }

    #[test]
    fn i32_compiles_csharp() {
        let expected: String = "Int32".into();
        let actual = BType::i32.compile(Target::CSharp);
        assert_eq!(expected, actual);
    }

    #[test]
    fn i32_compiles_javascript() {
        let expected: String = "Number".into();
        let actual = BType::i32.compile(Target::JavaScript);
        assert_eq!(expected, actual);
    }

    #[test]
    fn bool_compiles_rust() {
        let expected: String = "bool".into();
        let actual = BType::bool.compile(Target::Rust);
        assert_eq!(expected, actual);
    }

    #[test]
    fn bool_compiles_csharp() {
        let expected: String = "bool".into();
        let actual = BType::bool.compile(Target::CSharp);
        assert_eq!(expected, actual);
    }

    #[test]
    fn bool_compiles_javascript() {
        let expected: String = "Boolean".into();
        let actual = BType::bool.compile(Target::JavaScript);
        assert_eq!(expected, actual);
    }
}
