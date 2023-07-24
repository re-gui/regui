use std::cell::RefCell;
use std::ops::Deref;
use std::{rc::Rc, any::Any};

use repaint::BasicPainter;
use repaint::methods::PaintStyle;
use repaint::{nalgebra::Vector2, Color, base::pen::Pen};
use taffy::style::{Dimension, LengthPercentageAuto, LengthPercentage};
use taffy::tree::NodeId;

use crate::css::{CssColor, CssNamedColors};
use crate::{Widget, SPainter, TaffyContext};

use crate::TaffyStyle;
use crate::TaffyDisplay;

use crate::TaffySize;
use crate::TaffyRect;

pub struct Frame {
    children: Vec<Rc<RefCell<dyn Widget>>>,
    leaf: NodeId,
    style: FrameStyle,
    taffy: TaffyContext,

    frame_count: RefCell<usize>,
}

pub struct FrameStyle {
    pub taffy_style: TaffyStyle,
}

impl Default for FrameStyle {
    fn default() -> Self {
        Self {
            taffy_style: TaffyStyle {
                display: TaffyDisplay::Block,
                size: TaffySize {
                    //width: Dimension::Auto,
                    //height: Dimension::Auto,
                    width: Dimension::Percent(0.5),
                    height: Dimension::Percent(0.5),
                },
                margin: TaffyRect {
                    left: LengthPercentageAuto::Length(0.0),
                    right: LengthPercentageAuto::Length(0.0),
                    top: LengthPercentageAuto::Length(0.0),
                    bottom: LengthPercentageAuto::Length(0.0),
                },
                padding: TaffyRect {
                    left: LengthPercentage::Length(5.0),
                    right: LengthPercentage::Length(5.0),
                    top: LengthPercentage::Length(5.0),
                    bottom: LengthPercentage::Length(5.0),
                },
                flex_grow: 1.0,
                ..Default::default()
            }
        }
    }
}

impl Frame {
    pub fn new(ctx: TaffyContext) -> Self {
        let style = FrameStyle::default();
        Self {
            children: Vec::new(),
            leaf: ctx.on_taffy_mut(|taffy| taffy.new_leaf(style.taffy_style.clone())).unwrap(),
            style,
            taffy: ctx,
            frame_count: RefCell::new(0),
        }
    }
    pub fn modify_style(&mut self, f: impl FnOnce(&mut FrameStyle)) {
        f(&mut self.style);
        self.taffy.on_taffy_mut(|taffy| taffy.set_style(self.leaf, self.style.taffy_style.clone())).unwrap();
    }
    pub fn set_style(&mut self, style: FrameStyle) {
        self.style = style;
        self.taffy.on_taffy_mut(|taffy| taffy.set_style(self.leaf, self.style.taffy_style.clone())).unwrap();
    }
    pub fn add_child(&mut self, child: impl Widget) {
        let child = Rc::new(RefCell::new(child));
        self.taffy.on_taffy_mut(|taffy| taffy.add_child(self.leaf, child.borrow().box_layout_leaf())).unwrap();
        self.children.push(child);
    }
}

impl Widget for Frame {
    fn box_layout_leaf(&self) -> NodeId {
        self.leaf
    }
    fn children(&self) -> &[Rc<RefCell<dyn Widget>>] {
        &self.children
    }
    fn paint<'canvas>(
        &mut self,
        painter: &mut SPainter<'canvas>,
        size: Vector2<f64>,
        _resources: Option<Box<dyn Any>>
    ) -> Option<Box<dyn Any>> {
        let pen: Pen<Color> = Color::RED.into();
        let pen: Pen<Color> = Color::from(CssColor::Named(CssNamedColors::blueviolet)).into();
        //pen.paint.anti_alias = false;
        let style = PaintStyle::Stroke(pen);

        painter.with_save(|painter| {
            //painter.rotate((*self.frame_count.borrow().deref() as f64).to_radians() / 50.0).unwrap();
            painter.rect((0.0, 0.0, size.x, size.y).into(), style);
        });

        self.frame_count.replace_with(|x| *x + 1);

        self.taffy.request_repaint();

        None
    }
}