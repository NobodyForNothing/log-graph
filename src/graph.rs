use crate::data::LogData;
use std::error::Error;
use tiny_skia::{Color, Paint, Pixmap};
use tiny_skia_path::{PathBuilder, Stroke, Transform};

/// Draw a graph from [data] and store at [png_path].
pub fn draw(data: LogData, png_path: String, line_width: f32) -> Result<(), Box<dyn Error>> {
    let data = data.data;
    let colors: [Color; 7] = [
        Color::from_rgba8(255, 0, 0, 255),
        Color::from_rgba8(0, 255, 0, 255),
        Color::from_rgba8(0, 0, 255, 255),
        Color::from_rgba8(0, 255, 255, 255),
        Color::from_rgba8(255, 255, 0, 255),
        Color::from_rgba8(255, 0, 255, 255),
        Color::from_rgba8(0, 0, 0, 255),
    ];

    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let mut pixmap = Pixmap::new(2560, 1440).unwrap();
    pixmap.fill(Color::WHITE);

    let transform = Transform::identity();

    for (i, (series, line)) in data.iter().enumerate() {
        let mut pb = PathBuilder::new();
        pb.move_to(0., 0.);
        let min_val = line.iter().min_by_key(|e| e.value).unwrap().value;
        let max_val = line.iter().max_by_key(|e| e.value).unwrap().value;
        let min_time = line.iter().min_by_key(|e| e.time).unwrap().time;
        let max_time = line.iter().max_by_key(|e| e.time).unwrap().time;
        for p in line {
            pb.line_to(
                (((p.time - min_time) as f32) / (max_time - min_time) as f32)
                    * pixmap.width() as f32,
                (&((p.value - min_val) as f32) / (max_val - min_val) as f32)
                    * pixmap.height() as f32,
            );
        }
        let path = pb.finish().unwrap();
        let mut stroke = Stroke::default();
        stroke.width = line_width;

        let color = colors[i % colors.len()];
        paint.set_color(color);
        println!(
            "{series}: r{} g{}, b{}",
            &color.red(),
            &color.green(),
            &color.blue()
        );
        pixmap.stroke_path(&path, &paint, &stroke, transform, None);
    }

    pixmap.save_png(png_path).unwrap();

    Ok(())
}

/*use std::num::NonZeroU32;

use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use glutin_winit::DisplayBuilder;

use glutin::{
    config::ConfigTemplateBuilder,
    context::ContextAttributesBuilder,
    context::PossiblyCurrentContext,
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use femtovg::renderer::OpenGl;
use glutin::display::Display;
use crate::parser::Entry;

pub fn draw(data: HashMap<u64, Vec<Entry>>) -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let (_context, gl_display) = create_window(&event_loop);
    let renderer = unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _) }
        .expect("Cannot create renderer");

    Ok(())
}

// https://femtovg.github.io/1_getting_started/1_setting_up.html
fn create_window(event_loop: &EventLoop<()>) -> (PossiblyCurrentContext, Display) {
    let window_builder = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1000., 600.))
        .with_title("Femtovg");

    let template = ConfigTemplateBuilder::new().with_alpha_size(8);

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) = display_builder
        .build(event_loop, template, |mut configs| configs.next().unwrap())
        .unwrap();

    let window = window.unwrap();

    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new()
        .build(Some(window.raw_window_handle().expect("Couldn't get window handle")));

    let mut not_current_gl_context =
        Some(unsafe { gl_display.create_context(&gl_config, &context_attributes).unwrap() });

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        window.window_handle().unwrap(),
        NonZeroU32::new(1000).unwrap(),
        NonZeroU32::new(600).unwrap(),
    );

    let surface = unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

    (not_current_gl_context.take().unwrap().make_current(&surface).unwrap(), gl_display)
}
*/
