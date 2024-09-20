use crate::ui;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{widgets::ListDirection, DefaultTerminal};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Timesheet {
    name: String,
    entries: Vec<TimesheetEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimesheetEntry {
    date: String,
    time_in: String,
    time_out: String,
}

#[derive(PartialEq, Eq)]
pub enum CurrentScreen {
    Main,
    ProjectEditing,
    ProjectAdding,
    ClockingInOut,
    Exiting,
}

pub enum CurrentProject {
    Name,
    Data,
}

enum ScrollDirection {
    Up,
    Down,
}

pub struct App {
    pub running: bool,
    pub current_screen: CurrentScreen,
    pub loaded_project: Option<Timesheet>,
    pub project_list: Vec<String>,
    pub highlighted_project: usize,
    pub project_name_input: String,
    pub selected_project: usize,
}

impl App {
    pub fn new() -> App {
        App {
            running: true,
            current_screen: CurrentScreen::Main,
            loaded_project: None,
            project_list: vec![
                "MIDAS".to_string(),
                "SAEBRS".to_string(),
                "LOGGR".to_string(),
            ],
            highlighted_project: 0,
            selected_project: 0,
            project_name_input: "".to_string(),
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let _ = self.make_loggr_dir();
        self.running = true;
        while self.running {
            terminal.draw(|frame| ui::draw_ui(self, frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                return Ok(());
            }

            match self.current_screen {
                CurrentScreen::Main => match (key.modifiers, key.code) {
                    (_, KeyCode::Char('h')) => self.current_screen = CurrentScreen::ProjectEditing,
                    _ => {}
                },
                CurrentScreen::ProjectEditing => match (key.modifiers, key.code) {
                    (_, KeyCode::Up | KeyCode::Char('k')) => {
                        self.scroll_project_list(ScrollDirection::Up)
                    }
                    (_, KeyCode::Down | KeyCode::Char('j')) => {
                        self.scroll_project_list(ScrollDirection::Down)
                    }
                    (_, KeyCode::Right | KeyCode::Char('l')) => {
                        self.current_screen = CurrentScreen::Main
                    }
                    (_, KeyCode::Char('a')) => self.current_screen = CurrentScreen::ProjectAdding,
                    (_, KeyCode::Enter) => self.select_project(),
                    _ => {}
                },
                CurrentScreen::ProjectAdding => match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                        self.current_screen = CurrentScreen::ProjectEditing;
                        self.project_name_input = "".to_string();
                    }

                    (_, KeyCode::Char(value)) => {
                        self.project_name_input.push(value);
                    }

                    (_, KeyCode::Backspace) => {
                        self.project_name_input.pop();
                    }

                    (_, KeyCode::Enter) => {
                        self.project_list.push(self.project_name_input.clone());
                        self.current_screen = CurrentScreen::ProjectEditing;
                        self.project_name_input = "".to_string();
                    }

                    _ => {}
                },
                CurrentScreen::ClockingInOut => todo!(),
                CurrentScreen::Exiting => todo!(),
            }

            match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
                _ => {}
            }
        }

        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn select_project(&mut self) {
        self.selected_project = self.highlighted_project;
    }

    fn scroll_project_list(&mut self, direction: ScrollDirection) {
        match direction {
            ScrollDirection::Up => {
                if self.highlighted_project == 0 {
                    self.highlighted_project = 0;
                } else {
                    self.highlighted_project -= 1;
                }
            }

            ScrollDirection::Down => {
                if self.highlighted_project + 1 > self.project_list.len() - 1 {
                    self.highlighted_project = self.project_list.len() - 1;
                } else {
                    self.highlighted_project += 1;
                }
            }
        }
    }

    fn make_loggr_dir(&mut self) -> std::io::Result<()> {
        if self.assert_loggr_dir() {
            return Ok(());
        }

        // Change to home env
        let _f = std::fs::create_dir("/home/gabriel/.loggr")?;
        Ok(())
    }

    fn assert_loggr_dir(&self) -> bool {
        let path = std::path::Path::new("/home/gabriel/.loggr/");
        return path.exists() && path.is_dir();
    }

    fn load_timesheet(&mut self) {
        // Load timesheet json file, get each entry object and put into new
        // TimesheetEntry, put into timesheet vector
    }

    fn save_timesheet(&self) {
        todo!();
    }
}
