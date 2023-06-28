
use std::{fs::File, io::Write, cell::RefCell};

use repaint::{Color, Painter, Canvas, BasicPainter, base::{pen::Pen, defs::linalg::Vec2f64, shapes::path::{PathBuilder, PathCommand}}, WithPathResource};
use repaint_with_skia_safe::{skia_safe::{self, Surface}, SkiaCanvas, make_skia_context, SkiaPainter};

fn main() {
    let mut surface = Surface::new_raster_n32_premul((200, 200)).expect("no surface!");
    surface.canvas().clear(skia_safe::Color::WHITE);

    {
        let ctx = RefCell::new(make_skia_context());

        let mut canvas = SkiaCanvas::new(&mut surface, &ctx);
        let mut painter = canvas.painter().unwrap();

        let start = std::time::Instant::now();
        draw(&mut painter);
        let end = std::time::Instant::now();
        println!("draw took {:?}", end - start);
    }

    let image = surface.image_snapshot();
    let data = image.encode_to_data(skia_safe::EncodedImageFormat::PNG).unwrap();
    let mut file = File::create("test.png").unwrap();
    let bytes = data.as_bytes();
    file.write_all(bytes).unwrap();
}

type SPainter<'canvas> = SkiaPainter<'canvas, 'canvas, 'canvas>;

fn draw<'canvas>(painter: &mut SPainter<'canvas>) {
    painter.clear(Color::WHITE);

    let ci = Circle {
        center: (20.0, 20.0),
        radius: 10.0,
    };

    for i in 0..1 {
        ci.draw(painter);
    }

    let mut builder = PathBuilder::new();
    builder.push(PathCommand::MoveTo(Vec2f64::new(20.0, 20.0)));
    builder.push(PathCommand::LineTo(Vec2f64::new(40.0, 42.0)));


}


trait Element {
}

struct Circle {
    center: (f32, f32),
    radius: f32,
}

impl Node for Circle {
    fn draw<'canvas>(&self, painter: &mut SPainter<'canvas>) {
        let pen: Pen<_> = Color::RED.into();
        painter.line(
            Vec2f64::new(self.center.0 as f64 - self.radius as f64, self.center.1 as f64),
            Vec2f64::new(self.center.0  as f64 + self.radius  as f64, self.center.1 as f64),
            pen.clone()
        );
        painter.line(
            Vec2f64::new(self.center.0 as f64, self.center.1 as f64 - self.radius as f64),
            Vec2f64::new(self.center.0 as f64, self.center.1 as f64 + self.radius as f64),
            pen
        );
    }
}

