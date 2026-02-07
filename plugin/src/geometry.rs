use crate::parser::ast::{self, View};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

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

#[derive(Default)]
struct GlobalSettings {
    scale: Option<f64>,
    cover: f64,
    stroke: Option<Stroke>,
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

fn generate_section(section: &ast::Section, settings: &GlobalSettings) -> Vec<Drawing> {
    let mut drawings = Vec::new();
    let props = &section.properties;

    // Determine which views to generate
    let show_section = match &props.view {
        Some(View::Section) | Some(View::Both) | None => true,
        _ => false,
    };
    let show_longitudinal = matches!(&props.view, Some(View::Longitudinal) | Some(View::Both));

    // Get dimensions for both views
    let (width, height) = match &props.shape {
        Some(ast::Shape::Rect { width, height }) => (*width, *height),
        Some(ast::Shape::Circle { diameter }) => (*diameter, *diameter),
        _ => (30.0, 60.0), // Default beam size
    };

    let cover = props
        .concrete
        .as_ref()
        .and_then(|c| c.cover)
        .unwrap_or(settings.cover)
        .max(2.5);

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
                        stroke: Some(Stroke {
                            color: "black".to_string(),
                            width: 0.08, // 0.8mm in real life
                        }),
                        fill: None,
                        group: Some("concrete".to_string()),
                    });
                }
                ast::Shape::Circle { diameter } => {
                    d.add(Primitive::Circle {
                        x: 0.0,
                        y: 0.0,
                        radius: diameter / 2.0,
                        stroke: Some(Stroke {
                            color: "black".to_string(),
                            width: 0.08,
                        }),
                        fill: None,
                        group: Some("concrete".to_string()),
                    });
                }
                ast::Shape::Polygon { points } => {
                    d.add(Primitive::Path {
                        points: points.clone(),
                        closed: true,
                        stroke: Some(Stroke {
                            color: "black".to_string(),
                            width: 0.08,
                        }),
                        fill: None,
                        group: Some("concrete".to_string()),
                    });
                }
            }
        }

        // Add stirrup in section view
        if let Some(ties) = &props.ties {
            let tie_color = get_color_for_size(&ties.size);
            let tie_thickness = parse_size(&ties.size);
            if let Some(shape) = &props.shape {
                match shape {
                    ast::Shape::Rect { width, height } => {
                        d.add(Primitive::Rect {
                            x: -width / 2.0 + cover + tie_thickness / 2.0,
                            y: -height / 2.0 + cover + tie_thickness / 2.0,
                            width: width - 2.0 * cover - tie_thickness,
                            height: height - 2.0 * cover - tie_thickness,
                            rounded: None,
                            stroke: Some(Stroke {
                                color: tie_color,
                                width: tie_thickness, // Provided in CM, draw.typ will scale
                            }),
                            fill: None,
                            group: Some("stirrup".to_string()),
                        });
                    }
                    ast::Shape::Circle { diameter } => {
                        d.add(Primitive::Circle {
                            x: 0.0,
                            y: 0.0,
                            radius: diameter / 2.0 - cover - tie_thickness / 2.0,
                            stroke: Some(Stroke {
                                color: tie_color,
                                width: tie_thickness,
                            }),
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
            let stirrup_size = props
                .ties
                .as_ref()
                .map(|t| t.size.clone())
                .unwrap_or("#3".to_string());
            let stirrup_radius = parse_size(&stirrup_size) / 2.0;

            // Use the largest bar size for offset calculation
            let max_bar_radius = rebar
                .bars
                .iter()
                .map(|b| parse_size(&b.size) / 2.0)
                .fold(0.0_f64, |a, b| a.max(b));

            let steel_offset = cover + 2.0 * stirrup_radius + max_bar_radius;

            // Generate positions for ALL bars, then assign sizes
            let total_count = rebar.bars.iter().map(|b| b.count).sum::<u32>() as usize;

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

            // Expand bar groups to match positions: [(size, color), ...]
            let bar_specs: Vec<_> = rebar
                .bars
                .iter()
                .flat_map(|b| {
                    let radius = parse_size(&b.size) / 2.0;
                    let color = get_color_for_size(&b.size);
                    std::iter::repeat((radius, color)).take(b.count as usize)
                })
                .collect();

            // Draw each bar with its correct size and color
            for (i, (x, y)) in positions.iter().enumerate() {
                let (radius, color) = bar_specs
                    .get(i)
                    .cloned()
                    .unwrap_or((max_bar_radius, "black".to_string()));

                d.add(Primitive::Circle {
                    x: *x,
                    y: *y,
                    radius,
                    stroke: Some(Stroke {
                        color: color.clone(),
                        width: 0.05,
                    }),
                    fill: Some(color),
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

    // === Longitudinal View (Side elevation) ===
    if show_longitudinal {
        let mut d = Drawing::new();
        d.id = Some(format!("{} (Long.)", section.id));
        d.view = Some("longitudinal".to_string());
        d.scale = settings.scale;

        // Assume a standard span length (can be parameterized later)
        let span = props.length.unwrap_or(200.0); // 2 meters default to fit A4 better

        // Concrete outline (side view)
        d.add(Primitive::Rect {
            x: 0.0,
            y: -height / 2.0,
            width: span,
            height: height,
            rounded: None,
            stroke: Some(Stroke {
                color: "black".to_string(),
                width: 1.0,
            }),
            fill: None,
            group: Some("concrete".to_string()),
        });

        // Draw longitudinal bars as lines
        for rebar in &props.rebar {
            let stirrup_size = props
                .ties
                .as_ref()
                .map(|t| t.size.clone())
                .unwrap_or("#3".to_string());
            let stirrup_thickness = parse_size(&stirrup_size);

            let bar_size = rebar
                .bars
                .first()
                .map(|b| b.size.clone())
                .unwrap_or("#4".to_string());
            let bar_thickness = parse_size(&bar_size);
            let bar_color = get_color_for_size(&bar_size);

            // Same offset as in section view
            let steel_offset = cover + stirrup_thickness + bar_thickness / 2.0;

            let y_pos = match rebar.pattern {
                ast::RebarPattern::Top => height / 2.0 - steel_offset,
                ast::RebarPattern::Bottom => -height / 2.0 + steel_offset,
                _ => 0.0,
            };

            // Main longitudinal bar line
            d.add(Primitive::Path {
                points: vec![(cover, y_pos), (span - cover, y_pos)],
                closed: false,
                stroke: Some(Stroke {
                    color: bar_color.clone(),
                    width: bar_thickness,
                }),
                fill: None,
                group: Some("rebar".to_string()),
            });

            // Add callout at right end of bar
            let callout_text = format_rebar_callout(&rebar.bars);
            let (offset_y, side) = match rebar.pattern {
                ast::RebarPattern::Top => (height * 0.15, "top"),
                ast::RebarPattern::Bottom => (-height * 0.15, "bottom"),
                _ => (0.0, "right"),
            };

            d.add(Primitive::LeaderLine {
                start: (span - cover, y_pos),
                end: (span + 10.0, y_pos + offset_y),
                text: callout_text,
                side: side.to_string(),
                group: Some("callout".to_string()),
            });
        }

        // Draw stirrups as vertical lines (Symmetric Distribution)
        if let Some(ties) = &props.ties {
            let tie_color = get_color_for_size(&ties.size);
            let tie_thickness = parse_size(&ties.size);

            // Vertical range matching the section view stirrup cage
            let y_min = -height / 2.0 + cover + tie_thickness / 2.0;
            let y_max = height / 2.0 - cover - tie_thickness / 2.0;

            let mut positions: BTreeSet<String> = BTreeSet::new();

            // Calculate positions from Start to Middle
            let mut x_start = cover;
            for spacing in &ties.dist {
                if let ast::Spacing::Fixed { count, dist } = spacing {
                    for _ in 0..*count {
                        x_start += dist;
                        if x_start < span / 2.0 {
                            positions.insert(format!("{:.4}", x_start));
                        }
                    }
                }
            }

            // Calculate positions from End to Middle (Mirrored)
            let mut x_end = span - cover;
            for spacing in &ties.dist {
                if let ast::Spacing::Fixed { count, dist } = spacing {
                    for _ in 0..*count {
                        x_end -= dist;
                        if x_end > span / 2.0 {
                            positions.insert(format!("{:.4}", x_end));
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
                        Some(*dist)
                    } else {
                        None
                    }
                })
                .unwrap_or(20.0);

            // Find the boundaries of the middle zone
            let mut left_bound = cover;
            for spacing in &ties.dist {
                if let ast::Spacing::Fixed { count, dist } = spacing {
                    left_bound += *count as f64 * dist;
                } else {
                    break;
                }
            }

            let mut right_bound = span - cover;
            for spacing in &ties.dist {
                if let ast::Spacing::Fixed { count, dist } = spacing {
                    right_bound -= *count as f64 * dist;
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
                        let x = left_bound + i as f64 * actual_middle_dist;
                        positions.insert(format!("{:.4}", x));
                    }
                } else {
                    // Just one in the middle if it fits
                    let mid_x = (left_bound + right_bound) / 2.0;
                    positions.insert(format!("{:.4}", mid_x));
                }
            }

            // Add the stirrup primitives
            for x_str in positions {
                let x: f64 = x_str.parse().unwrap();
                d.add(Primitive::Path {
                    points: vec![(x, y_min), (x, y_max)],
                    closed: false,
                    stroke: Some(Stroke {
                        color: tie_color.clone(),
                        width: tie_thickness,
                    }),
                    fill: None,
                    group: Some("stirrup".to_string()),
                });
            }
        }

        drawings.push(d);
    }

    drawings
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
    let per_side = (count + 1) / 2;
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

    let mut pos = Vec::new();
    // Corners first
    pos.push((-w / 2.0, -h / 2.0));
    pos.push((w / 2.0, -h / 2.0));
    pos.push((w / 2.0, h / 2.0));
    pos.push((-w / 2.0, h / 2.0));

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

#[allow(dead_code)]
fn parse_size(size_str: &str) -> f64 {
    if size_str.starts_with("#") {
        if let Ok(num) = size_str[1..].parse::<f64>() {
            return num * 0.3175; // 1/8 inch in cm
        }
    } else {
        let content = size_str.trim_end_matches("\"");
        if content.contains('/') {
            let parts: Vec<&str> = content.split('/').collect();
            if parts.len() == 2 {
                if let (Ok(num), Ok(den)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                    return (num / den) * 2.54;
                }
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
    let s = size.trim_end_matches("\"");
    match s {
        "#3" | "3/8" => "#CC7000".to_string(), // Dark Orange
        "#4" | "1/2" => "#CC0000".to_string(), // Dark Red
        "#5" | "5/8" => "#800080".to_string(), // Purple
        "#6" | "3/4" => "#000080".to_string(), // Navy Blue
        "#8" | "1" => "#006400".to_string(),   // Dark Green
        _ => {
            // Check original for #
            if size.starts_with("#") {
                match size {
                    "#3" => "#CC7000".to_string(),
                    "#4" => "#CC0000".to_string(),
                    "#5" => "#800080".to_string(),
                    "#6" => "#000080".to_string(),
                    "#8" => "#006400".to_string(),
                    _ => "black".to_string(),
                }
            } else {
                "black".to_string()
            }
        }
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
