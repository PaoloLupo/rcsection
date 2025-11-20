use crate::parser::ast::{RebarPattern, Section, Shape};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drawing {
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
    Text {
        x: f64,
        y: f64,
        content: String,
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
            primitives: Vec::new(),
        }
    }

    pub fn add(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }
}

pub fn generate(section: &Section) -> Drawing {
    let mut drawing = Drawing::new();
    let props = &section.properties;

    // Draw Concrete Shape
    if let Some(shape) = &props.shape {
        match shape {
            Shape::Rect { width, height } => {
                drawing.add(Primitive::Rect {
                    x: -width / 2.0,
                    y: -height / 2.0,
                    width: *width,
                    height: *height,
                    stroke: Some(Stroke {
                        color: "black".to_string(),
                        width: 1.0,
                    }),
                    fill: None,
                    group: Some("concrete".to_string()),
                });
            }
            Shape::Circle { diameter } => {
                drawing.add(Primitive::Circle {
                    x: 0.0,
                    y: 0.0,
                    radius: diameter / 2.0,
                    stroke: Some(Stroke {
                        color: "black".to_string(),
                        width: 1.0,
                    }),
                    fill: None,
                    group: Some("concrete".to_string()),
                });
            }
        }
    }

    // Draw Stirrups
    let cover = props.cover.unwrap_or(4.0);
    let mut stirrup_size = 0.95; // Default #3
    if let Some(ties) = &props.ties {
        stirrup_size = parse_size(&ties.size);
    }

    let inset = cover + stirrup_size / 2.0;

    if let Some(shape) = &props.shape {
        match shape {
            Shape::Rect { width, height } => {
                drawing.add(Primitive::Rect {
                    x: -width / 2.0 + inset,
                    y: -height / 2.0 + inset,
                    width: width - 2.0 * inset,
                    height: height - 2.0 * inset,
                    stroke: Some(Stroke {
                        color: "blue".to_string(),
                        width: 2.0,
                    }),
                    fill: None,
                    group: Some("stirrup".to_string()),
                });
            }
            Shape::Circle { diameter } => {
                drawing.add(Primitive::Circle {
                    x: 0.0,
                    y: 0.0,
                    radius: diameter / 2.0 - inset,
                    stroke: Some(Stroke {
                        color: "blue".to_string(),
                        width: 2.0,
                    }),
                    fill: None,
                    group: Some("stirrup".to_string()),
                });
            }
        }
    }

    // Draw Rebar
    // Layering Logic:
    // Top: First = Outer (Layer 0), Next = Inner (Layer 1...)
    // Bot: First = Inner (Layer N), Last = Outer (Layer 0)

    let base_inset = cover + stirrup_size;

    // Separate top and bot entries to handle layering
    let mut top_entries = Vec::new();
    let mut bot_entries = Vec::new();
    let mut other_entries = Vec::new();

    for entry in &props.rebar {
        match entry.pattern {
            RebarPattern::Top => top_entries.push(entry),
            RebarPattern::Bottom => bot_entries.push(entry),
            _ => other_entries.push(entry),
        }
    }

    // Process Top (Sequential: Outer -> Inner)
    for (i, entry) in top_entries.iter().enumerate() {
        let bar_diam = parse_size(&entry.size);
        let layer_offset = (i as f64) * (bar_diam + 2.5);
        let vertical_inset = base_inset + bar_diam / 2.0 + layer_offset;
        // Horizontal inset should be constant for all layers (based on outer bounds)
        // But we need to account for the bar radius of THIS bar to keep it inside stirrups
        let horizontal_inset = base_inset + bar_diam / 2.0;

        draw_linear_pattern(
            &mut drawing,
            props.shape.as_ref(),
            entry,
            vertical_inset,
            horizontal_inset,
            true,
        );
    }

    // Process Bot (Reverse: Inner -> Outer... WAIT)
    // Rule: "bot: First line declared is INNER (highest), Last line is OUTER (lowest)"
    // So we should iterate in reverse order to go from Outer to Inner?
    // No, if we iterate in reverse (Last -> First), we go Outer -> Inner.
    // So the Last entry is Layer 0 (Outer).

    for (i, entry) in bot_entries.iter().rev().enumerate() {
        let bar_diam = parse_size(&entry.size);
        let layer_offset = (i as f64) * (bar_diam + 2.5);
        let vertical_inset = base_inset + bar_diam / 2.0 + layer_offset;
        let horizontal_inset = base_inset + bar_diam / 2.0;

        draw_linear_pattern(
            &mut drawing,
            props.shape.as_ref(),
            entry,
            vertical_inset,
            horizontal_inset,
            false,
        );
    }

    // Process Others
    for entry in other_entries {
        let bar_diam = parse_size(&entry.size);
        let current_inset = base_inset + bar_diam / 2.0;
        // TODO: Handle sides/perim properly
        if let RebarPattern::Perimeter = entry.pattern {
            if let Some(Shape::Circle { diameter }) = props.shape {
                let r = diameter / 2.0 - current_inset;
                let step_angle = 2.0 * std::f64::consts::PI / (entry.count as f64);

                for i in 0..entry.count {
                    let angle = (i as f64) * step_angle;
                    let x = r * angle.cos();
                    let y = r * angle.sin();
                    drawing.add(Primitive::Circle {
                        x,
                        y,
                        radius: bar_diam / 2.0,
                        stroke: None,
                        fill: Some("red".to_string()),
                        group: Some("rebar".to_string()),
                    });
                }
            }
        }
    }

    // Draw Label
    let label_y = if let Some(Shape::Rect { height, .. }) = props.shape {
        -height / 2.0 - 5.0
    } else if let Some(Shape::Circle { diameter }) = props.shape {
        -diameter / 2.0 - 5.0
    } else {
        0.0
    };

    drawing.add(Primitive::Text {
        x: 0.0,
        y: label_y,
        content: section.id.clone(),
    });

    drawing
}

fn draw_linear_pattern(
    drawing: &mut Drawing,
    shape: Option<&Shape>,
    entry: &crate::parser::ast::RebarEntry,
    v_inset: f64,
    h_inset: f64,
    is_top: bool,
) {
    let bar_diam = parse_size(&entry.size);
    if let Some(Shape::Rect { width, height }) = shape {
        let y = if is_top {
            height / 2.0 - v_inset
        } else {
            -height / 2.0 + v_inset
        };
        let start_x = -width / 2.0 + h_inset;
        let end_x = width / 2.0 - h_inset;
        let step = if entry.count > 1 {
            (end_x - start_x) / (entry.count as f64 - 1.0)
        } else {
            0.0
        };

        for i in 0..entry.count {
            let x = if entry.count > 1 {
                start_x + (i as f64) * step
            } else {
                0.0
            };
            drawing.add(Primitive::Circle {
                x,
                y,
                radius: bar_diam / 2.0,
                stroke: None,
                fill: Some("red".to_string()),
                group: Some("rebar".to_string()),
            });
        }
    }
}

fn parse_size(size_str: &str) -> f64 {
    if size_str.starts_with("#") {
        if let Ok(num) = size_str[1..].parse::<f64>() {
            return num * 0.3175; // 1/8 inch in cm
        }
    } else if size_str.ends_with("\"") {
        let content = size_str.trim_end_matches("\"");
        if content.contains('/') {
            let parts: Vec<&str> = content.split('/').collect();
            if parts.len() == 2 {
                if let (Ok(num), Ok(den)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                    return (num / den) * 2.54;
                }
            }
        } else {
            if let Ok(num) = content.parse::<f64>() {
                return num * 2.54;
            }
        }
    }
    1.27 // Default
}
