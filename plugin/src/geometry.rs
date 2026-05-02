use crate::parser::ast::{self, View};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drawing {
    pub id: Option<String>,
    pub view: Option<String>, // "section", "longitudinal", etc.
    pub scale: Option<f64>,
    pub primitives: Vec<Primitive>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Primitive {
    Rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        rounded: Option<f64>,
        stroke: Option<Stroke>,
        fill: Option<String>,
        group: Option<String>,
    },
    Circle {
        x: f64,
        y: f64,
        radius: f64,
        stroke: Option<Stroke>,
        fill: Option<String>,
        group: Option<String>,
    },
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        stroke: Option<Stroke>,
        group: Option<String>,
    },
    Path {
        points: Vec<(f64, f64)>,
        closed: bool,
        stroke: Option<Stroke>,
        fill: Option<String>,
        group: Option<String>,
    },
    Text {
        x: f64,
        y: f64,
        content: String,
        group: Option<String>,
    },
    Dimension {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        text: Option<String>,
        group: Option<String>,
    },
    /// Leader line with arrow for rebar callouts (ACI style)
    LeaderLine {
        /// Start point (near the rebar group)
        start: (f64, f64),
        /// End point (where text is placed)
        end: (f64, f64),
        /// Annotation text (e.g., "3#6" or "2 3/8\"")
        text: String,
        /// Which side the callout points to (for text anchor)
        side: String,
        group: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub color: String,
    pub width: f64,
}

impl Drawing {
    pub fn new() -> Self {
        Self {
            id: None,
            view: None,
            scale: None,
            primitives: Vec::new(),
        }
    }

    pub fn add(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }
}

pub fn generate(nodes: &[ast::AstNode]) -> Vec<Drawing> {
    let mut drawings = Vec::new();
    let mut global_settings = GlobalSettings::default();

    for node in nodes {
        match node {
            ast::AstNode::Set(set) => {
                for prop in &set.properties {
                    match prop {
                        ast::GlobalProperty::Scale(s) => global_settings.scale = Some(*s),
                        ast::GlobalProperty::Cover(c) => global_settings.cover = *c,
                        ast::GlobalProperty::Stroke(s) => {
                            global_settings.stroke = Some(Stroke {
                                color: s.color.clone(),
                                width: s.width,
                            })
                        }
                        ast::GlobalProperty::Unit(u) => {
                            global_settings.unit_factor = parse_unit_factor(u);
                        }
                        ast::GlobalProperty::Style(s) => {
                            global_settings.style = parse_style_preset(s);
                            if global_settings.style == StylePreset::Spd {
                                global_settings.monochrome = true;
                            }
                        }
                        ast::GlobalProperty::Monochrome(m) => {
                            global_settings.monochrome = *m;
                        }
                        _ => {}
                    }
                }
            }
            ast::AstNode::Section(section) => {
                drawings.extend(generate_section(section, &global_settings));
            }
            ast::AstNode::Drawing(drawing_block) => {
                let mut d = Drawing::new();
                d.id = Some(drawing_block.id.clone());
                for element in &drawing_block.elements {
                    process_drawing_element(&mut d, element);
                }
                drawings.push(d);
            }
        }
    }

    drawings
}

struct GlobalSettings {
    scale: Option<f64>,
    cover: f64,
    stroke: Option<Stroke>,
    unit_factor: f64,
    monochrome: bool,
    style: StylePreset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StylePreset {
    Default,
    Spd,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            scale: None,
            cover: 0.0,
            stroke: None,
            unit_factor: 1.0,
            monochrome: true,
            style: StylePreset::Spd,
        }
    }
}

fn parse_style_preset(style: &str) -> StylePreset {
    match style.to_lowercase().as_str() {
        "spd" | "gost" | "tech" | "technical" | "professional" => StylePreset::Spd,
        _ => StylePreset::Default,
    }
}

fn concrete_stroke(settings: &GlobalSettings) -> Stroke {
    match settings.style {
        StylePreset::Spd => Stroke {
            color: "black".to_string(),
            width: 0.12,
        },
        StylePreset::Default => Stroke {
            color: "black".to_string(),
            width: 0.08,
        },
    }
}

fn stirrup_outline_stroke(settings: &GlobalSettings, color: String) -> Stroke {
    match settings.style {
        StylePreset::Spd => Stroke { color, width: 0.05 },
        StylePreset::Default => Stroke { color, width: 0.03 },
    }
}

fn rebar_visual(settings: &GlobalSettings, bar_color: String) -> (Stroke, Option<String>) {
    match settings.style {
        StylePreset::Spd => (
            Stroke {
                color: "black".to_string(),
                width: 0.06,
            },
            None,
        ),
        StylePreset::Default => {
            if settings.monochrome {
                (
                    Stroke {
                        color: "black".to_string(),
                        width: 0.05,
                    },
                    None,
                )
            } else {
                (
                    Stroke {
                        color: bar_color.clone(),
                        width: 0.05,
                    },
                    Some(bar_color),
                )
            }
        }
    }
}

fn rebar_line_stroke(settings: &GlobalSettings, bar_color: String, width: f64) -> Stroke {
    match settings.style {
        StylePreset::Spd => Stroke {
            color: "black".to_string(),
            width: 0.08,
        },
        StylePreset::Default => {
            if settings.monochrome {
                Stroke {
                    color: "black".to_string(),
                    width: 0.08,
                }
            } else {
                Stroke {
                    color: bar_color,
                    width,
                }
            }
        }
    }
}

fn parse_unit_factor(unit: &str) -> f64 {
    match unit.to_lowercase().as_str() {
        "mm" => 0.1,
        "cm" => 1.0,
        "m" => 100.0,
        "in" | "\"" => 2.54,
        "ft" | "'" => 30.48,
        _ => 1.0,
    }
}

fn apply_unit(value: f64, factor: f64) -> f64 {
    value * factor
}

fn process_drawing_element(drawing: &mut Drawing, element: &ast::DrawingElement) {
    match element {
        ast::DrawingElement::Primitive(p) => {
            drawing.add(map_primitive(p));
        }
        ast::DrawingElement::View(view) => {
            // For now, flatten views into the same drawing or handle as groups
            for el in &view.elements {
                process_drawing_element(drawing, el);
            }
        }
    }
}

fn map_primitive(p: &ast::Primitive) -> Primitive {
    match p {
        ast::Primitive::Rect {
            x,
            y,
            width,
            height,
            rounded,
        } => Primitive::Rect {
            x: *x,
            y: *y,
            width: *width,
            height: *height,
            rounded: *rounded,
            stroke: Some(Stroke {
                color: "black".to_string(),
                width: 1.0,
            }),
            fill: None,
            group: None,
        },
        ast::Primitive::Circle { x, y, radius } => Primitive::Circle {
            x: *x,
            y: *y,
            radius: *radius,
            stroke: Some(Stroke {
                color: "black".to_string(),
                width: 1.0,
            }),
            fill: None,
            group: None,
        },
        ast::Primitive::Line { x1, y1, x2, y2 } => Primitive::Line {
            x1: *x1,
            y1: *y1,
            x2: *x2,
            y2: *y2,
            stroke: Some(Stroke {
                color: "black".to_string(),
                width: 1.0,
            }),
            group: None,
        },
        ast::Primitive::Path { points, closed } => Primitive::Path {
            points: points.clone(),
            closed: *closed,
            stroke: Some(Stroke {
                color: "black".to_string(),
                width: 1.0,
            }),
            fill: None,
            group: None,
        },
        ast::Primitive::Label { text, x, y, .. } => Primitive::Text {
            x: *x,
            y: *y,
            content: text.clone(),
            group: None,
        },
        ast::Primitive::Dimension {
            x1,
            y1,
            x2,
            y2,
            text,
        } => Primitive::Dimension {
            x1: *x1,
            y1: *y1,
            x2: *x2,
            y2: *y2,
            text: text.clone(),
            group: None,
        },
    }
}

fn get_section_dims(shape: &Option<ast::Shape>, factor: f64) -> (f64, f64) {
    let (w, h) = match shape {
        Some(ast::Shape::Rect { width, height }) => (*width, *height),
        Some(ast::Shape::Circle { diameter }) => (*diameter, *diameter),
        _ => (30.0, 60.0),
    };
    (apply_unit(w, factor), apply_unit(h, factor))
}

fn get_cover(props: &ast::SectionProperties, settings: &GlobalSettings) -> f64 {
    let cover = props
        .concrete
        .as_ref()
        .and_then(|c| c.cover)
        .unwrap_or(settings.cover);
    apply_unit(cover.max(2.5), settings.unit_factor)
}

fn get_stirrup_info(props: &ast::SectionProperties) -> (String, f64) {
    let size = props
        .ties
        .as_ref()
        .map(|t| t.size.clone())
        .unwrap_or_else(|| "#3".to_string());
    let radius = parse_size(&size) / 2.0;
    (size, radius)
}

fn generate_section(section: &ast::Section, settings: &GlobalSettings) -> Vec<Drawing> {
    let mut drawings = Vec::new();
    let props = &section.properties;

    // Determine which views to generate
    let show_section = matches!(&props.view, Some(View::Section) | Some(View::Both) | None);
    let show_longitudinal = matches!(&props.view, Some(View::Longitudinal) | Some(View::Both));
    let show_elevation = matches!(&props.view, Some(View::Elevation) | Some(View::Both));

    // Get dimensions for both views
    let (width, height) = get_section_dims(&props.shape, settings.unit_factor);
    let cover = get_cover(props, settings);

    // === Section View (Cross-section) ===
    if show_section {
        let mut d = Drawing::new();
        d.id = Some(section.id.clone());
        d.view = Some("section".to_string());
        d.scale = settings.scale;

        if let Some(shape) = &props.shape {
            match shape {
                ast::Shape::Rect { width, height } => {
                    d.add(Primitive::Rect {
                        x: -width / 2.0,
                        y: -height / 2.0,
                        width: *width,
                        height: *height,
                        rounded: None,
                        stroke: Some(concrete_stroke(settings)),
                        fill: None,
                        group: Some("concrete".to_string()),
                    });
                }
                ast::Shape::Circle { diameter } => {
                    d.add(Primitive::Circle {
                        x: 0.0,
                        y: 0.0,
                        radius: diameter / 2.0,
                        stroke: Some(concrete_stroke(settings)),
                        fill: None,
                        group: Some("concrete".to_string()),
                    });
                }
                ast::Shape::Polygon { points } => {
                    d.add(Primitive::Path {
                        points: points.clone(),
                        closed: true,
                        stroke: Some(concrete_stroke(settings)),
                        fill: None,
                        group: Some("concrete".to_string()),
                    });
                }
            }
        }

        // Add stirrup in section view
        if let Some(ties) = &props.ties {
            let tie_color = if settings.monochrome || settings.style == StylePreset::Spd {
                "black".to_string()
            } else {
                get_color_for_size(&ties.size)
            };
            let tie_thickness = parse_size(&ties.size);
            let bend_radius = get_tie_bend_radius(&ties.size);
            if let Some(shape) = &props.shape {
                match shape {
                    ast::Shape::Rect { width, height } => {
                        let outline = stirrup_outline_stroke(settings, tie_color.clone());
                        // Outer contour of stirrup
                        d.add(Primitive::Rect {
                            x: -width / 2.0 + cover,
                            y: -height / 2.0 + cover,
                            width: width - 2.0 * cover,
                            height: height - 2.0 * cover,
                            rounded: Some(bend_radius),
                            stroke: Some(outline.clone()),
                            fill: None,
                            group: Some("stirrup".to_string()),
                        });
                        // Inner contour of stirrup (hole)
                        d.add(Primitive::Rect {
                            x: -width / 2.0 + cover + tie_thickness,
                            y: -height / 2.0 + cover + tie_thickness,
                            width: width - 2.0 * cover - 2.0 * tie_thickness,
                            height: height - 2.0 * cover - 2.0 * tie_thickness,
                            rounded: Some(bend_radius.max(0.0)),
                            stroke: Some(outline),
                            fill: None,
                            group: Some("stirrup".to_string()),
                        });
                    }
                    ast::Shape::Circle { diameter } => {
                        let outline = stirrup_outline_stroke(settings, tie_color.clone());
                        // Outer contour
                        d.add(Primitive::Circle {
                            x: 0.0,
                            y: 0.0,
                            radius: diameter / 2.0 - cover,
                            stroke: Some(outline.clone()),
                            fill: None,
                            group: Some("stirrup".to_string()),
                        });
                        // Inner contour (hole)
                        d.add(Primitive::Circle {
                            x: 0.0,
                            y: 0.0,
                            radius: (diameter / 2.0 - cover - tie_thickness).max(0.0),
                            stroke: Some(outline),
                            fill: None,
                            group: Some("stirrup".to_string()),
                        });
                    }
                    _ => {}
                }
            }
        }

        // Add rebar circles in section view
        for rebar in &props.rebar {
            let (_stirrup_size, _stirrup_radius) = get_stirrup_info(props);

            let positions = get_rebar_positions(rebar, props, settings);

            // Expand bar groups to match positions: [(size, color), ...]
            let bar_specs: Vec<_> = rebar
                .bars
                .iter()
                .flat_map(|b| {
                    let radius = parse_size(&b.size) / 2.0;
                    let color = get_color_for_size(&b.size);
                    std::iter::repeat_n((radius, color), b.count as usize)
                })
                .collect();

            // Draw each bar with its correct size and color
            for (i, (x, y)) in positions.iter().enumerate() {
                let (radius, color) = bar_specs
                    .get(i)
                    .cloned()
                    .unwrap_or((1.0, "black".to_string()));
                let (stroke, fill_color) = rebar_visual(settings, color);

                d.add(Primitive::Circle {
                    x: *x,
                    y: *y,
                    radius,
                    stroke: Some(stroke),
                    fill: fill_color,
                    group: Some("rebar".to_string()),
                });
            }

            // Generate callout for this rebar group
            if !positions.is_empty() {
                // Pick the best bar position for the arrow origin based on pattern
                let (arrow_origin, side) = match rebar.pattern {
                    ast::RebarPattern::Top => {
                        // Use rightmost bar for top callout
                        let pos = positions
                            .iter()
                            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                            .unwrap();
                        (*pos, "top")
                    }
                    ast::RebarPattern::Bottom => {
                        // Use rightmost bar for bottom callout
                        let pos = positions
                            .iter()
                            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                            .unwrap();
                        (*pos, "bottom")
                    }
                    ast::RebarPattern::Sides => {
                        // Use topmost bar on right side
                        let pos = positions
                            .iter()
                            .filter(|(x, _)| *x > 0.0)
                            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                            .unwrap_or(positions.first().unwrap());
                        (*pos, "right")
                    }
                    ast::RebarPattern::Perimeter => {
                        // Use top-right corner bar
                        let pos = positions
                            .iter()
                            .max_by(|a, b| (a.0 + a.1).partial_cmp(&(b.0 + b.1)).unwrap())
                            .unwrap();
                        (*pos, "top-right")
                    }
                    _ => (*positions.first().unwrap(), "right"),
                };

                // Format text: preserve original size notation
                let callout_text = format_rebar_callout(&rebar.bars);

                // Calculate end point - use absolute offsets to ensure arrow extends outside section
                let (offset_x, offset_y) = match rebar.pattern {
                    ast::RebarPattern::Top => {
                        (10.0_f64.max(width * 0.15), 8.0_f64.max(height * 0.1))
                    }
                    ast::RebarPattern::Bottom => {
                        (10.0_f64.max(width * 0.15), -8.0_f64.min(-height * 0.1))
                    }
                    ast::RebarPattern::Sides => {
                        (10.0_f64.max(width * 0.15), 5.0_f64.max(height * 0.05))
                    }
                    ast::RebarPattern::Perimeter => {
                        (10.0_f64.max(width * 0.15), 8.0_f64.max(height * 0.1))
                    }
                    _ => (10.0, 0.0),
                };

                let end_x = arrow_origin.0 + offset_x;
                let end_y = arrow_origin.1 + offset_y;

                d.add(Primitive::LeaderLine {
                    start: (arrow_origin.0, arrow_origin.1),
                    end: (end_x, end_y),
                    text: callout_text,
                    side: side.to_string(),
                    group: Some("callout".to_string()),
                });
            }
        }

        // Add section dimensions (width at bottom, height at left) - ONLY FOR RECT SHAPES
        if let Some(ast::Shape::Rect { .. }) = &props.shape {
            let dim_offset = 8.0; // Distance from section edge to dimension line

            // Width dimension (bottom)
            d.add(Primitive::Dimension {
                x1: -width / 2.0,
                y1: -height / 2.0 - dim_offset,
                x2: width / 2.0,
                y2: -height / 2.0 - dim_offset,
                text: Some(format!("{:.0}", width)),
                group: Some("dimension".to_string()),
            });

            // Height dimension (left)
            d.add(Primitive::Dimension {
                x1: -width / 2.0 - dim_offset,
                y1: -height / 2.0,
                x2: -width / 2.0 - dim_offset,
                y2: height / 2.0,
                text: Some(format!("{:.0}", height)),
                group: Some("dimension".to_string()),
            });
        }

        drawings.push(d);
    }

    // === Longitudinal View (Side elevation / Beam elevation) ===
    if show_longitudinal {
        drawings.push(generate_longitudinal_drawing(
            section,
            props,
            settings,
            "longitudinal",
            false,
        ));
    }

    // === Vertical Elevation (Column elevation) ===
    if show_elevation {
        drawings.push(generate_longitudinal_drawing(
            section,
            props,
            settings,
            "elevation",
            true,
        ));
    }

    drawings
}

fn generate_longitudinal_drawing(
    section: &ast::Section,
    props: &ast::SectionProperties,
    settings: &GlobalSettings,
    view_name: &str,
    is_vertical: bool,
) -> Drawing {
    let mut d = Drawing::new();
    d.id = Some(format!(
        "{} ({})",
        section.id,
        if is_vertical { "Elev." } else { "Long." }
    ));
    d.view = Some(view_name.to_string());
    d.scale = settings.scale;

    let (width, height) = get_section_dims(&props.shape, settings.unit_factor);
    let cover = get_cover(props, settings);
    let span = apply_unit(props.length.unwrap_or(200.0), settings.unit_factor);

    if is_vertical {
        // --- VERTICAL ORIENTATION (Column) ---
        // Concrete outline
        d.add(Primitive::Rect {
            x: -width / 2.0,
            y: 0.0,
            width,
            height: span,
            rounded: None,
            stroke: Some(concrete_stroke(settings)),
            fill: None,
            group: Some("concrete".to_string()),
        });

        // Rebar
        for rebar in &props.rebar {
            let bar_size = rebar
                .bars
                .first()
                .map(|b| b.size.clone())
                .unwrap_or("#4".to_string());
            let bar_thickness = parse_size(&bar_size);
            let bar_color = get_color_for_size(&bar_size);

            let positions = get_rebar_positions(rebar, props, settings);

            // Project X coordinates (for vertical elevation)
            let mut unique_x: Vec<f64> = positions.iter().map(|p| p.0).collect();
            unique_x.sort_by(|a, b| a.partial_cmp(b).unwrap());
            unique_x.dedup_by(|a, b| (*a - *b).abs() < 0.1);

            for x_pos in &unique_x {
                d.add(Primitive::Path {
                    points: vec![(*x_pos, cover), (*x_pos, span - cover)],
                    closed: false,
                    stroke: Some(rebar_line_stroke(settings, bar_color.clone(), bar_thickness)),
                    fill: None,
                    group: Some("rebar".to_string()),
                });
            }

            // Callout: pointing horizontally from the side of the bar
            if let Some(first_pos) = positions
                .iter()
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            {
                let callout_text = format_rebar_callout(&rebar.bars);
                let x_pos = first_pos.0;
                let (offset_x, side) = if x_pos < 0.0 {
                    (-width * 0.4 - 10.0, "left") // Left side, text anchored east
                } else {
                    (width * 0.4 + 10.0, "right") // Right side, text anchored west
                };

                // Position callout at 80% of the column height
                let y_anchor = span * 0.8;
                d.add(Primitive::LeaderLine {
                    start: (x_pos, y_anchor),
                    end: (x_pos + offset_x, y_anchor),
                    text: callout_text,
                    side: side.to_string(),
                    group: Some("callout".to_string()),
                });
            }
        }

        // Stirrups
        if let Some(ties) = &props.ties {
            let tie_color = if settings.monochrome || settings.style == StylePreset::Spd {
                "black".to_string()
            } else {
                get_color_for_size(&ties.size)
            };
            let tie_thickness = parse_size(&ties.size);
            let x_min = -width / 2.0 + cover + tie_thickness / 2.0;
            let x_max = width / 2.0 - cover - tie_thickness / 2.0;

            let positions = calculate_longitudinal_spacings(span, cover, ties, settings.unit_factor);
            for y in positions {
                d.add(Primitive::Path {
                    points: vec![(x_min, y), (x_max, y)],
                    closed: false,
                    stroke: Some(stirrup_outline_stroke(settings, tie_color.clone())),
                    fill: None,
                    group: Some("stirrup".to_string()),
                });
            }
        }
    } else {
        // --- HORIZONTAL ORIENTATION (Beam) ---
        d.add(Primitive::Rect {
            x: 0.0,
            y: -height / 2.0,
            width: span,
            height,
            rounded: None,
            stroke: Some(concrete_stroke(settings)),
            fill: None,
            group: Some("concrete".to_string()),
        });

        for rebar in &props.rebar {
            let bar_size = rebar
                .bars
                .first()
                .map(|b| b.size.clone())
                .unwrap_or("#4".to_string());
            let bar_thickness = parse_size(&bar_size);
            let bar_color = get_color_for_size(&bar_size);

            let positions = get_rebar_positions(rebar, props, settings);

            // Project Y coordinates (for horizontal longitudinal)
            let mut unique_y: Vec<f64> = positions.iter().map(|p| p.1).collect();
            unique_y.sort_by(|a, b| a.partial_cmp(b).unwrap());
            unique_y.dedup_by(|a, b| (*a - *b).abs() < 0.1);

            for y_pos in &unique_y {
                d.add(Primitive::Path {
                    points: vec![(cover, *y_pos), (span - cover, *y_pos)],
                    closed: false,
                    stroke: Some(rebar_line_stroke(settings, bar_color.clone(), bar_thickness)),
                    fill: None,
                    group: Some("rebar".to_string()),
                });
            }

            if let Some(first_pos) = positions
                .iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            {
                let callout_text = format_rebar_callout(&rebar.bars);
                let y_pos = first_pos.1;
                let (offset_y, side) = if y_pos > 0.0 {
                    (height * 0.15, "top")
                } else {
                    (-height * 0.15, "bottom")
                };

                d.add(Primitive::LeaderLine {
                    start: (span - cover, y_pos),
                    end: (span + 10.0, y_pos + offset_y),
                    text: callout_text,
                    side: side.to_string(),
                    group: Some("callout".to_string()),
                });
            }
        }

        if let Some(ties) = &props.ties {
            let tie_color = if settings.monochrome || settings.style == StylePreset::Spd {
                "black".to_string()
            } else {
                get_color_for_size(&ties.size)
            };
            let tie_thickness = parse_size(&ties.size);
            let y_min = -height / 2.0 + cover + tie_thickness / 2.0;
            let y_max = height / 2.0 - cover - tie_thickness / 2.0;

            let positions = calculate_longitudinal_spacings(span, cover, ties, settings.unit_factor);
            for x in positions {
                d.add(Primitive::Path {
                    points: vec![(x, y_min), (x, y_max)],
                    closed: false,
                    stroke: Some(stirrup_outline_stroke(settings, tie_color.clone())),
                    fill: None,
                    group: Some("stirrup".to_string()),
                });
            }
        }
    }

    d
}

fn get_rebar_positions(
    rebar: &ast::RebarEntry,
    props: &ast::SectionProperties,
    settings: &GlobalSettings,
) -> Vec<(f64, f64)> {
    let (width, height) = get_section_dims(&props.shape, settings.unit_factor);
    let cover = get_cover(props, settings);
    let (_stirrup_size, stirrup_radius) = get_stirrup_info(props);

    let bar_radii: Vec<f64> = rebar
        .bars
        .iter()
        .flat_map(|b| std::iter::repeat_n(parse_size(&b.size) / 2.0, b.count as usize))
        .collect();

    let max_bar_radius = bar_radii.iter().copied().fold(0.0_f64, |a, b| a.max(b));

    let steel_offset = cover + 2.0 * stirrup_radius + max_bar_radius;
    let total_count = bar_radii.len();

    let positions = match rebar.pattern {
        ast::RebarPattern::Top => distribute_bars_horizontal(
            total_count,
            width - 2.0 * steel_offset,
            height / 2.0 - steel_offset,
        ),
        ast::RebarPattern::Bottom => distribute_bars_horizontal(
            total_count,
            width - 2.0 * steel_offset,
            -height / 2.0 + steel_offset,
        ),
        ast::RebarPattern::Sides => distribute_bars_sides(
            total_count,
            height - 2.0 * steel_offset,
            width / 2.0 - steel_offset,
        ),
        ast::RebarPattern::Perimeter => {
            if let Some(ast::Shape::Circle { diameter }) = &props.shape {
                distribute_bars_circle(total_count, diameter / 2.0 - steel_offset)
            } else {
                distribute_bars_perim(
                    total_count,
                    width - 2.0 * steel_offset,
                    height - 2.0 * steel_offset,
                )
            }
        }
        _ => vec![],
    };

    // Adjust positions: smaller bars sit closer to the stirrup (outer face),
    // larger bars sit further in (inner face).
    adjust_positions_by_bar_radius(&positions, &rebar.pattern, max_bar_radius, &bar_radii, width, height)
}

fn adjust_positions_by_bar_radius(
    positions: &[(f64, f64)],
    pattern: &ast::RebarPattern,
    max_radius: f64,
    bar_radii: &[f64],
    width: f64,
    height: f64,
) -> Vec<(f64, f64)> {
    let half_w = width / 2.0;
    let half_h = height / 2.0;
    let eps = 0.5; // tolerance for face detection

    positions
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| {
            let r = bar_radii.get(i).copied().unwrap_or(max_radius);
            let delta = max_radius - r;
            match pattern {
                ast::RebarPattern::Top => (x, y + delta),         // push outward (up)
                ast::RebarPattern::Bottom => (x, y - delta),      // push outward (down)
                ast::RebarPattern::Sides => {
                    if x > 0.0 {
                        (x - delta, y)
                    } else {
                        (x + delta, y)
                    }
                }
                ast::RebarPattern::Perimeter => {
                    // Determine face by proximity to edges
                    let on_top = (y - half_h).abs() < eps;
                    let on_bottom = (y + half_h).abs() < eps;
                    let on_right = (x - half_w).abs() < eps;
                    let on_left = (x + half_w).abs() < eps;

                    if on_top {
                        (x, y + delta)
                    } else if on_bottom {
                        (x, y - delta)
                    } else if on_right {
                        (x - delta, y)
                    } else if on_left {
                        (x + delta, y)
                    } else {
                        // Corner or intermediate: adjust radially outward
                        let mag = (x * x + y * y).sqrt();
                        if mag > 0.0 {
                            let scale = (mag + delta) / mag;
                            (x * scale, y * scale)
                        } else {
                            (x, y)
                        }
                    }
                }
                _ => (x, y),
            }
        })
        .collect()
}

fn calculate_longitudinal_spacings(
    span: f64,
    cover: f64,
    ties: &ast::StirrupsConfig,
    factor: f64,
) -> Vec<f64> {
    let mut positions = Vec::new();

    let mut insert = |x: f64| {
        if !positions.iter().any(|p| (p - x).abs() < 0.01) {
            positions.push(x);
        }
    };

    // Calculate positions from Start to Middle
    let mut x_start = cover;
    for spacing in &ties.dist {
        if let ast::Spacing::Fixed { count, dist } = spacing {
            let d = apply_unit(*dist, factor);
            for _ in 0..*count {
                x_start += d;
                if x_start < span / 2.0 {
                    insert(x_start);
                }
            }
        }
    }

    // Calculate positions from End to Middle (Mirrored)
    let mut x_end = span - cover;
    for spacing in &ties.dist {
        if let ast::Spacing::Fixed { count, dist } = spacing {
            let d = apply_unit(*dist, factor);
            for _ in 0..*count {
                x_end -= d;
                if x_end > span / 2.0 {
                    insert(x_end);
                }
            }
        }
    }

    // Fill the middle with Rest spacing
    let rest_dist = ties
        .dist
        .iter()
        .find_map(|s| {
            if let ast::Spacing::Rest { dist } = s {
                Some(apply_unit(*dist, factor))
            } else {
                None
            }
        })
        .unwrap_or_else(|| apply_unit(20.0, factor));

    let mut left_bound = cover;
    for spacing in &ties.dist {
        if let ast::Spacing::Fixed { count, dist } = spacing {
            left_bound += *count as f64 * apply_unit(*dist, factor);
        } else {
            break;
        }
    }

    let mut right_bound = span - cover;
    for spacing in &ties.dist {
        if let ast::Spacing::Fixed { count, dist } = spacing {
            right_bound -= *count as f64 * apply_unit(*dist, factor);
        } else {
            break;
        }
    }

    if left_bound < right_bound {
        let middle_len = right_bound - left_bound;
        let count = (middle_len / rest_dist).floor() as i32;
        if count > 0 {
            let actual_middle_dist = middle_len / (count + 1) as f64;
            for i in 1..=count {
                insert(left_bound + i as f64 * actual_middle_dist);
            }
        } else {
            insert((left_bound + right_bound) / 2.0);
        }
    }

    positions.sort_by(|a, b| a.partial_cmp(b).unwrap());
    positions
}

/// Distribute bars horizontally across a width at a given y position
fn distribute_bars_horizontal(count: usize, available_width: f64, y: f64) -> Vec<(f64, f64)> {
    if count == 0 {
        return vec![];
    }
    if count == 1 {
        return vec![(0.0, y)];
    }

    let spacing = available_width / (count - 1) as f64;
    let start_x = -available_width / 2.0;

    (0..count)
        .map(|i| (start_x + i as f64 * spacing, y))
        .collect()
}

fn distribute_bars_sides(count: usize, available_height: f64, x: f64) -> Vec<(f64, f64)> {
    if count == 0 {
        return vec![];
    }
    let per_side = count.div_ceil(2);
    let mut pos = Vec::new();

    if per_side > 0 {
        let spacing = if per_side > 1 {
            available_height / (per_side - 1) as f64
        } else {
            0.0
        };
        let start_y = -available_height / 2.0;

        for i in 0..per_side {
            let y = start_y + i as f64 * spacing;
            pos.push((x, y));
            if pos.len() < count {
                pos.push((-x, y));
            }
        }
    }
    pos
}

fn distribute_bars_perim(count: usize, w: f64, h: f64) -> Vec<(f64, f64)> {
    if count == 0 {
        return vec![];
    }
    if count == 1 {
        return vec![(0.0, 0.0)];
    }

    let mut pos = vec![
        (-w / 2.0, -h / 2.0),
        (w / 2.0, -h / 2.0),
        (w / 2.0, h / 2.0),
        (-w / 2.0, h / 2.0),
    ];

    if count <= 4 {
        return pos.into_iter().take(count).collect();
    }

    // Remaining intermediate bars
    let remaining = count - 4;
    let pairs = remaining / 2;
    let loose = remaining % 2;

    // Distribute pairs proportionally to face lengths
    let h_pairs = ((pairs as f64 * w) / (w + h)).round() as usize;
    let v_pairs = pairs - h_pairs;

    let n_bottom = h_pairs + loose;
    let n_top = h_pairs;
    let n_right = v_pairs;
    let n_left = v_pairs;

    // Bottom
    if n_bottom > 0 {
        let s = w / (n_bottom + 1) as f64;
        for i in 1..=n_bottom {
            pos.push((-w / 2.0 + i as f64 * s, -h / 2.0));
        }
    }
    // Top
    if n_top > 0 {
        let s = w / (n_top + 1) as f64;
        for i in 1..=n_top {
            pos.push((-w / 2.0 + i as f64 * s, h / 2.0));
        }
    }
    // Right
    if n_right > 0 {
        let s = h / (n_right + 1) as f64;
        for i in 1..=n_right {
            pos.push((w / 2.0, -h / 2.0 + i as f64 * s));
        }
    }
    // Left
    if n_left > 0 {
        let s = h / (n_left + 1) as f64;
        for i in 1..=n_left {
            pos.push((-w / 2.0, -h / 2.0 + i as f64 * s));
        }
    }

    pos
}

fn distribute_bars_circle(count: usize, radius: f64) -> Vec<(f64, f64)> {
    if count == 0 {
        return vec![];
    }
    let mut pos = Vec::new();
    let angle_step = 2.0 * std::f64::consts::PI / count as f64;
    for i in 0..count {
        let angle = i as f64 * angle_step;
        pos.push((radius * angle.cos(), radius * angle.sin()));
    }
    pos
}

/// Bend radius for stirrup corners (hook radius).
/// Based on ACI 318 / E.060: D ≈ 4·db → radius = D/2 = 2·db
fn get_tie_bend_radius(size_str: &str) -> f64 {
    let db = parse_size(size_str);
    2.0 * db
}

#[allow(dead_code)]
fn parse_size(size_str: &str) -> f64 {
    if let Some(stripped) = size_str.strip_prefix('#') {
        if let Ok(num) = stripped.parse::<f64>() {
            return num * 0.3175; // 1/8 inch in cm
        }
    } else {
        let content = size_str.trim_end_matches('"');
        if content.contains('/') {
            let parts: Vec<&str> = content.split('/').collect();
            if parts.len() == 2
                && let (Ok(num), Ok(den)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>())
            {
                return (num / den) * 2.54;
            }
        } else if let Ok(num) = content.parse::<f64>() {
            // Assume inches if no specific unit but parsed as number
            return num * 2.54;
        }
    }
    1.27 // Default
}

#[allow(dead_code)]
fn get_color_for_size(size: &str) -> String {
    let s = size.trim_end_matches('"');
    match s {
        "#3" | "3/8" => "#CC7000".to_string(), // Dark Orange
        "#4" | "1/2" => "#CC0000".to_string(), // Dark Red
        "#5" | "5/8" => "#800080".to_string(), // Purple
        "#6" | "3/4" => "#000080".to_string(), // Navy Blue
        "#8" | "1" => "#006400".to_string(),   // Dark Green
        _ => "black".to_string(),
    }
}

/// Format rebar callout text, grouping by size and preserving original notation.
/// Examples: "3#6", "2Ø3/8\"", "2#8+1Ø3/8\""
fn format_rebar_callout(bars: &[ast::BarGroup]) -> String {
    use std::collections::BTreeMap;

    // Group bars by size, summing counts
    let mut grouped: BTreeMap<&str, u32> = BTreeMap::new();
    for bar in bars {
        *grouped.entry(&bar.size).or_insert(0) += bar.count;
    }

    // Format each group
    grouped
        .iter()
        .map(|(size, count)| {
            if size.starts_with("#") {
                // Compact format for # notation: "3#6"
                format!("{}{}", count, size)
            } else {
                // Fractional format with Ø: "2Ø3/8\""
                format!("{}Ø{}", count, size)
            }
        })
        .collect::<Vec<_>>()
        .join("+")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_section_dims_rect() {
        let shape = Some(ast::Shape::Rect {
            width: 30.0,
            height: 60.0,
        });
        assert_eq!(get_section_dims(&shape, 1.0), (30.0, 60.0));
    }

    #[test]
    fn test_get_section_dims_circle() {
        let shape = Some(ast::Shape::Circle { diameter: 40.0 });
        assert_eq!(get_section_dims(&shape, 1.0), (40.0, 40.0));
    }

    #[test]
    fn test_get_section_dims_default() {
        assert_eq!(get_section_dims(&None, 1.0), (30.0, 60.0));
    }

    #[test]
    fn test_get_section_dims_with_unit() {
        let shape = Some(ast::Shape::Rect {
            width: 300.0,
            height: 600.0,
        });
        assert_eq!(get_section_dims(&shape, 0.1), (30.0, 60.0));
    }

    #[test]
    fn test_get_cover_from_props() {
        let props = ast::SectionProperties {
            shape: None,
            concrete: Some(ast::ConcreteProperties {
                fc: None,
                cover: Some(5.0),
            }),
            rebar: vec![],
            ties: None,
            view: None,
            length: None,
        };
        let settings = GlobalSettings {
            scale: None,
            cover: 4.0,
            stroke: None,
            unit_factor: 1.0,
            monochrome: false,
            style: StylePreset::Default,
        };
        assert_eq!(get_cover(&props, &settings), 5.0);
    }

    #[test]
    fn test_get_cover_from_settings() {
        let props = ast::SectionProperties {
            shape: None,
            concrete: None,
            rebar: vec![],
            ties: None,
            view: None,
            length: None,
        };
        let settings = GlobalSettings {
            scale: None,
            cover: 4.0,
            stroke: None,
            unit_factor: 1.0,
            monochrome: false,
            style: StylePreset::Default,
        };
        assert_eq!(get_cover(&props, &settings), 4.0);
    }

    #[test]
    fn test_get_cover_min_value() {
        let props = ast::SectionProperties {
            shape: None,
            concrete: Some(ast::ConcreteProperties {
                fc: None,
                cover: Some(1.0),
            }),
            rebar: vec![],
            ties: None,
            view: None,
            length: None,
        };
        let settings = GlobalSettings {
            scale: None,
            cover: 0.5,
            stroke: None,
            unit_factor: 1.0,
            monochrome: false,
            style: StylePreset::Default,
        };
        assert_eq!(get_cover(&props, &settings), 2.5);
    }

    #[test]
    fn test_get_cover_with_unit() {
        let props = ast::SectionProperties {
            shape: None,
            concrete: Some(ast::ConcreteProperties {
                fc: None,
                cover: Some(40.0),
            }),
            rebar: vec![],
            ties: None,
            view: None,
            length: None,
        };
        let settings = GlobalSettings {
            scale: None,
            cover: 40.0,
            stroke: None,
            unit_factor: 0.1,
            monochrome: false,
            style: StylePreset::Default,
        };
        assert_eq!(get_cover(&props, &settings), 4.0);
    }

    #[test]
    fn test_parse_size_tagged() {
        assert!((parse_size("#4") - 1.27).abs() < 0.01);
        assert!((parse_size("#8") - 2.54).abs() < 0.01);
    }

    #[test]
    fn test_parse_size_fraction() {
        assert!((parse_size("3/8") - 0.9525).abs() < 0.001);
        assert!((parse_size("1/2") - 1.27).abs() < 0.001);
    }

    #[test]
    fn test_parse_size_inch() {
        assert!((parse_size("1\"") - 2.54).abs() < 0.001);
    }

    #[test]
    fn test_parse_size_default() {
        assert!((parse_size("invalid") - 1.27).abs() < 0.001);
    }

    #[test]
    fn test_parse_unit_factor() {
        assert_eq!(parse_unit_factor("cm"), 1.0);
        assert_eq!(parse_unit_factor("mm"), 0.1);
        assert_eq!(parse_unit_factor("m"), 100.0);
        assert_eq!(parse_unit_factor("in"), 2.54);
        assert_eq!(parse_unit_factor("\""), 2.54);
        assert_eq!(parse_unit_factor("ft"), 30.48);
        assert_eq!(parse_unit_factor("unknown"), 1.0);
    }

    #[test]
    fn test_apply_unit() {
        assert_eq!(apply_unit(10.0, 1.0), 10.0);
        assert_eq!(apply_unit(10.0, 0.1), 1.0);
        assert_eq!(apply_unit(10.0, 100.0), 1000.0);
    }

    #[test]
    fn test_parse_style_preset() {
        assert!(matches!(parse_style_preset("spd"), StylePreset::Spd));
        assert!(matches!(
            parse_style_preset("professional"),
            StylePreset::Spd
        ));
        assert!(matches!(parse_style_preset("default"), StylePreset::Default));
    }

    #[test]
    fn test_get_color_for_size() {
        assert_eq!(get_color_for_size("#4"), "#CC0000");
        assert_eq!(get_color_for_size("1/2"), "#CC0000");
        assert_eq!(get_color_for_size("#8\""), "#006400");
        assert_eq!(get_color_for_size("unknown"), "black");
    }

    #[test]
    fn test_distribute_bars_horizontal() {
        let pos = distribute_bars_horizontal(3, 20.0, 5.0);
        assert_eq!(pos.len(), 3);
        assert_eq!(pos[0], (-10.0, 5.0));
        assert_eq!(pos[1], (0.0, 5.0));
        assert_eq!(pos[2], (10.0, 5.0));
    }

    #[test]
    fn test_distribute_bars_horizontal_single() {
        let pos = distribute_bars_horizontal(1, 20.0, 5.0);
        assert_eq!(pos, vec![(0.0, 5.0)]);
    }

    #[test]
    fn test_distribute_bars_sides() {
        let pos = distribute_bars_sides(4, 20.0, 8.0);
        assert_eq!(pos.len(), 4);
        assert_eq!(pos[0], (8.0, -10.0));
        assert_eq!(pos[1], (-8.0, -10.0));
        assert_eq!(pos[2], (8.0, 10.0));
        assert_eq!(pos[3], (-8.0, 10.0));
    }

    #[test]
    fn test_distribute_bars_perim_corners_only() {
        let pos = distribute_bars_perim(4, 20.0, 30.0);
        assert_eq!(pos.len(), 4);
        assert_eq!(pos[0], (-10.0, -15.0));
        assert_eq!(pos[1], (10.0, -15.0));
        assert_eq!(pos[2], (10.0, 15.0));
        assert_eq!(pos[3], (-10.0, 15.0));
    }

    #[test]
    fn test_distribute_bars_perim_with_intermediates() {
        let pos = distribute_bars_perim(8, 20.0, 20.0);
        assert_eq!(pos.len(), 8);
    }

    #[test]
    fn test_distribute_bars_circle() {
        let pos = distribute_bars_circle(4, 10.0);
        assert_eq!(pos.len(), 4);
        assert!((pos[0].0 - 10.0).abs() < 0.001);
        assert!((pos[0].1).abs() < 0.001);
    }

    #[test]
    fn test_calculate_longitudinal_spacings() {
        let ties = ast::StirrupsConfig {
            size: "#3".to_string(),
            dist: vec![
                ast::Spacing::Fixed { count: 2, dist: 5.0 },
                ast::Spacing::Rest { dist: 20.0 },
            ],
        };
        let positions = calculate_longitudinal_spacings(100.0, 4.0, &ties, 1.0);
        assert!(!positions.is_empty());
        // First positions should be near start
        assert!(positions[0] > 4.0);
        // Positions should be sorted
        for i in 1..positions.len() {
            assert!(positions[i] > positions[i - 1]);
        }
    }

    #[test]
    fn test_format_rebar_callout_single_group() {
        let bars = vec![ast::BarGroup {
            count: 3,
            size: "#6".to_string(),
        }];
        assert_eq!(format_rebar_callout(&bars), "3#6");
    }

    #[test]
    fn test_format_rebar_callout_multiple_groups() {
        let bars = vec![
            ast::BarGroup {
                count: 2,
                size: "#8".to_string(),
            },
            ast::BarGroup {
                count: 1,
                size: "3/8\"".to_string(),
            },
        ];
        let result = format_rebar_callout(&bars);
        assert!(result.contains("2#8"));
        assert!(result.contains("1Ø3/8\""));
    }
}
