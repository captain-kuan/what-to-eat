use ratatui::layout::Rect;
use ratatui::Frame;

use crate::app::App;
use crate::components::Lucky;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    frame.render_stateful_widget(
        Lucky::new(app.options.clone()),
        frame.size(),
        &mut app.lucky_num,
    );
    if app.popup.is_visible() {
        let popup_area = Rect {
            x: frame.size().width / 4,
            y: frame.size().height / 3,
            width: frame.size().width / 2,
            height: frame.size().height / 3,
        };

        if let Some(title) = app.options.get(app.end_num) {
            let _ = app.popup.draw(&title, frame, popup_area);
        }
    }
}
