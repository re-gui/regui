
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ControlEvent {
    Window(WindowEvent),
    Mouse(MouseEvent),
}

impl ControlEvent {
    pub fn from_nwg_event(event: &nwg::Event, event_data: &nwg::EventData, handle: &nwg::ControlHandle, control: &dyn WithNwgControlHandle) -> Option<Self> {
        let mouse_global_pos = nwg::GlobalCursor::position();
        let mouse_local_pos = nwg::GlobalCursor::local_position(handle, Some(mouse_global_pos));
        match event {
            nwg::Event::OnWindowClose => Some(ControlEvent::Window(WindowEvent::CloseRequest)),
            nwg::Event::OnWindowMaximize => Some(ControlEvent::Window(WindowEvent::Maximize)),
            nwg::Event::OnWindowMinimize => Some(ControlEvent::Window(WindowEvent::Minimize)),
            nwg::Event::OnMove => {
                let pos = control.position();
                Some(ControlEvent::Window(WindowEvent::Moved(pos.0, pos.1)))
            }
            nwg::Event::OnResize => {
                let size = control.size();
                Some(ControlEvent::Window(WindowEvent::Resized(size.0 as u32, size.1 as u32)))
            }
            nwg::Event::OnMouseMove => {
                let event = MouseEvent::Move {
                    x: mouse_local_pos.0,
                    y: mouse_local_pos.1,
                    global_x: mouse_global_pos.0,
                    global_y: mouse_global_pos.1,
                };
                Some(ControlEvent::Mouse(event))
            }
            nwg::Event::OnMousePress(button) => {
                let event = match button {
                    nwg::MousePressEvent::MousePressLeftDown => MouseEvent::Pressed {
                        x: mouse_local_pos.0,
                        y: mouse_local_pos.1,
                        global_x: mouse_global_pos.0,
                        global_y: mouse_global_pos.1,
                    },
                    nwg::MousePressEvent::MousePressLeftUp => MouseEvent::Released {
                        x: mouse_local_pos.0,
                        y: mouse_local_pos.1,
                        global_x: mouse_global_pos.0,
                        global_y: mouse_global_pos.1,
                    },
                    nwg::MousePressEvent::MousePressRightDown => MouseEvent::Pressed {
                        x: mouse_local_pos.0,
                        y: mouse_local_pos.1,
                        global_x: mouse_global_pos.0,
                        global_y: mouse_global_pos.1,
                    },
                    nwg::MousePressEvent::MousePressRightUp => MouseEvent::Released {
                        x: mouse_local_pos.0,
                        y: mouse_local_pos.1,
                        global_x: mouse_global_pos.0,
                        global_y: mouse_global_pos.1,
                    },
                    // TODO middle? x1? x2?
                };
                Some(ControlEvent::Mouse(event))
            }
            nwg::Event::OnMouseWheel => {
                let delta = match event_data {
                    nwg::EventData::OnMouseWheel(delta) => *delta,
                    _ => panic!("Unexpected event data type")
                };
                let event = MouseEvent::Wheel {
                    delta,
                    x: mouse_local_pos.0,
                    y: mouse_local_pos.1,
                    global_x: mouse_global_pos.0,
                    global_y: mouse_global_pos.1,
                };
                Some(ControlEvent::Mouse(event))
            }
            _ => None,
        }
    }
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
    X1,
    X2,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    CloseRequest,
    Maximize,
    Minimize,
    Moved(i32, i32),
    Resized(u32, u32),

    Paint,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MouseEvent {
    Pressed { x: i32, y: i32, global_x: i32, global_y: i32 },
    Released { x: i32, y: i32, global_x: i32, global_y: i32 },
    Move { x: i32, y: i32, global_x: i32, global_y: i32 }, // TODO delta
    Wheel { delta: i32, x: i32, y: i32, global_x: i32, global_y: i32 },
}