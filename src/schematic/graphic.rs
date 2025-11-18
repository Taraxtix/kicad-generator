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
        position: (f32, f32, f32), // (at x y rotation)
    },
    Pin {
        electrical_type: ElectricalType,
        pin_graphic_style: PinGraphicStyle,
        position: (f32, f32, f32),
        length: f32,
        name: String,
        name_text_effect: TextEffect,
        number: usize,
        number_text_effect: TextEffect,
    },
}

#[derive(Debug)]
pub struct Stroke {
    width: f32,
    ty: StrokeType,
    color: (f32, f32, f32, f32), //RGBA
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

#[derive(Debug)]
pub enum Fill {
    None,
    Outline,
    Background,
}

#[derive(Debug)]
pub struct TextEffect {
    font: Font,
}

#[derive(Debug)]
struct Font {
    size: (f32, f32), //(size HEIGHT WIDTH)
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
