use crate::app;
use crate::app::App;
use crate::app::CurrentScreen;
use crate::timesheet::Timesheet;
use crate::timesheet::TimesheetEntry;
use ratatui::layout::*;
use ratatui::style::*;
use ratatui::text::*;
use ratatui::widgets::*;
use ratatui::Frame;

pub fn draw_ui(app: &mut App, frame: &mut Frame) {
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
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(chunks_layout[1]);

    // Title
    let title = Paragraph::new("").bold().green().centered().block(
        Block::default()
            .borders(Borders::TOP)
            .border_type(BorderType::Rounded)
            .title("Loggr")
            .title_alignment(Alignment::Center),
    );

    let mut instruction_text: &str = "";

    match app.current_screen {
        CurrentScreen::Main => instruction_text = "`h` project mode | `CTRL + c quit`",
        CurrentScreen::ProjectEditing => {
            instruction_text =
                "`j/down` move down | `k/up` move up | `ENTER` to load | `a` append project | `l` list mode"
        }
        CurrentScreen::ProjectAdding => instruction_text = "type to enter a name | `CTRL q` back",
        CurrentScreen::ClockingInOut => todo!(),
        CurrentScreen::Exiting => todo!(),
    }

    let instructions = Paragraph::new(instruction_text).bold().green().centered();

    // Project list stuff
    let project_list_items: Vec<ListItem> = app
        .project_list
        .clone()
        .iter()
        .enumerate()
        .map(|(i, project)| {
            let content = if i == app.highlighted_project && i == app.selected_project {
                Span::styled(
                    format!("> {}", project),
                    Style::default()
                        .fg(Color::Yellow)
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                )
            } else if i == app.highlighted_project {
                Span::styled(
                    format!("> {}", project),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            } else if i == app.selected_project {
                Span::styled(
                    format!("  {}", project),
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw(format!("  {}", project))
            };

            ListItem::new(content).style(if i == app.highlighted_project {
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

    if app.current_screen == app::CurrentScreen::ProjectAdding {
        let popup_area = centered_rect(50, 20, frame.area());

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(1)])
            .split(popup_area);

        let popup_prompt = Paragraph::new(Line::from("Enter new project name: "));
        let popup_input = Paragraph::new(Line::from(app.project_name_input.clone()));

        frame.render_widget(popup_prompt, popup_chunks[0]);
        frame.render_widget(popup_input, popup_chunks[1]);
    }

    // Timesheet stuff
    let timesheet_entry_list: Vec<TimesheetEntry> =
        app.loaded_project.as_ref().unwrap().clone().entries;

    let mut timesheet_entry_list_string: Vec<String> = vec![];

    for i in 0..timesheet_entry_list.len() {
        timesheet_entry_list_string.push(format!(
            "{}, {}, {}",
            timesheet_entry_list[i].date,
            timesheet_entry_list[i].time_in,
            timesheet_entry_list[i].time_out
        ));
    }

    let entry_list_items: Vec<ListItem> = timesheet_entry_list_string
        .clone()
        .iter()
        .enumerate()
        .map(|(i, project)| {
            let content = if i == app.highlighted_project {
                Span::styled(
                    format!("> {}", project),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            } else if i == app.highlighted_project {
                Span::styled(
                    format!("> {}", project),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw(format!("  {}", project))
            };

            ListItem::new(content).style(if i == app.highlighted_project {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            })
        })
        .collect();

    let entry_list = List::new(entry_list_items).block(
        Block::default()
            .title("Entries")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(title, chunks_layout[0]);
    frame.render_widget(project_list, chunks_body[0]);
    frame.render_widget(entry_list, chunks_body[1]);
    frame.render_widget(instructions, chunks_layout[2]);
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
