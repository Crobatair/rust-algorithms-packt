
use crate::data::*;
use crate::store::EcsStore;
use crate::gen::GenManager;
use termion::raw::RawTerminal;
use termion::{cursor, color};

pub fn move_sys<D:EcsStore<Dir>, P: EcsStore<Pos>>(dd: &D, pp:&mut P){
  pp.for_each_mut(|g, p| {
    if let Some(d) = dd.get(g){
      p.x += d.vx;
      p.y += d.vy;
    }
  });
}

pub fn dir_sys<D:EcsStore<Dir>, P: EcsStore<Pos>>(dd: &mut D, pp:&P){
  let (w, h) = termion::terminal_size().unwrap();
  let (w, h) = ( w as i32, h as i32 );
  dd.for_each_mut(|g, dr|{
    match rand::random::<u8>()%5{
      0=> dr.vx += 1,
      1=> dr.vx -= 1,
      2=> dr.vy += 1,
      3=> dr.vy -= 1,
      _ => {}
    }

    dr.vx = std::cmp::min(3, dr.vx);
    dr.vy = std::cmp::min(3, dr.vy);
    dr.vx = std::cmp::max(-3, dr.vx);
    dr.vy = std::cmp::max(-3, dr.vy);

    if let Some(p) = pp.get(g){
      if p.x < 4 { dr.vx = 1 } 
      if p.y < 4 { dr.vy = 1 }
      if p.x + 4 > w { dr.vx = -1 }
      if p.x + 4 > h { dr.vy = -1 }
    }
  })
}

pub fn colision_sys<P: EcsStore<Pos>, S:EcsStore<Strenght>> (pp: &P, ss: &mut S) {
  let mut collisions = Vec::new();
  pp.for_each(|og, op| {
    pp.for_each(|ig, ip| {
      if (ip == op) && (ig != og) {
        collisions.push((og, ig));
      }
    });
  });

  for (og, ig) in collisions {
    let dam = match ss.get(og) {
      Some(b)=> b.s,
      None => continue,
    };

    let h_up = if let Some(bumpee) = ss.get_mut(ig){
      let n = bumpee.s + 1;
      bumpee.h -= dam;
      if bumpee.h <= 0 {
        n
      } else {
        0
      }
    } else {
      0
    };

    if h_up > 0 {
      if let Some(bumper) = ss.get_mut(og){
        bumper.h += h_up;
        bumper.s += 1;
      };
    };
  };
}


pub fn render_sys<T:std::io::Write,  P:EcsStore<Pos>, S:EcsStore<Strenght>>(t: &mut RawTerminal<T>, pp: &P, ss: &S){

  write!(t,"{}", termion::clear::All).ok();
  let (w, h) = termion::terminal_size().unwrap();
  let (w, h) = ( w as i32, h as i32 );

  pp.for_each(|g, p| {
    if let Some(st) = ss.get(g) {
      let col = match st.h {
        0 => color::Fg(color::White).to_string(),
        1 => color::Fg(color::Red).to_string(),
        2 => color::Fg(color::Yellow).to_string(),
        3 => color::Fg(color::Green).to_string(),
        _ => color::Fg(color::Blue).to_string(),
      };

      let x = (p.x % w) + 1; //
      let y = (p.y % h) + 1;

      write!(t, "{}{}{}", cursor::Goto(x as u16, y as u16), col, st.s).ok();
    }
  });
}

pub fn death_sys<S:EcsStore<Strenght>, P: EcsStore<Pos>, D:EcsStore<Dir>>(g: &mut GenManager, ss:&mut S, pp:&mut P, dd: &mut D){
  let mut to_kill = Vec::new();
  let (w, h) = termion::terminal_size().unwrap();
  let (w, h) = ( w as i32, h as i32 );
  pp.for_each(|g, p| if p.x > w || p.x < 0 || p.y > h || p.y < 0 {
    to_kill.push(g);
  });

  ss.for_each(|g, s| {
    if s.h <= 0 {
      to_kill.push(g);
    }
  });

  for tk in to_kill{
    g.drop(tk);
    pp.drop(tk);
    ss.drop(tk);
    dd.drop(tk);

  }



}