use serde::{Deserialize, Serialize};
use crossterm::{terminal, event::{self, Event, KeyCode, KeyModifiers}};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::{Block, Borders, List, ListItem, Paragraph}, layout::{Layout, Constraint, Direction}};
use std::io::stdout;
use std::env;

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    text: String,
    done: bool,
}

#[derive(PartialEq)]
enum AppMode {
    Normal,
    AddingTask,
    ConfirmingDelete,
    ShowingHelp,
}

fn load_tasks() -> Vec<Task> {
    serde_json::from_reader(std::fs::File::open("todos.json").unwrap_or_else(|_| {
        std::fs::File::create("todos.json").unwrap()
    })).unwrap_or_else(|_| vec![])
}

fn save_tasks(tasks: &[Task]) {
    serde_json::to_writer_pretty(std::fs::File::create("todos.json").unwrap(), tasks).unwrap();
}

fn main() -> std::io::Result<()> {
    let debug_mode = env::args().any(|arg| arg == "--debug");
    
    let stdout = stdout();
    terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut tasks = load_tasks();
    let mut selected = 0;
    let mut ui_visible = true;
    let mut debug_log: Vec<String> = Vec::new();
    let mut app_mode = AppMode::Normal;
    let mut input_text = String::new();
    
    if debug_mode {
        debug_log.push("Debug mode enabled".to_string());
        debug_log.push(format!("UI visible: {}", ui_visible));
    }

    loop {
        terminal.draw(|f| {
            let size = f.size();
            
            // Create main layout (with prompt area, debug area)
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(match (app_mode != AppMode::Normal && app_mode != AppMode::ShowingHelp, debug_mode) {
                    (true, true) => [Constraint::Min(8), Constraint::Length(3), Constraint::Length(8)].as_ref(),
                    (true, false) => [Constraint::Min(8), Constraint::Length(3)].as_ref(),
                    (false, true) => [Constraint::Min(10), Constraint::Length(8)].as_ref(),
                    (false, false) => [Constraint::Min(0)].as_ref(),
                })
                .split(size);
            
            // Only render todo list if UI is visible
            if ui_visible {
                // Main content area (todo list)
                let content_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Min(60), Constraint::Length(30)].as_ref())
                    .split(main_chunks[0]);

                let items: Vec<ListItem> = tasks.iter().enumerate().map(|(i, task)| {
                    let prefix = if task.done { "[x]" } else { "[ ]" };
                    let style = if i == selected { 
                        ratatui::style::Style::default().bg(ratatui::style::Color::Blue) 
                    } else { 
                        ratatui::style::Style::default() 
                    };
                    ListItem::new(format!("{} {}", prefix, task.text)).style(style)
                }).collect();

                let title = if app_mode == AppMode::Normal {
                    "TODO (h=help)"
                } else {
                    "TODO"
                };
                let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));
                f.render_widget(list, content_chunks[1]);
            }
            
            // Prompt area for input/confirmation (not for help mode)
            if app_mode != AppMode::Normal && app_mode != AppMode::ShowingHelp {
                let prompt_text = match app_mode {
                    AppMode::AddingTask => format!("Add task: {}", input_text),
                    AppMode::ConfirmingDelete => {
                        if !tasks.is_empty() && selected < tasks.len() {
                            format!("Delete '{}' ? (y/n)", tasks[selected].text)
                        } else {
                            "No task to delete".to_string()
                        }
                    }
                    AppMode::Normal | AppMode::ShowingHelp => String::new(),
                };
                let prompt_paragraph = Paragraph::new(prompt_text)
                    .block(Block::default().borders(Borders::ALL).title("Prompt"));
                
                let prompt_index = if debug_mode { 1 } else { 1 };
                f.render_widget(prompt_paragraph, main_chunks[prompt_index]);
            }
            
            // Help overlay
            if app_mode == AppMode::ShowingHelp {
                let help_text = "GOTTODO - Keyboard Shortcuts\n\n\
                    Navigation:\n\
                    • ↑/↓        Navigate tasks\n\
                    • Space      Toggle task completion\n\
                    • q          Quit application\n\n\
                    Task Management:\n\
                    • a          Add new task\n\
                    • d          Delete selected task\n\n\
                    Interface:\n\
                    • Ctrl+Space Hide/show todo list\n\
                    • h          Show/hide this help\n\
                    • Esc        Close help or cancel action\n\n\
                    Press any key to close this help...";
                
                let help_paragraph = Paragraph::new(help_text)
                    .block(Block::default().borders(Borders::ALL).title("Help"));
                f.render_widget(help_paragraph, main_chunks[0]);
            }
            
            // Debug area at bottom (always show if debug mode is on)
            if debug_mode {
                let debug_text = debug_log.iter().rev().take(6).rev().cloned().collect::<Vec<_>>().join("\n");
                let debug_paragraph = Paragraph::new(debug_text)
                    .block(Block::default().borders(Borders::ALL).title("Debug Log"));
                
                let debug_index = if app_mode != AppMode::Normal { 2 } else { 1 };
                f.render_widget(debug_paragraph, main_chunks[debug_index]);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if debug_mode {
                    debug_log.push(format!("Key pressed: {:?} with modifiers: {:?}", key.code, key.modifiers));
                    if debug_log.len() > 20 {
                        debug_log.remove(0);
                    }
                }
                
                match app_mode {
                    AppMode::Normal => {
                        match (key.code, key.modifiers) {
                            (KeyCode::Char('q'), _) => {
                                if debug_mode {
                                    debug_log.push("Quitting application".to_string());
                                }
                                break;
                            }
                            (KeyCode::Char(' '), KeyModifiers::CONTROL) => {
                                ui_visible = !ui_visible;
                                if debug_mode {
                                    debug_log.push(format!("UI toggled: visible={}", ui_visible));
                                }
                            }
                            (KeyCode::Char(' '), _) if ui_visible => {
                                let mut task_toggled = false;
                                let mut new_done_state = false;
                                if let Some(task) = tasks.get_mut(selected) {
                                    task.done = !task.done;
                                    new_done_state = task.done;
                                    task_toggled = true;
                                }
                                if task_toggled {
                                    save_tasks(&tasks);
                                    if debug_mode {
                                        debug_log.push(format!("Task {} toggled: done={}", selected, new_done_state));
                                    }
                                }
                            }
                            (KeyCode::Char('a'), _) if ui_visible => {
                                app_mode = AppMode::AddingTask;
                                input_text.clear();
                                if debug_mode {
                                    debug_log.push("Entered task creation mode".to_string());
                                }
                            }
                            (KeyCode::Char('d'), _) if ui_visible && !tasks.is_empty() => {
                                app_mode = AppMode::ConfirmingDelete;
                                if debug_mode {
                                    debug_log.push("Entered delete confirmation mode".to_string());
                                }
                            }
                            (KeyCode::Char('h'), _) if ui_visible => {
                                app_mode = AppMode::ShowingHelp;
                                if debug_mode {
                                    debug_log.push("Showing help".to_string());
                                }
                            }
                            (KeyCode::Down, _) if ui_visible => {
                                let old_selected = selected;
                                let max_index = tasks.len().saturating_sub(1);
                                selected = (selected + 1).min(max_index);
                                if debug_mode && old_selected != selected {
                                    debug_log.push(format!("Selection moved down: {} -> {}", old_selected, selected));
                                }
                            }
                            (KeyCode::Up, _) if ui_visible => {
                                let old_selected = selected;
                                if selected > 0 {
                                    selected -= 1;
                                }
                                if debug_mode && old_selected != selected {
                                    debug_log.push(format!("Selection moved up: {} -> {}", old_selected, selected));
                                }
                            }
                            _ => {
                                if debug_mode {
                                    debug_log.push("Unhandled key in Normal mode".to_string());
                                }
                            }
                        }
                    }
                    AppMode::AddingTask => {
                        match key.code {
                            KeyCode::Enter => {
                                if !input_text.trim().is_empty() {
                                    tasks.push(Task {
                                        text: input_text.trim().to_string(),
                                        done: false,
                                    });
                                    save_tasks(&tasks);
                                    if debug_mode {
                                        debug_log.push(format!("Added task: '{}'", input_text.trim()));
                                    }
                                }
                                app_mode = AppMode::Normal;
                                input_text.clear();
                            }
                            KeyCode::Esc => {
                                app_mode = AppMode::Normal;
                                input_text.clear();
                                if debug_mode {
                                    debug_log.push("Cancelled task creation".to_string());
                                }
                            }
                            KeyCode::Backspace => {
                                input_text.pop();
                            }
                            KeyCode::Char(c) => {
                                input_text.push(c);
                            }
                            _ => {
                                if debug_mode {
                                    debug_log.push("Unhandled key in AddingTask mode".to_string());
                                }
                            }
                        }
                    }
                    AppMode::ConfirmingDelete => {
                        match key.code {
                            KeyCode::Char('y') | KeyCode::Char('Y') => {
                                if selected < tasks.len() {
                                    let removed_task = tasks.remove(selected);
                                    if selected >= tasks.len() && tasks.len() > 0 {
                                        selected = tasks.len() - 1;
                                    }
                                    save_tasks(&tasks);
                                    if debug_mode {
                                        debug_log.push(format!("Deleted task: '{}'", removed_task.text));
                                    }
                                }
                                app_mode = AppMode::Normal;
                            }
                            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                                app_mode = AppMode::Normal;
                                if debug_mode {
                                    debug_log.push("Cancelled task deletion".to_string());
                                }
                            }
                            _ => {
                                if debug_mode {
                                    debug_log.push("Unhandled key in ConfirmingDelete mode".to_string());
                                }
                            }
                        }
                    }
                    AppMode::ShowingHelp => {
                        // Any key closes help
                        app_mode = AppMode::Normal;
                        if debug_mode {
                            debug_log.push("Closed help".to_string());
                        }
                    }
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
