//! #12929: After Reset Tool the Quick Selection tool gets a 30 px size while keeping
//! hardness, spacing and angle unchanged.

use serde_json::json;

use super::*;
use crate::canvas::paint_mouse;

fn app() -> PhotocraftApp {
    let mut app = PhotocraftApp::new(photocraft_engine::Session::new(), crate::Services::default());
    app.run("file.new", json!({"width": 100, "height": 60})).unwrap();
    app
}

/// #12929: After Reset Tool the Quick Selection tool's brush is 30 px while keeping
/// hardness, spacing and angle unchanged.
#[test]
fn quick_selection_reset_tip() {
    let mut app = app();
    app.ui.tool = Tool::QuickSelection;
    // First, set some non-default values to make sure they are preserved
    app.session.tools.brush.hardness = 0.5;
    app.session.tools.brush.spacing = 0.5;
    app.session.tools.brush.angle = 45.0;
    let brush = paint_mouse::fresh_brush(Tool::QuickSelection, &app.session.tools.brush);
    assert_eq!(brush.size, 30.0, "Quick Selection size should be 30 px");
    assert_eq!(brush.hardness, 0.5, "Quick Selection hardness should be kept");
    assert_eq!(brush.spacing, 0.5, "Quick Selection spacing should be kept");
    assert_eq!(brush.angle, 45.0, "Quick Selection angle should be kept");
}

/// #12929: Other tools keep their default tips after Reset Tool.
#[test]
fn quick_selection_reset_does_not_affect_other_tools() {
    let mut app = app();
    let brush = paint_mouse::fresh_brush(Tool::Brush, &app.session.tools.brush);
    assert_eq!((brush.size, brush.hardness, brush.spacing), (13.0, 0.0, 0.25));
}
