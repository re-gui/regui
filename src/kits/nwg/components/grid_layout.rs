use std::{rc::Rc, cell::RefCell, hash::Hash, collections::BTreeMap};

use native_windows_gui as nwg;

use crate::{component::{ComponentProps, Component}, kits::nwg::{NwgCtx, NwgWidget, NwgChildComponent}, functional_component::UiBuilder};

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Eq, Ord)]
pub struct ChildPlacement {
    pub col: u32,
    pub row: u32,
    pub col_span: u32,
    pub row_span: u32,
}

impl ChildPlacement {
    pub fn new(col: u32, row: u32, col_span: u32, row_span: u32) -> Self {
        Self {
            col,
            row,
            col_span,
            row_span,
        }
    }

    pub fn unit(col: u32, row: u32) -> Self {
        Self {
            col,
            row,
            col_span: 1,
            row_span: 1,
        }
    }

    pub fn is_unit(&self) -> bool {
        self.col_span == 1 && self.row_span == 1
    }
}

impl From<(u32, u32)> for ChildPlacement {
    fn from((col, row): (u32, u32)) -> Self {
        Self::unit(col, row)
    }
}

impl From<(u32, u32, u32, u32)> for ChildPlacement {
    fn from((col, row, col_span, row_span): (u32, u32, u32, u32)) -> Self {
        Self::new(col, row, col_span, row_span)
    }
}

#[derive(Clone)]
pub struct GridLayoutListBuilder<'builder>(&'builder UiBuilder<NwgCtx>, Vec<(ChildPlacement, Rc<RefCell<dyn NwgWidget>>)>);

impl<'builder> GridLayoutListBuilder<'builder> {
    pub fn new(builder: &'builder UiBuilder<NwgCtx>) -> Self {
        Self(builder, Vec::new())
    }
    pub fn build(self) -> Vec<(ChildPlacement, Rc<RefCell<dyn NwgWidget>>)> {
        self.1
    }
    pub fn with<WidgetProps, Widget: 'static>(mut self, placement: ChildPlacement, props: WidgetProps) -> Self // TODO remove static
    where
        WidgetProps: ComponentProps<NwgCtx, AssociatedComponent = Widget>,
        Widget: NwgWidget,
    {
        self.add(placement, props);
        self
    }
    pub fn add<WidgetProps, Widget: 'static>(&mut self, placement: ChildPlacement, props: WidgetProps) // TODO remove static
    where
        WidgetProps: ComponentProps<NwgCtx, AssociatedComponent = Widget>,
        Widget: NwgWidget,
    {
        self.1.push((
            placement,
            self.0.get(props)
        ));
    }
}

// TODO other conversions

#[derive(Clone)]
pub struct GridLayout {
    pub children: Vec<(ChildPlacement, Rc<RefCell<dyn NwgWidget>>)>,
    pub spacing: u32,
    pub margin: [u32; 4],
}

impl Default for GridLayout {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            spacing: 5,
            margin: [5, 5, 5, 5],
        }
    }
}

impl GridLayout {
    pub fn list_builder<'builder>(builder: &'builder UiBuilder<NwgCtx>) -> GridLayoutListBuilder<'builder> {
        GridLayoutListBuilder::new(builder)
    }
}

impl ComponentProps<NwgCtx> for GridLayout {
    type AssociatedComponent = GridLayoutComponent;
}

pub struct GridLayoutComponent {
    native_layout: Option<(nwg::ControlHandle, nwg::GridLayout)>,
    props: GridLayout,
}

impl GridLayoutComponent {

    fn build_layout(&mut self, parent_handle: nwg::ControlHandle, ctx: &NwgCtx) {
        let mut layout = Default::default();
        let builder = nwg::GridLayout::builder()
            .parent(parent_handle)
            //.spacing(1)
            //.margin(1)
            ;

        let builder = {
            let mut builder = builder;
            builder = builder.spacing(self.props.spacing);
            builder = builder.margin(self.props.margin);
            for (placement, child) in self.props.children.iter() {
                let mut child = child.borrow_mut();
                let handle = child.set_parent_and_get_handle(parent_handle, ctx);
                if placement.is_unit() {
                    builder = builder.child(placement.col, placement.row, handle);
                } else {
                    builder = builder.child_item(nwg::GridLayoutItem::new(handle, placement.col, placement.row, placement.col_span, placement.row_span));
                }
            }
            builder
        };

        builder.build(&mut layout).unwrap(); // TODO

        self.native_layout = Some((parent_handle, layout));
    }
}

impl Component<NwgCtx> for GridLayoutComponent {
    type Props = GridLayout;
    fn build(_ctx: &NwgCtx, props: Self::Props) -> Self {
        Self {
            native_layout: None,
            props,
        }
    }
    fn changed(&mut self, props: Self::Props, ctx: &NwgCtx) {
        if let Some((parent_window, native_layout)) = &mut self.native_layout {
            if props.spacing != self.props.spacing {
                native_layout.spacing(props.spacing);
            }
            if props.margin != self.props.margin {
                native_layout.margin(props.margin);
            }

            let children_changed =
                self.props.children.len() != props.children.len() ||
                self.props.children.iter().zip(props.children.iter())
                    .any(|(old, new)|
                        new.0 != old.0 || !Rc::ptr_eq(&new.1, &old.1)
                    );

            if children_changed {
                #[derive(Clone)]
                struct Data {
                    /// None if to add
                    old_placement: Option<ChildPlacement>,
                    /// None if to remove
                    new_placement: Option<ChildPlacement>,
                }

                let mut children_placement_map: BTreeMap<WidgetWrapper, Data> = self.props.children.iter()
                    .map(|(placement, widget)| (
                        WidgetWrapper(widget.clone()),
                        Data {
                            old_placement: Some(placement.clone()),
                            new_placement: None
                        },
                    ))
                    .collect();

                for (placement, widget) in props.children.iter() {
                    if let Some(data) = children_placement_map.get_mut(&WidgetWrapper(widget.clone())) {
                        data.new_placement = Some(placement.clone());
                    } else {
                        children_placement_map.insert(
                            WidgetWrapper(widget.clone()),
                            Data {
                                old_placement: None,
                                new_placement: Some(placement.clone()),
                            }
                        );
                    }
                }

                assert!(children_placement_map.iter().all(|(_, data)| data.old_placement.is_some() || data.new_placement.is_some()));

                // remove old widgets
                for (widget, data) in &children_placement_map {
                    if data.new_placement.is_none() {
                        // There are two options:
                        // - native_layout.remove_child(c)
                        // - native_layout.remove_child_by_pos(col, row)
                        // we use the first one because the widget in a certain position may have changed
                        let w = widget.0.borrow();
                        let handle = w.current_handle().unwrap();
                        native_layout.remove_child(handle);
                    }
                }

                // add or move widgets
                for (widget, data) in &children_placement_map {
                    if let Some(new_placement) = &data.new_placement {
                        if let Some(old_placement) = &data.old_placement {
                            // move the widget
                            let w = widget.0.borrow();
                            let handle = w.current_handle().unwrap();
                            if new_placement.col_span == old_placement.col_span && new_placement.row_span == old_placement.row_span {
                                native_layout.move_child(handle, new_placement.col, new_placement.row);
                            } else {
                                native_layout.remove_child(handle);
                                native_layout.add_child_item(nwg::GridLayoutItem::new(handle, new_placement.col, new_placement.row, new_placement.col_span, new_placement.row_span));
                            }
                        } else {
                            // add the widget
                            let mut w = widget.0.borrow_mut();
                            let handle = w.set_parent_and_get_handle(*parent_window, ctx);
                            if new_placement.is_unit() {
                                native_layout.add_child(new_placement.col, new_placement.row, handle);
                            } else {
                                native_layout.add_child_item(nwg::GridLayoutItem::new(handle, new_placement.col, new_placement.row, new_placement.col_span, new_placement.row_span));
                            }
                        }
                    }
                }

                // update the max col and row
                //let mut max_col = 0;
                //let mut max_row = 0;
                //for (_, data) in &children_placement_map {
                //    if let Some(new_placement) = &data.new_placement {
                //        max_col = max_col.max(new_placement.col + new_placement.col_span);
                //        max_row = max_row.max(new_placement.row + new_placement.row_span);
                //    }
                //}
                //native_layout.max_row(Some(max_row));
                //native_layout.max_column(Some(max_col));
            }
        }

        self.props = props;
    }
}

impl NwgChildComponent for GridLayoutComponent {
    fn set_parent_handle(&mut self, parent_window: nwg::ControlHandle, ctx: &NwgCtx) {
        if let Some((parent, _)) = &self.native_layout {
            if parent == &parent_window {
                return;
            }
        }

        self.build_layout(parent_window, ctx);
    }
}


#[derive(Clone)]
struct WidgetWrapper(Rc<RefCell<dyn NwgWidget>>);
impl PartialEq for WidgetWrapper {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl PartialOrd for WidgetWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.as_ptr().partial_cmp(&other.0.as_ptr())
    }
}
impl Eq for WidgetWrapper {}
impl Ord for WidgetWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.as_ptr().cmp(&other.0.as_ptr())
    }
}