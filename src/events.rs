// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_void;
use std::mem;
use cairo;

pub use self::event_type::EventType;
pub use self::owner_change::OwnerChange;
pub use self::setting_action::SettingAction;
pub use self::property_state::PropertyState;
pub use self::crossing_mode::CrossingMode;
pub use self::notify_type::NotifyType;
pub use self::scroll_direction::ScrollDirection;
pub use self::visibility_state::VisibilityState;

pub mod event_type {
    #[repr(C)]
    #[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
    pub enum EventType {
        Nothing           = -1,
        Delete            = 0,
        Destroy           = 1,
        Expose            = 2,
        MotionNotify      = 3,
        ButtonPress       = 4,
        DoubleButtonPress = 5,
        TripleButtonPress = 6,
        ButtonRelease     = 7,
        KeyPress          = 8,
        KeyRelease        = 9,
        EnterNotify       = 10,
        LeaveNotify       = 11,
        FocusChange       = 12,
        Configure         = 13,
        Map               = 14,
        Unmap             = 15,
        PropertyNotify    = 16,
        SelectionClear    = 17,
        SelectionRequest  = 18,
        SelectionNotify   = 19,
        ProximityIn       = 20,
        ProximityOut      = 21,
        DragEnter         = 22,
        DragLeave         = 23,
        DragMotion        = 24,
        DragStatus        = 25,
        DropStart         = 26,
        DropFinished      = 27,
        ClientEvent       = 28,
        VisibilityNotify  = 29,
        Scroll            = 31,
        WindowState       = 32,
        Setting           = 33,
        OwnerChange       = 34,
        GrabBroken        = 35,
        Damage            = 36,
        TouchBegin        = 37,
        TouchUpdate       = 38,
        TouchEnd          = 39,
        TouchCancel       = 40
    }
}

pub trait Event: Sized {
    fn get_send_event(&self) -> bool {
        unsafe {
            let event_any : &EventAny = mem::transmute(self);
            event_any.send_event == 0
        }
    }
}

// TODO unfinished, see #25

#[repr(C)]
pub struct EventAny {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,
}

impl Event for EventAny {}

#[repr(C)]
pub struct EventExpose {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub area : cairo::RectangleInt,
    region : *mut c_void, //TODO cairo_region_t
    pub count : i8 /* If non-zero, how many more events follow. */
}

impl Event for EventExpose {}

#[repr(C)]
pub struct EventVisibility{
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub state : ::VisibilityState
}

impl Event for EventVisibility {}

#[repr(C)]
pub struct EventMotion {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub time : u32,
    pub x : f64,
    pub y : f64,
    pub axes : *mut f64,
    pub state : ::enums::modifier_type::ModifierType,
    pub is_hint : i16,
    device : *mut ::Device,
    pub x_root : f64,
    pub y_root : f64
}

impl Event for EventMotion {}

#[repr(C)]
pub struct EventButton {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub time : u32,
    pub x : f64,
    pub y : f64,
    pub axes : *mut f64,
    pub state : ::enums::modifier_type::ModifierType,
    pub button : u32,
    device : *mut ::Device,
    pub x_root : f64,
    pub y_root : f64
}

impl Event for EventButton {}

#[repr(C)]
pub struct EventTouch {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub time : u32,
    pub x : f64,
    pub y : f64,
    pub axes : *mut f64,
    pub state : ::enums::modifier_type::ModifierType,
    sequence : *mut c_void, //::EventSequence
    pub emulating_pointer : i32, // boolean
    device : *mut ::Device,
    pub x_root : f64,
    pub y_root : f64
}

impl Event for EventTouch {}

#[repr(C)]
pub struct EventScroll {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub time : u32,
    pub x : f64,
    pub y : f64,
    pub state : ::enums::modifier_type::ModifierType,
    pub direction : ::ScrollDirection,
    device : *mut ::Device,
    pub x_root : f64,
    pub y_root : f64,
    pub delta_x : f64,
    pub delta_y : f64
}

impl Event for EventScroll {}

#[repr(C)]
pub struct EventKey {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub time : u32,
    pub state : ::enums::modifier_type::ModifierType,
    pub keyval : u32,
    pub length : i32,
    pub string : *mut char,
    pub hardware_keycode : u16,
    pub group : u8,
    pub is_modifier: u32
}

impl Event for EventKey {}

#[repr(C)]
pub struct EventCrossing {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub subwindow : ::Window,
    pub time : u32,
    pub x : f64,
    pub y : f64,
    pub x_root : f64,
    pub y_root : f64,
    pub mode : ::CrossingMode,
    pub detail : ::NotifyType,
    pub focus : i32, // boolean
    pub state : ::enums::modifier_type::ModifierType
}

impl Event for EventCrossing {}

#[repr(C)]
pub struct EventFocus {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub _in : i16
}

impl Event for EventFocus {}

#[repr(C)]
pub struct EventConfigure {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub x : i32,
    pub y : i32,
    pub width : i32,
    pub height : i32
}

impl Event for EventConfigure {}

#[repr(C)]
pub struct EventProperty {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub atom : ::Atom,
    pub time : u32,
    pub state : u32 //FIXME
}

impl Event for EventProperty {}

#[repr(C)]
pub struct EventSelection {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub selection : ::Atom,
    pub target : ::Atom,
    pub property : ::Atom,
    pub time : u32,
    requestor : *mut ::Window
}

impl Event for EventSelection {}

#[repr(C)]
pub struct EventOwnerChange {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    owner : *mut ::Window,
    pub reason : ::OwnerChange,
    pub selection : ::Atom,
    pub time : u32,
    pub selection_time : u32
}

impl Event for EventOwnerChange {}

#[repr(C)]
pub struct EventProximity {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub time : u32,
    device : *mut ::Device
}

impl Event for EventProximity {}

#[repr(C)]
pub struct EventSetting {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub action : ::SettingAction,
    pub name : *mut char
}

impl Event for EventSetting {}

#[repr(C)]
pub struct EventWindowState {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub changed_mask : ::WindowState,
    pub new_window_state : ::WindowState
}

impl Event for EventWindowState {}

#[repr(C)]
pub struct EventGrabBroken {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    pub keyboard : i32, // boolean
    pub implicit : i32, // boolean
    pub grab_window : *mut ::Window
}

impl Event for EventGrabBroken  {}

#[repr(C)]
pub struct EventDND {
    pub _type : ::EventType,
    pub window : *mut ::Window,
    send_event : i8,

    context : *mut c_void, //::DragContext
    pub time : u32,
    pub x_root : i16, //short
    pub y_root : i16  //short
}

impl Event for EventDND  {}


//Supporting types

pub mod visibility_state {
    #[repr(C)]
    #[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
    pub enum VisibilityState{
        VisibilityUnobscured,
        VisibilityPartial,
        VisibilityFullyObscured
    }
}

pub mod scroll_direction {
    #[repr(C)]
    #[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
    pub enum ScrollDirection{
        ScrollUp,
        ScrollDown,
        ScrollLeft,
        ScrollRight,
        ScrollSmooth
    }
}

pub mod notify_type {
    #[repr(C)]
    #[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
    pub enum NotifyType{
        NotifyAncestor   = 0,
        NotifyVirtual    = 1,
        NotifyInferior   = 2,
        NotifyNonlinear  = 3,
        NotifyNonlinearVirtual  = 4,
        NotifyUnknown    = 5
    }
}

pub mod crossing_mode {
    #[repr(C)]
    #[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
    pub enum CrossingMode{
        CrossingNormal,
        CrossingGrab,
        CrossingUngrab,
        CrossingGtkGrab,
        CrossingGtkUngrab,
        CrossingStateChanged,
        CrossingTouchBegin,
        CrossingTouchEnd,
        CrossingDeviceSwitch
    }
}

pub mod property_state {
    #[repr(C)]
    #[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
    pub enum PropertyState{
        PropertyNewValue,
        PropertyDelete
    }
}

pub mod setting_action {
    #[repr(C)]
    #[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
    pub enum SettingAction{
        SettingActionNew,
        SettingActionChanged,
        SettingActionDeleted
    }
}

pub mod owner_change {
    #[repr(C)]
    #[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
    pub enum OwnerChange{
        OwnerChangeNewOwner,
        OwnerChangeDestroy,
        OwnerChangeClose
    }
}
