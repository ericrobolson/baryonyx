mod bproperty;
mod bstruct;
mod btype;

/// Core pieces for compilation
mod compileable {
    /// The various targets a language may have
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Target {
        Rust,
        CSharp,
        JavaScript,
    }

    /// Translates the given object to the target language
    pub trait Compile {
        /// Converts the given object to the target language
        fn compile(&self, language: Target) -> String;
    }
}

fn main() {
    println!("Hello, world!");
}
