//#![windows_subsystem = "windows"]

use std::{rc::{Rc}, cell::RefCell, collections::BinaryHeap, cmp::Reverse};




use regui_repaint::{Widget, TaffyContext, windowing::{ReLoop, SkiaWindow, WidgetWindow}, widgets::{Frame}, SPainter};
use repaint::{nalgebra::{Vector2, Transform2}, BasicPainter, Color, base::transform::Transform2d};

use taffy::{tree::{Layout}, prelude::{Size}, style::{AvailableSpace, Dimension, Position}};

use winit::{event::WindowEvent, event_loop::ControlFlow};



fn main() {
    let mut re_loop = ReLoop::new();

    //let window = BasicSkiaWindow::new(&mut re_loop);
    //let window = BasicSkiaWindow::new(&mut re_loop);
    let _w = WidgetWindow::new(
        &mut re_loop,
        |taffy| {
            let mut root = Frame::new(taffy.clone());
            root.modify_style(|style| {
                //style.taffy_style.display = taffy::style::Display::Flex;
                //style.taffy_style.flex_direction = taffy::style::FlexDirection::Column;
                style.taffy_style.size = Size {
                    width: Dimension::Percent(0.5),
                    height: Dimension::Percent(0.5),
                };
                style.taffy_style.min_size = Size {
                    width: Dimension::Length(100.0),
                    height: Dimension::Length(100.0),
                };
            });

            let mut child = Frame::new(taffy.clone());
            child.modify_style(|style| {
                style.taffy_style.size = Size {
                    width: Dimension::Length(100.0),
                    height: Dimension::Length(100.0),
                }
            });
            root.add_child(child);
            let mut child = Frame::new(taffy.clone());
            child.modify_style(|style| {
                style.taffy_style.size = Size {
                    width: Dimension::Length(100.0),
                    height: Dimension::Length(100.0)
                };
                style.taffy_style.position = Position::Absolute;
                style.taffy_style.inset = taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Length(50.0),
                    right: taffy::style::LengthPercentageAuto::Auto,
                    top: taffy::style::LengthPercentageAuto::Percent(0.25),
                    bottom: taffy::style::LengthPercentageAuto::Auto,
                }
            });
            root.add_child(child);

            let mut child = Frame::new(taffy.clone());
            let mut child2 = Frame::new(taffy.clone());
            child.add_child(child2);
            root.add_child(child);

            root
        }
    );

    re_loop.run();

}
