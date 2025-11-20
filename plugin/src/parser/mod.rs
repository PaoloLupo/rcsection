pub mod ast;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::all)]
    grammar,
    "/parser/grammar.rs"
);

pub fn parse(input: &str) -> Result<Vec<ast::Section>, String> {
    let preprocessed = preprocess(input);
    grammar::SectionsParser::new()
        .parse(&preprocessed)
        .map_err(|e| e.to_string())
}

fn preprocess(input: &str) -> String {
    let mut output = String::new();
    let mut lines = input.lines();
    let mut current_indent = 0;

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("#") {
            continue;
        }

        let indent = line.chars().take_while(|c| c.is_whitespace()).count();

        // Heuristic: Section headers end with ":"
        if trimmed.ends_with(":") {
            if current_indent > 0 {
                output.push_str("}\n");
                current_indent = 0;
            }
            output.push_str(trimmed);
            output.push_str(" {\n");
            current_indent = 4; // Assume indentation
        } else {
            output.push_str(trimmed);
            output.push_str("\n");
        }
    }

    if current_indent > 0 {
        output.push_str("}\n");
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess() {
        let input = r#"
beam "V-1":
    30 x 60
    top 2 #6
"#;
        let expected = "beam \"V-1\": {\n30 x 60\ntop 2 #6\n}\n";
        assert_eq!(preprocess(input), expected);
    }
}
