use std::path::Path;

mod dir;
mod format;

use dir::{DirContent, DirContents};

use iced::pure::{button, column, container, row, scrollable, text_input, Element};
use iced::{Alignment, Command};

#[derive(Debug, Clone)]
pub enum Message {
    BackDir,
    InputChanged(String),
    Selected(Selected),
}

#[derive(Debug, Clone)]
pub enum Selected {
    Dir(String),
    File(String),
}

pub struct Explorer {
    input: String,
    dirs: DirContents,
}

impl Default for Explorer {
    fn default() -> Self {
        let audio_dir = dirs::audio_dir().and_then(|dir| dir.into_os_string().into_string().ok());

        let dirs = audio_dir
            .as_ref()
            .map(|dir| DirContents::new(dir))
            .unwrap_or_default();

        Self {
            input: audio_dir.unwrap_or_default(),
            dirs,
        }
    }
}

impl Explorer {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(input) => {
                self.input = input;
                self.dirs = DirContents::new(&self.input);
            }
            Message::Selected(selected) => match selected {
                Selected::Dir(dir) => {
                    self.input = dir;
                    self.dirs = DirContents::new(&self.input);
                }
                Selected::File(_) => {
                    // do nothing
                }
            },
            Message::BackDir => {
                let current_dir = self.input.clone();
                let path = Path::new(&current_dir);

                if let Some(back_dir) = path.parent() {
                    if let Some(back_dir) = back_dir.as_os_str().to_str() {
                        self.input = back_dir.to_string();
                        self.dirs = DirContents::new(&self.input);
                    }
                }
            }
        }
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let input = text_input("Search", &self.input, Message::InputChanged).padding(5);
        let back_button = button("Back").on_press(Message::BackDir);

        let search = row()
            .push(input)
            .push(back_button)
            .padding(5)
            .align_items(Alignment::Center);

        let mut results = column().spacing(10);

        for dir in self.dirs.list().iter() {
            if let Some(full_path) = dir.path().to_str() {
                if let Some(name) = dir.name() {
                    match dir {
                        DirContent::Dir(_) => {
                            results = results.push(container(button(name).on_press(
                                Message::Selected(Selected::Dir(full_path.to_string().clone())),
                            )))
                        }
                        DirContent::AudioFile(_) => {
                            results = results.push(container(button(name).on_press(
                                Message::Selected(Selected::File(full_path.to_string().clone())),
                            )))
                        }
                    }
                }
            }
        }

        let results = scrollable(results);

        let content = column().push(search).push(results);

        container(content).center_x().center_y().into()
    }
}
