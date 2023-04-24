//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/StringInfo.cs>

/// Information about a string. Stored inside a string table, which is
/// produced from the Compiler.
///
/// You do not create instances of this class yourself. They are
/// generated by the [`Compiler`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StringInfo {
    /// The original text of the string.
    pub text: String,

    /// The name of the node that this string was found in.
    pub node_name: String,

    /// The line number at which this string was found in the file.
    pub line_number: usize,

    /// The name of the file this string was found in.
    pub file_name: String,

    /// Indicates whether this string's line ID was implicitly
    /// generated.
    ///
    /// Implicitly generated line IDs are not guaranteed to remain the
    /// same across multiple compilations. To ensure that a line ID
    /// remains the same, you must define it by adding a line tag to the
    /// line.
    pub is_implicit_tag: bool,

    /// The metadata (i.e. hashtags) associated with this string.
    ///
    /// This array will contain any hashtags associated with this
    /// string besides the `#line:` hashtag.
    pub metadata: Vec<String>,
}
