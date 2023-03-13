/// Set of utilities and helpers to manipulate polyglot AST objects.
///
/// This module contains errors types, the Language enum as well as a few conversions functions.
pub mod util {
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("Invalid argument received")]
    pub struct InvalidArgumentError;

    /// An enumeration that represents all languages supported by this crate. Current options are Python, JavaScript and Java.
    pub enum Language {
        Python,
        JavaScript,
        Java,
    }

    pub fn strip_quotes(s: &str) -> String {
        let mut tmp = s.chars();
        tmp.next();
        tmp.next_back();
        String::from(tmp.as_str())
    }

    /// Returns the treesitter language corresponding to the string slice passed.
    ///
    /// If the string slice does not match any supported language, the return value will be an InvalidArgumentError.
    /// # Examples
    /// Valid use-case:
    /// ```
    /// use polyglot_ast::util;
    ///
    /// let language = util::language_string_to_treesitter("python").expect("Python is a supported polyglot AST language");
    ///
    /// assert_eq!(language, tree_sitter_python::language());
    /// ```
    /// Invalid use-case:
    /// ```
    /// use polyglot_ast::util;
    /// use util::InvalidArgumentError;
    ///
    /// let language = util::language_string_to_treesitter("go");
    /// let invalid: InvalidArgumentError = match language {
    ///     Ok(_) => panic!("Go is not a supported language"),
    ///     Err(e) => e,
    /// };
    /// ```
    pub fn language_string_to_treesitter(
        lang: &str,
    ) -> Result<tree_sitter::Language, InvalidArgumentError> {
        Ok(language_enum_to_treesitter(&language_string_to_enum(lang)?))
    }

    /// Returns the treesitter language corresponding to the Language enum reference passed.
    ///
    /// # Example
    /// ```
    /// use polyglot_ast::util;
    /// use util::Language;
    ///
    /// let language = util::language_enum_to_treesitter(&Language::Python);
    ///
    /// assert_eq!(language, tree_sitter_python::language());
    /// ```
    pub fn language_enum_to_treesitter(lang: &Language) -> tree_sitter::Language {
        match lang {
            Language::Python => tree_sitter_python::language(),
            Language::JavaScript => tree_sitter_javascript::language(),
            Language::Java => tree_sitter_java::language(),
        }
    }

    /// Returns the Language enum corresponding to the passed string slice
    /// If the string slice does not match any supported language, the return value will be an InvalidArgumentError.
    /// # Examples
    /// Valid use-case:
    /// ```
    /// use polyglot_ast::util;
    /// use util::Language;
    ///
    /// let language = util::language_string_to_enum("python").expect("Python is a supported polyglot AST language");
    ///
    /// assert!(matches!(language, Language::Python));
    /// ```
    /// Invalid use-case:
    /// ```
    /// use polyglot_ast::util;
    /// use util::InvalidArgumentError;
    ///
    /// let language = util::language_string_to_treesitter("go");
    /// let invalid: InvalidArgumentError = match language {
    ///     Ok(_) => panic!("Go is not a supported language"),
    ///     Err(e) => e,
    /// };
    /// ```
    pub fn language_string_to_enum(lang: &str) -> Result<Language, InvalidArgumentError> {
        match lang {
            "python" => Ok(Language::Python),
            "js" | "javascript" => Ok(Language::JavaScript),
            "java" => Ok(Language::Java),
            _ => Err(InvalidArgumentError),
        }
    }
}

pub mod polyglot_tree;
pub use polyglot_tree::polyglot_processor::{PolygotProcessor, TreePrinter};
pub use polyglot_tree::polyglot_zipper::PolyglotZipper;
pub use polyglot_tree::PolyglotTree;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
