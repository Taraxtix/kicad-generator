use std::{fmt::{Display, Formatter},
          str::FromStr};

use log::warn;
use serde::{Deserialize, Serialize};

use crate::{parser, schematic::Position};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Graphic {
    Arc {
        start:  (f32, f32),
        mid:    (f32, f32),
        end:    (f32, f32),
        stroke: Stroke,
        fill:   Fill,
    },
    Circle {
        center: (f32, f32),
        radius: f32,
        stroke: Stroke,
        fill:   Fill,
    },
    Bezier {
        points: Vec<(f32, f32)>,
        stroke: Stroke,
        fill:   Fill,
    },
    Polyline {
        points: Vec<(f32, f32)>,
        stroke: Stroke,
        fill:   Fill,
    },
    Rectangle {
        start:  (f32, f32),
        end:    (f32, f32),
        stroke: Stroke,
        fill:   Fill,
    },
    Text {
        text:     String,
        position: Position, // (at x y rotation)
    },
    Pin {
        electrical_type: ElectricalType,
        pin_graphic_style: PinGraphicStyle,
        position: Position,
        length: f32,
        hide: bool,
        name: String,
        name_text_effect: TextEffect,
        number: usize,
        number_text_effect: TextEffect,
        alternates: Vec<PinAlternate>,
    },
}

impl Graphic {
    pub fn extract_arc_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(arc")?;

        let (start, content) =
            parser::expect_regex(content, r#"\(start -?\d+(\.\d+)? -?\d+(\.\d+)?\)"#)?;
        let start = start.replace("(start ", "").replace(")", "");
        let start = start.split_once(' ').unwrap();
        let start = (f32::from_str(start.0).unwrap(), f32::from_str(start.1).unwrap());

        let (mid, content) =
            parser::expect_regex(content, r#"\(mid -?\d+(\.\d+)? -?\d+(\.\d+)?\)"#)?;
        let mid = mid.replace("(mid ", "").replace(")", "");
        let mid = mid.split_once(' ').unwrap();
        let mid = (f32::from_str(mid.0).unwrap(), f32::from_str(mid.1).unwrap());

        let (end, content) =
            parser::expect_regex(content, r#"\(end -?\d+(\.\d+)? -?\d+(\.\d+)?\)"#)?;
        let end = end.replace("(end ", "").replace(")", "");
        let end = end.split_once(' ').unwrap();
        let end = (f32::from_str(end.0).unwrap(), f32::from_str(end.1).unwrap());

        let (stroke, content) = Stroke::extract_from(content)?;
        let (fill, content) = Fill::extract_from(content)?;

        Ok((Self::Arc { start, mid, end, stroke, fill }, content))
    }

    pub fn extract_polyline_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(polyline")?;
        let mut content = parser::expect_str(content, "(pts")?;

        let mut points = vec![];

        while content.starts_with("(xy") {
            let (point, left) =
                parser::expect_regex(content, r#"\(xy -?\d+(\.\d+)? -?\d+(\.\d+)?\)"#)?;
            let point = point.replace("(xy ", "").replace(")", "");
            let point = point.split_once(' ').unwrap();
            let point = (f32::from_str(point.0).unwrap(), f32::from_str(point.1).unwrap());
            points.push(point);
            content = left;
        }

        let content = parser::expect_str(content, ")")?;
        let (stroke, content) = Stroke::extract_from(content)?;
        let (fill, content) = Fill::extract_from(content)?;

        Ok((Self::Polyline { points, stroke, fill }, content))
    }

    pub fn extract_rectangle_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(rectangle")?;
        let (start, content) =
            parser::expect_regex(content, r#"\(start -?\d+(\.\d+)? -?\d+(\.\d+)?\)"#)?;
        let start = start.replace("(start ", "").replace(")", "");
        let start = start.split_once(' ').unwrap();
        let start = (f32::from_str(start.0).unwrap(), f32::from_str(start.1).unwrap());

        let (end, content) =
            parser::expect_regex(content, r#"\(end -?\d+(\.\d+)? -?\d+(\.\d+)?\)"#)?;
        let end = end.replace("(end ", "").replace(")", "");
        let end = end.split_once(' ').unwrap();
        let end = (f32::from_str(end.0).unwrap(), f32::from_str(end.1).unwrap());

        let (stroke, content) = Stroke::extract_from(content)?;
        let (fill, content) = Fill::extract_from(content)?;
        let content = parser::expect_str(content, ")")?;

        Ok((Self::Rectangle { start, end, stroke, fill }, content))
    }

    pub fn extract_circle_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(circle")?;
        let (center, content) =
            parser::expect_regex(content, r#"\(center -?\d+(\.\d+)? -?\d+(\.\d+)?\)"#)?;
        let center = center.replace("(center ", "").replace(")", "");
        let center = center.split_once(' ').unwrap();
        let center = (f32::from_str(center.0).unwrap(), f32::from_str(center.1).unwrap());

        let (radius, content) = parser::expect_regex(content, r#"\(radius \d+(\.\d+)?\)"#)?;
        let radius = radius.replace("(radius ", "").replace(")", "").parse::<f32>().unwrap();
        let (stroke, content) = Stroke::extract_from(content)?;
        let (fill, content) = Fill::extract_from(content)?;
        let content = parser::expect_str(content, ")")?;
        Ok((Self::Circle { center, radius, stroke, fill }, content))
    }

    pub fn extract_pin_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(pin")?;
        let (electrical_type, content) = parser::expect_regex(
            content,
            r#"input|output|bidirectional|tri_state|passive|free|unspecified|power_in|power_out|open_collector|open_emitter|no_connect"#,
        )?;
        let electrical_type = ElectricalType::from(electrical_type);
        let (pin_graphic_style, content) = parser::expect_regex(
            content,
            r#"line|inverted|clock|inverted_clock|input_low|clock_low|output_low|edge_clock_high|non_logic"#,
        )?;
        let pin_graphic_style = PinGraphicStyle::from(pin_graphic_style);
        let (position, content) = Position::extract_from(content)?;
        let (length, mut content) = parser::expect_regex(content, r#"\(length \d+(\.\d+)?\)"#)?;
        let length = length.replace("(length ", "").replace(")", "").parse::<f32>().unwrap();

        let hide = content.starts_with("hide");
        if hide {
            content = parser::expect_str(content, "hide")?;
        }

        let (name, content) = parser::expect_regex(content, r#"\(name "[^"]*""#)?;
        let name = name[7..name.len() - 1].to_string();
        let (name_text_effect, content) = TextEffect::extract_from(content)?;
        let content = parser::expect_str(content, ")")?;
        let (number, content) = parser::expect_regex(content, r#"\(number "[^"]+""#)?;
        let number = number[9..number.len() - 1].parse::<usize>().unwrap();
        let (number_text_effect, content) = TextEffect::extract_from(content)?;
        let mut content = parser::expect_str(content, ")")?;
        let mut alternates = vec![];
        while content.starts_with("(alternate") {
            let (alternate, left) = PinAlternate::extract_from(content)?;
            alternates.push(alternate);
            content = left;
        }
        let content = parser::expect_str(content, ")")?;

        Ok((
            Self::Pin {
                electrical_type,
                pin_graphic_style,
                position,
                length,
                hide,
                name,
                name_text_effect,
                number,
                number_text_effect,
                alternates,
            },
            content,
        ))
    }
}

impl Display for Graphic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Graphic::Arc { start: (sx, sy), mid: (mx, my), end: (ex, ey), stroke, fill } => {
                f.write_fmt(format_args!(
                    "(arc\n(start {sx} {sy})\n(mid {mx} {my})\n(end {ex} \
                     {ey})\n{stroke}\n{fill}\n)"
                ))
            },
            Graphic::Circle { center: (cx, cy), radius, stroke, fill } => {
                f.write_fmt(format_args!(
                    "(circle\n(center {cx} {cy})\n(radius {radius})\n{stroke}\n{fill})"
                ))
            },
            Graphic::Bezier { points, stroke, fill } => {
                f.write_str("(bezier\n(pts")?;
                for point in points {
                    f.write_fmt(format_args!("\n(xy {x} {y})", x = point.0, y = point.1))?;
                }
                f.write_fmt(format_args!(")\n{stroke}\n{fill}"))
            },
            Graphic::Polyline { points, stroke, fill } => {
                f.write_str("(polyline\n(pts")?;
                for point in points {
                    f.write_fmt(format_args!("\n(xy {x} {y})", x = point.0, y = point.1))?;
                }
                f.write_fmt(format_args!(")\n{stroke}\n{fill}"))
            },
            Graphic::Rectangle { start: (sx, sy), end: (ex, ey), stroke, fill } => {
                f.write_fmt(format_args!(
                    "\
            (rectangle\n(start {sx} {sy})\n(end {ex} {ey})\n{stroke}\n{fill})"
                ))
            },
            Graphic::Text { text, position } => {
                f.write_fmt(format_args!("(text\n(text \"{text}\")\n{position}\n)"))
            },
            Graphic::Pin {
                electrical_type,
                pin_graphic_style,
                position,
                length,
                hide,
                name,
                name_text_effect,
                number,
                number_text_effect,
                alternates,
            } => {
                f.write_fmt(format_args!(
                    "(pin\n{electrical_type}\n{pin_graphic_style}\n{position}\n(length \
                     {length})\n(hide {})\n(name \"{name}\" {name_text_effect})\n(number \
                     \"{number}\" {number_text_effect})\n",
                    if *hide { "yes" } else { "no" }
                ))?;
                // {alternates}\n))
                for alternate in alternates {
                    f.write_fmt(format_args!("{alternate}\n"))?;
                }
                f.write_str(")")
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct PinAlternate {
    name: String,
    electrical_type: ElectricalType,
    pin_graphic_style: PinGraphicStyle,
}

impl Display for PinAlternate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "(alternate \"{name}\" {electrical_type} {pin_graphic_style}))",
            name = self.name,
            electrical_type = self.electrical_type,
            pin_graphic_style = self.pin_graphic_style,
        ))
    }
}

impl PinAlternate {
    pub fn extract_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(alternate")?;
        let (name, content) = parser::expect_regex(content, r#""[^"]*""#)?;
        let name = name[1..name.len() - 1].to_string();
        let (electrical_type, content) = parser::expect_regex(
            content,
            r#"input|output|bidirectional|tri_state|passive|free|unspecified|power_in|power_out|open_collector|open_emitter|no_connect"#,
        )?;
        let electrical_type = ElectricalType::from(electrical_type);
        let (pin_graphic_style, content) = parser::expect_regex(
            content,
            r#"line|inverted|clock|inverted_clock|input_low|clock_low|output_low|edge_clock_high|non_logic"#,
        )?;
        let pin_graphic_style = PinGraphicStyle::from(pin_graphic_style);
        let content = parser::expect_str(content, ")")?;
        Ok((Self { name, electrical_type, pin_graphic_style }, content))
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Stroke {
    width: f32,
    ty:    StrokeType,
    color: Option<(f32, f32, f32, f32)>, // RGBA
}

impl Display for Stroke {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Stroke { width, ty, color } = self;
        f.write_fmt(format_args!("(stroke\n(width {width})\n(type {ty})\n"))?;
        if let Some((r, g, b, a)) = color {
            f.write_fmt(format_args!("(color {r} {g} {b} {a})\n"))?;
        }
        f.write_str(")")
    }
}

impl Stroke {
    fn extract_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(stroke")?;
        let (width, content) = parser::expect_regex(content, r#"\(width \d+(\.\d+)?\)"#)?;
        let width = width.replace("(width ", "").replace(")", "").parse::<f32>().unwrap();
        let (ty, content) = StrokeType::extract_from(content)?;
        let (color, content) = if content.starts_with("(color") {
            let (color, content) = parser::expect_regex(
                content,
                r#"\(color \d+(\.\d+)? \d+(\.\d+)? \d+(\.\d+)? \d+(\.\d+)?\)"#,
            )?;
            let color = color.replace("(color ", "").replace(")", "");
            let color = color.split(' ').map(|x| x.parse::<f32>().unwrap()).collect::<Vec<_>>();
            let color = Some((color[0], color[1], color[2], color[3]));
            (color, content)
        } else {
            (None, content)
        };
        let content = parser::expect_str(content, ")")?;
        Ok((Self { width, ty, color }, content))
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum StrokeType {
    Dash,
    DashDot,
    DashDotDot,
    Dot,
    Default,
    Solid,
}

impl Display for StrokeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StrokeType::Dash => f.write_str("dash"),
            StrokeType::DashDot => f.write_str("dash_dot"),
            StrokeType::DashDotDot => f.write_str("dash_dot_dot"),
            StrokeType::Dot => f.write_str("dot"),
            StrokeType::Default => f.write_str("default"),
            StrokeType::Solid => f.write_str("solid"),
        }
    }
}

impl StrokeType {
    fn extract_from(content: &str) -> Result<(Self, &str), String> {
        let (ty, content) = parser::expect_regex(
            content,
            r#"\(type (dash|dashdot|dashdotdot|dot|default|solid)\)"#,
        )?;
        Ok((
            match ty {
                "(type dash)" => Self::Dash,
                "(type dashdot)" => Self::DashDot,
                "(type dashdotdot)" => Self::DashDotDot,
                "(type dot)" => Self::Dot,
                "(type default)" => Self::Default,
                "(type solid)" => Self::Solid,
                _ => unreachable!(),
            },
            content,
        ))
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Fill {
    None,
    Outline,
    Background,
}

impl Display for Fill {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Fill::None => f.write_str("(fill\n(type none)\n)"),
            Fill::Outline => f.write_str("(fill\n(type outline)\n)"),
            Fill::Background => f.write_str("(fill\n(type background)\n)"),
        }
    }
}

impl Fill {
    fn extract_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(fill")?;

        let (ty, content) = parser::expect_regex(content, r#"\(type (none|background|outline)\)"#)?;
        let ty = match ty {
            "(type none)" => Self::None,
            "(type background)" => Self::Background,
            "(type outline)" => Self::Outline,
            _ => unreachable!(),
        };

        let content = parser::expect_str(content, ")")?;

        Ok((ty, content))
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct TextEffect {
    font:    Font,
    justify: Option<String>,
    hide:    bool,
}

impl TextEffect {
    pub fn extract_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(effects")?;
        let (font, mut content) = Font::extract_from(content)?;

        let mut it = Self { font, hide: false, justify: None };

        loop {
            if content.starts_with(")") {
                content = parser::expect_str(content, ")")?;
                break;
            } else if content.starts_with("(hide") {
                let (hide, left) = parser::expect_regex(content, r#"\(hide (yes|no)\)"#)?;
                let hide = &hide[6..hide.len() - 1];
                it.hide = hide == "yes";
                content = left;
            } else if content.starts_with("(justify") {
                let (justify, left) = parser::expect_regex(content, r#"\(justify [^\)]+\)"#)?;
                let justify = &justify[9..justify.len() - 1];
                it.justify = Some(justify.to_string());
                content = left;
            } else {
                let (skipped, left) = parser::expect_regex(content, r#"\([^\)]*\)"#)?;
                warn!("Skipped: {}", skipped);
                content = left;
            }
        }

        Ok((it, content))
    }
}

impl Display for TextEffect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let TextEffect { font, justify, hide } = self;
        f.write_fmt(format_args!("(effects\n{font}\n"))?;
        if let Some(justify) = justify {
            f.write_fmt(format_args!("(justify {justify})\n"))?;
        }
        if *hide {
            f.write_str("hide\n")?;
        }
        f.write_str(")")
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
struct Font {
    size:   (f32, f32), //(size HEIGHT WIDTH)
    italic: bool,
}

impl Display for Font {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("(font\n(size {} {})", self.size.0, self.size.1))?;
        if self.italic {
            f.write_str(" (italic yes)")?;
        }
        f.write_str("\n)")
    }
}

impl Font {
    fn extract_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(font")?;
        let (size, mut content) =
            parser::expect_regex(content, r#"\(size \d+(\.\d+)? \d+(\.\d+)?\)"#)?;
        let size = size.replace("(size ", "").replace(")", "");
        let size = size.split_once(' ').unwrap();
        let size = (f32::from_str(size.0).unwrap(), f32::from_str(size.1).unwrap());

        let mut italic = false;

        while !content.starts_with(")") {
            if content.starts_with("(italic yes)") {
                let left = parser::expect_str(content, "(italic yes)")?;
                italic = true;
                content = left;
            } else {
                let (skipped, left) = parser::expect_regex(content, r#"\([^\)]*\)"#)?;
                warn!("Skipped: {}", skipped);
                content = left;
            }
        }
        let content = parser::expect_str(content, ")")?;

        Ok((Self { size, italic }, content))
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum ElectricalType {
    Input,
    Output,
    Bidirectional,
    TriState,
    Passive,
    Free,
    Unspecified,
    PowerIn,
    PowerOut,
    OpenCollector,
    OpenEmitter,
    NoConnect,
}

impl From<&str> for ElectricalType {
    fn from(value: &str) -> Self {
        match value {
            "input" => Self::Input,
            "output" => Self::Output,
            "bidirectional" => Self::Bidirectional,
            "tri_state" => Self::TriState,
            "passive" => Self::Passive,
            "free" => Self::Free,
            "unspecified" => Self::Unspecified,
            "power_in" => Self::PowerIn,
            "power_out" => Self::PowerOut,
            "open_collector" => Self::OpenCollector,
            "open_emitter" => Self::OpenEmitter,
            "no_connect" => Self::NoConnect,
            _ => unreachable!("Unknown ElectricalType"),
        }
    }
}

impl Display for ElectricalType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ElectricalType::Input => "input",
            ElectricalType::Output => "output",
            ElectricalType::Bidirectional => "bidirectional",
            ElectricalType::TriState => "tri_state",
            ElectricalType::Passive => "passive",
            ElectricalType::Free => "free",
            ElectricalType::Unspecified => "unspecified",
            ElectricalType::PowerIn => "power_in",
            ElectricalType::PowerOut => "power_out",
            ElectricalType::OpenCollector => "open_collector",
            ElectricalType::OpenEmitter => "open_emitter",
            ElectricalType::NoConnect => "no_connect",
        })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum PinGraphicStyle {
    Line,
    Inverted,
    Clock,
    InvertedClock,
    InputLow,
    ClockLow,
    OutputLow,
    EdgeClockHigh,
    NonLogic,
}

impl From<&str> for PinGraphicStyle {
    fn from(value: &str) -> Self {
        match value {
            "line" => Self::Line,
            "inverted" => Self::Inverted,
            "clock" => Self::Clock,
            "inverted_clock" => Self::InvertedClock,
            "input_low" => Self::InputLow,
            "clock_low" => Self::ClockLow,
            "output_low" => Self::OutputLow,
            "edge_clock_high" => Self::EdgeClockHigh,
            "non_logic" => Self::NonLogic,
            _ => unreachable!("Unknown pin graphic style"),
        }
    }
}

impl Display for PinGraphicStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PinGraphicStyle::Line => "line",
            PinGraphicStyle::Inverted => "inverted",
            PinGraphicStyle::Clock => "clock",
            PinGraphicStyle::InvertedClock => "inverted_clock",
            PinGraphicStyle::InputLow => "input_low",
            PinGraphicStyle::ClockLow => "clock_low",
            PinGraphicStyle::OutputLow => "output_low",
            PinGraphicStyle::EdgeClockHigh => "edge_clock_high",
            PinGraphicStyle::NonLogic => "non_logic",
        })
    }
}
