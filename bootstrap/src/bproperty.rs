use crate::{btype::BType, compileable::*};

/// A property
pub struct BProperty {
    pub name: String,
    pub btype: BType,
    pub public: bool,
}

impl Compile for BProperty {
    fn compile(&self, language: Target) -> String {
        match language {
            Target::Rust => {
                let public = match self.public {
                    true => "pub ",
                    false => "",
                };

                format!("{}{}: {}", public, self.name, self.btype.compile(language))
            }
            Target::CSharp => {
                let public = match self.public {
                    true => "public",
                    false => "private",
                };
                format!("{} {} {}", public, self.btype.compile(language), self.name)
            }
            Target::JavaScript => format!("{}", self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn case1() -> BProperty {
        BProperty {
            name: "garbage_day".into(),
            btype: BType::bool,
            public: false,
        }
    }

    fn case2() -> BProperty {
        BProperty {
            name: "foo_bar".into(),
            btype: BType::i32,
            public: true,
        }
    }

    #[test]
    fn bproperty_compiles_rust() {
        let target = Target::Rust;
        let input = case1();
        let expected: String = "garbage_day: bool".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);

        let input = case2();
        let expected: String = "pub foo_bar: i32".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);
    }

    #[test]
    fn bproperty_compiles_csharp() {
        let target = Target::CSharp;
        let input = case1();
        let expected: String = "private bool garbage_day".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);

        let input = case2();
        let expected: String = "public Int32 foo_bar".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);
    }

    #[test]
    fn bproperty_compiles_javascript() {
        let target = Target::JavaScript;
        let input = case1();
        let expected: String = "garbage_day".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);

        let input = case2();
        let expected: String = "foo_bar".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);
    }
}
