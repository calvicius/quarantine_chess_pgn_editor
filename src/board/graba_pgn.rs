extern crate gtk;

use gtk::*;

use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use std::f32;
use std::sync::mpsc;
use std::thread;

use super::tablero;


pub fn elige_fichero () -> Option<String> {
  let dialog = gtk::FileChooserDialog::with_buttons(
                  Some("Abrir Fichero"),
                  None::<&Window>,   // es el parent
                  gtk::FileChooserAction::Save,
                  &[(&"OK", gtk::ResponseType::Ok)]
              );
  let mut resul_opt: Option<String> = None;
  
  dialog.connect_key_press_event( move |widget, event_key| {
      if gdk::EventKey::get_keyval(event_key) == gdk::enums::key::Escape {
          widget.destroy(); 
          Inhibit(true);
      }
      if gdk::EventKey::get_keyval(event_key) == gdk::enums::key::Return {
          let resultado: gtk::ResponseType = gtk::ResponseType::Ok.into();
          
          if resultado == gtk::ResponseType::Ok {
              Inhibit(true);
          }
          else {
              Inhibit(false);
          }
      }
      Inhibit(false)
  });
  
  dialog.show_all();
  let result = dialog.run();
  if result == gtk::ResponseType::Ok.into() {
      let selected_file = dialog.get_filename().expect("error al seleccionar fichero");
      let f = selected_file.as_path().to_str().expect("error path_to_str").to_string();
      resul_opt = Some(f);
  }
  
  dialog.destroy();
  
  resul_opt
}


// muestra el popup alert box
pub fn alerta(msg: &str) { // note 1
    // https://gtk-rs.org/docs/gtk/struct.Dialog.html#method.new_with_buttons
    let dialog = gtk::Dialog::new_with_buttons(
                    Some("Alerta"),
                    None::<&Window>,   // es el parent
                    gtk::DialogFlags::MODAL,
                    &[(&"OK", gtk::ResponseType::Ok)]
                );
    // https://gtk-rs.org/docs/gtk/trait.GtkWindowExt.html#tymethod.set_position
    dialog.set_position(gtk::WindowPosition::CenterAlways);
    let top_area = dialog.get_content_area(); // -> Box
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 3);
    top_area.pack_start(&hbox, false, true, 3);
    
    let stock = gtk::Image::new_from_icon_name(Some("dialog-warning"), gtk::IconSize::Dialog);
    hbox.pack_start(&stock, true, true, 3);
    
    let lbl_msg = gtk::Label::new(Some(msg));
    hbox.pack_start(&lbl_msg, true, true, 3);
    
    dialog.show_all();
    dialog.run();
    dialog.destroy();
}


pub fn graba_fichero (nom_fich: &str) {
  let path = nom_fich.clone();
  
  let (tx, rx) = mpsc::channel();
  thread::spawn(move|| {
    let cab = crea_cabecera();
    tx.send(cab).unwrap();
  });
  let cabecera = rx.recv().unwrap();
  
  let (tx, rx) = mpsc::channel();
  thread::spawn(move|| {
    let p = crea_pgn();
    tx.send(p).unwrap();
  });
  let pgn = rx.recv().unwrap();
  
  let texto_grabar = format!("{}{}", cabecera, pgn);
  
  let f = File::open(&path);
  
  if f.is_err(){
    // si el archivo no está, lo creamos
    let mut f_hash = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    if let Err(e) = write!(f_hash, "{}", texto_grabar) {
      let msg = format!("No se puede abrir el fichero - {} ", e);
      alerta(&msg);
    }
  }
  else {
    let mut f_hash = OpenOptions::new()
        .write(true)
        .append(true)   // si el fichero ya esta creado añadimos otra partida
        .open(path)
        .unwrap();
    if let Err(e) = write!(f_hash, "{}", texto_grabar) {
        let msg = format!("No se puede escribir en el fichero - {} ", e);
        alerta(&msg);
    }
  }
}


fn crea_cabecera () -> String {
  let cab: tablero::Cabecera;
  unsafe {
    cab = tablero::CABECERA.clone().unwrap();
  }
  let mut cab_txt: String = String::new();
  // campos obligatorios
  let mut temp = format!("[Event \"{}\"]\n", cab.event);
  cab_txt = format!("{}{}", cab_txt, temp);
  temp = format!("[Site \"{}\"]\n", cab.site);
  cab_txt = format!("{}{}", cab_txt, temp);
  temp = format!("[Date \"{}\"]\n", cab.date);
  cab_txt = format!("{}{}", cab_txt, temp);
  temp = format!("[Round \"{}\"]\n", cab.round);
  cab_txt = format!("{}{}", cab_txt, temp);
  temp = format!("[White \"{}\"]\n", cab.white);
  cab_txt = format!("{}{}", cab_txt, temp);
  temp = format!("[Black \"{}\"]\n", cab.black);
  cab_txt = format!("{}{}", cab_txt, temp);
  temp = format!("[Result \"{}\"]\n", cab.result);
  cab_txt = format!("{}{}", cab_txt, temp);
  // campos opcionales
  temp = format!("[ECO \"{}\"]\n", cab.eco);
  cab_txt = format!("{}{}", cab_txt, temp);
  temp = format!("[WhiteElo \"{}\"]\n", cab.white_elo);
  cab_txt = format!("{}{}", cab_txt, temp);
  temp = format!("[BlackElo \"{}\"]\n", cab.black_elo);
  cab_txt = format!("{}{}", cab_txt, temp);
  if cab.fen != "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string() {
    temp = format!("[FEN \"{}\"]\n", cab.fen);
    cab_txt = format!("{}{}", cab_txt, temp);
  }
  
  // añadimos una linea en blanco
  cab_txt = format!("{}\n", cab_txt);
  
  cab_txt
}

/*
TODO
Cut to 80 chars per line
http://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c4.3
DONE
*/
fn crea_pgn () -> String {
  let mut texto_pgn: String = String::new();
  let mut arr_jugadas: Vec<String> = Vec::new();
  let gl_arr_partida: Vec<Vec<tablero::MoveT>>;
  
  unsafe {
    gl_arr_partida = tablero::JUGADAS.clone().unwrap();
  }
  
  for i in 1..gl_arr_partida[0].len() {
    let num: f32 = gl_arr_partida[0][i].num_jug.parse::<f32>().unwrap();
    let num_jugada = (num / 2.0).ceil();
    
    if arr_jugadas.len() == 0 {
      if gl_arr_partida[0][i].turno == "b".to_string(){ //han jugado las negras
        let token = format!("{}...", num_jugada);
        arr_jugadas.push(token);
      } else if gl_arr_partida[0][i].turno == "w".to_string(){ //han jugado las blancas
        let token = format!("{}.", num_jugada);
        arr_jugadas.push(token);
      }
    }
    else {
      if gl_arr_partida[0][i].turno == "w".to_string() { //han jugado las blancas
        //  arrJugadas.push(numJugada + '.');
        let token = format!("{}.", num_jugada);
        arr_jugadas.push(token);
      }
      if gl_arr_partida[0][i].turno == "b".to_string() && 
                arr_jugadas[arr_jugadas.len() - 1] == ")".to_string(){
        //  arrJugadas.push(parseInt(numJugada).toString() + '. ...');
        let token = format!("{}...", num_jugada);
        arr_jugadas.push(token);
      }
    }
    
    arr_jugadas.push(gl_arr_partida[0][i].san.clone());
    if gl_arr_partida[0][i].nag.len() > 0 {
        arr_jugadas.push(gl_arr_partida[0][i].nag.clone());
    }
    if gl_arr_partida[0][i].comen.len() > 0 {
      //  arrJugadas.push('{' + gl_arrPartida[0][i].comen + '}');
      //arr_jugadas.push(gl_arr_partida[0][i].comen.clone());
      // para poder hacer cada linea del pgn de 80 cars. aprox.
      let com: Vec<String> = gl_arr_partida[0][i].comen
          .split(" ")
          .map(|s| s.to_string())
          .collect();
      for elem in com {
        arr_jugadas.push(elem);
      }
    }
    
    for j in 0..gl_arr_partida[0][i].sub_var.len() {
      let mut arr_var = crea_var_pgn(gl_arr_partida[0][i].sub_var[j], &gl_arr_partida);
      arr_jugadas.append(&mut arr_var);
    }
  }
  
  let mut linea: String = String::new();
  let mut vec_lineas: Vec<String> = Vec::new();
  for mov in arr_jugadas {
    if mov == "(".to_string() {
      let temp = format!("{}", mov);
      linea.push_str(temp.as_str());
    }
    else if mov == ")".to_string() {
      // el anterior caracter siempre es un espacio
      if linea.len() > 0 {
        let _espacio = linea.pop().expect("error en pop crea_pgn");
        let temp = format!("{} ", mov);
        linea.push_str(temp.as_str());
      }
    }
    else {
      // añadimos un espacio despues del movimiento
      let temp = format!("{} ", mov);
      linea.push_str(temp.as_str());
    }
    
    if linea.len() > 70 {
      // el anterior caracter siempre es un espacio.
      // lo eliminamos y añadimos un salto de linea
      let _espacio = linea.pop().expect("error en pop crea_pgn");
      linea.push_str("\n");
      vec_lineas.push(linea.clone());
      linea.truncate(0);  // la dejamos en longitud cero
    }
  }
  
  if linea.len() == 0 {
    // la linea anterior ha terminado con \nag
    vec_lineas.push("\n".to_string());
  }
  else {
    // el ultimo caracter es un espacio
    let _espacio = linea.pop().expect("error en pop crea_pgn");
    // al final hay que dejar una linea en blanco
    linea.push_str("\n\n");
    vec_lineas.push(linea.clone());
  }
  
  for lin in vec_lineas {
    texto_pgn = format!("{}{}", texto_pgn, lin);
  }
  //print!("{}", texto_pgn);
  //println!("---");
  texto_pgn
}


fn crea_var_pgn (num_var: usize, 
        gl_arr_partida: &Vec<Vec<tablero::MoveT>>) -> Vec<String> {
  
  let mut idx = num_var;
  let mut arr_jugadas: Vec<String> = Vec::new();
  let mut anadida_jug: bool = false;
  
  // tomamos el elem[0] que es igual a '(' 
  arr_jugadas.push(gl_arr_partida[idx][0].san.clone());
  
  for i in 1..gl_arr_partida[idx].len() {
    let mut num_jugada: i32 = 0;
    let mut turno:String = "".to_string();
    let v_fen = gl_arr_partida[idx][i].fen.clone();
    
    if gl_arr_partida[idx][i].san != ")".to_string() {
      let arr_fen: Vec<String> = v_fen.split(" ").map(|s| s.to_string()).collect();
      num_jugada = arr_fen[5].parse::<i32>().unwrap();
      turno = arr_fen[1].clone();
    }
    
    if !anadida_jug{
      if turno == "w".to_string() { //han jugado las negras
        //  arrJugadas.push(parseInt(numJugada - 1).toString() + '. ...');
        let token = format!("{}...", num_jugada - 1);
        arr_jugadas.push(token);
      } else if turno == "b".to_string() { //han jugado las blancas
          //arrJugadas.push(numJugada + '.');
        let token = format!("{}.", num_jugada);
        arr_jugadas.push(token);
      }
      anadida_jug = true;
    }
    else if arr_jugadas.len() > 1 {
      if turno == "b".to_string() { //han jugado las blancas
        //  arrJugadas.push(numJugada + '.');
        let token = format!("{}.", num_jugada);
        arr_jugadas.push(token);
      }
      if turno == "w".to_string() && 
            (arr_jugadas[arr_jugadas.len()-1] == ")".to_string() || 
                arr_jugadas[arr_jugadas.len()-1] == "(".to_string()){
        //arrJugadas.push(parseInt(numJugada - 1).toString() + '. ...');
        let token = format!("{}...", num_jugada - 1);
        arr_jugadas.push(token);
      }
    }
    
    arr_jugadas.push(gl_arr_partida[idx][i].san.clone());
    if gl_arr_partida[idx][i].nag.len() > 0 {
      arr_jugadas.push(gl_arr_partida[idx][i].nag.clone());
    }
    if gl_arr_partida[idx][i].comen.len() > 0 {
      //arr_jugadas.push(gl_arr_partida[idx][i].comen.clone());
      // para poder hacer cada linea del pgn de 80 cars. aprox.
      let com: Vec<String> = gl_arr_partida[idx][i].comen
          .split(" ")
          .map(|s| s.to_string())
          .collect();
      for elem in com {
        arr_jugadas.push(elem);
      }
    }
    
    // ind_viejo es una variable para recuperar el num variante idx una vez
    // retorne de la sub- ... - sub-subvariante
    let ind_viejo = idx.clone();
    for j in 0..gl_arr_partida[idx][i].sub_var.len() {
        let hijo = gl_arr_partida[idx][i].sub_var[j];
        let mut arr_sub_var = crea_var_pgn(hijo, &gl_arr_partida);
        
        arr_jugadas.append(&mut arr_sub_var);
    }
    idx = ind_viejo;
  }
  
  
  arr_jugadas
}