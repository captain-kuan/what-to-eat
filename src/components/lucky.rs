use std::rc::Rc;

use ratatui::{buffer::Buffer, layout::*, style::*, widgets::*};

pub struct Lucky {
    options: Vec<String>,
}
impl Lucky {
    pub fn new(options: Vec<String>) -> Self {
        Lucky { options }
    }
}
impl StatefulWidget for Lucky {
    type State = usize;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut rect_list: Vec<Rect> = vec![];
        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(3),
                Constraint::Min(3),
                Constraint::Min(3),
            ])
            .split(area);

        for i in 0..3 {
            let inner: Rc<[Rect]> = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Min(3),
                    Constraint::Min(3),
                    Constraint::Min(3),
                ])
                .split(outer[i]);
            for j in 0..3 {
                rect_list.push(inner[j]);
            }
        }
        let mut options = self.options;
        options.splice(4..4, ["回车键开始".to_string()]);
        let state = if state.to_owned() < 4 {
            state.to_owned()
        } else {
            state.to_owned() + 1
        };
        for i in 0..options.len() {
            Paragraph::new(options[i].to_owned())
                .centered()
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(if i == state {
                            Color::Yellow
                        } else {
                            Color::White
                        })),
                )
                .render(rect_list[i], buf);
        }
    }
}
