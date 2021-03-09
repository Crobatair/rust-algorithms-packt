
mod gen;
mod store;
mod data;
mod systems;

use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use store::EcsStore;
use std::io::Write;

fn main() {

    // created a channel to recive inputs
    let ( ch_s, ch_r ) = std::sync::mpsc::channel();
    
    // spwaned a new thread
    // when a key is pressed from stdin,
    // used channel sender to send the key pressed, if many pressed will send eachone on new send event
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        for k in stdin.keys() {
            ch_s.send(k).ok();
        }
    });

    let (w, h) = termion::terminal_size().unwrap();
    let (w, h) = ( w as i32, h as i32 );


    let mut screen = std::io::stdout().into_raw_mode().unwrap();
    let mut gen = gen::GenManager::new();
    let mut strengths = store::VecStore::new();
    let mut dirs = store::VecStore::new();
    let mut poss = store::VecStore::new();
    let mut pass = 0;


    loop{

        // create an entity to create one element for each loop
        let g = gen.next();
        strengths.add(g, data::Strenght{s:1, h:5});
        dirs.add(g, data::Dir{vx:0, vy:0});
        poss.add(g, data::Pos{x: (rand::random::<i32>()%w) as i32, y: (rand::random::<i32>()%h) as i32});

        systems::dir_sys(&mut dirs, &poss);
        systems::move_sys(&dirs, &mut poss);
        systems::colision_sys(&poss, &mut strengths);
        systems::death_sys(&mut gen, &mut strengths, &mut poss, &mut dirs);
        systems::render_sys(&mut screen, &poss, &strengths);


        // write to (1, 1) how many passes
        // flush writter
        write!(&mut screen, "{} Pass = {}", termion::cursor::Goto(1,1), pass).ok();
        pass += 1;
        screen.flush().ok();

        // to handle each event recive
        while let Ok(Ok(k)) = ch_r.try_recv(){
            match k {
                Key::Char('q')=> return,
                // handle any key presses related to moves
                _ =>{},
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    }



}
