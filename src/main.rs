use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, size},
};

use figlet_rs::{Toilet};

use std::{
    io::{stdout, Write},
    time::Duration,
};
use time::macros::format_description;
use time::OffsetDateTime;

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();

    let (cols, _ows) = size()?;

    execute!(stdout, EnterAlternateScreen, Hide)?;
    let timeformat = format_description!("[hour]:[minute]:[second]");
    let timeformatdate = format_description!("[year]:[month]:[day]");

    let mono9_font = Toilet::mono9().unwrap();
    loop {
        execute!(stdout, Clear(ClearType::FromCursorDown), MoveTo(0, 0))?;

        if let Ok(now) = OffsetDateTime::now_local(){
            let time = now.format(&timeformat).unwrap();
            let date = now.format(&timeformatdate).unwrap();
            let width = cols as usize;
            let timefiglet = mono9_font.convert(&time).unwrap().to_string();
            for line in timefiglet.lines() {
                println!("{:^width$}", line, width = width);
            }
            print!("{:^width$}", date, width = width);
            stdout.flush()?;
        } else{
            println!("failed to find timezone");
            break;
        }

            if let Event::Key(event) = read()? {
                if event.code == KeyCode::Char('q') { break;}
            }

    }

    execute!(stdout, Show, LeaveAlternateScreen)?;
    Ok(())
}
