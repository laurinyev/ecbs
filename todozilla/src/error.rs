macro_rules! define_error_codes {
    {
        $(
            #[doc = $doc:expr]
            pub const $name:ident : i32 = $val:expr;
        )*
    } => {
        #[allow(non_camel_case_types)]
        #[allow(unused)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        #[repr(i32)]
        pub enum ErrorCode {
            $(
                #[doc = $doc]
                $name = $val,
            )*
        }

        impl ErrorCode {
            /// Returns the integer value of the error code.
            pub const fn code(self) -> i32 {
                self as i32
            }

            /// Returns the name of the error code as a string slice.
            pub const fn name(self) -> &'static str {
                match self {
                    $(
                        Self::$name => stringify!($name),
                    )*
                }
            }

            /// Returns the documentation/description of the error code.
            pub const fn description(self) -> &'static str {
                match self {
                    $(
                        Self::$name => $doc,
                    )*
                }
            }
        }
    };
}

define_error_codes! {
    /// An unexpected internal failure occurs.
    pub const INTERNAL: i32 = 1;

    /// Reading from or writing to the terminal fails.
    pub const IO: i32 = 2;

    /// `TZ.yml` does not exist.
    pub const CONFIG_NOT_FOUND: i32 = 10;

    /// `TZ.yml` exists but cannot be read.
    pub const CONFIG_UNREADABLE: i32 = 11;

    /// `TZ.yml` cannot be parsed or fails validation.
    pub const CONFIG_INVALID: i32 = 12;

    /// `TZ.yml` cannot be written by `init`.
    pub const CONFIG_WRITE_FAILED: i32 = 13;

    /// `TZ.yml` already exists.
    pub const CONFIG_ALREADY_EXISTS: i32 = 14;

    /// `TZ.md` does not exist.
    pub const MARKDOWN_NOT_FOUND: i32 = 20;

    /// `TZ.md` exists but cannot be read.
    pub const MARKDOWN_UNREADABLE: i32 = 21;

    /// `TZ.md` cannot be written.
    pub const MARKDOWN_WRITE_FAILED: i32 = 22;

    /// The Code Markings heading is missing from `TZ.md`.
    pub const MARKINGS_HEADING_NOT_FOUND: i32 = 23;

    /// The Ephemeral TODOs heading is missing from `TZ.md`.
    pub const EPHEMERAL_HEADING_NOT_FOUND: i32 = 24;

    /// The command needs Ephemeral TODOs, but `ephemeral_heading` is not configured.
    pub const EPHEMERAL_DISABLED: i32 = 25;

    /// The configured `base_dir` does not exist.
    pub const BASE_DIR_NOT_FOUND: i32 = 30;

    /// The source tree cannot be traversed.
    pub const DIRECTORY_WALK_FAILED: i32 = 31;

    /// A git repository cannot be found.
    pub const GIT_REPOSITORY_NOT_FOUND: i32 = 40;

    /// A git command cannot be run or fails unexpectedly.
    pub const GIT_COMMAND_FAILED: i32 = 41;

    /// Updating the git index fails in `exclude`.
    pub const GIT_INDEX_UPDATE_FAILED: i32 = 42;

    /// The git pre-commit hook cannot be created or updated.
    pub const HOOK_WRITE_FAILED: i32 = 43;
}

pub type Result<T> = std::result::Result<T, ErrorCode>;

pub fn error<T>(code: ErrorCode, underlying: Option<String>) -> Result<T> {
    eprintln!("todozilla: error[TZ{:03} {}]:{}", code.code(), code.name(), code.description());

    if underlying.is_some() {
        eprintln!("  caused by: {}", underlying.unwrap());
    }

    Err(code)
}

