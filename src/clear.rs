use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::{stdout, Result};

pub fn clear_terminal() -> Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;
    Ok(())
}