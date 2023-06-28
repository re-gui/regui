use std::{rc::{Rc, Weak}, any::Any, cell::RefCell, fs::File, io::Write, path::Display, future::Future, task, collections::BinaryHeap, cmp::Reverse, ops::Deref};

use std::pin::Pin;

use regui::function_component::ComponentFunction;
use regui_repaint::{css::CssNamedColors, Widget, SPainter, TaffyContext, windowing::{windowing_loop, SkiaWindow, ReWindow, ReLoop, BasicSkiaWindow}, widgets::{self, Frame}};
use repaint::{nalgebra::Vector2, Canvas, BasicPainter, Color, base::pen::Pen, methods::PaintStyle};
use repaint_with_skia_safe::{SkiaPainter, skia_safe::Surface, SkiaCanvas};
use taffy::{tree::{Layout, NodeId}, Taffy, prelude::{Size, Rect}, style::{Dimension, AvailableSpace, LengthPercentage, LengthPercentageAuto, FlexDirection, Position}};


use repaint_with_skia_safe::skia_safe;

use regui_repaint::*;
use winit::{event::WindowEvent, event_loop::ControlFlow};

struct WidgetTree {
    widget: Rc<RefCell<dyn Widget>>,
    layout: Layout,
    children: RefCell<BinaryHeap<Reverse<WidgetTree>>>,
}

impl PartialEq for WidgetTree {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl PartialOrd for WidgetTree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.layout.order.partial_cmp(&other.layout.order)
    }
}

impl Eq for WidgetTree {
}

impl Ord for WidgetTree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.layout.order.cmp(&other.layout.order)
    }
}

fn compute_paint_tree_inner(
    taffy: &TaffyContext,
    wg: Rc<RefCell<dyn Widget>>,
    last_positioned: &Option<&mut WidgetTree>,
) -> Option<WidgetTree> {

    let style = taffy.get_style(wg.borrow().box_layout_leaf());

    // TODO maybe incorrect
    //let poisitioned = style.position != Position::Relative;
    let poisitioned = true; // everything other than static https://developer.mozilla.org/en-US/docs/Web/CSS/position#types_of_positioning but taffy doesn't support static

    let layout = taffy.get_layout(wg.borrow().box_layout_leaf());

    let mut tree = WidgetTree {
        widget: wg.clone(),
        layout,
        children: RefCell::new(BinaryHeap::new()),
    };

    let children = {
        let wg = wg.borrow();
        let mut children = Vec::new();
        for child in wg.children().iter() {
            let child = compute_paint_tree_inner(
                taffy,
                child.clone(),
                    last_positioned
            );
            if let Some(child) = child {
                children.push(child);
            }
        }
        children
    };

    for child in children {
        tree.children.borrow_mut().push(Reverse(child));
    }

    if let Some(last_positioned) = last_positioned {
        last_positioned.children.borrow_mut().push(Reverse(tree));
        None
    } else {
        Some(tree)
    }
}

fn compute_paint_tree(taffy: &TaffyContext, root: Rc<RefCell<dyn Widget>>, available_space: Size<AvailableSpace>) -> WidgetTree {
    taffy.compute_layout(root.borrow().box_layout_leaf(), available_space);

    compute_paint_tree_inner(
        taffy,
        root,
        &None,
    ).unwrap()
}

struct W {
    inner: Rc<RefCell<BasicSkiaWindow>>,
    root: Rc<RefCell<dyn Widget>>,
    taffy: TaffyContext,
    paint_tree: Option<WidgetTree>,
}

impl W {
    fn new<Wg: Widget>(
        re_loop: &mut ReLoop,
        root: impl FnOnce(TaffyContext) -> Wg,
    ) -> Self {
        let inner = BasicSkiaWindow::new(re_loop);

        let taffy = TaffyContext::new({
            let inner = Rc::downgrade(&inner);
            move || {
                if let Some(inner) = inner.upgrade() {
                    inner.borrow_mut().request_redraw();
                }
            }
        });

        let root: Rc<RefCell<dyn Widget>> = Rc::new(RefCell::new(root(taffy.clone())));

        inner.borrow_mut().on_event({
            move |event| {
                match event {
                    WindowEvent::CloseRequested => Some(ControlFlow::Exit),
                    _ => None,
                }
            }
        });

        inner.borrow_mut().on_paint({
            let root = root.clone();
            let taffy = taffy.clone();
            move |painter, size| {
                painter.clear(Color::WHITE.into());

                let w = size.x as f32;
                let h = size.y as f32;

                taffy.compute_layout(
                    root.borrow().box_layout_leaf(),
                    Size {
                        height: AvailableSpace::Definite(h),
                        width: AvailableSpace::Definite(w)
                    });
    
                let l = taffy.layout(root.borrow().box_layout_leaf());
    
                painter.with_save(|mut painter| {
                    painter.translate(Vector2::<f64>::new(
                        l.location.x as f64, l.location.y as f64)
                    ).unwrap();
                    root.borrow().paint(&mut painter, Vector2::<f64>::new(l.size.width as f64, l.size.height as f64), None);
                });
            }
        });

        Self {
            inner,
            root,
            taffy,
            paint_tree: None,
        }
    }

    fn paint_tree(&mut self, size: Size<AvailableSpace>) -> &WidgetTree {
        let to_recompute = self.paint_tree.is_none() || self.taffy.update_requested();

        if to_recompute {
            self.paint_tree = Some(compute_paint_tree(
                &self.taffy,
                self.root.clone(),
                size,
            ));
        }

        self.paint_tree.as_ref().unwrap()
    }
}

fn main() {
    let mut re_loop = ReLoop::new();

    //let window = BasicSkiaWindow::new(&mut re_loop);
    //let window = BasicSkiaWindow::new(&mut re_loop);
    let w = W::new(
        &mut re_loop,
        |taffy| Frame::new(taffy)
    );

    re_loop.run();

}
