#![allow(unused_variables, unused_mut, dead_code, unused_imports, unused_labels)]
use basic_project::{frame, render};
use std::{error::Error, thread, io};
use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, cursor::{Hide, Show},
event::Event, event, event::KeyCode};
use rusty_audio::Audio;
use std::time::{Duration, Instant};
use crossbeam::channel;
use basic_project::frame::drawable;
use basic_project::player::Player;

fn main() -> Result<(), Box<dyn Error>>{
    //music
    let mut audio = Audio::new();
    audio.add("kill", "kill.mp3");
    audio.add("theme", "theme.mp3");
    audio.play("kill"); //this play audio
    
    
    //terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //render loop
    let (render_tx, render_rx) = channel::unbounded();
    let handle = thread::spawn(move||{
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let mut cur_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &cur_frame, false);
            last_frame = cur_frame;
        }
    });
    //player
    let mut player = Player::new();
    let mut instant = Instant::now();

    //game
    'gameloop: loop{
        player.shoot();
        //draw
        let mut cur_frame = frame::new_frame();
        let delta = instant.elapsed();
        instant = Instant::now();
        //input
        while event::poll(Duration::default())?{
            if let Event::Key(key_event) = event::read()?{
                match key_event.code{
                    KeyCode::Left => {player.move_left();}
                    KeyCode::Right => {player.move_right();}
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        //upadate
        player.update(delta);

        //render
        player.draw(&mut cur_frame);
        let _ = render_tx.send(cur_frame);
        thread::sleep(Duration::from_millis(1));
    }


    //clean up
    drop(render_tx);
    handle.join().unwrap();
    audio.wait(); //this wait for audio to finish
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(()) //this return Ok
}
