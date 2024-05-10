use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Borders, Tabs};
use ratatui::Frame;

use crate::app::App;
use crate::components::Lucky;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(3), Constraint::Percentage(90)])
        .split(frame.size());
    frame.render_widget(
        Tabs::new(vec!["转盘", "设置"])
            .block(Block::default().title("Tabs").borders(Borders::ALL))
            .style(Style::default().white())
            .highlight_style(Style::default().yellow())
            .select(app.selected_tab)
            .divider("|")
            .padding("", ""),
        layout[0],
    );
    match app.selected_tab {
        0 => {
            frame.render_stateful_widget(
                Lucky::new(app.options.clone()),
                layout[1],
                &mut app.lucky_num,
            );
        }
        1 => {
            
        }
        _ => {}
    }

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
