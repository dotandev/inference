#![warn(clippy::all, clippy::pedantic)]

use std::fs;

use proc_macro2::TokenTree;
use syn::{parse_file, visit::Visit};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct InferenceDocumentationConfig {
    pub working_directory: String,
}

impl InferenceDocumentationConfig {
    pub fn from_cmd_line_args(
        mut args: impl Iterator<Item = String>,
    ) -> Result<InferenceDocumentationConfig, &'static str> {
        args.next();
        let working_directory = args.next().unwrap_or(String::from("."));

        //conver to absolute path
        let working_directory = match std::fs::canonicalize(&working_directory) {
            Ok(path) => path.to_string_lossy().to_string(),
            Err(_) => return Err("Failed to convert to absolute path"),
        };

        if !std::path::Path::new(&working_directory).exists() {
            return Err("Working directory does not exist");
        }

        Ok(InferenceDocumentationConfig { working_directory })
    }
}

struct CommentVisitor {}

impl<'ast> Visit<'ast> for CommentVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        for attr in &item_fn.attrs {
            let res: syn::Expr = match attr.parse_args() {
                Ok(lit_str) => lit_str,
                _ => continue,                
            };
        }
    }
}

// fn extract_comments_from_file<P: AsRef<std::Path>>(path: P) -> Vec<String> {
//     let content = fs::read_to_string(path).expect("Unable to read file");
//     let syntax = parse_file(&content).expect("Unable to parse file");

//     let mut visitor = CommentVisitor { comments: vec![] };
//     visitor.visit_file(&syntax);

//     visitor.comments
// }

pub fn build_inference_documentation(config: &InferenceDocumentationConfig) {
    WalkDir::new(&config.working_directory)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "rs"))
        .for_each(|entry| {
            let file_content = fs::read_to_string(entry.path()).unwrap();
            let rust_file = parse_file(&file_content).unwrap();
            CommentVisitor {}.visit_file(&rust_file);
        });
}
