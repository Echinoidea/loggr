use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::DefaultTerminal;

use crate::ui;

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

pub struct App {
    pub running: bool,
    pub current_screen: CurrentScreen,
    pub loaded_project: Option<CurrentProject>,
    pub project_list: Vec<String>,
    pub highlighted_project: usize,
    pub project_name_input: String,
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
            project_name_input: "".to_string(),
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
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
                    (_, KeyCode::Up | KeyCode::Char('k')) => self.scroll_project_list_up(),
                    (_, KeyCode::Down | KeyCode::Char('j')) => self.scroll_project_list_down(),
                    (_, KeyCode::Right | KeyCode::Char('l')) => {
                        self.current_screen = CurrentScreen::Main
                    }
                    (_, KeyCode::Char('a')) => self.current_screen = CurrentScreen::ProjectAdding,
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
                // Add other key handlers here.
                _ => {}
            }
        }

        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }

    // These two functions are so bad lol
    fn scroll_project_list_up(&mut self) {
        if self.highlighted_project == 0 {
            self.highlighted_project = 0;
        } else {
            self.highlighted_project -= 1;
        }
    }

    fn scroll_project_list_down(&mut self) {
        if self.highlighted_project + 1 > self.project_list.len() - 1 {
            self.highlighted_project = self.project_list.len() - 1;
        } else {
            self.highlighted_project += 1;
        }
    }
}
