pub mod ast;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::all)]
    grammar,
    "/parser/grammar.rs"
);

pub fn parse(input: &str) -> Result<Vec<ast::Section>, String> {
    grammar::SectionsParser::new()
        .parse(input)
        .map_err(|e| e.to_string())
}
