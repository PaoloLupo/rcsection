use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Section {
    pub kind: SectionType,
    pub id: String,
    pub properties: SectionProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SectionType {
    Beam,
    Column,
    Wall,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SectionProperties {
    pub shape: Option<Shape>,
    pub cover: Option<f64>,    // in cm
    pub concrete: Option<f64>, // f'c
    pub rebar: Option<RebarConfig>,
    pub stirrups: Option<StirrupsConfig>,
    pub ties: Option<StirrupsConfig>, // Ties share structure with Stirrups
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Shape {
    Rect { width: f64, height: f64 }, // cm
    Circle { diameter: f64 },         // cm
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RebarConfig {
    pub entries: Vec<RebarEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RebarEntry {
    pub pattern: RebarPattern,
    pub count: u32,
    pub size: String, // e.g., "#3", "1/2\""
    pub layer: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RebarPattern {
    Top,
    Bottom,
    Left,
    Right,
    Perimeter,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StirrupsConfig {
    pub size: String,
    pub dist: Vec<Spacing>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Spacing {
    Fixed { count: u32, dist: f64 }, // dist in cm
    Rest { dist: f64 },              // dist in cm
}

#[derive(Debug, Clone)]
pub enum Property {
    Shape(Shape),
    Cover(f64),
    Concrete(f64),
    Rebar(RebarConfig),
    Stirrups(StirrupsConfig),
    Ties(StirrupsConfig),
}
