extern crate gio;
//extern crate gtk;
//extern crate gdk;
//extern crate glib;

use board::uci::gio::prelude::*;
use gtk::prelude::*;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

use super::ajedrez;


#[derive(Clone)]
pub struct Engine {
  motor: gio::Subprocess,
  visor: gtk::TextView,
  pub scrolled_win: gtk::ScrolledWindow,
  start_fen: String
}


impl Engine {
  pub fn init (path: &str) -> Self {
    let os_string = std::ffi::OsString::from(path);
    let os_str = std::ffi::OsStr::new(&os_string); // -> &OsStr
    let p = gio::Subprocess::newv(&[os_str], 
            gio::SubprocessFlags::STDIN_PIPE |
            gio::SubprocessFlags::STDOUT_PIPE |
            gio::SubprocessFlags::STDERR_PIPE)
            .expect("error al crear el subproceso Engine");
    
    let v_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let sw = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    sw.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic); 
    
    let text_view = gtk::TextView::new();
    sw.add(&text_view);
    let buffer = text_view.get_buffer().expect("error al crear el buffer");
    buffer.set_text("Press <ESC> to stop calculations\n");
    
    Engine {
      motor: p,
      visor: text_view,
      scrolled_win: sw,
      start_fen: v_fen
    }
  }
  
  pub fn get_handshake (&mut self) {
    let vec_gu8 = self.motor.get_stdout_pipe().unwrap()
        .read_bytes(1024, None::<&gio::Cancellable>).unwrap();
    let uci_response = self.read_left_output_no_moves(vec_gu8);
    let buffer = self.visor.get_buffer()
        .expect("error al crear el buffer");
    for elem in uci_response {
      let mut iter = buffer.get_end_iter();
      buffer.insert(&mut iter, &elem);
    }
    self.visor.show_all();
  }
  
  
  pub fn get_is_ready (&mut self) -> String {
    let mut retorno: String = String::new();
    let s = b"isready\n";
    let _i = self.motor.get_stdin_pipe().unwrap()
        .write_all(s, None::<&gio::Cancellable>).unwrap();
    let vec_gu8 = self.motor.get_stdout_pipe().unwrap()
        .read_bytes(1024, None::<&gio::Cancellable>).unwrap();
    let uci_response = self.read_left_output_no_moves(vec_gu8);
    let buffer = self.visor.get_buffer()
        .expect("error al crear el buffer");
    for elem in uci_response {
      let mut iter = buffer.get_end_iter();
      buffer.insert(&mut iter, &elem);
      retorno = elem;
    }
    self.visor.show_all();
    retorno
  }
  
  
  pub fn get_uci_options (&mut self) {
    let s = b"uci\n";
    let _i = self.motor.get_stdin_pipe().unwrap()
        .write_all(s, None::<&gio::Cancellable>).unwrap();
    let vec_gu8 = self.motor.get_stdout_pipe().unwrap()
        .read_bytes(4096, None::<&gio::Cancellable>).unwrap();
    let uci_response = self.read_left_output_no_moves(vec_gu8);
    let buffer = self.visor.get_buffer()
        .expect("error al crear el buffer");
    for elem in uci_response {
      let mut iter = buffer.get_end_iter();
      buffer.insert(&mut iter, &elem);
    }
    self.visor.show_all();
  }
  
  
  pub fn set_options (&mut self, opt: &[u8]) {
    let _i = self.motor.get_stdin_pipe().unwrap()
        .write_all(opt, None::<&gio::Cancellable>).unwrap();
    // here there is no output from engine
  }
  
  
  pub fn set_initial_pos (&mut self) {
    self.start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let pos = "position startpos\n".as_bytes();
    let _i = self.motor.get_stdin_pipe().unwrap()
        .write_all(pos, None::<&gio::Cancellable>).unwrap();
    // here there is no output from engine
  }
  
  
  pub fn set_initial_pos_with_moves (&mut self, moves: &str) {
    // we need the fen after moves
    let mov = moves.clone();
    let mov_vec: Vec<String> = mov.split(" ").map(|s| s.to_string()).collect();
    let mut board = ajedrez::Tablero::init();
    let _fen_valida = ajedrez::setup_inicio(&mut board);
    
    for elem in mov_vec {
      let char_vec: Vec<char> = elem.chars().collect();
      let m1 = format!("{}{}", char_vec[0], char_vec[1]);
      let m2 = format!("{}{}", char_vec[2], char_vec[3]);
      let mut move_prom = "".to_string();
      let move_from = m1.as_str();
      let move_to = m2.as_str();
      
      if char_vec.len() == 5 {
        let move_prom = char_vec[4].clone().to_uppercase().to_string();
      }
      
      let movim = (move_from, move_to, move_prom.as_str());
      let resul = ajedrez::mueve_algebra(&mut board, movim);
    }
    self.start_fen = ajedrez::get_fen(&mut board);
    // ... and now we go to the engine
    let p = format!("position startpos moves {}\n", moves);
    let pos = p.clone();
    let pos = pos.as_str().as_bytes();
    let _i = self.motor.get_stdin_pipe().unwrap()
        .write_all(pos, None::<&gio::Cancellable>).unwrap();
    // here there is no output from engine
  }
  
  
  pub fn set_stop (&self) {
    let pos = "stop\n".as_bytes();
    let _i = self.motor.get_stdin_pipe().unwrap()
        .write_all(pos, None::<&gio::Cancellable>).unwrap();
    // here there is no output from engine
  }
  
  pub fn set_pos_fen (&mut self, fen: &str) {
    self.start_fen = fen.to_string();
    let p = format!("position fen {}\n", fen);
    let pos = p.clone();
    let pos = pos.as_str().as_bytes();
    let _i = self.motor.get_stdin_pipe().unwrap()
        .write_all(pos, None::<&gio::Cancellable>).unwrap();
    // here there is no output from engine
  }
  
  
  pub fn go_depth(&mut self, depth: i32) {
    let p = format!("go depth {}\n", depth);
    let pos = p.clone();
    let pos = pos.as_str().as_bytes();
    let _i = self.motor.get_stdin_pipe().unwrap()
        .write_all(pos, None::<&gio::Cancellable>).unwrap();
    self.write_moves_stdout();
  }
  
  
  pub fn go_infinite(&mut self) {
    let pos = b"go infinite\n";
    let _i = self.motor.get_stdin_pipe().unwrap()
        .write_all(pos, None::<&gio::Cancellable>).unwrap();
    
    self.write_moves_stdout();
  }
  
  
  pub fn go_by_time (&mut self, movetime: i32) {
    let p = format!("go movetime {}\n", movetime);
    let pos = p.clone();
    let pos = pos.as_str().as_bytes();
    let _i = self.motor.get_stdin_pipe().unwrap()
        .write_all(pos, None::<&gio::Cancellable>).unwrap();
    self.write_moves_stdout();
  }
  
  fn write_moves_stdout (&mut self) {
    loop {
      let vec_gu8 = self.motor.get_stdout_pipe().unwrap()
            .read_bytes(1024, None::<&gio::Cancellable>).unwrap();
      let fin = self.read_left_output(vec_gu8);
      
      //thread::sleep(Duration::from_millis(100));
      if fin {
        break
      }
    }
  }
  
  
  fn read_left_output_no_moves(&mut self, datos: glib::Bytes) -> Vec<String> {
    let mut s = String::new();
    let vec_u8 = std::ops::Deref::deref(&datos);
    let mut salida: Vec<String> = Vec::new();
    
    for i in 0..vec_u8.len() {
      s.push(vec_u8[i] as char);
      if vec_u8[i] as char == '\n' {
        salida.push(s.clone());
        s.clear();
      }
    }
    salida
  }
  
  
  
  fn read_left_output(&mut self, datos: glib::Bytes) -> bool {
    let v_fen = self.start_fen.clone();
    let mut s = String::new();
    let vec_u8 = std::ops::Deref::deref(&datos);
    let mut fin_analisis = false;
    let buffer = self.visor.get_buffer()
              .expect("error al crear el buffer");
    
    for i in 0..vec_u8.len() {
      s.push(vec_u8[i] as char);
      if vec_u8[i] as char == '\n' {
        let fen_clon = v_fen.clone();
        //sync thread
        let (tx, rx) = mpsc::channel();
        let s_clon = s.clone();
        thread::spawn(move || {
          let linea = write_pretty (s_clon, fen_clon);
          tx.send(linea).unwrap();
        });
        let linea = rx.recv().unwrap();
        if linea.contains("Mate") {
          self.set_stop();
          thread::sleep(Duration::from_millis(100));
        }
        if linea.len() > 2 {
          let mut iter = buffer.get_end_iter();
          buffer.insert(&mut iter, &linea);
          self.visor.show_all();
          // scroll to bottom
          let adj = self.scrolled_win.get_vadjustment().unwrap();
          adj.set_value(adj.get_upper());
          self.scrolled_win.set_vadjustment(Some(&adj));
          // refresh screen
          while gtk::events_pending() {
            gtk::main_iteration();
          }
          if linea.contains("Mejor") {  // || linea.contains("Mate") {
            fin_analisis = true;
            break
          }
        }
        
        s.clear();
      }
    }
    
    fin_analisis
  }
}


fn write_pretty(linea: String, v_fen: String) -> String {
  let arr_fen: Vec<String> = v_fen.split(" ").map(|s| s.to_string()).collect();
  let mut num_mov = 0;
  
  // get the SAN notation
  let mut board = ajedrez::Tablero::init();
  let _fen_valida = ajedrez::set_fen(v_fen.as_str(), &mut board);
  
  let lin_vec: Vec<String> = linea.split(" ").map(|s| s.to_string()).collect();
  if lin_vec[0] == "bestmove" {
    let char_vec: Vec<char> = lin_vec[1].chars().collect();
    let m1 = format!("{}{}", char_vec[0], char_vec[1]);
    let m2 = format!("{}{}", char_vec[2], char_vec[3]);
    let mut move_prom = "".to_string();
    let move_from = m1.as_str();
    let move_to = m2.as_str();
    
    if char_vec.len() == 5 {
      let move_prom = char_vec[4].clone().to_uppercase().to_string();
    }
    
    let movim = (move_from, move_to, move_prom.as_str());
    let resul = ajedrez::mueve_algebra(&mut board, movim);
    if arr_fen[1] == "w" {
      return format!("Mejor jugada:\t1. {}\n", resul.0); //&lin_vec[1]);
    }
    else {
      return format!("Mejor jugada:\t1... {}\n", resul.0); //&lin_vec[1]);
    }
  }
  else if lin_vec[0] == "info" {
    let mut retorno = String::new();
    let mut movs = false;
    for i in 1..lin_vec.len() {
      if lin_vec[i-1] == "depth" {
        retorno = format!("Depth: {} ", &lin_vec[i]);
      }
      if lin_vec[i-1] == "cp" {
        let eval = lin_vec[i].parse::<f32>().unwrap() / 100.0;
        retorno = format!("{}\tEval.:\t{:>6.2} ", retorno, eval);
      }
      if lin_vec[i-1] == "mate" {
        retorno = format!("{}\tMate:\t{:>6} ", retorno, lin_vec[i]);
      }
      if lin_vec[i-1] == "pv" {
        retorno = format!("{}\tMoves.: ", retorno);
        num_mov += 1;
        movs = true;
      }
      if movs {
        let s = &lin_vec[i];
        let char_vec: Vec<char> = s.chars().collect();
        let m1 = format!("{}{}", char_vec[0], char_vec[1]);
        let m2 = format!("{}{}", char_vec[2], char_vec[3]);
        let mut move_prom = "".to_string();
        let move_from = m1.as_str();
        let move_to = m2.as_str();
        
        if char_vec.len() == 5 {
          let move_prom = char_vec[4].clone().to_uppercase().to_string();
        }
        
        let movim = (move_from, move_to, move_prom.as_str());
        let resul = ajedrez::mueve_algebra(&mut board, movim);
        num_mov += 1;
        if num_mov % 2 == 0 {
          if arr_fen[1] == "b" && num_mov == 2 {
            retorno = format!("{} {}... {} ", retorno, num_mov/2, resul.0);
            num_mov += 1;
          }
          else {
            retorno = format!("{} {}. {} ", retorno, num_mov/2, resul.0);
          }
        }
        else {
          retorno = format!("{} {} ", retorno, resul.0);
        }
      }
    }
    
    if movs || retorno.contains("Mate") {
      retorno = retorno.trim().to_string();
      retorno = format!("{}\n", retorno);
      return retorno;
    }
  }
  // the other possibilities are missed
  "".to_string()
}




/*
http://wbec-ridderkerk.nl/html/UCIProtocol.html

Step 1: Initiate your chess engine executable in command line
Step 2: Type: isready (this step isn't necessary for stockfish, 
        but some engines do (e.g. Discocheck and Quazar)
    Output: readyok
Step 3: Type 'uci'
    The Output, should provide the engine ID, version number, and author information, 
        followed by a list of all supported uci options, such as Hash, Threads, MultiPV, Ponder, etc...
    This also shows you the default setting for each parameter
    The uci string always ends on a newline 'uciok'
    Sample output from stockfish 10 for reference: here

Step 4: How to change a supported UCI Option (Generic Formula)
    setoption name [supported uci option] value [value you want to change it to]
        e.g. to change hash size to 1024 MB and use 2 threads, type the following 
        into commandline:
    setoption name hash value 1024
    setoption name threads value 2
    *Note: that the option name is case insensitive, so you could write instead : 
        setoption name HaSh value 1024, and get the same results


Step 5: Set or change the position
    How to set the Starting Position
        position startpos
    How to Move (e.g. move pawn to e4 from starting position)
        position startpos moves e2e4
            Note that you must use uci notation (a variant of long algebraic notation) 
              of moves which only includes the square it comes from and square it goes to;
            In order to castle kingside, you must use the notation e1g1 (or e8g8), 
              to castle queenside : e1c1 (e8c8), e7e8q (for promotion)
    How to set a Position with a specific fen string
        position fen [fen string here]
            e.g. change position to this 
              fen : 4kb1r/p2rqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/2K4R w k - 0 14
            position fen 4kb1r/p2rqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/2K4R w k - 0 14
        How to make a move from a specific fen position (using above example)
            position fen 4kb1r/p2rqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/2K4R w k - 0 14 moves h1d1


Step 6: Search / Analysis... Type 'go', followed by any number of commands:
    infinite
    depth [ply depth]
    movetime [time in ms]
    Note: there are other options available, but they really aren't useful without 
        a gui (such as setting movestogo, winc, binc)

*/
