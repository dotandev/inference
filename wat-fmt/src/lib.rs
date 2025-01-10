#![no_std]
#![warn(clippy::pedantic)]
extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

/// A library for pretty-formatting WAT (WebAssembly text) files.
///
/// # Note about `no_std`
/// This code uses `alloc` to provide `String`, `Vec`, and other heap-allocated
/// structures. It does not rely on the standard library (`std`), so it should
/// be compatible with `no_std` environments that provide an allocator.
///
/// # Examples
///
/// ```ignore
/// // These examples use `println!`, which is part of `std`.
/// // If you're in a `no_std` environment, remove or rewrite them accordingly.
///
/// /// use wat_fmt::{format, format_with_indent};
///
/// let unformatted = "(module (func (param i32) (result i32) (i32.add (i32.const 1)(i32.const 2))))";
///
/// // Use default indentation (2 spaces):
/// let pretty_default = format(unformatted);
/// // println!("{}", pretty_default);
///
/// // Use custom indentation (4 spaces):
/// let pretty_4_spaces = format_with_indent(unformatted, 4);
/// // println!("{}", pretty_4_spaces);
/// ```
///
/// Pretty-format a WAT string using a default indentation of 2 spaces.
///
/// # Arguments
/// * `input` - A string slice containing unformatted or poorly formatted WAT code.
///
/// # Returns
/// * A new `String` containing the pretty-formatted WAT code.
#[must_use]
pub fn format(input: &str) -> String {
    format_with_indent(input, 2)
}

/// Pretty-format a WAT string using a configurable indentation level.
///
/// # Arguments
/// * `input` - A string slice containing unformatted or poorly formatted WAT code.
/// * `indent_size` - The number of spaces used per indentation level.
///
/// # Returns
/// * A new `String` containing the pretty-formatted WAT code.
#[must_use]
pub fn format_with_indent(input: &str, indent_size: usize) -> String {
    let tokens = tokenize(input);
    format_tokens(&tokens, indent_size)
}

/// Splits the input into tokens (parentheses and sequences of non-whitespace, non-parenthesis chars).
fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    // A simple tokenizer that separates '(', ')' and sequences of other characters
    for c in input.chars() {
        match c {
            '(' | ')' => {
                // If we were building a token, push it before the parenthesis
                if !current.trim().is_empty() {
                    tokens.push(current.trim().to_string());
                }
                current.clear();
                // Push the parenthesis as a separate token
                tokens.push(c.to_string());
            }
            ' ' | '\n' | '\r' | '\t' => {
                // Whitespace boundary: push current token if it's non-empty
                if !current.trim().is_empty() {
                    tokens.push(current.trim().to_string());
                }
                current.clear();
            }
            _ => {
                // Keep building the current token
                current.push(c);
            }
        }
    }
    // If there's any leftover token, push it
    if !current.trim().is_empty() {
        tokens.push(current.trim().to_string());
    }
    tokens
}

/// Formats tokens into a multiline string with proper indentation.
fn format_tokens(tokens: &[String], indent_size: usize) -> String {
    let mut result = String::new();
    let mut current_indent = 0;
    let mut start_of_line = true;
    let mut block_just_opened = false;

    let block_open_names = [
        "module", "func", "block", "loop", "if", "else", "then", "forall", "exists", "unique",
        "assume",
    ];
    let mut blocks_stack: Vec<String> = Vec::new();

    for token in tokens {
        match token.as_str() {
            "(" => {
                // Write a newline + indent before '(' if not at start of line
                if !start_of_line {
                    result.push('\n');
                }
                // Expand capacity for spaces
                for _ in 0..(current_indent * indent_size) {
                    result.push(' ');
                }
                result.push('(');
                current_indent += 1;
                start_of_line = false; // Just wrote '('
                block_just_opened = true;
            }
            ")" => {
                let current_block = blocks_stack.pop().unwrap_or_default();
                if block_open_names.contains(&current_block.as_str()) {
                    // Decrease indent before writing ')'
                    current_indent = current_indent.saturating_sub(1);
                    result.push('\n');
                    for _ in 0..(current_indent * indent_size) {
                        result.push(' ');
                    }
                    result.push(')');
                    start_of_line = false; // Just wrote ')'
                } else {
                    result.push(')');
                }
            }
            _ => {
                // Normal token
                if block_just_opened {
                    block_just_opened = false;
                    result.push_str(token);
                    blocks_stack.push(token.clone());
                } else {
                    // If it's the start of a line, indent; otherwise, add a space
                    if start_of_line {
                        for _ in 0..(current_indent * indent_size) {
                            result.push(' ');
                        }
                    } else {
                        result.push(' ');
                    }
                    result.push_str(token);
                    start_of_line = false;
                }
            }
        }
    }

    result
}
