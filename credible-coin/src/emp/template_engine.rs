use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::sync::Mutex;
lazy_static! {
    static ref WRITE_LOCK: Mutex<()> = Mutex::new(());
}
/// A simple template engine which handles dynamic ad-hoc c++ script generation
#[derive(Debug)]
pub struct TemplateEngine {}

impl TemplateEngine {
    /// Constructs a new instance of the `TemplateEngine`.
    ///
    /// This function initializes and returns a new `TemplateEngine` object.
    /// ```rust
    /// use credible_coin::emp::template_engine::TemplateEngine;
    /// let engine = TemplateEngine::new();
    /// ```

    pub fn new() -> Self {
        TemplateEngine {}
    }
    /// Renders a template by replacing placeholders with their corresponding
    /// values.
    ///
    /// The function looks for placeholders in the format `<<key_name>>` within
    /// the provided `template` and replaces them with their corresponding
    /// values from the `placeholders` HashMap.
    ///
    /// # Arguments
    ///
    /// * `template`: The template string containing placeholders to be
    ///   replaced.
    /// * `placeholders`: A HashMap where the key is the placeholder name
    ///   (without the surrounding `<< >>`) and the value is the string to
    ///   replace the placeholder with.
    ///
    /// # Returns
    ///
    /// A new `String` where all placeholders in the `template` have been
    /// replaced with their corresponding values from the `placeholders`
    /// HashMap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use credible_coin::emp::template_engine::TemplateEngine;
    /// let mut placeholders = HashMap::new();
    /// placeholders.insert("name".to_string(), "Alice".to_string());
    ///
    /// let template = "Hello, <<name>>!";
    /// let engine = TemplateEngine::new();
    /// let rendered = engine.render(&template, placeholders);
    ///
    /// assert_eq!(rendered, "Hello, Alice!");
    /// ```
    ///
    pub fn render(&self, template: &str, placeholders: HashMap<String, String>) -> String {
        let mut output = String::from(template);
        for (key, value) in placeholders {
            let token = format!("<<{}>>", key);
            output = output.replace(&token, &value);
        }
        output
    }

    /// Writes a given string (`finalized_template`) to a file with the
    /// specified `file_name` within the provided `directory`.
    ///
    /// The output file will have a `.cpp` extension.
    ///
    /// # Arguments
    ///
    /// * `finalized_template`: The content to write to the file.
    /// * `file_name`: The name of the file (without extension) to which the
    ///   content should be written.
    /// * `directory`: The directory in which the file will be created. Can be
    ///   passed as a `&str`, `String`, or any type that implements
    ///   `AsRef<Path>`.
    ///
    /// # Returns
    ///
    /// * An `io::Result<()>` indicating the success or failure of the write
    ///   operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::path::Path;
    /// use credible_coin::emp;
    /// let content = "#include<iostream>\nint main() { std::cout << \"Hello, world!\"; return 0; }";
    /// let result = emp::template_engine::TemplateEngine::write_to_file(content, "hello_world", Path::new("foo.txt"));
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    ///
    /// * The path cannot be created or accessed.
    /// * Writing to the file fails.
    ///
    pub fn write_to_file<P: AsRef<Path>>(
        finalized_template: &str,
        file_name: &str,
        directory: P,
    ) -> io::Result<()> {
        let _lock: std::sync::MutexGuard<'_, ()> = WRITE_LOCK.lock().unwrap();
        let path = directory.as_ref().join(format!("{}.cpp", file_name));
        let mut file = File::create(&path)?;
        file.write_all(finalized_template.as_bytes())?;
        Ok(())
    }
}
