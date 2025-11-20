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
pub enum View {
    Section,
    Longitudinal,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SectionProperties {
    pub shape: Option<Shape>,
    pub cover: Option<f64>, // in cm
    pub span: Option<f64>,  // in cm
    pub view: Option<View>,
    pub scale_section: Option<f64>,
    pub scale_long: Option<f64>,
    pub concrete: Option<f64>,        // fc in kg/cm2
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
    Cover(f64),
    Span(f64),
    Concrete(f64),
    Rebar(RebarEntry),
    Ties(StirrupsConfig),
    View(View),
    Scale(f64),
    ScaleSection(f64),
    ScaleLong(f64),
}
