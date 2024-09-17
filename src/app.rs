use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::*;
use ratatui::style::*;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::*;
use ratatui::DefaultTerminal;
use ratatui::Frame;

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
    running: bool,
    current_screen: CurrentScreen,
    loaded_project: Option<CurrentProject>,
    project_list: Vec<String>,
    highlighted_project: usize,
    project_name_input: String,
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
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let chunks_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Title block
                Constraint::Min(1),    // Body
                Constraint::Length(1), // Footer instructions
            ])
            .split(frame.area());

        let chunks_body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20)])
            .split(chunks_layout[1]);

        // Title
        let title = Paragraph::new("").bold().green().centered().block(
            Block::default()
                .borders(Borders::TOP)
                .border_type(BorderType::Rounded)
                .title("Loggr")
                .title_alignment(Alignment::Center),
        );

        let instruction_text = if self.current_screen == CurrentScreen::Main {
            "`h` project mode | `l` list mode"
        } else if self.current_screen == CurrentScreen::ProjectEditing {
            "`a` append project | `x` delete project"
        } else {
            ""
        };

        let instructions = Paragraph::new(instruction_text).bold().green().centered();

        // Project list stuff
        let project_list_items: Vec<ListItem> = self
            .project_list
            .clone()
            .iter()
            .enumerate()
            .map(|(i, project)| {
                let content = if i == self.highlighted_project {
                    Span::styled(
                        format!("> {}", project),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::raw(format!("  {}", project))
                };

                ListItem::new(content).style(if i == self.highlighted_project {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                })
            })
            .collect();

        let project_list = List::new(project_list_items).block(
            Block::default()
                .title("Projects")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded),
        );

        if self.current_screen == CurrentScreen::ProjectAdding {
            let popup_area = App::centered_rect(50, 20, frame.area());

            let popup_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)])
                .split(popup_area);

            let popup_prompt = Paragraph::new(Line::from("Enter new project name: "));
            let popup_input = Paragraph::new(Line::from(self.project_name_input.clone()));

            frame.render_widget(popup_prompt, popup_chunks[0]);
            frame.render_widget(popup_input, popup_chunks[1]);
        }

        // Timesheet stuff

        // Render widgets
        frame.render_widget(title, chunks_layout[0]);
        frame.render_widget(project_list, chunks_body[0]);
        frame.render_widget(instructions, chunks_layout[2]);
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

    fn on_key_event(&mut self, key: KeyEvent) {
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
                (_, KeyCode::Char('q')) => {
                    self.current_screen = CurrentScreen::ProjectEditing;
                }
                _ => {}
            },

            _ => {}
        };

        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn add_project(&mut self) {
        self.project_list.push("Project 1".to_string());
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

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        // Cut the given rectangle into three vertical pieces
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r); // What does this do? I know what it does now

        // Then cut the middle vertical piece into three width-wise pieces
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1] // Return the middle chunk
    }
}
