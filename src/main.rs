use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,},
};
use figlet_rs::{FIGlet,Toilet,};
use std::{
    io::{stdout, Write},
    time::Duration,
};
use time::macros::format_description;
use time::OffsetDateTime;
use clap::Parser;

enum figletfonts{
    Standard(FIGlet),
    Small(FIGlet),
    Big(FIGlet),
    Slant(FIGlet),
    Smblock(Toilet),
    Mono9(Toilet),
    Mono12(Toilet),
    Future(Toilet),
    Wideterm(Toilet),

}

impl figletfonts{
    fn load(name: &str) -> Result<Self, String>{
        match name.to_lowercase().as_str(){
            "standard" => Ok(figletfonts::Standard(FIGlet::standard().unwrap())),
            "small" => Ok(figletfonts::Small(FIGlet::small().unwrap())),
            "big" => Ok(figletfonts::Big(FIGlet::big().unwrap())),
            "slant" => Ok(figletfonts::Slant(FIGlet::slant().unwrap())),
            "smblock" => Ok(figletfonts::Smblock(Toilet::smblock().unwrap())),
            "mono9" => Ok(figletfonts::Mono9(Toilet::mono9().unwrap())),
            "mono12" => Ok(figletfonts::Mono12(Toilet::mono12().unwrap())),
            "future" => Ok(figletfonts::Future(Toilet::future().unwrap())),
            "wideterm" => Ok(figletfonts::Wideterm(Toilet::wideterm().unwrap())),
            _ => Err(format!("unknown font, please choose one of the following: standard, small, big, slant, smblock, mono9, mono12,future,wideterm"))
        }
    }

    fn convertor(&self, text: &str) -> String {
        match self {
            figletfonts::Standard(f) => f.convert(text).unwrap().to_string(),
            figletfonts::Small(f) => f.convert(text).unwrap().to_string(),
            figletfonts::Big(f) => f.convert(text).unwrap().to_string(),
            figletfonts::Slant(f) => f.convert(text).unwrap().to_string(),
            figletfonts::Smblock(f) => f.convert(text).unwrap().to_string(),
            figletfonts::Mono9(f) => f.convert(text).unwrap().to_string(),
            figletfonts::Mono12(f) => f.convert(text).unwrap().to_string(),
            figletfonts::Future(f) => f.convert(text).unwrap().to_string(),
            figletfonts::Wideterm(f) => f.convert(text).unwrap().to_string(),
        }
    }


}

fn main() -> std::io::Result<()> {
    #[derive(Parser)]
    struct Args {
        #[arg(short = 'p')]
        plaintext: bool,

        #[arg(short = 'f', long = "font", default_value = "mono9")]
        font: String,
    }
    let args = Args::parse();
    let font = if args.plaintext {
        None
    } else {
        match figletfonts::load(&args.font) {
            Ok(f) => Some(f),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    };
    let mut stdout = stdout();
    let (cols, _rows) = size()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    let timeformat = format_description!("[hour]:[minute]:[second]");
    let timeformatdate = format_description!("[year]:[month]:[day]");

    loop {
        execute!(stdout, Clear(ClearType::FromCursorDown), MoveTo(0, 0))?;
        if let Ok(now) = OffsetDateTime::now_local() {
            let time = now.format(&timeformat).unwrap();
            let date = now.format(&timeformatdate).unwrap();
            let width = cols as usize;

            match &font {
                Some(f) => {
                    let timefiglet = f.convertor(&time);
                    for line in timefiglet.lines() {
                        write!(stdout, "{:^width$}\r\n", line, width = width)?;
                    }
                }
                None => {
                    write!(stdout, "{:^width$}\r\n", time, width = width)?;
                }
            }


            write!(stdout, "{:^width$}", date, width = width)?;
            stdout.flush()?;
        } else {
            write!(stdout, "failed to find timezone\r\n")?;
            stdout.flush()?;
            break;
        }

        if poll(Duration::from_secs(1))? {
            if let Event::Key(event) = read()? {
                if event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout, Show, LeaveAlternateScreen)?;
    Ok(())
}
