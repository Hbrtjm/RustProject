use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame, Terminal};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
    // let mut terminal = DefaultTerminal::new()?;
    // terminal.clear()?;

    // let mut events = event::EventStream::new();
    // while let Some(event) = events.next()? {
    //     match event {
    //         Event::Key(key) => {
    //             if key == 'q' {
    //                 break;
    //             }
    //         }
    //         Event::Resize => {
    //             terminal.draw(|f| draw(f))?;
    //         }
    //         _ => {}
    //     }
    // }

    // Ok(())
    
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("The awful story begins", frame.area());
}