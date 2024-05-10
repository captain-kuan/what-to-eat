use anyhow::Result;
use ratatui::Frame;
use ratatui::{buffer::Buffer, layout::*, style::*, widgets::*};

/// Application.
#[derive(Debug)]
pub struct Popup {
    visible: bool,
}

impl Default for Popup {
    fn default() -> Self {
        Self { visible: false }
    }
}

impl Popup {
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn show(&mut self) {
        self.visible = true;
    }
    pub fn draw(&self, title: &str, f: &mut Frame, area: Rect) -> Result<()> {
        let mut title = title.to_string();
        if self.is_visible() {
            f.render_widget(Clear, area);
            f.render_stateful_widget(self, area, &mut title)
        }

        Ok(())
    }
}

impl StatefulWidget for &Popup {
    type State = String;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        Paragraph::new(state.to_string())
            .wrap(Wrap { trim: true })
            .style(Style::new().yellow())
            .block(
                Block::new()
                    .title("今天吃")
                    .title_style(Style::new().white().bold())
                    .borders(Borders::ALL)
                    .border_style(Style::new().red()),
            )
            .render(area, buf);
    }
}
