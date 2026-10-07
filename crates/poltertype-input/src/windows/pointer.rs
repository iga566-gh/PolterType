//! Which mouse messages move the caret — the Windows half of the
//! pointer marker the macOS, X11 and evdev listeners already send.

use windows::Win32::UI::WindowsAndMessaging::{
    WM_LBUTTONDOWN, WM_MBUTTONDOWN, WM_RBUTTONDOWN, WM_XBUTTONDOWN, XBUTTON1, XBUTTON2,
};

/// The button a `WH_MOUSE_LL` message pressed, numbered the way X11
/// numbers it (1 left, 2 middle, 3 right, 8 and 9 the side buttons),
/// or `None` for anything that does not move the caret.
///
/// Presses only, the same contract as the other listeners: releases
/// are noise, and movement and the wheel scroll the view without
/// putting the caret anywhere. A side button is in because a browser
/// reads it as back/forward, which takes the caret with the page.
///
/// `mouse_data` is `MSLLHOOKSTRUCT::mouseData`, whose high word names
/// the side button on `WM_XBUTTONDOWN`.
pub(super) fn pointer_button(msg: u32, mouse_data: u32) -> Option<u32> {
    match msg {
        WM_LBUTTONDOWN => Some(1),
        WM_MBUTTONDOWN => Some(2),
        WM_RBUTTONDOWN => Some(3),
        WM_XBUTTONDOWN => match (mouse_data >> 16) as u16 {
            XBUTTON1 => Some(8),
            XBUTTON2 => Some(9),
            _ => None,
        },
        _ => None,
    }
}
