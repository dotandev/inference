//! This crate is used to generate documentation for the inference engine.
//! It generates documentation by extracting docstrings and inference specifications from the source code.

#![warn(clippy::all, clippy::pedantic)]

use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, MAIN_SEPARATOR},
};
use syn::{parse_file, spanned::Spanned, visit::Visit, Expr};
use walkdir::WalkDir;

/// Configuration for the inference documentation.
/// `working_directory` is the directory where the source code is located.
/// `output_directory` is the directory where the documentation will be saved.
#[derive(Debug)]
pub struct InferenceDocumentationConfig {
    pub working_directory: String,
    pub output_directory: String,
}

impl InferenceDocumentationConfig {
    pub fn from_cmd_line_args(
        mut args: impl Iterator<Item = String>,
    ) -> Result<InferenceDocumentationConfig, &'static str> {
        args.next();
        let working_directory = args.next().unwrap_or(String::from("."));

        let working_directory = match std::fs::canonicalize(&working_directory) {
            Ok(path) => path.to_string_lossy().to_string(),
            Err(_) => return Err("Failed to convert to absolute path"),
        };

        if !std::path::Path::new(&working_directory).exists() {
            return Err("Working directory does not exist");
        }

        let output_directory = args
            .next()
            .unwrap_or(String::from("./inference_documentation_output"));
        if !std::path::Path::new(&output_directory).exists() {
            if let Err(_) = std::fs::create_dir(&output_directory) {
                return Err("Failed to create output directory");
            }
        }

        Ok(InferenceDocumentationConfig {
            working_directory,
            output_directory,
        })
    }
}

struct DocstringsGrabber<'file_content> {
    file_name: String,
    file_content: &'file_content String,
    fn_loc_map: HashMap<String, (usize, usize, usize, usize)>,
}

impl DocstringsGrabber<'_> {
    fn parse_file_level_docstring(&mut self) -> String {
        let mut lines = self.file_content.lines();
        let mut docstring = String::new();
        while let Some(line) = lines.next() {
            if line.starts_with("//!") {
                let mut docstring_line = line.trim_start_matches("//!").trim().to_string();
                if docstring_line.starts_with("#") {
                    docstring_line = format!("#{}", docstring_line);
                }
                docstring.push_str(docstring_line.as_str());
                docstring.push('\n');
            } else {
                break;
            }
        }
        docstring
    }

    fn parse_fn_docstring(&self, fn_name: String) -> String {
        let line_number = self.fn_loc_map.get(&fn_name).unwrap().0;
        let mut v_docstring = Vec::new();
        for line in self.file_content.lines().rev().skip(self.file_content.lines().count() - line_number - 1).into_iter() {
            if line.starts_with("/") {
                let docstring_line = line.trim_start_matches(|c: char| c == '/').trim().to_string();
                v_docstring.push(docstring_line.clone());
                v_docstring.push(String::from("\n"));
            } else {
                break;
            }
        }
        v_docstring.reverse();
        v_docstring.join("")
    }

    fn save(&mut self, file_root_directory: &String, output_directory: &String) {
        let inner_file_path = self
            .file_name
            .replace(file_root_directory, "")
            .trim_start_matches(MAIN_SEPARATOR)
            .to_string();

        let path = Path::new(output_directory).join(inner_file_path.replace(".rs", ".md"));
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut file = fs::File::create(path).unwrap();
        writeln!(file, "# {}", inner_file_path.replace(MAIN_SEPARATOR, "::")).unwrap();
        writeln!(file, "{}", self.parse_file_level_docstring()).unwrap();
        let mut fn_loc_map: Vec<_> = self.fn_loc_map.iter().collect();
        fn_loc_map.sort_by(|a, b| a.1.0.cmp(&b.1.0));
        for (item_name, loc) in fn_loc_map {
            writeln!(
                file,
                "### {}: {}",
                item_name,
                format!("[{}:{} - {}:{}]", loc.0, loc.1, loc.2, loc.3)
            )
            .unwrap();
            writeln!(file, "---").unwrap();
            writeln!(file, "{}", self.parse_fn_docstring(item_name.clone())).unwrap();
        }
    }
}

impl<'ast, 'file_content> Visit<'ast> for DocstringsGrabber<'file_content> {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let fn_name = item_fn.sig.ident.to_string();
        let span_start = item_fn.span().start();
        let span_end = item_fn.span().end();
        self.fn_loc_map.insert(
            fn_name,
            (
                span_start.line,
                span_start.column,
                span_end.line,
                span_end.column,
            ),
        );
        syn::visit::visit_item_fn(self, item_fn);
    }

    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        for attr in &item_mod.attrs {
            if attr.path().is_ident("inference_spec") {
                let _: Expr = attr.parse_args().unwrap();
            }
        }
        syn::visit::visit_item_mod(self, item_mod);
    }

    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        syn::visit::visit_macro(self, i);
    }
}

pub fn build_inference_documentation(config: &InferenceDocumentationConfig) {
    WalkDir::new(&config.working_directory)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "rs"))
        .for_each(|entry| {
            let file_content = fs::read_to_string(entry.path()).unwrap();
            let rust_file = parse_file(&file_content).unwrap();
            let mut visitor = DocstringsGrabber {
                file_name: String::from(entry.path().to_str().unwrap()),
                file_content: &file_content,
                fn_loc_map: HashMap::new(),
            };
            visitor.visit_file(&rust_file);
            visitor.save(&config.working_directory, &config.output_directory);
        });
}
