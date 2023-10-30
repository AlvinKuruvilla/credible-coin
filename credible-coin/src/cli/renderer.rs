use bat::{Input, PrettyPrinter};
/// Pretty-prints the content of a CSV file in the terminal.
///
/// This function uses bat's `PrettyPrinter` module to format and display the
/// content of the specified CSV file with line numbers, grid, and headers.
///
/// # Arguments
///
/// * `path`: The path to the CSV file to be pretty-printed.
///
/// # Panics
///
/// The function will panic if the file cannot be read or if there's an error
/// during the printing process.
///
pub(crate) fn render_csv(path: &str) {
    PrettyPrinter::new()
        .language("csv")
        .line_numbers(true)
        .grid(true)
        .header(true)
        .input(Input::from_file(path).kind("CSV"))
        .print()
        .unwrap();
}
/// Pretty-prints the content of a txt file in the terminal.
///
/// This function uses bat's `PrettyPrinter` module to format and display the
/// content of the specified txt file with line numbers, grid, and headers.
///
/// # Arguments
///
/// * `path`: The path to the CSV file to be pretty-printed.
///
/// # Panics
///
/// The function will panic if the file cannot be read or if there's an error
/// during the printing process.
///
pub(crate) fn render_txt(path: &str) {
    PrettyPrinter::new()
        .language("txt")
        .line_numbers(true)
        .grid(true)
        .header(true)
        .input(Input::from_file(path).kind("CSV"))
        .print()
        .unwrap();
}
#[macro_export]
/// A macro to pretty print a file preview based on the file extension
macro_rules! render_file_preview {
    ($path:expr) => {
        match crate::cli::exchange::asset_database::get_extension_from_filename($path) {
            Some("csv") => crate::cli::renderer::render_csv($path),
            Some("txt") => crate::cli::renderer::render_txt($path),
            _ => eprintln!("Unsupported file extension!"),
        }
    };
}
