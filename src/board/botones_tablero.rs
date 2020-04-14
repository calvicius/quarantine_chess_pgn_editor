use gtk::*;

use super::tablero;
use super::ajedrez;
use super::tablero_interno;
use super::presenta_jugadas;
use super::progreso_carga;


pub fn gira_tablero (_widget: &gtk::Button,
            area_tablero: &gtk::DrawingArea) {
  
  {
    let mut obj_tab = tablero::VAR_TABLERO.write().unwrap();
    obj_tab.flipped = !obj_tab.flipped;
    let mut board = ajedrez::Tablero::init();
    let fen = obj_tab.fen_actual.clone();
    let fen_str = fen.as_str();
    let fen_valida = ajedrez::set_fen(fen_str, &mut board);
    obj_tab.turno = board.to_move;
    if fen_valida {
      let grafico = ajedrez::tablero_grafico(&mut board);
      let tablero_interno = tablero_interno::procesa_notacion(grafico, obj_tab.flipped);
      obj_tab.tablero_interno = tablero_interno;
    }
  }
  area_tablero.queue_draw();
}


pub fn jugada_anterior (_widget: &gtk::Button,
            area_tablero: &gtk::DrawingArea,
            visor_partida: &gtk::TextView) {
  
  let textbuffer = visor_partida.get_buffer().expect("error");
  let gl_jugada_actual: tablero::MoveT;
  let gl_arr_partida: Vec<Vec<tablero::MoveT>>;
  
  unsafe {
    gl_jugada_actual = tablero::JUGADA_ACTUAL.clone().unwrap();
    gl_arr_partida = tablero::JUGADAS.clone().unwrap();
  }
  let indice: Vec<String> = gl_jugada_actual.idx_jug
          .split("Var")
          .map(|s| s.to_string())
          .collect();
  let arr_idx: Vec<String> = indice[1]
          .split("Mv")
          .map(|s| s.to_string())
          .collect();
  let num_var = arr_idx[0].parse::<usize>().unwrap();
  let index = gl_arr_partida[num_var]
          .iter()
          .position(|r| r.idx_jug == gl_jugada_actual.idx_jug)
          .unwrap();
  if (index-1) < 1 {
    return;
  }
  let jugada_anterior = gl_arr_partida[num_var][index - 1].clone();
  // si no hay un iterador activo 
  // de otro modo no estamos sobre ninguna jugada activa
  unsafe {
    let iterador = tablero::ITER_CURSOR.clone();
    match iterador {
      Some(_iterador) => {},
      None => {
        return;
      },
    };
  }
  // vamos a limpiar los tagas con color de fondo_amarillo
  let mut iterador: gtk::TextIter;
  unsafe {
    let iters = tablero::ITERS_FONDO_TAG.clone();
    match iters {
      Some(iters) => {
          textbuffer.remove_tag_by_name("fondo_amarillo", &iters.0, &iters.1);
        },
      None => {},
    };
    iterador = tablero::ITER_CURSOR.clone().expect("error linea 69 botone_tablero");
    tablero::JUGADA_ACTUAL = Some(jugada_anterior.clone());
  }
  
  // calculamos las variantes y subvariantes precedentes
  // para iterar sobre sus cierres ")"
  let (num_subvar, num_movims) = presenta_jugadas::calcula_parentesis (jugada_anterior.clone());
  
  if num_subvar == 0 {
    if let Some(resul) = gtk::TextIter::backward_search(&mut iterador,
                    "\t",
                    gtk::TextSearchFlags::VISIBLE_ONLY,
                    None) {
      iterador = resul.0.clone();
      
      if let Some(resul_1) = gtk::TextIter::backward_search(&mut iterador,
                      " ",
                      gtk::TextSearchFlags::VISIBLE_ONLY,
                      None) {
        iterador = resul_1.0.clone();
        if let Some(resul_2) = gtk::TextIter::backward_search(&mut iterador,
                      " ",
                      gtk::TextSearchFlags::VISIBLE_ONLY,
                      None) {
          textbuffer.apply_tag_by_name("fondo_amarillo", &resul_2.1, &resul_1.0);
          iterador = resul_2.1.clone();
          unsafe {
            tablero::ITERS_FONDO_TAG = Some((resul_2.1, resul_1.0));
            tablero::ITER_CURSOR = Some(iterador.clone());
          }
        }
      }
      else {
        return;
      }
    }
    else {
      return;
    }
  }
  else {
    for _ in 0..=num_movims {
      let resul = gtk::TextIter::backward_search(&mut iterador,
                      "\t",
                      gtk::TextSearchFlags::VISIBLE_ONLY,
                      None).unwrap();  // -> Option<(textiter, textiter)>
      iterador = resul.0.clone();
      unsafe {
        tablero::ITER_CURSOR = Some(iterador.clone());
      }
    }
    
    let resul = gtk::TextIter::backward_search(&mut iterador,
                    " ",
                    gtk::TextSearchFlags::VISIBLE_ONLY,
                    None).unwrap();
    iterador = resul.0.clone();
    let resul_1 = gtk::TextIter::backward_search(&mut iterador,
                    " ",
                    gtk::TextSearchFlags::VISIBLE_ONLY,
                    None).unwrap();
    //let clon_inicio = iter_inicio.clone();
    textbuffer.apply_tag_by_name("fondo_amarillo", &resul_1.1, &resul.0);
    unsafe {
      tablero::ITERS_FONDO_TAG = Some((resul_1.1, resul.0));
    }
  }
  
  // ahora llevamos la fen al tablero
  presenta_jugadas::muestra_arg_link(&jugada_anterior.fen,
                area_tablero);
}


pub fn jugada_primera (_widget: &gtk::Button,
            area_tablero: &gtk::DrawingArea,
            visor_partida: &gtk::TextView) {
  
  // si no hay un iterador activo 
  // de otro modo no estamos sobre ninguna jugada activa
  let mut iterador: gtk::TextIter;
  unsafe {
    iterador = match tablero::ITER_CURSOR.clone() {
      Some(iterador) => iterador,
      None => return,
    };
  }
  // vamos a limpiar los tagas con color de fondo_amarillo
  let textbuffer = visor_partida.get_buffer().expect("error");
  let gl_jugada_actual: tablero::MoveT;
  let gl_arr_partida: Vec<Vec<tablero::MoveT>>;
  
  unsafe {
    gl_jugada_actual = tablero::JUGADA_ACTUAL.clone().unwrap();
    gl_arr_partida = tablero::JUGADAS.clone().unwrap();
  }
  let indice: Vec<String> = gl_jugada_actual.idx_jug
          .split("Var")
          .map(|s| s.to_string())
          .collect();
  let arr_idx: Vec<String> = indice[1]
          .split("Mv")
          .map(|s| s.to_string())
          .collect();
  let num_var = arr_idx[0].parse::<usize>().unwrap();
  let index = gl_arr_partida[num_var]
          .iter()
          .position(|r| r.idx_jug == gl_jugada_actual.idx_jug)
          .unwrap();
  if (index-1) < 1 {
    return;
  }
  else {
    unsafe {
      let iters = tablero::ITERS_FONDO_TAG.clone();
      match iters {
        Some(iters) => {
            textbuffer.remove_tag_by_name("fondo_amarillo", &iters.0, &iters.1);
          },
        None => {},
      };
    }
    //let num_subvar: i32;
    let mut num_movims: i32 = 0;
    for i in (1..index).rev() {
      let (_v, m) = presenta_jugadas::calcula_parentesis (gl_arr_partida[num_var][i].clone());
      num_movims += m;
    }
    let total_movims = num_movims + (index -1) as i32;
    
    for _ in 0..total_movims {
      let resul = gtk::TextIter::backward_search(&mut iterador,
                      "\t",
                      gtk::TextSearchFlags::VISIBLE_ONLY,
                      None).unwrap();  // -> Option<(textiter, textiter)>
      iterador = resul.0.clone();
      unsafe {
        tablero::ITER_CURSOR = Some(iterador.clone());
      }
    }
    
    if let Some(resul) = gtk::TextIter::backward_search(&mut iterador,
                    " ",
                    gtk::TextSearchFlags::VISIBLE_ONLY,
                    None) {
      iterador = resul.0.clone();
      if let Some(resul_1) = gtk::TextIter::backward_search(&mut iterador,
                      " ",
                      gtk::TextSearchFlags::VISIBLE_ONLY,
                      None) {
        textbuffer.apply_tag_by_name("fondo_amarillo", &resul_1.1, &resul.0);
        unsafe {
          tablero::ITERS_FONDO_TAG = Some((resul_1.1, resul.0));
          tablero::JUGADA_ACTUAL = Some(gl_arr_partida[num_var][1].clone());
        }
      }
      else { return; }
    }
    else { return; }
  }
  
  // ahora llevamos la fen al tablero
  presenta_jugadas::muestra_arg_link(&gl_arr_partida[num_var][1].fen,
                area_tablero);
}



pub fn jugada_siguiente (_widget: &gtk::Button,
            area_tablero: &gtk::DrawingArea,
            visor_partida: &gtk::TextView) {
  
  let textbuffer = visor_partida.get_buffer().expect("error");
  let gl_jugada_actual: tablero::MoveT;
  let gl_arr_partida: Vec<Vec<tablero::MoveT>>;
  
  unsafe {
    gl_jugada_actual = tablero::JUGADA_ACTUAL.clone().unwrap();
    gl_arr_partida = tablero::JUGADAS.clone().unwrap();
  }
  let indice: Vec<String> = gl_jugada_actual.idx_jug
          .split("Var")
          .map(|s| s.to_string())
          .collect();
  let arr_idx: Vec<String> = indice[1]
          .split("Mv")
          .map(|s| s.to_string())
          .collect();
  let num_var = arr_idx[0].parse::<usize>().unwrap();
  let index = gl_arr_partida[num_var]
          .iter()
          .position(|r| r.idx_jug == gl_jugada_actual.idx_jug)
          .unwrap();
  if (index + 1) > gl_arr_partida[num_var].len() - 2 {
    return;
  }
  let jugada_siguiente = gl_arr_partida[num_var][index + 1].clone();
  // si no hay un iterador activo 
  // de otro modo no estamos sobre ninguna jugada activa
  unsafe {
    let iterador = tablero::ITER_CURSOR.clone();
    match iterador {
      Some(_iterador) => {},
      None => {
        return;
      },
    };
  }
  // vamos a limpiar los tagas con color de fondo_amarillo
  let mut iterador: gtk::TextIter;
  unsafe {
    let iters = tablero::ITERS_FONDO_TAG.clone();
    match iters {
      Some(iters) => {
          textbuffer.remove_tag_by_name("fondo_amarillo",&iters.0, &iters.1);
        },
      None => {},
    };
    iterador = tablero::ITER_CURSOR.clone()
          .expect("error linea 69 botone_tablero");
    tablero::JUGADA_ACTUAL = Some(jugada_siguiente.clone());
  }
  
  // calculamos las variantes y subvariantes precedentes
  // para iterar sobre sus cierres ")"
  let (num_subvar, num_movims) = 
        presenta_jugadas::calcula_parentesis (gl_arr_partida[num_var][index].clone());
  
  if num_subvar == 0 {
    if let Some(resul) = gtk::TextIter::forward_search(&mut iterador,
                    "\t",
                    gtk::TextSearchFlags::VISIBLE_ONLY,
                    None) { // -> Option<(textiter, textiter)>
      iterador = resul.1.clone();
      unsafe {
        tablero::ITER_CURSOR = Some(iterador.clone());
      }
      if let Some(resul_1) = gtk::TextIter::forward_search(&mut iterador,
                      " ",
                      gtk::TextSearchFlags::VISIBLE_ONLY,
                      None) {
        iterador = resul_1.1.clone();
        if let Some(resul_2) = gtk::TextIter::forward_search(&mut iterador,
                      " ",
                      gtk::TextSearchFlags::VISIBLE_ONLY,
                      None) {
          textbuffer.apply_tag_by_name("fondo_amarillo", &resul_1.1, &resul_2.0);
          unsafe {
            tablero::ITERS_FONDO_TAG = Some((resul_1.1, resul_2.0));
          }
        }
        else { return; }
      }
      else {
        return;
      }
    }
    else {
      return;
    }
  }
  else {
    for _ in 0..=num_movims {
      let resul = gtk::TextIter::forward_search(&mut iterador,
                      "\t",
                      gtk::TextSearchFlags::VISIBLE_ONLY,
                      None).unwrap();  // -> Option<(textiter, textiter)>
      iterador = resul.1.clone();
      unsafe {
        tablero::ITER_CURSOR = Some(iterador.clone());
      }
    }
    
    let resul = gtk::TextIter::forward_search(&mut iterador,
                    " ",
                    gtk::TextSearchFlags::VISIBLE_ONLY,
                    None).unwrap();
    iterador = resul.1.clone();
    let resul_1 = gtk::TextIter::forward_search(&mut iterador,
                    " ",
                    gtk::TextSearchFlags::VISIBLE_ONLY,
                    None).unwrap();
    //let clon_inicio = iter_inicio.clone();
    textbuffer.apply_tag_by_name("fondo_amarillo", &resul.1, &resul_1.0);
    unsafe {
      tablero::ITERS_FONDO_TAG = Some((resul.1, resul_1.0));
    }
  }
  
  // ahora llevamos la fen al tablero
  presenta_jugadas::muestra_arg_link(&jugada_siguiente.fen,
                area_tablero);
}



pub fn jugada_ultima (_widget: &gtk::Button,
            area_tablero: &gtk::DrawingArea,
            visor_partida: &gtk::TextView) {
  
  let textbuffer = visor_partida.get_buffer().expect("error");
  let gl_jugada_actual: tablero::MoveT;
  let gl_arr_partida: Vec<Vec<tablero::MoveT>>;
  
  unsafe {
    gl_jugada_actual = tablero::JUGADA_ACTUAL.clone().unwrap();
    gl_arr_partida = tablero::JUGADAS.clone().unwrap();
  }
  let indice: Vec<String> = gl_jugada_actual.idx_jug
          .split("Var")
          .map(|s| s.to_string())
          .collect();
  let arr_idx: Vec<String> = indice[1]
          .split("Mv")
          .map(|s| s.to_string())
          .collect();
  let num_var = arr_idx[0].parse::<usize>().unwrap();
  let index = gl_arr_partida[num_var]
          .iter()
          .position(|r| r.idx_jug == gl_jugada_actual.idx_jug)
          .unwrap();
  
  // si no hay un iterador activo 
  // de otro modo no estamos sobre ninguna jugada activa
  let mut iterador: gtk::TextIter;
  unsafe {
    iterador = match tablero::ITER_CURSOR.clone() {
      Some(iterador) => iterador,
      None => return,
    };
  }
  
  if (index + 1) > gl_arr_partida[num_var].len() - 2 {
    return;
  }
  else {
    // vamos a limpiar los tagas con color de fondo_amarillo
    unsafe {
      let iters = tablero::ITERS_FONDO_TAG.clone();
      match iters {
        Some(iters) => {
            textbuffer.remove_tag_by_name("fondo_amarillo", &iters.0, &iters.1);
          },
        None => {},
      };
    }
    
    let mut num_movims: i32 = 0;
    for i in index..gl_arr_partida[num_var].len() {
      let (_v, m) = presenta_jugadas::calcula_parentesis (gl_arr_partida[num_var][i].clone());
      num_movims += m;
    }
    // vamos a ver si hay subvariantes en la ultima jugada
    let (_v, m) = presenta_jugadas::calcula_parentesis (gl_arr_partida[num_var][gl_arr_partida[num_var].len() - 2].clone());
    let total_movims = num_movims + (gl_arr_partida[num_var].len() - 2 - index) as i32;
    
    for _ in 0..total_movims - m {
      let resul = gtk::TextIter::forward_search(&mut iterador,
                      "\t",
                      gtk::TextSearchFlags::VISIBLE_ONLY,
                      None).unwrap();  // -> Option<(textiter, textiter)>
      iterador = resul.1.clone();
      unsafe {
        tablero::ITER_CURSOR = Some(iterador.clone());
      }
    }
    
    let resul = gtk::TextIter::forward_search(&mut iterador,
                    " ",
                    gtk::TextSearchFlags::VISIBLE_ONLY,
                    None).unwrap();
    iterador = resul.1.clone();
    let resul_1 = gtk::TextIter::forward_search(&mut iterador,
                    " ",
                    gtk::TextSearchFlags::VISIBLE_ONLY,
                    None).unwrap();
    //let clon_inicio = iter_inicio.clone();
    textbuffer.apply_tag_by_name("fondo_amarillo", &resul.1, &resul_1.0);
    unsafe {
      tablero::ITERS_FONDO_TAG = Some((resul.1, resul_1.0));
      tablero::JUGADA_ACTUAL = Some(
              gl_arr_partida[num_var][gl_arr_partida[num_var].len() - 2]
              .clone()
              );
    }
  }
  // ahora llevamos la fen al tablero
  presenta_jugadas::muestra_arg_link(
          &gl_arr_partida[num_var][gl_arr_partida[num_var].len() - 2].fen,
          area_tablero);
}



pub fn borra_jugadas (_widget: &gtk::Button,
            area_tablero: &gtk::DrawingArea,
            visor_partida: &gtk::TextView) {
  
  let textbuffer = visor_partida.get_buffer().expect("error");
  let gl_jugada_actual: tablero::MoveT;
  let mut gl_arr_partida: Vec<Vec<tablero::MoveT>>;
  
  unsafe {
    gl_jugada_actual = tablero::JUGADA_ACTUAL.clone().unwrap();
    gl_arr_partida = tablero::JUGADAS.clone().unwrap();
  }
  
  // comprobamos que estamos en algun sitio
  if gl_jugada_actual.idx_jug == "Var0Mv0".to_string() {
    //println!("no estamos en ningun sitio");
    return;
  }
  let indice: Vec<String> = gl_jugada_actual.idx_jug
          .split("Var")
          .map(|s| s.to_string())
          .collect();
  let arr_idx: Vec<String> = indice[1]
          .split("Mv")
          .map(|s| s.to_string())
          .collect();
  let num_var = arr_idx[0].parse::<usize>().unwrap();
  let index = gl_arr_partida[num_var]
          .iter()
          .position(|r| r.idx_jug == gl_jugada_actual.idx_jug)
          .unwrap();
  //eliminamos el ultimo elemento de la variante
  let mut ultimo = gl_arr_partida[num_var].pop()
          .expect("error al hacer pop");
  // borramos hasta el final
  // desde la jugada siguiente
  gl_arr_partida[num_var].truncate(index + 1);
  
  
  // ahora restauramos el ultimo elemento pero con idx_jug actualizado
  let tx_num_jug_final = gl_arr_partida[num_var][index].num_jug.clone();
  let num_jug_final: i32 = tx_num_jug_final.parse::<i32>().unwrap() + 1;
  let idx_final = format!("Var{}Mv{}", num_var, num_jug_final); 
  ultimo.idx_jug = idx_final;
  ultimo.num_jug = num_jug_final.to_string();
  gl_arr_partida[num_var].push(ultimo);
  
  unsafe {
    tablero::JUGADAS = Some(gl_arr_partida);
  }
  // mostramos un poup mientras se recarga la partida
  let carga = progreso_carga::PopUp::init("Regenerando estructura.Espere ...");
  while gtk::events_pending() {
    gtk::main_iteration();
  }
  let arr_jugadas = presenta_jugadas::crea_texto_partida (&visor_partida, &area_tablero);
  textbuffer.delete(&mut textbuffer.get_start_iter(), &mut textbuffer.get_end_iter());
  presenta_jugadas::escribe_jugadas (arr_jugadas, &textbuffer, &visor_partida, &area_tablero);
  visor_partida.show_all();
  while gtk::events_pending() {
    gtk::main_iteration();
  }
  
  // ahora actualizamos el tablero a la posición inicial y
  // reseteamos las variables globales de posicionamiento
  let fen_inicio: String;
  unsafe {
    let temp = tablero::CABECERA.clone();
    fen_inicio = temp.unwrap().fen;
    tablero::JUGADA_ACTUAL = None;
    tablero::COORDENADAS = None;
    tablero::ITER_CURSOR = None;
    tablero::ITERS_FONDO_TAG = None;
  }
  
  presenta_jugadas::muestra_arg_link (&fen_inicio, &area_tablero);
  // destruimos el popup
  std::thread::sleep(std::time::Duration::from_millis(500));
  carga.destruye();
}



pub fn borra_variante (_widget: &gtk::Button,
            area_tablero: &gtk::DrawingArea,
            visor_partida: &gtk::TextView) {
            
  let textbuffer = visor_partida.get_buffer().expect("error");
  let gl_jugada_actual: tablero::MoveT;
  let mut gl_arr_partida: Vec<Vec<tablero::MoveT>>;
  
  unsafe {
    gl_jugada_actual = tablero::JUGADA_ACTUAL.clone().unwrap();
    gl_arr_partida = tablero::JUGADAS.clone().unwrap();
  }
  
  // comprobamos que estamos en algun sitio
  if gl_jugada_actual.idx_jug == "Var0Mv0".to_string() {
    //println!("no estamos en ningun sitio");
    return;
  }
  
  let indice: Vec<String> = gl_jugada_actual.idx_jug
          .split("Var")
          .map(|s| s.to_string())
          .collect();
  let arr_idx: Vec<String> = indice[1]
          .split("Mv")
          .map(|s| s.to_string())
          .collect();
  let num_var = arr_idx[0].parse::<usize>().unwrap();
  
  // la variante principal (0) no se permite borrarla
  let mut encontrado = false;
  if num_var > 0 {
    //buscamos el nodo padre
    for i in 0..gl_arr_partida.len() {
      for j in 0..gl_arr_partida[i].len() {
        let arr = &mut gl_arr_partida[i][j].sub_var;
        if arr.len() > 0 {
          for m in 0..arr.len() {
            if arr[m] == num_var {
              arr.remove(m);
              encontrado = true;
              break;
            }
          }
          if encontrado {break;}
        }
        if encontrado {break;}
      }
      if encontrado {break;}
    }
  }
  
  unsafe {
    tablero::JUGADAS = Some(gl_arr_partida);
  }
  // mostramos un poup mientras se recarga la partida
  let carga = progreso_carga::PopUp::init("Regenerando estructura.Espere ...");
  while gtk::events_pending() {
    gtk::main_iteration();
  }
  let arr_jugadas = presenta_jugadas::crea_texto_partida (&visor_partida, &area_tablero);
  textbuffer.delete(&mut textbuffer.get_start_iter(), &mut textbuffer.get_end_iter());
  presenta_jugadas::escribe_jugadas (arr_jugadas, &textbuffer, &visor_partida, &area_tablero);
  visor_partida.show_all();
  while gtk::events_pending() {
    gtk::main_iteration();
  }
  
  // ahora actualizamos el tablero a la posición inicial y
  // reseteamos las variables globales de posicionamiento
  let fen_inicio: String;
  unsafe {
    let arr = tablero::JUGADAS.clone().unwrap();
    fen_inicio = arr[0][0].fen.clone();
    tablero::JUGADA_ACTUAL = Some(arr[0][0].clone());
    tablero::COORDENADAS = None;
    tablero::ITER_CURSOR = None;
    tablero::ITERS_FONDO_TAG = None;
  }
  
  presenta_jugadas::muestra_arg_link (&fen_inicio, &area_tablero);
  // destruimos el popup
  std::thread::sleep(std::time::Duration::from_millis(500));
  carga.destruye();
}