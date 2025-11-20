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

    let drawings: Vec<geometry::Drawing> = sections.iter().map(|s| geometry::generate(s)).collect();

    let expr = cbor_encode(&drawings).map_err_to_string()?;
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_beam() {
        let input = r#"
            Beam "V-101" {
                Shape: Rect(30cm, 60cm);
                Cover: 4cm;
                Concrete: 210;
                Rebar {
                    Top: 2 x 1/2";
                    Bottom: 3 x 1";
                    Bottom: 2 x 3/4" layer 1;
                }
                Stirrups {
                    Size: 3/8";
                    Dist: 1@5cm, 5@10cm, Rest@20cm;
                }
            }
        "#;
        let result = parser::parse(input);
        assert!(result.is_ok());
        let sections = result.unwrap();
        assert_eq!(sections.len(), 1);
        let section = &sections[0];
        assert_eq!(section.id, "V-101");
        // Add more assertions as needed
    }

    #[test]
    fn test_parse_column() {
        let input = r#"
            Column "C-Esq" {
                Shape: Rect(40cm, 40cm);
                Cover: 4cm;
                Concrete: 280;
                Rebar {
                    Perimeter: 12 x #6;
                }
                Ties {
                    Size: #3;
                    Dist: Rest@15cm;
                }
            }
        "#;
        let result = parser::parse(input);
        assert!(result.is_ok());
    }
}
