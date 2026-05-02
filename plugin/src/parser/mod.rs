pub mod ast;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused_imports)]
    grammar,
    "/parser/grammar.rs"
);

pub fn parse(input: &str) -> Result<Vec<ast::AstNode>, String> {
    let preprocessed = preprocess(input);
    grammar::SectionsParser::new()
        .parse(&preprocessed)
        .map_err(|e| e.to_string())
}

fn preprocess(input: &str) -> String {
    let mut output = String::new();
    let mut indent_stack = vec![0];

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        let indent = line.chars().take_while(|c| c.is_whitespace()).count();

        while indent < *indent_stack.last().unwrap() {
            indent_stack.pop();
            output.push_str("}\n");
        }

        if trimmed.ends_with(":") {
            output.push_str(trimmed);
            output.push_str(" {\n");
            indent_stack.push(indent + 1); // Any increase in indent works
        } else {
            output.push_str(trimmed);
            output.push('\n');
        }
    }

    while indent_stack.len() > 1 {
        indent_stack.pop();
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
