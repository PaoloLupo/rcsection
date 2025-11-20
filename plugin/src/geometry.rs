use crate::parser::ast::{RebarPattern, Section, Shape, View};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drawing {
    pub id: Option<String>,
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
            scale: None,
            primitives: Vec::new(),
        }
    }

    pub fn add(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }
}

pub fn generate(section: &Section) -> Vec<Drawing> {
    let mut drawings = Vec::new();
    let props = &section.properties;

    // Determine which views to generate
    let show_section = match &props.view {
        Some(View::Section) | Some(View::Both) | None => true,
        Some(View::Longitudinal) => false,
    };

    let show_longitudinal = match &props.view {
        Some(View::Longitudinal) | Some(View::Both) => true,
        Some(View::Section) => false,
        None => props.span.is_some(), // Default to showing longitudinal if span is present
    };

    // --- Cross Section Drawing ---
    if show_section {
        let mut section_drawing = Drawing::new();
        section_drawing.id = Some(format!("{} (Section)", section.id));
        section_drawing.id = Some(format!("{} (Section)", section.id));
        section_drawing.scale = props.scale_section;

        // Draw Concrete Shape
        if let Some(shape) = &props.shape {
            match shape {
                Shape::Rect { width, height } => {
                    section_drawing.add(Primitive::Rect {
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
                    section_drawing.add(Primitive::Circle {
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

        // Calculate max bar diameter for bending radius
        let mut max_bar_diam = 0.0;
        for entry in &props.rebar {
            let d = parse_size(&entry.size);
            if d > max_bar_diam {
                max_bar_diam = d;
            }
        }
        if max_bar_diam == 0.0 {
            max_bar_diam = 0.95; // Fallback to #3 stirrup size if no rebar
        }

        // Draw Stirrups (Section View)
        let cover = props.cover.unwrap_or(4.0);
        let mut stirrup_size = 0.95; // Default #3
        if let Some(ties) = &props.ties {
            stirrup_size = parse_size(&ties.size);
        }

        let inset = cover + stirrup_size / 2.0;

        if let Some(shape) = &props.shape {
            match shape {
                Shape::Rect { width, height } => {
                    let w = width - 2.0 * inset;
                    let h = height - 2.0 * inset;
                    let x = -width / 2.0 + inset;
                    let y = -height / 2.0 + inset;

                    let inner_r = (2.0 * stirrup_size).max(max_bar_diam / 2.0);
                    let r = inner_r + stirrup_size / 2.0;

                    // Generate rounded rect path
                    let mut points = Vec::new();

                    // Start top-left (after corner)
                    // Top edge
                    points.push((x + r, y + h));
                    points.push((x + w - r, y + h));

                    // Top-right corner
                    add_arc(&mut points, x + w - r, y + h - r, r, PI / 2.0, 0.0);

                    // Right edge
                    points.push((x + w, y + h - r));
                    points.push((x + w, y + r));

                    // Bottom-right corner
                    add_arc(&mut points, x + w - r, y + r, r, 0.0, -PI / 2.0);

                    // Bottom edge
                    points.push((x + w - r, y));
                    points.push((x + r, y));

                    // Bottom-left corner
                    add_arc(&mut points, x + r, y + r, r, -PI / 2.0, -PI);

                    // Left edge
                    points.push((x, y + r));
                    points.push((x, y + h - r));

                    // Top-left corner
                    add_arc(&mut points, x + r, y + h - r, r, PI, PI / 2.0);

                    // Add main stirrup path
                    section_drawing.add(Primitive::Path {
                        points: points.clone(),
                        closed: true,
                        stroke: Some(Stroke {
                            color: "#000080".to_string(),
                            width: 2.0,
                        }), // Navy
                        fill: None,
                        group: Some("stirrup".to_string()),
                    });
                }
                Shape::Circle { diameter } => {
                    section_drawing.add(Primitive::Circle {
                        x: 0.0,
                        y: 0.0,
                        radius: diameter / 2.0 - inset,
                        stroke: Some(Stroke {
                            color: "#000080".to_string(),
                            width: 2.0,
                        }),
                        fill: None,
                        group: Some("stirrup".to_string()),
                    });
                }
            }
        }

        // Draw Rebar (Section View)
        let base_inset = cover + stirrup_size;

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

        for (i, entry) in top_entries.iter().enumerate() {
            let bar_diam = parse_size(&entry.size);
            let layer_offset = (i as f64) * (bar_diam + 2.5);
            let vertical_inset = base_inset + bar_diam / 2.0 + layer_offset;
            let horizontal_inset = base_inset + bar_diam / 2.0;
            draw_linear_pattern(
                &mut section_drawing,
                props.shape.as_ref(),
                entry,
                vertical_inset,
                horizontal_inset,
                true,
            );
        }

        for (i, entry) in bot_entries.iter().rev().enumerate() {
            let bar_diam = parse_size(&entry.size);
            let layer_offset = (i as f64) * (bar_diam + 2.5);
            let vertical_inset = base_inset + bar_diam / 2.0 + layer_offset;
            let horizontal_inset = base_inset + bar_diam / 2.0;
            draw_linear_pattern(
                &mut section_drawing,
                props.shape.as_ref(),
                entry,
                vertical_inset,
                horizontal_inset,
                false,
            );
        }

        for entry in other_entries {
            let bar_diam = parse_size(&entry.size);
            let current_inset = base_inset + bar_diam / 2.0;
            if let RebarPattern::Perimeter = entry.pattern {
                if let Some(Shape::Circle { diameter }) = props.shape {
                    let r = diameter / 2.0 - current_inset;
                    let step_angle = 2.0 * PI / (entry.count as f64);

                    for i in 0..entry.count {
                        let angle = (i as f64) * step_angle;
                        let x = r * angle.cos();
                        let y = r * angle.sin();
                        add_rebar_circle(&mut section_drawing, x, y, bar_diam, &entry.size);
                    }
                }
            }
        }

        drawings.push(section_drawing);
    }

    // --- Longitudinal Drawing ---
    if show_longitudinal {
        if let Some(span) = props.span {
            let mut long_drawing = Drawing::new();
            long_drawing.id = Some(format!("{} (Longitudinal)", section.id));
            long_drawing.id = Some(format!("{} (Longitudinal)", section.id));
            long_drawing.scale = props.scale_long;

            let height = match &props.shape {
                Some(Shape::Rect { height, .. }) => *height,
                Some(Shape::Circle { diameter }) => *diameter,
                None => 0.0,
            };

            if height > 0.0 {
                // Draw Concrete (Longitudinal)
                long_drawing.add(Primitive::Rect {
                    x: 0.0,
                    y: -height / 2.0,
                    width: span,
                    height: height,
                    stroke: Some(Stroke {
                        color: "black".to_string(),
                        width: 1.0,
                    }),
                    fill: None,
                    group: Some("concrete".to_string()),
                });

                let cover = props.cover.unwrap_or(4.0);

                // Draw Hatched Ends (Supports)
                let support_width = 20.0;
                add_hatched_rect(
                    &mut long_drawing,
                    -support_width,
                    -height / 2.0,
                    support_width,
                    height,
                );
                add_hatched_rect(
                    &mut long_drawing,
                    span,
                    -height / 2.0,
                    support_width,
                    height,
                );

                let mut stirrup_size = 0.95; // Default #3
                if let Some(ties) = &props.ties {
                    stirrup_size = parse_size(&ties.size);
                }
                let base_inset = cover + stirrup_size;

                // Draw Longitudinal Bars (Top)
                for entry in &props.rebar {
                    if let RebarPattern::Top = entry.pattern {
                        let bar_diam = parse_size(&entry.size);
                        let color = get_color_for_size(&entry.size);
                        let y = height / 2.0 - base_inset - bar_diam / 2.0;

                        // Draw as filled Rect
                        long_drawing.add(Primitive::Rect {
                            x: cover,
                            y: y - bar_diam / 2.0,
                            width: span - 2.0 * cover,
                            height: bar_diam,
                            stroke: None,
                            fill: Some(color),
                            group: Some("rebar_long".to_string()),
                        });
                    }
                }

                // Draw Longitudinal Bars (Bottom)
                for entry in &props.rebar {
                    if let RebarPattern::Bottom = entry.pattern {
                        let bar_diam = parse_size(&entry.size);
                        let color = get_color_for_size(&entry.size);
                        let y = -height / 2.0 + base_inset + bar_diam / 2.0;

                        // Draw as filled Rect
                        long_drawing.add(Primitive::Rect {
                            x: cover,
                            y: y - bar_diam / 2.0,
                            width: span - 2.0 * cover,
                            height: bar_diam,
                            stroke: None,
                            fill: Some(color),
                            group: Some("rebar_long".to_string()),
                        });
                    }
                }

                // Draw Stirrups
                if let Some(ties) = &props.ties {
                    let mut left_x = cover;
                    let mut right_x = span - cover;
                    let stirrup_color = get_color_for_size(&ties.size);

                    for spacing in &ties.dist {
                        match spacing {
                            crate::parser::ast::Spacing::Fixed { count, dist } => {
                                for _ in 0..*count {
                                    left_x += dist;
                                    if left_x >= right_x {
                                        break;
                                    }
                                    add_stirrup_line(
                                        &mut long_drawing,
                                        left_x,
                                        height,
                                        cover,
                                        stirrup_size,
                                        &stirrup_color,
                                    );

                                    right_x -= dist;
                                    if right_x <= left_x {
                                        break;
                                    }
                                    add_stirrup_line(
                                        &mut long_drawing,
                                        right_x,
                                        height,
                                        cover,
                                        stirrup_size,
                                        &stirrup_color,
                                    );
                                }
                            }
                            crate::parser::ast::Spacing::Rest { dist } => {
                                let gap = right_x - left_x;
                                if gap > 0.0 {
                                    let num_spaces = (gap / dist).ceil() as u32;
                                    if num_spaces > 0 {
                                        let actual_dist = gap / (num_spaces as f64);
                                        for k in 1..num_spaces {
                                            let pos = left_x + (k as f64) * actual_dist;
                                            add_stirrup_line(
                                                &mut long_drawing,
                                                pos,
                                                height,
                                                cover,
                                                stirrup_size,
                                                &stirrup_color,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            drawings.push(long_drawing);
        }
    }

    drawings
}

fn add_stirrup_line(
    drawing: &mut Drawing,
    x: f64,
    height: f64,
    cover: f64,
    size: f64,
    color: &str,
) {
    let y_top = height / 2.0 - cover;
    let y_bot = -height / 2.0 + cover;
    let h = y_top - y_bot;

    drawing.add(Primitive::Rect {
        x: x - size / 2.0,
        y: y_bot,
        width: size,
        height: h,
        stroke: None,
        fill: Some(color.to_string()),
        group: Some("stirrup_long".to_string()),
    });
}

fn add_arc(
    points: &mut Vec<(f64, f64)>,
    cx: f64,
    cy: f64,
    r: f64,
    start_angle: f64,
    end_angle: f64,
) {
    let steps = 10;
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let angle = start_angle + t * (end_angle - start_angle);
        points.push((cx + r * angle.cos(), cy + r * angle.sin()));
    }
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
            add_rebar_circle(drawing, x, y, bar_diam, &entry.size);
        }
    }
}

fn add_rebar_circle(drawing: &mut Drawing, x: f64, y: f64, diam: f64, size_str: &str) {
    let color = get_color_for_size(size_str);

    // Outer circle (Stroke)
    drawing.add(Primitive::Circle {
        x,
        y,
        radius: diam / 2.0,
        stroke: Some(Stroke {
            color: color.clone(),
            width: 1.0,
        }),
        fill: None,
        group: Some("rebar_outline".to_string()),
    });

    // Inner circle (Fill) - slightly smaller to leave a gap? Or just fill?
    // User said: "dos circulos concentricos, el externo que represente el ancho del acero con la corruga y el circulo interior con fill"
    // Let's make the inner circle 80% of diameter.
    drawing.add(Primitive::Circle {
        x,
        y,
        radius: diam / 2.0 * 0.8,
        stroke: None,
        fill: Some(color),
        group: Some("rebar_core".to_string()),
    });
}

fn get_color_for_size(size: &str) -> String {
    // Darker colors for legibility
    match size {
        "#3" | "3/8\"" => "#CC7000".to_string(), // Dark Orange
        "#4" | "1/2\"" => "#CC0000".to_string(), // Dark Red
        "#5" | "5/8\"" => "#800080".to_string(), // Purple
        "#6" | "3/4\"" => "#000080".to_string(), // Navy Blue
        "#8" | "1\"" => "#006400".to_string(),   // Dark Green
        _ => "black".to_string(),
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

fn add_hatched_rect(drawing: &mut Drawing, x: f64, y: f64, width: f64, height: f64) {
    // Draw Rect Outline
    drawing.add(Primitive::Rect {
        x,
        y,
        width,
        height,
        stroke: Some(Stroke {
            color: "black".to_string(),
            width: 1.0,
        }),
        fill: None,
        group: Some("support".to_string()),
    });

    // Draw Hatching (Diagonal Lines)
    let spacing = 5.0;
    let num_lines = ((width + height) / spacing) as i32;

    for i in 0..num_lines {
        let offset = (i as f64) * spacing;
        // Line equation: Y = X - offset (relative to rect origin)
        // We want to clip this line to the rectangle [x, x+width] x [y, y+height]
        // In local coords (0,0) to (width, height):
        // line is y_local = x_local - C, where C varies.
        // Actually, simpler: iterate diagonals.
        // Start point candidates: (x + offset, y) or (x, y + offset)

        // Let's just draw simple 45 degree lines.
        // P1 = (x + offset, y), P2 = (x + offset - height, y + height)
        // We need to clamp to the box.

        // Simplified approach: iterate x from x to x+width+height with step.
        // Draw line from (current_x, y) to (current_x - height, y + height)
        // Clamp to x_min=x, x_max=x+width.

        // Even simpler: just draw lines and let the renderer handle clipping? No, renderer doesn't clip.
        // We must calculate intersection.

        // Let's try a different hatch: ZigZag lines at the interface?
        // User said "achurados" which means hatched.
        // Let's implement a simple hatch.

        let x1 = x + offset;
        let y1 = y;
        let x2 = x + offset - height; // 45 degrees backwards
        let y2 = y + height;

        // We have a line segment from (x1, y1) to (x2, y2).
        // We need to clip it to the rectangle [x, x+width] x [y, y+height].
        // y range is already [y, y+height].
        // We just need to clip x range to [x, x+width].

        // This is tricky because x varies with y.
        // Let's use a simpler hatch: vertical lines? No, "achurado" usually implies diagonal.

        // Let's try drawing lines from bottom edge to top edge.
        // x_bottom = x + offset
        // x_top = x + offset + height (forward slash /)
        // Clip x_bottom and x_top? No.

        // Correct logic for 45 deg lines in rect (0,0,w,h):
        // Line: y = x - c  => c = x - y.
        // Intersects:
        // Bottom (y=0): x = c
        // Top (y=h): x = c + h
        // Left (x=0): y = -c
        // Right (x=w): y = w - c

        // We iterate c from -h to w.
        // For each c:
        // P1 (enter): max(0, c) -> y = max(0, c) - c = 0 if c>0 else -c
        // P2 (exit): min(w, c+h) -> y = min(w, c+h) - c

        let c = (i as f64) * spacing - height;
        if c > width {
            break;
        }

        let x_start = c.max(0.0);
        let x_end = (c + height).min(width);

        if x_start < x_end {
            let y_start = x_start - c;
            let y_end = x_end - c;

            drawing.add(Primitive::Path {
                points: vec![(x + x_start, y + y_start), (x + x_end, y + y_end)],
                closed: false,
                stroke: Some(Stroke {
                    color: "black".to_string(),
                    width: 0.5,
                }),
                fill: None,
                group: Some("hatch".to_string()),
            });
        }
    }
}
