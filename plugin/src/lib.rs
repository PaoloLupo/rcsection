mod geometry;
mod parser;

#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::wasm_func;

fn cbor_encode<T>(value: &T) -> Result<Vec<u8>, ciborium::ser::Error<std::io::Error>>
where
    T: serde::Serialize + ?Sized,
{
    let mut writer = Vec::new();
    ciborium::into_writer(value, &mut writer)?;
    Ok(writer)
}

trait MapErrToString<T> {
    fn map_err_to_string(self) -> Result<T, String>;
}

impl<T, E: ToString> MapErrToString<T> for Result<T, E> {
    fn map_err_to_string(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}

#[cfg(target_arch = "wasm32")]
wasm_minimal_protocol::initiate_protocol!();

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn priv_parse(expr: &[u8]) -> Result<Vec<u8>, String> {
    let expr: String = ciborium::from_reader(expr).map_err_to_string()?;
    let nodes = parser::parse(&expr).map_err_to_string()?;
    let expr = cbor_encode(&nodes).map_err_to_string()?;
    Ok(expr)
}

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn priv_parse_and_generate(expr: &[u8]) -> Result<Vec<u8>, String> {
    let expr: String = ciborium::from_reader(expr).map_err_to_string()?;
    let nodes = parser::parse(&expr).map_err_to_string()?;

    let drawings = geometry::generate(&nodes);

    let expr = cbor_encode(&drawings).map_err_to_string()?;
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::AstNode;

    #[test]
    fn test_parse_section() {
        let input = r#"
            section "V-101":
                shape rect 30 60
                top 2 #6
                bot 3 #8
        "#;
        let result = parser::parse(input);
        assert!(result.is_ok());
        let nodes = result.unwrap();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            AstNode::Section(section) => {
                assert_eq!(section.id, "V-101");
            }
            _ => panic!("Expected Section node"),
        }
    }

    #[test]
    fn test_parse_set_block() {
        let input = r#"
            set:
                unit "cm"
                scale 1:20
                cover 4
        "#;
        let result = parser::parse(input);
        assert!(result.is_ok(), "Parse error: {:?}", result.err());
        let nodes = result.unwrap();
        assert_eq!(nodes.len(), 1);
        assert!(matches!(&nodes[0], AstNode::Set(_)));
    }
}
