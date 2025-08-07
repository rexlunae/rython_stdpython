//! Python string module implementation
//! 
//! This module provides string-related constants and template classes.
//! Implementation matches Python's string module API.

use crate::{PyException, python_function};

// String constants
pub const ascii_lowercase: &str = "abcdefghijklmnopqrstuvwxyz";
pub const ascii_uppercase: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ascii_letters: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const digits: &str = "0123456789";
pub const hexdigits: &str = "0123456789abcdefABCDEF";
pub const octdigits: &str = "01234567";
pub const punctuation: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
pub const printable: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\x0b\x0c";
pub const whitespace: &str = " \t\n\r\x0b\x0c";

python_function! {
    /// string.capwords - capitalize words in string
    pub fn capwords<S>(s: S, sep: Option<String>) -> String
    where [S: AsRef<str>]
    [signature: (s, sep=None)]
    [concrete_types: (String, Option<String>) -> String]
    {
        let s = s.as_ref();
        match sep {
            Some(separator) => {
                s.split(&separator)
                    .map(|word| {
                        let mut chars: Vec<char> = word.chars().collect();
                        if let Some(first_char) = chars.first_mut() {
                            *first_char = first_char.to_uppercase().next().unwrap_or(*first_char);
                        }
                        for ch in chars.iter_mut().skip(1) {
                            *ch = ch.to_lowercase().next().unwrap_or(*ch);
                        }
                        chars.into_iter().collect::<String>()
                    })
                    .collect::<Vec<String>>()
                    .join(&separator)
            }
            None => {
                s.split_whitespace()
                    .map(|word| {
                        let mut chars: Vec<char> = word.chars().collect();
                        if let Some(first_char) = chars.first_mut() {
                            *first_char = first_char.to_uppercase().next().unwrap_or(*first_char);
                        }
                        for ch in chars.iter_mut().skip(1) {
                            *ch = ch.to_lowercase().next().unwrap_or(*ch);
                        }
                        chars.into_iter().collect::<String>()
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            }
        }
    }
}

/// Template - simple template substitution
#[derive(Debug, Clone)]
pub struct Template {
    template: String,
    delimiter: char,
}

impl Template {
    /// Create a new template
    pub fn new<S: AsRef<str>>(template: S) -> Self {
        Self {
            template: template.as_ref().to_string(),
            delimiter: '$',
        }
    }
    
    /// Create template with custom delimiter
    pub fn with_delimiter<S: AsRef<str>>(template: S, delimiter: char) -> Self {
        Self {
            template: template.as_ref().to_string(),
            delimiter,
        }
    }
    
    /// Substitute variables from mapping
    pub fn substitute<K, V>(&self, mapping: &[(K, V)]) -> Result<String, PyException>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut result = self.template.clone();
        let mut substituted = std::collections::HashSet::new();
        
        // Find all variable references
        for (key, value) in mapping {
            let key_str = key.as_ref();
            
            // Handle both $var and ${var} forms
            let simple_var = format!("{}{}", self.delimiter, key_str);
            let braced_var = format!("{}{{{}}}", self.delimiter, key_str);
            
            if result.contains(&simple_var) {
                result = result.replace(&simple_var, value.as_ref());
                substituted.insert(key_str.to_string());
            }
            
            if result.contains(&braced_var) {
                result = result.replace(&braced_var, value.as_ref());
                substituted.insert(key_str.to_string());
            }
        }
        
        // Check for unsubstituted variables
        if result.contains(self.delimiter) {
            // Look for remaining variable references
            let chars: Vec<char> = result.chars().collect();
            let mut i = 0;
            while i < chars.len() {
                if chars[i] == self.delimiter {
                    if i + 1 < chars.len() {
                        if chars[i + 1] == '{' {
                            // Find closing brace
                            let mut end = i + 2;
                            while end < chars.len() && chars[end] != '}' {
                                end += 1;
                            }
                            if end < chars.len() {
                                let var_name: String = chars[i+2..end].iter().collect();
                                if !substituted.contains(&var_name) {
                                    return Err(crate::key_error(format!("'{}' variable not provided", var_name)));
                                }
                                i = end + 1;
                                continue;
                            }
                        } else if chars[i + 1].is_ascii_alphabetic() || chars[i + 1] == '_' {
                            // Find end of identifier
                            let mut end = i + 2;
                            while end < chars.len() && (chars[end].is_ascii_alphanumeric() || chars[end] == '_') {
                                end += 1;
                            }
                            let var_name: String = chars[i+1..end].iter().collect();
                            if !substituted.contains(&var_name) {
                                return Err(crate::key_error(format!("'{}' variable not provided", var_name)));
                            }
                            i = end;
                            continue;
                        }
                    }
                }
                i += 1;
            }
        }
        
        Ok(result)
    }
    
    /// Safe substitute - leave unmatched variables as-is
    pub fn safe_substitute<K, V>(&self, mapping: &[(K, V)]) -> String
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut result = self.template.clone();
        
        for (key, value) in mapping {
            let key_str = key.as_ref();
            let simple_var = format!("{}{}", self.delimiter, key_str);
            let braced_var = format!("{}{{{}}}", self.delimiter, key_str);
            
            if result.contains(&simple_var) {
                result = result.replace(&simple_var, value.as_ref());
            }
            
            if result.contains(&braced_var) {
                result = result.replace(&braced_var, value.as_ref());
            }
        }
        
        result
    }
    
    /// Get template string
    pub fn template_string(&self) -> &str {
        &self.template
    }
    
    /// Get delimiter
    pub fn delimiter(&self) -> char {
        self.delimiter
    }
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.template)
    }
}

/// Formatter - string formatting operations
#[derive(Debug)]
pub struct Formatter;

impl Formatter {
    /// Format string with positional arguments
    pub fn format<S: AsRef<str>>(template: S, args: &[&dyn std::fmt::Display]) -> String {
        let template = template.as_ref();
        let mut result = template.to_string();
        
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, &format!("{}", arg));
        }
        
        result
    }
    
    /// Format string with named arguments
    pub fn format_map<S: AsRef<str>, K: AsRef<str>, V: std::fmt::Display>(
        template: S, 
        kwargs: &[(K, V)]
    ) -> String {
        let template = template.as_ref();
        let mut result = template.to_string();
        
        for (key, value) in kwargs {
            let placeholder = format!("{{{}}}", key.as_ref());
            result = result.replace(&placeholder, &format!("{}", value));
        }
        
        result
    }
    
    /// Validate format string
    pub fn vformat<S: AsRef<str>>(template: S) -> Result<Vec<String>, PyException> {
        let template = template.as_ref();
        let mut placeholders = Vec::new();
        let mut chars = template.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '{' {
                if chars.peek() == Some(&'{') {
                    chars.next(); // Skip escaped brace
                    continue;
                }
                
                let mut placeholder = String::new();
                let mut found_end = false;
                
                while let Some(ch) = chars.next() {
                    if ch == '}' {
                        found_end = true;
                        break;
                    }
                    placeholder.push(ch);
                }
                
                if !found_end {
                    return Err(crate::value_error("Unmatched '{' in format string"));
                }
                
                placeholders.push(placeholder);
            } else if ch == '}' {
                if chars.peek() != Some(&'}') {
                    return Err(crate::value_error("Unmatched '}' in format string"));
                }
                chars.next(); // Skip escaped brace
            }
        }
        
        Ok(placeholders)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_capwords() {
        assert_eq!(capwords("hello world", None), "Hello World");
        assert_eq!(capwords("hello-world", Some("-".to_string())), "Hello-World");
        assert_eq!(capwords("HELLO WORLD", None), "Hello World");
    }
    
    #[test]
    fn test_template() {
        let tmpl = Template::new("Hello $name! Welcome to $place.");
        let mapping = [("name", "Alice"), ("place", "Wonderland")];
        
        assert_eq!(
            tmpl.substitute(&mapping).unwrap(),
            "Hello Alice! Welcome to Wonderland."
        );
        
        let tmpl2 = Template::new("Hello ${name}! Welcome to ${place}.");
        assert_eq!(
            tmpl2.substitute(&mapping).unwrap(),
            "Hello Alice! Welcome to Wonderland."
        );
    }
    
    #[test]
    fn test_template_missing_var() {
        let tmpl = Template::new("Hello $name! Welcome to $place.");
        let mapping = [("name", "Alice")]; // missing 'place'
        
        assert!(tmpl.substitute(&mapping).is_err());
    }
    
    #[test]
    fn test_safe_substitute() {
        let tmpl = Template::new("Hello $name! Welcome to $place.");
        let mapping = [("name", "Alice")]; // missing 'place'
        
        assert_eq!(
            tmpl.safe_substitute(&mapping),
            "Hello Alice! Welcome to $place."
        );
    }
}