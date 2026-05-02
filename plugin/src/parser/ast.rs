use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum AstNode {
    Set(SetBlock),
    Section(Section),
    Drawing(DrawingBlock),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetBlock {
    pub properties: Vec<GlobalProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GlobalProperty {
    Unit(String),
    Scale(f64),
    Stroke(StrokeConfig),
    Font(FontConfig),
    Cover(f64),
    Monochrome(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StrokeConfig {
    pub color: String,
    pub width: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FontConfig {
    pub family: String,
    pub size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Section {
    pub id: String,
    pub properties: SectionProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SectionProperties {
    pub shape: Option<Shape>,
    pub concrete: Option<ConcreteProperties>,
    pub rebar: Vec<RebarEntry>,
    pub ties: Option<StirrupsConfig>,
    pub view: Option<View>,
    pub length: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConcreteProperties {
    pub fc: Option<f64>,
    pub cover: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DrawingBlock {
    pub id: String,
    pub elements: Vec<DrawingElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DrawingElement {
    Primitive(Primitive),
    View(ViewBlock),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ViewBlock {
    pub name: String,
    pub elements: Vec<DrawingElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum View {
    Section,
    Longitudinal,
    Both,
    Top,
    Elevation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Shape {
    Rect { width: f64, height: f64 },
    Circle { diameter: f64 },
    Polygon { points: Vec<(f64, f64)> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RebarEntry {
    pub pattern: RebarPattern,
    pub bars: Vec<BarGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BarGroup {
    pub count: u32,
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RebarPattern {
    Top,
    Bottom,
    Sides,
    Perimeter,
    Grid { spacing: f64 },
    Uniform,
    Path { points: Vec<(f64, f64)> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StirrupsConfig {
    pub size: String,
    pub dist: Vec<Spacing>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Spacing {
    Fixed { count: u32, dist: f64 },
    Rest { dist: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Primitive {
    Rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        rounded: Option<f64>,
    },
    Circle {
        x: f64,
        y: f64,
        radius: f64,
    },
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    },
    Path {
        points: Vec<(f64, f64)>,
        closed: bool,
    },
    Label {
        text: String,
        x: f64,
        y: f64,
        mode: Option<LabelMode>,
    },
    Dimension {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        text: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LabelMode {
    Callout,
    Legend,
}

// Helper enums for grammar parsing
#[derive(Debug, Clone)]
pub enum RawSectionItem {
    Shape(Shape),
    Concrete(ConcreteProperties),
    RebarList(Vec<RebarEntry>),
    Ties(StirrupsConfig),
    View(View),
    Length(f64),
}

#[derive(Debug, Clone)]
pub enum ConcreteProp {
    Fc(f64),
    Cover(f64),
}
