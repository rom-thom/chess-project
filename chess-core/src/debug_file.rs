

use std::{fs::OpenOptions, io::Write, path::Path};



#[cfg(feature = "log")]
#[macro_export]
macro_rules! log {
    // We have reached `path = ...` at the end.
    (
        @parse
        [$first:expr $(, $rest:expr)*]
        path = $path:expr
        $(,)?
    ) => {{
        let full_path = std::path::Path::new("chess_log").join($path);

        $crate::log!(
            @write full_path;
            $first
            $(, $rest)*
        )
    }};

    // Consume one logging argument and continue parsing.
    (
        @parse
        [$($previous:expr),*]
        $next:expr,
        $($remaining:tt)+
    ) => {{
        $crate::log!(
            @parse
            [$($previous,)* $next]
            $($remaining)+
        )
    }};

    // No custom path was provided.
    (
        @parse
        [$($previous:expr),*]
        $last:expr
        $(,)?
    ) => {{
        $crate::log!(
            @write std::path::Path::new(
                "chess_log/search_scores.log"
            );
            $($previous,)*
            $last
        )
    }};

    // Internal writing implementation.
    (
        @write $path:expr;
        $first:expr
        $(, $rest:expr)*
    ) => {{
        use std::io::Write as _;

        (|| -> std::io::Result<()> {
            let path: std::path::PathBuf = ($path).into();

            // OpenOptions cannot create missing parent directories.
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let mut log_file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)?;

            writeln!(
                log_file,
                "[{}:{}:{}]: {:#?}",
                file!(),
                line!(),
                column!(),
                &$first,
            )?;

            $(
                writeln!(
                    log_file,
                    "[{}:{}:{}]: {:#?}",
                    file!(),
                    line!(),
                    column!(),
                    &$rest,
                )?;
            )*

            Ok(())
        })()
    }};

    // Public entry point.
    ($($input:tt)+) => {{
        $crate::log!(@parse [] $($input)+)
    }};
}


#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! log {
    ($($val:tt)*) => {
        Ok::<(), std::io::Error>(())
    };
}