use bat::{Input, PrettyPrinter};

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
macro_rules! render_file_preview {
    ($path:expr) => {
        match crate::cli::exchange::asset_database::get_extension_from_filename($path) {
            Some("csv") => crate::cli::renderer::render_csv($path),
            Some("txt") => crate::cli::renderer::render_txt($path),
            _ => eprintln!("Unsupported file extension!"),
        }
    };
}
