use ratatui::Frame;

pub trait PopupAnimation {
    fn render(&mut self, frame: &mut Frame);
}
