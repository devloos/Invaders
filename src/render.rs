use std::io::Stdout;

use crossterm::{
    style::{Color, SetBackgroundColor},
    terminal::ClearType,
    QueueableCommand,
};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(ClearType::All).unwrap();
        stdout.queue(SetBackgroundColor(Co))
    }
}
