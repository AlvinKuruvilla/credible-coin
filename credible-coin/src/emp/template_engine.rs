use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub struct TemplateEngine {}

impl TemplateEngine {
    pub fn new() -> Self {
        TemplateEngine {}
    }

    pub fn render(&self, template: &str, placeholders: HashMap<String, String>) -> String {
        let mut output = String::from(template);
        for (key, value) in placeholders {
            let token = format!("<<{}>>", key);
            output = output.replace(&token, &value);
        }
        output
    }
    pub fn write_to_file(
        finalized_template: &str,
        file_name: &str,
        directory: String,
    ) -> io::Result<()> {
        let path = Path::new(&directory).join(format!("{}.cpp", file_name));
        let mut file = File::create(&path)?;
        for line in finalized_template.lines() {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }
}
