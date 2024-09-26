use crate::timesheet::{Timesheet, TimesheetEntry};
use crate::{files, ui};
use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::window_size,
};
use ratatui::DefaultTerminal;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq)]
pub enum CurrentScreen {
    Main,
    ProjectEditing,
    ProjectAdding,
    ClockingInOut,
    Exiting,
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
    pub highlighted_entry: usize,
}

impl App {
    pub fn new() -> App {
        App {
            running: true,
            current_screen: CurrentScreen::Main,
            loaded_project: None,
            project_list: vec![],
            highlighted_project: 0,
            selected_project: 0,
            project_name_input: "".to_string(),
            highlighted_entry: 0,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let _ = files::make_loggr_dir();

        self.project_list.push(
            files::load_timesheet("MIDAS2".to_string())?
                .name
                .to_string(),
        );

        self.loaded_project = Some(files::load_timesheet("MIDAS2".to_string())?);

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
                    (_, KeyCode::Char('k') | KeyCode::Up) => {
                        self.scroll_entries_list(ScrollDirection::Up)
                    }
                    (_, KeyCode::Char('j') | KeyCode::Down) => {
                        self.scroll_entries_list(ScrollDirection::Down)
                    }
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
                    (_, KeyCode::Char('c') | KeyCode::Char('C')) => {
                        if let Some(project) = &mut self.loaded_project {
                            project.clock_io();
                        } else {
                            // Handle the case where `loaded_project` is None
                            println!("No project is loaded.");
                        }
                    }
                    (_, KeyCode::Enter) => self.select_project()?,
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

                    (_, KeyCode::Enter) => self.add_project()?,

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

    fn select_project(&mut self) -> Result<()> {
        self.selected_project = self.highlighted_project;
        self.loaded_project = Some(files::load_timesheet(
            self.project_list[self.selected_project].clone(),
        )?);

        Ok(())
    }

    fn add_project(&mut self) -> Result<()> {
        self.project_list.push(self.project_name_input.clone());
        self.current_screen = CurrentScreen::ProjectEditing;

        files::save_timesheet(Timesheet::new(self.project_name_input.clone(), vec![]))?;

        self.project_name_input = "".to_string();

        Ok(())
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

    fn scroll_entries_list(&mut self, direction: ScrollDirection) {
        if let Some(project) = &mut self.loaded_project {
            let entries = &project.entries;
            match direction {
                ScrollDirection::Up => {
                    if self.highlighted_entry == 0 {
                        self.highlighted_entry = 0;
                    } else {
                        self.highlighted_entry -= 1;
                    }
                }

                ScrollDirection::Down => {
                    if self.highlighted_entry + 1 > entries.len() - 1 {
                        self.highlighted_entry = entries.len() - 1;
                    } else {
                        self.highlighted_entry += 1;
                    }
                }
            }
        } else {
            // Handle the case where `loaded_project` is None
            println!("No project is loaded.");
        }
    }
}
