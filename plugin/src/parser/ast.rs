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
    pub cover: Option<f64>,           // in cm
    pub fc: Option<f64>,              // f'c (renamed from concrete)
    pub rebar: Vec<RebarEntry>,       // Flattened list of rebar lines
    pub ties: Option<StirrupsConfig>, // Unified ties/stirrups
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Shape {
    Rect { width: f64, height: f64 }, // cm
    Circle { diameter: f64 },         // cm
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RebarEntry {
    pub pattern: RebarPattern,
    pub count: u32,
    pub size: String, // e.g., "#3", "1/2\""
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RebarPattern {
    Top,
    Bottom,
    Sides,
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

// Helper enum for parsing mixed properties
#[derive(Debug, Clone)]
pub enum RawProperty {
    Shape(Shape),
    Cover(f64),
    Fc(f64),
    Rebar(RebarEntry),
    TiesStart(String),
    Spacing(Spacing),
}
