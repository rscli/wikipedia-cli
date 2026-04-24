use std::io::{self, IsTerminal, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
};

use crate::output::Theme;

const MAX_VISIBLE: usize = 10;

pub fn select(t: &Theme, header: &str, query: &str, items: &[String]) -> Option<usize> {
    if items.is_empty() || !io::stdout().is_terminal() {
        return None;
    }

    let mut stdout = io::stdout();
    let visible = items.len().min(MAX_VISIBLE);

    let _ = writeln!(
        stdout,
        "\n{}════════════════════════════════════════{}",
        t.dim, t.reset
    );
    let _ = writeln!(stdout, "{}\"{}\" {}{}\n", t.warn, query, header, t.reset);

    if terminal::enable_raw_mode().is_err() {
        return None;
    }

    let mut selected: usize = 0;
    let mut offset: usize = 0;

    draw(&mut stdout, t, items, selected, offset, visible);

    let result = loop {
        let Ok(Event::Key(key)) = event::read() else {
            continue;
        };
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if selected > 0 {
                    selected -= 1;
                    if selected < offset {
                        offset = selected;
                    }
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if selected + 1 < items.len() {
                    selected += 1;
                    if selected >= offset + visible {
                        offset = selected + 1 - visible;
                    }
                }
            }
            KeyCode::Enter => break Some(selected),
            KeyCode::Esc | KeyCode::Char('q') => break None,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break None,
            _ => continue,
        }
        let lines = visible + 1;
        let _ = execute!(
            stdout,
            cursor::MoveUp(lines as u16),
            Clear(ClearType::FromCursorDown)
        );
        draw(&mut stdout, t, items, selected, offset, visible);
    };

    let _ = terminal::disable_raw_mode();

    let lines = visible + 1;
    let _ = execute!(
        stdout,
        cursor::MoveUp(lines as u16),
        Clear(ClearType::FromCursorDown)
    );

    result
}

fn draw(
    stdout: &mut io::Stdout,
    t: &Theme,
    items: &[String],
    selected: usize,
    offset: usize,
    visible: usize,
) {
    let end = (offset + visible).min(items.len());
    for (i, item) in items.iter().enumerate().take(end).skip(offset) {
        if i == selected {
            let _ = write!(stdout, "\r  {}▸{} {}\r\n", t.bullet, t.reset, item);
        } else {
            let _ = write!(stdout, "\r    {}{}{}\r\n", t.dim, item, t.reset);
        }
    }
    let counter = if items.len() > MAX_VISIBLE {
        format!(" [{}/{}]", selected + 1, items.len())
    } else {
        String::new()
    };
    let _ = write!(
        stdout,
        "\r{}↑/↓ select · Enter open · Esc quit{}{}\r",
        t.dim, counter, t.reset
    );
    let _ = stdout.flush();
}
