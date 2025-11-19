use crate::parser;
use crate::parser::expect_regex;
use crate::schematic::Position;
use log::warn;
use std::str::FromStr;

#[derive(Debug)]
pub enum Graphic {
    Arc {
        start: (f32, f32),
        mid: (f32, f32),
        end: (f32, f32),
        stroke: Stroke,
        fill: Fill,
    },
    Circle {
        center: (f32, f32),
        radius: f32,
        stroke: Stroke,
        fill: Fill,
    },
    Bezier {
        points: Vec<(f32, f32)>,
        stroke: Stroke,
        fill: Fill,
    },
    Polyline {
        points: Vec<(f32, f32)>,
        stroke: Stroke,
        fill: Fill,
    },
    Rectangle {
        start: (f32, f32),
        end: (f32, f32),
        stroke: Stroke,
        fill: Fill,
    },
    Text {
        text: String,
        position: Position, // (at x y rotation)
    },
    Pin {
        electrical_type: ElectricalType,
        pin_graphic_style: PinGraphicStyle,
        position: Position,
        length: f32,
        name: String,
        name_text_effect: TextEffect,
        number: usize,
        number_text_effect: TextEffect,
    },
}

impl Graphic {
    pub fn extract_rectangle_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(rectangle")?;
        let (start, content) =
            parser::expect_regex(content, r#"\(start -?\d+(\.\d+)? -?\d+(\.\d+)?\)"#)?;
        let start = start.replace("(start ", "").replace(")", "");
        let start = start.split_once(' ').unwrap();
        let start = (
            f32::from_str(start.0).unwrap(),
            f32::from_str(start.1).unwrap(),
        );

        let (end, content) =
            parser::expect_regex(content, r#"\(end -?\d+(\.\d+)? -?\d+(\.\d+)?\)"#)?;
        let end = end.replace("(end ", "").replace(")", "");
        let end = end.split_once(' ').unwrap();
        let end = (f32::from_str(end.0).unwrap(), f32::from_str(end.1).unwrap());

        let (stroke, content) = Stroke::extract_from(content)?;
        let (fill, content) = Fill::extract_from(content)?;
        let content = parser::expect_str(content, ")")?;

        Ok((
            Self::Rectangle {
                start,
                end,
                stroke,
                fill,
            },
            content,
        ))
    }

    pub fn extract_circle_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(circle")?;
        let (center, content) =
            parser::expect_regex(content, r#"\(center -?\d+(\.\d+)? -?\d+(\.\d+)?\)"#)?;
        let center = center.replace("(center ", "").replace(")", "");
        let center = center.split_once(' ').unwrap();
        let center = (
            f32::from_str(center.0).unwrap(),
            f32::from_str(center.1).unwrap(),
        );

        let (radius, content) = parser::expect_regex(content, r#"\(radius \d+(\.\d+)?\)"#)?;
        let radius = radius
            .replace("(radius ", "")
            .replace(")", "")
            .parse::<f32>()
            .unwrap();
        let (stroke, content) = Stroke::extract_from(content)?;
        let (fill, content) = Fill::extract_from(content)?;
        let content = parser::expect_str(content, ")")?;
        Ok((
            Self::Circle {
                center,
                radius,
                stroke,
                fill,
            },
            content,
        ))
    }

    pub fn extract_pin_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(pin")?;
        let (electrical_type, content) = parser::expect_regex(
            content,
            r#"input|output|bidirectional|tristate|passive|free|unspecified|powerin|powerout"#,
        )?;
        let electrical_type = ElectricalType::from(electrical_type);
        let (pin_graphic_style, content) = parser::expect_regex(
            content,
            r#"line|inverted|clock|invertedclock|inputlow|clocklow|outputlow|edgeclockhigh|nonlogic"#,
        )?;
        let pin_graphic_style = PinGraphicStyle::from(pin_graphic_style);
        let (position, content) = Position::extract_from(content)?;
        let (length, content) = parser::expect_regex(content, r#"\(length \d+(\.\d+)?\)"#)?;
        let length = length
            .replace("(length ", "")
            .replace(")", "")
            .parse::<f32>()
            .unwrap();
        let (name, content) = expect_regex(content, r#"\(name "[^"]+""#)?;
        let name = name[7..name.len() - 1].to_string();
        let (name_text_effect, content) = TextEffect::extract_from(content)?;
        let content = parser::expect_str(content, ")")?;
        let (number, content) = expect_regex(content, r#"\(number "[^"]+""#)?;
        let number = number[9..number.len() - 1].parse::<usize>().unwrap();
        let (number_text_effect, content) = TextEffect::extract_from(content)?;
        let content = parser::expect_str(content, ")")?;
        let content = parser::expect_str(content, ")")?;

        Ok((
            Self::Pin {
                electrical_type,
                pin_graphic_style,
                position,
                length,
                name,
                name_text_effect,
                number,
                number_text_effect,
            },
            content,
        ))
    }
}

#[derive(Debug)]
pub struct Stroke {
    width: f32,
    ty: StrokeType,
    color: Option<(f32, f32, f32, f32)>, //RGBA
}

impl Stroke {
    fn extract_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(stroke")?;
        let (width, content) = parser::expect_regex(content, r#"\(width \d+(\.\d+)?\)"#)?;
        let width = width
            .replace("(width ", "")
            .replace(")", "")
            .parse::<f32>()
            .unwrap();
        let (ty, content) = StrokeType::extract_from(content)?;
        let (color, content) = if content.starts_with("(color") {
            let (color, content) = parser::expect_regex(
                content,
                r#"\(color \d+(\.\d+)? \d+(\.\d+)? \d+(\.\d+)? \d+(\.\d+)?\)"#,
            )?;
            let color = color.replace("(color ", "").replace(")", "");
            let color = color
                .split(' ')
                .map(|x| x.parse::<f32>().unwrap())
                .collect::<Vec<_>>();
            let color = Some((color[0], color[1], color[2], color[3]));
            (color, content)
        } else {
            (None, content)
        };
        let content = parser::expect_str(content, ")")?;
        Ok((Self { width, ty, color }, content))
    }
}

#[derive(Debug)]
pub enum StrokeType {
    Dash,
    DashDot,
    DashDotDot,
    Dot,
    Default,
    Solid,
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

#[derive(Debug)]
pub enum Fill {
    None,
    Outline,
    Background,
}

impl Fill {
    //      (fill
    //         (type background)
    //     )
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

#[derive(Debug)]
pub struct TextEffect {
    font: Font,
    hide: bool,
}

impl TextEffect {
    pub fn extract_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(effects")?;
        let (font, mut content) = Font::extract_from(content)?;

        let mut it = Self { font, hide: false };

        loop {
            if content.starts_with(")") {
                content = parser::expect_str(content, ")")?;
                break;
            } else if content.starts_with("(hide") {
                let (hide, left) = parser::expect_regex(content, r#"\(hide (yes|no)\)"#)?;
                let hide = &hide[6..hide.len() - 1];
                it.hide = hide == "yes";
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

#[derive(Debug)]
struct Font {
    size: (f32, f32), //(size HEIGHT WIDTH)
}

impl Font {
    fn extract_from(content: &str) -> Result<(Self, &str), String> {
        let content = parser::expect_str(content, "(font")?;
        let (size, mut content) =
            parser::expect_regex(content, r#"\(size \d+(\.\d+)? \d+(\.\d+)?\)"#)?;
        let size = size.replace("(size ", "").replace(")", "");
        let size = size.split_once(' ').unwrap();
        let size = (
            f32::from_str(size.0).unwrap(),
            f32::from_str(size.1).unwrap(),
        );

        while !content.starts_with(")") {
            let (skipped, left) = parser::expect_regex(content, r#"\([^\)]*\)"#)?;
            warn!("Skipped: {}", skipped);
            content = left;
        }
        let content = parser::expect_str(content, ")")?;

        Ok((Self { size }, content))
    }
}

#[derive(Debug)]
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
            "tristate" => Self::TriState,
            "passive" => Self::Passive,
            "free" => Self::Free,
            "unspecified" => Self::Unspecified,
            "powerin" => Self::PowerIn,
            "powerout" => Self::PowerOut,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
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
            "invertedclock" => Self::InvertedClock,
            "inputlow" => Self::InputLow,
            "clocklow" => Self::ClockLow,
            "outputlow" => Self::OutputLow,
            "edgeclockhigh" => Self::EdgeClockHigh,
            "nonlogic" => Self::NonLogic,
            _ => unreachable!(),
        }
    }
}
