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
    let sections = parser::parse(&expr).map_err_to_string()?;
    let expr = cbor_encode(&sections).map_err_to_string()?;
    Ok(expr)
}

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn priv_parse_and_generate(expr: &[u8]) -> Result<Vec<u8>, String> {
    let expr: String = ciborium::from_reader(expr).map_err_to_string()?;
    let sections = parser::parse(&expr).map_err_to_string()?;

    let drawings: Vec<geometry::Drawing> = sections
        .iter()
        .flat_map(|s| geometry::generate(s))
        .collect();

    let expr = cbor_encode(&drawings).map_err_to_string()?;
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_beam() {
        let input = r#"
            beam "V-101":
                30 x 60
                cover 4
                fc 210
                top 2 1/2"
                bot 3 1"
                bot 2 3/4"
                ties 3/8" 1@5 5@10 rto@20
        "#;
        let result = parser::parse(input);
        assert!(result.is_ok());
        let sections = result.unwrap();
        assert_eq!(sections.len(), 1);
        let section = &sections[0];
        assert_eq!(section.id, "V-101");
    }

    #[test]
    fn test_parse_column() {
        let input = r#"
            column "C-Esq":
                40 x 40
                cover 4
                fc 280
                perim 12 #6
                ties #3 rto@15
        "#;
        let result = parser::parse(input);
        assert!(result.is_ok());
    }
}
