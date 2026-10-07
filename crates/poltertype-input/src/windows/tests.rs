//! What the Windows listener reads off the mouse. Runs on CI's
//! `windows-latest` job only.

use windows::Win32::UI::WindowsAndMessaging::{
    WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MOUSEHWHEEL, WM_MOUSEMOVE, WM_MOUSEWHEEL,
    WM_RBUTTONDOWN, WM_RBUTTONUP, WM_XBUTTONDOWN, WM_XBUTTONUP, XBUTTON1, XBUTTON2,
};

use super::pointer::pointer_button;

/// The high word of `MSLLHOOKSTRUCT::mouseData` names the side button.
fn side(button: u16) -> u32 {
    u32::from(button) << 16
}

#[test]
fn every_button_press_is_numbered_the_way_x11_numbers_it() {
    assert_eq!(pointer_button(WM_LBUTTONDOWN, 0), Some(1));
    assert_eq!(pointer_button(WM_MBUTTONDOWN, 0), Some(2));
    assert_eq!(pointer_button(WM_RBUTTONDOWN, 0), Some(3));
    assert_eq!(pointer_button(WM_XBUTTONDOWN, side(XBUTTON1)), Some(8));
    assert_eq!(pointer_button(WM_XBUTTONDOWN, side(XBUTTON2)), Some(9));
}

#[test]
fn releases_movement_and_the_wheel_put_the_caret_nowhere() {
    for msg in [
        WM_LBUTTONUP,
        WM_RBUTTONUP,
        WM_MOUSEMOVE,
        WM_MOUSEWHEEL,
        WM_MOUSEHWHEEL,
    ] {
        assert_eq!(pointer_button(msg, 0), None, "message {msg:#x}");
    }
    assert_eq!(pointer_button(WM_XBUTTONUP, side(XBUTTON1)), None);
}

/// A wheel delta lives in the same high word; on anything but a side
/// button it must not be read as one.
#[test]
fn a_side_button_needs_a_side_button_message() {
    assert_eq!(pointer_button(WM_MOUSEWHEEL, side(XBUTTON1)), None);
    assert_eq!(pointer_button(WM_XBUTTONDOWN, 0), None);
}
