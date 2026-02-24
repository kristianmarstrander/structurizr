//! Shape rendering implementation

use crate::model::{ElementStyle, Shape};
use svg::node::element::{Circle, Ellipse, Line, Path, Polygon, Rectangle, Text as SvgText};
use svg::node::element::path::Data;
use svg::Node;

pub struct ShapeRenderer;

impl ShapeRenderer {
    /// Render a shape as SVG element
    pub fn render_shape(
        shape: &Shape,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        style: &ElementStyle,
    ) -> Box<dyn Node> {
        let fill = style.get_background();
        let stroke = style.get_stroke();
        let stroke_width = style.get_stroke_width();
        let dasharray = style.get_dasharray();
        
        match shape {
            Shape::Box => Self::render_box(x, y, width, height, 3, &fill, &stroke, stroke_width, dasharray),
            Shape::RoundedBox => Self::render_box(x, y, width, height, 20, &fill, &stroke, stroke_width, dasharray),
            Shape::Circle => Self::render_circle(x, y, width, &fill, &stroke, stroke_width, dasharray),
            Shape::Ellipse => Self::render_ellipse(x, y, width, height, &fill, &stroke, stroke_width, dasharray),
            Shape::Hexagon => Self::render_hexagon(x, y, width, &fill, &stroke, stroke_width, dasharray),
            Shape::Diamond => Self::render_diamond(x, y, width, height, &fill, &stroke, stroke_width, dasharray),
            Shape::Cylinder => Self::render_cylinder(x, y, width, height, &fill, &stroke, stroke_width, dasharray),
            Shape::Pipe => Self::render_pipe(x, y, width, height, &fill, &stroke, stroke_width, dasharray),
            Shape::Person => Self::render_person(x, y, width, height, &fill, &stroke, stroke_width, dasharray),
            Shape::Robot => Self::render_robot(x, y, width, height, &fill, &stroke, stroke_width, dasharray),
            Shape::Folder => Self::render_folder(x, y, width, height, &fill, &stroke, stroke_width, dasharray),
            Shape::WebBrowser => Self::render_web_browser(x, y, width, height, &fill, &stroke, stroke_width, dasharray),
            Shape::Window => Self::render_window(x, y, width, height, &fill, &stroke, stroke_width, dasharray),
            Shape::Terminal => Self::render_terminal(x, y, width, height, &fill, &stroke, stroke_width, dasharray),
            Shape::Shell => Self::render_shell(x, y, width, height, &fill, &stroke, stroke_width, dasharray),
            Shape::MobileDevicePortrait => Self::render_mobile_portrait(x, y, width, height, &fill, &stroke, stroke_width, dasharray),
            Shape::MobileDeviceLandscape => Self::render_mobile_landscape(x, y, width, height, &fill, &stroke, stroke_width, dasharray),
            Shape::Component => Self::render_component(x, y, width, height, &fill, &stroke, stroke_width, dasharray),
        }
    }

    fn render_box(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        corner_radius: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let mut rect = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", width)
            .set("height", height)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if corner_radius > 0 {
            rect = rect.set("rx", corner_radius).set("ry", corner_radius);
        }
        
        if !dasharray.is_empty() {
            rect = rect.set("stroke-dasharray", dasharray);
        }
        
        Box::new(rect)
    }

    fn render_circle(
        x: i32,
        y: i32,
        width: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let radius = width / 2;
        let mut circle = Circle::new()
            .set("cx", x + radius as i32)
            .set("cy", y + radius as i32)
            .set("r", radius)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            circle = circle.set("stroke-dasharray", dasharray);
        }
        
        Box::new(circle)
    }

    fn render_ellipse(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let mut ellipse = Ellipse::new()
            .set("cx", x + (width / 2) as i32)
            .set("cy", y + (height / 2) as i32)
            .set("rx", width / 2)
            .set("ry", height / 2)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            ellipse = ellipse.set("stroke-dasharray", dasharray);
        }
        
        Box::new(ellipse)
    }

    fn render_hexagon(
        x: i32,
        y: i32,
        width: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let w = width as f32;
        let h = ((w / 2.0) * 3.0_f32.sqrt()).floor() as u32;
        
        let points = format!(
            "{},{} {},{} {},{} {},{} {},{} {},{}",
            x + (w / 4.0) as i32, y,
            x + (3.0 * w / 4.0) as i32, y,
            x + width as i32, y + (h / 2) as i32,
            x + (3.0 * w / 4.0) as i32, y + h as i32,
            x + (w / 4.0) as i32, y + h as i32,
            x, y + (h / 2) as i32
        );
        
        let mut polygon = Polygon::new()
            .set("points", points)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            polygon = polygon.set("stroke-dasharray", dasharray);
        }
        
        Box::new(polygon)
    }

    fn render_diamond(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let points = format!(
            "{},{} {},{} {},{} {},{}",
            x + (width / 2) as i32, y,
            x + width as i32, y + (height / 2) as i32,
            x + (width / 2) as i32, y + height as i32,
            x, y + (height / 2) as i32
        );
        
        let mut polygon = Polygon::new()
            .set("points", points)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            polygon = polygon.set("stroke-dasharray", dasharray);
        }
        
        Box::new(polygon)
    }

    fn render_cylinder(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let lid_radius = 45;
        let w = width as f32;
        let h = height as f32;
        
        let path_data = Data::new()
            .move_to((0, lid_radius / 2))
            .elliptical_arc_to((w / 2.0, lid_radius as f32 / 2.0, 0, 0, 0, w, 0))
            .elliptical_arc_to((w / 2.0, lid_radius as f32 / 2.0, 0, 0, 0, -w, 0))
            .line_by((0, h - lid_radius as f32))
            .elliptical_arc_to((w / 2.0, lid_radius as f32 / 2.0, 0, 0, 0, w, 0))
            .line_by((0, -(h - lid_radius as f32)))
            .close();
        
        let mut path = Path::new()
            .set("d", path_data)
            .set("transform", format!("translate({},{})", x, y))
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            path = path.set("stroke-dasharray", dasharray);
        }
        
        Box::new(path)
    }

    fn render_pipe(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let lid_radius = 45;
        let w = width as f32;
        let h = height as f32;
        
        let path_data = Data::new()
            .move_to((lid_radius / 2, 0))
            .elliptical_arc_to((lid_radius as f32 / 2.0, h / 2.0, 0, 0, 1, 0, h))
            .elliptical_arc_to((lid_radius as f32 / 2.0, h / 2.0, 0, 0, 1, 0, -h))
            .line_by((w - lid_radius as f32, 0))
            .elliptical_arc_to((lid_radius as f32 / 2.0, h / 2.0, 0, 0, 1, 0, h))
            .line_by((-(w - lid_radius as f32), 0))
            .close();
        
        let mut path = Path::new()
            .set("d", path_data)
            .set("transform", format!("translate({},{})", x, y))
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            path = path.set("stroke-dasharray", dasharray);
        }
        
        Box::new(path)
    }

    fn render_person(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        // Person shape: circle head + rounded body
        let mut group = svg::node::element::Group::new();
        
        // Body (rounded rectangle)
        let body_y = y + (height as f32 / 2.5) as i32;
        let body_height = height - (height as f32 / 2.5) as u32;
        let mut body = Rectangle::new()
            .set("x", x)
            .set("y", body_y)
            .set("width", width)
            .set("height", body_height)
            .set("rx", 90)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            body = body.set("stroke-dasharray", dasharray);
        }
        group = group.add(body);
        
        // Head (circle)
        let head_radius = height as f32 / 4.5;
        let mut head = Circle::new()
            .set("cx", x + (width / 2) as i32)
            .set("cy", y + head_radius as i32)
            .set("r", head_radius)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            head = head.set("stroke-dasharray", dasharray);
        }
        group = group.add(head);
        
        // Legs (lines)
        let leg_top = y + (height as f32 / 1.5) as i32;
        let leg_bottom = y + height as i32;
        
        let mut leg1 = Line::new()
            .set("x1", x + (width / 5) as i32)
            .set("y1", leg_top)
            .set("x2", x + (width / 5) as i32)
            .set("y2", leg_bottom)
            .set("stroke", stroke)
            .set("stroke-width", 1);
        
        if !dasharray.is_empty() {
            leg1 = leg1.set("stroke-dasharray", dasharray);
        }
        group = group.add(leg1);
        
        let mut leg2 = Line::new()
            .set("x1", x + width as i32 - (width / 5) as i32)
            .set("y1", leg_top)
            .set("x2", x + width as i32 - (width / 5) as i32)
            .set("y2", leg_bottom)
            .set("stroke", stroke)
            .set("stroke-width", 1);
        
        if !dasharray.is_empty() {
            leg2 = leg2.set("stroke-dasharray", dasharray);
        }
        group = group.add(leg2);
        
        Box::new(group)
    }

    fn render_robot(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let mut group = svg::node::element::Group::new();
        
        // Body
        let body_y = y + (height as f32 / 2.5) as i32;
        let body_height = height - (height as f32 / 2.5) as u32;
        let mut body = Rectangle::new()
            .set("x", x)
            .set("y", body_y)
            .set("width", width)
            .set("height", body_height)
            .set("rx", 40)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            body = body.set("stroke-dasharray", dasharray);
        }
        group = group.add(body);
        
        // Antenna
        let antenna_width = width as f32 / 1.8;
        let antenna_height = height as f32 / 10.0;
        let antenna_x = x + ((width as f32 - antenna_width) / 2.0) as i32;
        let antenna_y = y + ((width as f32 / 2.25 - antenna_height) / 2.0) as i32;
        
        let mut antenna = Rectangle::new()
            .set("x", antenna_x)
            .set("y", antenna_y)
            .set("width", antenna_width)
            .set("height", antenna_height)
            .set("rx", 10)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            antenna = antenna.set("stroke-dasharray", dasharray);
        }
        group = group.add(antenna);
        
        // Head
        let head_width = width as f32 / 2.25;
        let head_height = height as f32 / 2.25;
        let head_x = x + ((height as f32 - head_height) / 2.0) as i32;
        
        let mut head = Rectangle::new()
            .set("x", head_x)
            .set("y", y)
            .set("width", head_width)
            .set("height", head_height)
            .set("rx", 40)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            head = head.set("stroke-dasharray", dasharray);
        }
        group = group.add(head);
        
        // Legs
        let leg_top = y + (height as f32 / 1.5) as i32;
        let leg_bottom = y + height as i32;
        
        let mut leg1 = Line::new()
            .set("x1", x + (width / 5) as i32)
            .set("y1", leg_top)
            .set("x2", x + (width / 5) as i32)
            .set("y2", leg_bottom)
            .set("stroke", stroke)
            .set("stroke-width", 1);
        
        if !dasharray.is_empty() {
            leg1 = leg1.set("stroke-dasharray", dasharray);
        }
        group = group.add(leg1);
        
        let mut leg2 = Line::new()
            .set("x1", x + width as i32 - (width / 5) as i32)
            .set("y1", leg_top)
            .set("x2", x + width as i32 - (width / 5) as i32)
            .set("y2", leg_bottom)
            .set("stroke", stroke)
            .set("stroke-width", 1);
        
        if !dasharray.is_empty() {
            leg2 = leg2.set("stroke-dasharray", dasharray);
        }
        group = group.add(leg2);
        
        Box::new(group)
    }

    fn render_folder(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let mut group = svg::node::element::Group::new();
        
        // Tab
        let mut tab = Rectangle::new()
            .set("x", x + 15)
            .set("y", y)
            .set("width", width / 3)
            .set("height", height / 4)
            .set("rx", 15)
            .set("ry", 15)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            tab = tab.set("stroke-dasharray", dasharray);
        }
        group = group.add(tab);
        
        // Main folder body
        let mut body = Rectangle::new()
            .set("x", x)
            .set("y", y + (height / 8) as i32)
            .set("width", width)
            .set("height", height - height / 8)
            .set("rx", 6)
            .set("ry", 6)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            body = body.set("stroke-dasharray", dasharray);
        }
        group = group.add(body);
        
        Box::new(group)
    }

    fn render_web_browser(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let mut group = svg::node::element::Group::new();
        
        // Window frame
        let mut frame = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", width)
            .set("height", height)
            .set("rx", 10)
            .set("ry", 10)
            .set("fill", stroke)
            .set("stroke-width", 0);
        
        if !dasharray.is_empty() {
            frame = frame.set("stroke-dasharray", dasharray);
        }
        group = group.add(frame);
        
        // Content area
        let content = Rectangle::new()
            .set("x", x + stroke_width as i32)
            .set("y", y + 40)
            .set("width", width - stroke_width * 2)
            .set("height", height - 40 - stroke_width)
            .set("rx", 10)
            .set("ry", 10)
            .set("fill", fill)
            .set("stroke-width", 0);
        group = group.add(content);
        
        // Address bar
        let address_bar = Rectangle::new()
            .set("x", x + 100)
            .set("y", y + 10)
            .set("width", width - 110)
            .set("height", 20)
            .set("rx", 10)
            .set("ry", 10)
            .set("fill", fill)
            .set("stroke-width", 0);
        group = group.add(address_bar);
        
        // Control buttons
        for i in 0..3 {
            let button = Circle::new()
                .set("cx", x + 20 + i * 30)
                .set("cy", y + 20)
                .set("r", 10)
                .set("fill", fill)
                .set("stroke-width", 0);
            group = group.add(button);
        }
        
        Box::new(group)
    }

    fn render_window(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let mut group = svg::node::element::Group::new();
        
        // Window frame
        let mut frame = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", width)
            .set("height", height)
            .set("rx", 10)
            .set("ry", 10)
            .set("fill", stroke)
            .set("stroke-width", 0);
        
        if !dasharray.is_empty() {
            frame = frame.set("stroke-dasharray", dasharray);
        }
        group = group.add(frame);
        
        // Content area
        let content = Rectangle::new()
            .set("x", x + stroke_width as i32)
            .set("y", y + 40)
            .set("width", width - stroke_width * 2)
            .set("height", height - 40 - stroke_width)
            .set("rx", 10)
            .set("ry", 10)
            .set("fill", fill)
            .set("stroke-width", 0);
        group = group.add(content);
        
        // Control buttons
        for i in 0..3 {
            let button = Circle::new()
                .set("cx", x + 20 + i * 30)
                .set("cy", y + 20)
                .set("r", 10)
                .set("fill", fill)
                .set("stroke-width", 0);
            group = group.add(button);
        }
        
        Box::new(group)
    }

    fn render_terminal(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let mut group = svg::node::element::Group::new();
        
        // Window frame
        let mut frame = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", width)
            .set("height", height)
            .set("rx", 10)
            .set("ry", 10)
            .set("fill", stroke)
            .set("stroke-width", 0);
        
        if !dasharray.is_empty() {
            frame = frame.set("stroke-dasharray", dasharray);
        }
        group = group.add(frame);
        
        // Content area
        let content = Rectangle::new()
            .set("x", x + stroke_width as i32)
            .set("y", y + 40)
            .set("width", width - stroke_width * 2)
            .set("height", height - 40 - stroke_width)
            .set("rx", 10)
            .set("ry", 10)
            .set("fill", fill)
            .set("stroke-width", 0);
        group = group.add(content);
        
        // Prompt text
        let text = SvgText::new(">_")
            .set("x", x + 50)
            .set("y", y + 90)
            .set("text-anchor", "middle")
            .set("fill", stroke)
            .set("font-size", "50px")
            .set("font-family", "Courier New, Arial")
            .set("font-weight", "bold");
        group = group.add(text);
        
        // Control buttons
        for i in 0..3 {
            let button = Circle::new()
                .set("cx", x + 20 + i * 30)
                .set("cy", y + 20)
                .set("r", 10)
                .set("fill", fill)
                .set("stroke-width", 0);
            group = group.add(button);
        }
        
        Box::new(group)
    }

    fn render_shell(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let mut group = svg::node::element::Group::new();
        
        // Shell background
        let mut shell = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", width)
            .set("height", height)
            .set("rx", 10)
            .set("ry", 10)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            shell = shell.set("stroke-dasharray", dasharray);
        }
        group = group.add(shell);
        
        // Prompt text
        let text = SvgText::new(">_")
            .set("x", x + 50)
            .set("y", y + 50)
            .set("text-anchor", "middle")
            .set("fill", stroke)
            .set("font-size", "50px")
            .set("font-family", "Courier New, Arial")
            .set("font-weight", "bold");
        group = group.add(text);
        
        Box::new(group)
    }

    fn render_mobile_portrait(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: &str,
        stroke: &str,
        _stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let mut group = svg::node::element::Group::new();
        
        // Device frame
        let mut frame = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", width)
            .set("height", height)
            .set("rx", 20)
            .set("ry", 20)
            .set("fill", stroke)
            .set("stroke", stroke)
            .set("stroke-width", 5);
        
        if !dasharray.is_empty() {
            frame = frame.set("stroke-dasharray", dasharray);
        }
        group = group.add(frame);
        
        // Screen
        let screen = Rectangle::new()
            .set("x", x + 10)
            .set("y", y + 40)
            .set("width", width - 20)
            .set("height", height - 80)
            .set("rx", 5)
            .set("ry", 5)
            .set("fill", fill)
            .set("stroke-width", 0);
        group = group.add(screen);
        
        // Home button
        let button = Circle::new()
            .set("cx", x + (width / 2) as i32)
            .set("cy", y + height as i32 - 20)
            .set("r", 10)
            .set("fill", fill)
            .set("stroke-width", 0);
        group = group.add(button);
        
        // Speaker
        let speaker = Line::new()
            .set("x1", x + ((width - 50) / 2) as i32)
            .set("y1", y + 20)
            .set("x2", x + (width - (width - 50) / 2) as i32)
            .set("y2", y + 20)
            .set("stroke", fill)
            .set("stroke-width", 5);
        group = group.add(speaker);
        
        Box::new(group)
    }

    fn render_mobile_landscape(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: &str,
        stroke: &str,
        _stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let mut group = svg::node::element::Group::new();
        
        // Device frame
        let mut frame = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", width)
            .set("height", height)
            .set("rx", 20)
            .set("ry", 20)
            .set("fill", stroke)
            .set("stroke", stroke)
            .set("stroke-width", 5);
        
        if !dasharray.is_empty() {
            frame = frame.set("stroke-dasharray", dasharray);
        }
        group = group.add(frame);
        
        // Screen
        let screen = Rectangle::new()
            .set("x", x + 40)
            .set("y", y + 10)
            .set("width", width - 80)
            .set("height", height - 20)
            .set("rx", 5)
            .set("ry", 5)
            .set("fill", fill)
            .set("stroke-width", 0);
        group = group.add(screen);
        
        // Home button
        let button = Circle::new()
            .set("cx", x + 20)
            .set("cy", y + (height / 2) as i32)
            .set("r", 10)
            .set("fill", fill)
            .set("stroke-width", 0);
        group = group.add(button);
        
        // Speaker
        let speaker = Line::new()
            .set("x1", x + width as i32 - 20)
            .set("y1", y + ((height - 50) / 2) as i32)
            .set("x2", x + width as i32 - 20)
            .set("y2", y + (height - (height - 50) / 2) as i32)
            .set("stroke", fill)
            .set("stroke-width", 5);
        group = group.add(speaker);
        
        Box::new(group)
    }

    fn render_component(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: &str,
        stroke: &str,
        stroke_width: u32,
        dasharray: &str,
    ) -> Box<dyn Node> {
        let mut group = svg::node::element::Group::new();
        
        let block_width = width / 6;
        let block_height = height / 8;
        
        // Main rectangle
        let mut main = Rectangle::new()
            .set("x", x + (block_width / 2) as i32)
            .set("y", y)
            .set("width", width - block_width / 2)
            .set("height", height)
            .set("rx", 10)
            .set("ry", 10)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            main = main.set("stroke-dasharray", dasharray);
        }
        group = group.add(main);
        
        // Left blocks
        let mut block1 = Rectangle::new()
            .set("x", x)
            .set("y", y + (block_height as f32 * 0.6) as i32)
            .set("width", block_width)
            .set("height", block_height)
            .set("rx", 5)
            .set("ry", 5)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            block1 = block1.set("stroke-dasharray", dasharray);
        }
        group = group.add(block1);
        
        let mut block2 = Rectangle::new()
            .set("x", x)
            .set("y", y + (block_height * 2) as i32)
            .set("width", block_width)
            .set("height", block_height)
            .set("rx", 5)
            .set("ry", 5)
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width);
        
        if !dasharray.is_empty() {
            block2 = block2.set("stroke-dasharray", dasharray);
        }
        group = group.add(block2);
        
        Box::new(group)
    }
}
