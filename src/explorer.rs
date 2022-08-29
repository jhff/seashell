// extern crate dirs;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{fs, io};

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
    dirs: Vec<PathBuf>,
    input: String,
}

static SUPPORTED_FORMATS: &[&str] = &["mp3", "flac", "wav", "ogg"];

impl Default for Explorer {
    fn default() -> Self {
        let audio_dir = dirs::audio_dir().and_then(|dir| dir.into_os_string().into_string().ok());

        let dirs = audio_dir
            .as_ref()
            .and_then(|dir| explore(dir).ok())
            .unwrap_or_default();

        Self {
            dirs,
            input: audio_dir.unwrap_or_default(),
        }
    }
}

impl Explorer {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(input) => {
                self.input = input;

                if let Ok(dirs) = explore(&self.input) {
                    self.dirs = dirs;
                }
            }
            Message::Selected(selected) => match selected {
                Selected::Dir(dir) => {
                    self.input = dir;

                    if let Ok(dirs) = explore(&self.input) {
                        self.dirs = dirs;
                    }
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

                        if let Ok(dirs) = explore(&self.input) {
                            self.dirs = dirs;
                        }
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

        for dir in &self.dirs {
            let full_path = dir.as_path();

            if let Some(file) = full_path.file_name() {
                if let Some(file) = file.to_str() {
                    if let Some(full_dir) = dir.as_path().to_str() {
                        match (full_path.is_file(), full_path.is_dir()) {
                            (true, false) => {
                                results = results.push(container(button(file).on_press(
                                    Message::Selected(Selected::File(full_dir.to_string().clone())),
                                )))
                            }
                            (false, true) => {
                                results = results.push(container(button(file).on_press(
                                    Message::Selected(Selected::Dir(full_dir.to_string().clone())),
                                )))
                            }
                            _ => {}
                        };
                    }
                }
            }
        }

        let results = scrollable(results);

        let content = column().push(search).push(results);

        container(content).center_x().center_y().into()
    }
}

fn explore(dir: &str) -> Result<Vec<PathBuf>, io::Error> {
    let mut entries = fs::read_dir(dir)?
        .filter(|res| {
            res.as_ref()
                .map(|e| {
                    if let Ok(metadata) = e.metadata() {
                        if metadata.is_dir() {
                            return true;
                        } else {
                            if let Some(extension) = e.path().extension() {
                                return is_audio_file(extension);
                            }
                        }
                    }
                    false
                })
                .unwrap()
        })
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<PathBuf>, io::Error>>()?;

    entries.sort();

    Ok(entries)
}

fn is_audio_file(extension: &OsStr) -> bool {
    SUPPORTED_FORMATS.iter().any(|&format| format == extension)
}
