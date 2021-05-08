use crate::{bproperty::BProperty, compileable::*};

/// A Struct
pub struct BStruct {
    pub name: String,
    pub properties: Vec<BProperty>,
    pub public: bool,
}

impl Compile for BStruct {
    fn compile(&self, language: Target) -> String {
        match language {
            Target::Rust => {
                let name = &self.name;
                let properties = {
                    self.properties
                        .iter()
                        .map(|p| p.compile(language))
                        .collect::<Vec<String>>()
                        .join(",")
                };
                let public = match self.public {
                    true => "pub ",
                    false => "",
                };

                format!("{}struct {} {{{}}}", public, name, properties)
            }
            Target::CSharp => {
                let name = &self.name;
                let properties = {
                    self.properties
                        .iter()
                        .map(|p| format!("{};", p.compile(language)))
                        .collect::<Vec<String>>()
                        .join("")
                };
                let public = match self.public {
                    true => "public",
                    false => "private",
                };

                format!("{} class {} {{{}}}", public, name, properties)
            }
            Target::JavaScript => {
                let name = &self.name;
                let properties = {
                    self.properties
                        .iter()
                        .map(|p| {
                            let public = match p.public {
                                true => "",
                                false => "private_",
                            };

                            format!("{}{};", public, p.compile(language))
                        })
                        .collect::<Vec<String>>()
                        .join("")
                };
                let public = match self.public {
                    true => "",
                    false => "private_",
                };

                format!("class {}{} {{{}}}", public, name, properties)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::btype::BType;

    fn case1() -> BStruct {
        BStruct {
            name: "garbage_day".into(),
            properties: vec![],
            public: false,
        }
    }

    fn case2() -> BStruct {
        BStruct {
            name: "foo_bar".into(),
            properties: vec![BProperty {
                name: "fooey".into(),
                btype: BType::bool,
                public: false,
            }],
            public: true,
        }
    }

    fn case3() -> BStruct {
        BStruct {
            name: "food".into(),
            properties: vec![
                BProperty {
                    name: "apple".into(),
                    btype: BType::i32,
                    public: false,
                },
                BProperty {
                    name: "banana".into(),
                    btype: BType::bool,
                    public: true,
                },
            ],
            public: true,
        }
    }

    #[test]
    fn bstruct_case1_rust() {
        let target = Target::Rust;

        let input = case1();
        let expected: String = "struct garbage_day {}".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);
    }

    #[test]
    fn bstruct_case2_rust() {
        let target = Target::Rust;

        let input = case2();
        let expected: String = "pub struct foo_bar {fooey: bool}".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);
    }

    #[test]
    fn bstruct_case3_rust() {
        let target = Target::Rust;

        let input = case3();
        let expected: String = "pub struct food {apple: i32,pub banana: bool}".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);
    }

    #[test]
    fn bstruct_case1_csharp() {
        let target = Target::CSharp;

        let input = case1();
        let expected: String = "private class garbage_day {}".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);
    }

    #[test]
    fn bstruct_case2_csharp() {
        let target = Target::CSharp;

        let input = case2();
        let expected: String = "public class foo_bar {private bool fooey;}".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);
    }

    #[test]
    fn bstruct_case3_csharp() {
        let target = Target::CSharp;

        let input = case3();
        let expected: String = "public class food {private Int32 apple;public bool banana;}".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);
    }

    #[test]
    fn bstruct_case1_javascript() {
        let target = Target::JavaScript;

        let input = case1();
        let expected: String = "class private_garbage_day {}".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);
    }

    #[test]
    fn bstruct_case2_javascript() {
        let target = Target::JavaScript;

        let input = case2();
        let expected: String = "class foo_bar {private_fooey;}".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);
    }

    #[test]
    fn bstruct_case3_javascript() {
        let target = Target::JavaScript;

        let input = case3();
        let expected: String = "class food {private_apple;banana;}".into();
        let actual = input.compile(target);
        assert_eq!(expected, actual);
    }
}
