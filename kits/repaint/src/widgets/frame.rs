use std::cell::RefCell;
use std::{rc::Rc, any::Any};

use repaint::BasicPainter;
use repaint::methods::PaintStyle;
use repaint::{nalgebra::Vector2, Color, base::pen::Pen};
use taffy::style::{Dimension, LengthPercentageAuto};
use taffy::tree::NodeId;

use crate::{Widget, SPainter, TaffyContext};

use crate::TaffyStyle;

use crate::TaffySize;
use crate::TaffyRect;

pub struct Frame {
    children: Vec<Rc<RefCell<dyn Widget>>>,
    leaf: NodeId,
    style: FrameStyle,
    taffy: TaffyContext,
}

pub struct FrameStyle {
    taffy_style: TaffyStyle,
}

impl Default for FrameStyle {
    fn default() -> Self {
        Self {
            taffy_style: TaffyStyle {
                //display: TaffyDisplay::Block,
                size: TaffySize {
                    //width: Dimension::Auto,
                    //height: Dimension::Auto,
                    width: Dimension::Percent(1.0),
                    height: Dimension::Percent(1.0),
                },
                margin: TaffyRect {
                    left: LengthPercentageAuto::Length(0.0),
                    right: LengthPercentageAuto::Length(0.0),
                    top: LengthPercentageAuto::Length(0.0),
                    bottom: LengthPercentageAuto::Length(0.0),
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
            leaf: ctx.new_leaf(style.taffy_style.clone()),
            style,
            taffy: ctx,
        }
    }
    pub fn set_style(&mut self, style: FrameStyle) {
        self.style = style;
        self.taffy.set_style(self.leaf, self.style.taffy_style.clone());
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
        &self,
        painter: &mut SPainter<'canvas>,
        size: Vector2<f64>,
        _resources: Option<Box<dyn Any>>
    ) -> Option<Box<dyn Any>> {
        let pen: Pen<Color> = Color::RED.into();
        //pen.paint.anti_alias = false;
        let style = PaintStyle::Stroke(pen);
        painter.rect((0.0, 0.0, size.x, size.y).into(), style);

        None
    }
}