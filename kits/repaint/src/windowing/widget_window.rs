use std::{cell::RefCell, rc::Rc};

use repaint::{BasicPainter, nalgebra::{Vector2, Matrix2, Matrix4}, base::{transform::Transform2d, shapes::Shape, defs::rect::F64Rect}, Color, Canvas, ClipOperation, SaveLayerRec};
use taffy::{tree::Layout, prelude::Size, style::AvailableSpace};
use winit::event::WindowEvent;

use crate::{Widget, TaffyContext, SPainter};

use super::{SkiaWindow, ReLoop, ReWindow, BasicSkiaWindow};



#[derive(Clone)]
struct LayoutTree {
    widget: Rc<RefCell<dyn Widget>>,
    layout: Layout,
    layout_children: RefCell<Vec<LayoutTree>>,
}

fn compute_paint_tree_inner(
    taffy: &TaffyContext,
    wg: Rc<RefCell<dyn Widget>>,
    last_positioned: &Option<&mut LayoutTree>,
) -> Option<LayoutTree> {

    // TODO maybe incorrect
    //let poisitioned = style.position != Position::Relative;
    let _poisitioned = true; // everything other than static https://developer.mozilla.org/en-US/docs/Web/CSS/position#types_of_positioning but taffy doesn't support static

    let layout = taffy.on_taffy(|taffy| taffy.layout(wg.borrow().box_layout_leaf()).unwrap().clone());

    let tree = LayoutTree {
        widget: wg.clone(),
        layout,
        layout_children: RefCell::new(Vec::new()),
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
        tree.layout_children.borrow_mut().push(child);
    }

    if let Some(last_positioned) = last_positioned {
        last_positioned.layout_children.borrow_mut().push(tree);
        None
    } else {
        Some(tree)
    }
}

fn compute_paint_tree(taffy: &TaffyContext, root: Rc<RefCell<dyn Widget>>, available_space: Size<AvailableSpace>) -> LayoutTree {
    taffy.compute_layout(root.borrow().box_layout_leaf(), available_space);

    let mut tree = compute_paint_tree_inner(
        taffy,
        root,
        &None,
    ).unwrap();

    /// sort children by order
    fn sort_tree(node: &mut LayoutTree) {
        node.layout_children.borrow_mut().sort_by(|a, b| a.layout.order.cmp(&b.layout.order));
        for child in node.layout_children.borrow_mut().iter_mut() {
            sort_tree(child);
        }
    }

    sort_tree(&mut tree);

    tree
}

struct ContentManager {
    root: Rc<RefCell<dyn Widget>>,
    taffy: TaffyContext,
    paint_tree: RefCell<Option<(LayoutTree, Size<AvailableSpace>)>>,
}

impl ContentManager {
    fn on_paint_tree<R>(&self, size: Size<AvailableSpace>, f: impl FnOnce(&LayoutTree) -> R) -> R {
        let to_recompute = self.paint_tree.borrow().is_none() || self.taffy.update_requested();
        // check if size changed
        let to_recompute = if let Some((_, old_size)) = &*self.paint_tree.borrow() {
            to_recompute || (*old_size != size)
        } else {
            to_recompute
        };

        if to_recompute {
            self.paint_tree.replace(Some((
                compute_paint_tree(
                    &self.taffy,
                    self.root.clone(),
                    size,
                ),
                size,
            )));
        }

        let tree = self.paint_tree.borrow().clone().unwrap().0;

        f(&tree)
    }
}

pub struct WidgetWindow {
    skia_window: BasicSkiaWindow,
    content_manager: ContentManager,
}

impl WidgetWindow {
    pub fn new_no_register<Wg: Widget>(
        re_loop: &mut ReLoop,
        root: impl FnOnce(TaffyContext) -> Wg,
    ) -> Self {
        let skia_window = BasicSkiaWindow::new_no_register(re_loop);
        let taffy = TaffyContext::new();

        let root: Rc<RefCell<dyn Widget>> = Rc::new(RefCell::new(root(taffy.clone())));

        Self {
            skia_window,
            //root,
            //taffy,
            //paint_tree: RefCell::new(None),
            content_manager: ContentManager {
                root,
                taffy,
                paint_tree: RefCell::new(None),
            },
        }
    }

    pub fn new<Wg: Widget>(
        re_loop: &mut ReLoop,
        root: impl FnOnce(TaffyContext) -> Wg,
    ) -> Rc<RefCell<Self>> {
        let s = Self::new_no_register(re_loop, root);
        re_loop.register_window(s)
    }

    pub fn request_repaint(&mut self) {
        self.skia_window.request_repaint();
    }

    pub fn poll_commands_queue(&mut self) {
        while let Some(command) = self.content_manager.taffy.poll_on_window() {
            command(self);
        }
    }
}

impl ReWindow for WidgetWindow {
    fn instance(&self) -> &winit::window::Window {
        self.skia_window.instance()
    }

    fn handle_event(&mut self, event: &WindowEvent, control_flow: &mut winit::event_loop::ControlFlow) {
        self.skia_window.handle_event(event, control_flow);

        self.poll_commands_queue();
        //control_flow.set_poll();

        struct Entry {
            widget: Rc<RefCell<dyn Widget>>,
            transform: Matrix4<f64>,
            size: Vector2<f64>,
        }

        fn list_entries(entries: &mut Vec<Entry>, tree: &LayoutTree, parent_transform: Matrix4<f64>) {
            let layout = &tree.layout;
            let pos = Vector2::new(layout.location.x as f64, layout.location.y as f64);
            let pos = Transform2d::Translate(pos);
            let pos = pos.to_mat4x4().unwrap();
            let size = Vector2::new(layout.size.width as f64, layout.size.height as f64);
            entries.push(Entry {
                widget: tree.widget.clone(),
                transform: parent_transform * pos,
                size,
            });
        }

        let entries = {
            let mut entries = Vec::new();
            let s = self.skia_window.size();
            self.content_manager.on_paint_tree(
                Size {
                    height: AvailableSpace::Definite(s.1 as f32),
                    width: AvailableSpace::Definite(s.0 as f32)
                },
                |tree| {
                    list_entries(&mut entries, &tree, Matrix4::identity());
                }
            );
            entries
        };

        match event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            _ => {},
        }
    }

    fn main_events_cleared(&mut self, control_flow: &mut winit::event_loop::ControlFlow) {
        self.skia_window.main_events_cleared(control_flow);
    }

    fn draw(&mut self, control_flow: &mut winit::event_loop::ControlFlow) {
        self.skia_window.draw(control_flow);

        self.skia_window.paint_with_skia_painter(|painter| {
            let size = painter.canvas().shape();
            let size = Vector2::new(size.width() as f64, size.height() as f64);

            painter.clear(Color::WHITE.into());

            let w = size.x as f32;
            let h = size.y as f32;

            fn draw_tree_item(item: &LayoutTree, painter: &mut SPainter) {
                let layout = &item.layout;
                //println!("drawing {:?}", layout);

                let pos = Vector2::new(layout.location.x as f64, layout.location.y as f64);
                let pos = Transform2d::Translate(pos);
                let size = Vector2::new(layout.size.width as f64, layout.size.height as f64);
                painter.with_save(|painter| {
                    painter.concatenate_transform(&pos).unwrap();
                    // TODO painter.clip(&F64Rect::from((0.0, 0.0, size.x, size.y)), ClipOperation::Intersect).unwrap();
                    // TODO maybe overflow?
                    item.widget.borrow_mut().paint(painter, size, None); // TODO resources
                    for child in item.layout_children.borrow().iter() {
                        draw_tree_item(child, painter);
                    }
                });
                //}, Default::default());
            }

            self.content_manager.on_paint_tree(
                Size {
                    height: AvailableSpace::Definite(h),
                    width: AvailableSpace::Definite(w)
                },
                |tree| {
                    painter.with_save(|painter| {
                        draw_tree_item(&tree, painter);
                    });
                },
            );
        });
        //self.skia_window.request_repaint();

        self.poll_commands_queue();
        //control_flow.set_poll();
    }
}