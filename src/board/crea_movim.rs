use gtk::*;

use super::tablero;
use super::presenta_jugadas;

pub fn jugada_tablero (movim: (String, String, String, String, String),
              vfen: String,
              visor_partida: &gtk::TextView,
              d_area: &gtk::DrawingArea) {
  // lo primero vamos a limpiar los tagas con color de fondo_amarillo
  let textbuffer = visor_partida.get_buffer().expect("error");
  unsafe {
    let iters = tablero::ITERS_FONDO_TAG.clone();
    match iters {
      Some(iters) => {
          textbuffer.remove_tag_by_name("fondo_amarillo", &iters.0, &iters.1);
        },
      None => {},
    };
  }
  // si no hay un iterador activo creamos uno ficticio al principio de la partida
  unsafe {
    let iterador = tablero::ITER_CURSOR.clone();
    match iterador {
      Some(_iterador) => {},
      None => {
        let iter_inicio = textbuffer.get_start_iter();
        tablero::ITER_CURSOR = Some(iter_inicio);
      },
    };
  }
  
  
  let gl_jugada_actual: tablero::MoveT;
  let gl_arr_partida: Vec<Vec<tablero::MoveT>>;
  unsafe {
    let yyy = tablero::JUGADA_ACTUAL.clone();
    gl_jugada_actual = yyy.unwrap();
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
  
  if movim.1 == gl_arr_partida[num_var][index+1].uci {
    let mut iterador: gtk::TextIter;
    // avanzamos al movimiento siguiente del textview
    unsafe {
      tablero::JUGADA_ACTUAL = Some(gl_arr_partida[num_var][index+1].clone());
      iterador = tablero::ITER_CURSOR.clone()
              .expect("error linea 209 crea_movim");
    }
    
    let (num_subvar, num_movims) = 
            presenta_jugadas::calcula_parentesis (gl_arr_partida[num_var][index]
            .clone());
    
    if num_subvar == 0 {
      let resul = gtk::TextIter::forward_search(&mut iterador,
              "\t", // el final del movim.actual
              gtk::TextSearchFlags::VISIBLE_ONLY,
              None).unwrap();
      iterador = resul.1.clone();
      unsafe {
        tablero::ITER_CURSOR = Some(iterador.clone());
      }
      let resul_1 = gtk::TextIter::forward_search(&mut iterador,
              " ",
              gtk::TextSearchFlags::VISIBLE_ONLY,
              None).unwrap();
      iterador = resul_1.1.clone();
      let resul_2 = gtk::TextIter::forward_search(&mut iterador,
              " ",
              gtk::TextSearchFlags::VISIBLE_ONLY,
              None).unwrap();
      textbuffer.apply_tag_by_name("fondo_amarillo", &resul_1.1, &resul_2.0);
      unsafe {
        tablero::ITERS_FONDO_TAG = Some((resul_1.1, resul_2.0));
      }
    }
    else {
      for _ in 0..=num_movims {
        let resul = gtk::TextIter::forward_search(&mut iterador,
                "\t",
                gtk::TextSearchFlags::VISIBLE_ONLY,
                None).unwrap();
        iterador = resul.1.clone();
        unsafe {
          tablero::ITER_CURSOR = Some(iterador.clone());
        }
      }
      
      let resul_1 = gtk::TextIter::forward_search(&mut iterador,
              " ",
              gtk::TextSearchFlags::VISIBLE_ONLY,
              None).unwrap();
      iterador = resul_1.1.clone();
      let resul_2 = gtk::TextIter::forward_search(&mut iterador,
              " ",
              gtk::TextSearchFlags::VISIBLE_ONLY,
              None).unwrap();
      
      textbuffer.apply_tag_by_name("fondo_amarillo", &resul_1.1, &resul_2.0);
      unsafe {
        tablero::ITERS_FONDO_TAG = Some((resul_1.1, resul_2.0));
      }
    }
  }
  else {
    // se añade una jugada al final de la variante
    if gl_arr_partida[num_var][index+1].san == "0-1".to_string() || 
            gl_arr_partida[num_var][index+1].san == "1-0".to_string() || 
            gl_arr_partida[num_var][index+1].san == "1/2-1/2".to_string() || 
            gl_arr_partida[num_var][index+1].san == "*".to_string() || 
            gl_arr_partida[num_var][index+1].san == ")".to_string() {
    
        anade_jugada_final(movim, vfen, visor_partida, d_area);
    }
    else {
      anade_jugada_mitad(movim, vfen, visor_partida, d_area);
    }
  }
}



fn anade_jugada_final (movim: (String, String, String, String, String),
              vfen: String,
              visor_partida: &gtk::TextView,
              d_area: &gtk::DrawingArea) {
  
  let gl_jugada_actual: tablero::MoveT;
  let mut gl_arr_partida: Vec<Vec<tablero::MoveT>>;
  unsafe {
    let yyy = tablero::JUGADA_ACTUAL.clone();
    gl_jugada_actual = yyy.unwrap();
    gl_arr_partida = tablero::JUGADAS.clone().unwrap();
  }
  let indice: Vec<String> = gl_jugada_actual.idx_jug
          .split("Var")
          .map(|s| s.to_string())
          .collect();
  let arr_idx: Vec<String> = indice[1].split("Mv").map(|s| s.to_string()).collect();
  let num_var = arr_idx[0].parse::<usize>().unwrap();
  let index = gl_arr_partida[num_var]
          .iter()
          .position(|r| r.idx_jug == gl_jugada_actual.idx_jug)
          .unwrap();
  let mut jugada_siguiente = gl_arr_partida[num_var][index+1].clone();
  let margen_nuevo: i32 = gl_jugada_actual.profundidad.parse::<i32>().unwrap();
  
  // ahora creamos la estructura de la nueva jugada
  let gl_jugada_actual = tablero::MoveT{
      idx_jug: jugada_siguiente.idx_jug.clone(), 
      san: movim.0, 
      uci: movim.1, 
      num_jug: jugada_siguiente.num_jug.clone(), 
      turno: movim.2,
      fen: vfen, 
      nag: "".to_string(), 
      comen: "".to_string(), 
      sub_var: [].to_vec(), 
      profundidad: jugada_siguiente.profundidad.clone(),
  };
  
  //eliminamos el ultimo elemento de la variante
  let _elem_borrado = gl_arr_partida[num_var]
        .pop()
        .expect("error elem_borrado");  // -> Option
  //añadimos la nueva jugada
  gl_arr_partida[num_var].push(gl_jugada_actual.clone());
  
  // modificamos algunos datos del elemento borrado 
  // que es igual a jugada_siguiente
  let ult_num_jug: usize = jugada_siguiente.num_jug.parse::<usize>().unwrap() + 1;
  jugada_siguiente.idx_jug = format!("Var{}Mv{}", num_var, ult_num_jug);
  jugada_siguiente.num_jug = ult_num_jug.to_string();
  gl_arr_partida[num_var].push(jugada_siguiente.clone());
  
  //creamos un array temporal para la nueva jugada para ponerla en el textview
  let mut arr_temp: Vec<tablero::MoveT> = Vec::new();
  arr_temp.push(gl_jugada_actual.clone());
  
  let mut iterador: gtk::TextIter;
  unsafe {
    tablero::JUGADA_ACTUAL = Some(gl_jugada_actual.clone());
    tablero::JUGADAS = Some(gl_arr_partida.clone());
    iterador = tablero::ITER_CURSOR.clone().expect("error linea 168 crea_movim");
    // Vamos al final del movimiento completo que es un espacio
    let resul = gtk::TextIter::forward_search(&mut iterador,
                    " ",
                    gtk::TextSearchFlags::VISIBLE_ONLY,
                    None).unwrap();  // -> Option<(textiter, textiter)>
    // como la nueva variante se inserta visualmente en la siguiente jugada
    // repetimos lo mismo usando el iterend
    iterador = resul.1;
  }
  
  let textbuffer = visor_partida.get_buffer()
          .expect("error al obtener el buffer");
  
  // calculamos las variantes y subvariantes precedentes
  // para iterar sobre sus cierres ")"
  let (num_subvar, num_movims) = 
        presenta_jugadas::calcula_parentesis (gl_arr_partida[num_var][index]
        .clone());
  for _ in 0..=num_movims {
    let resul = gtk::TextIter::forward_search(&mut iterador,
                    "\t",
                    gtk::TextSearchFlags::VISIBLE_ONLY,
                    None).unwrap();
    iterador = resul.1.clone();
  }
  if num_subvar > 0 {
    let resul_1 = gtk::TextIter::forward_search(&mut iterador,
              "\n",
              gtk::TextSearchFlags::VISIBLE_ONLY,
              None).unwrap();
    iterador = resul_1.1.clone();
  }
  
  let marca1 = textbuffer.create_mark(Some("marca1"), &iterador, true)
          .expect("error al crear marca");
  for elem in arr_temp {
    let mov = presenta_jugadas::crea_una_jugada_final(num_subvar, elem, &visor_partida, &d_area);
    
    let lbl_gtxt = mov.lbl_num.get_text().expect("error en gstring");
    if lbl_gtxt.as_str().len() > 0 {
      let ancla= textbuffer.create_child_anchor(&mut iterador)
            .expect("error en ancla");
      visor_partida.add_child_at_anchor(&mov.lbl_num, &ancla);
    }
    
    textbuffer.insert(&mut iterador, " " );
    
    let ancla1= textbuffer.create_child_anchor(&mut iterador)
          .expect("error en ancla");
    visor_partida.add_child_at_anchor(&mov.btn_san, &ancla1);
    textbuffer.insert(&mut iterador, " " );
    
    let ancla3= textbuffer.create_child_anchor(&mut iterador)
          .expect("error en ancla");
    visor_partida.add_child_at_anchor(&mov.lbl_comen, &ancla3);
    
    let txt_linea = format!("\t");
    textbuffer.insert(&mut iterador, &txt_linea );
  }
  let marca2 = textbuffer.create_mark(Some("marca2"), &iterador, true)
          .expect("error al crear marca");
  visor_partida.show_all();
  
  let mut start_iter = textbuffer.get_iter_at_mark(&marca1);
  let mut end_iter = textbuffer.get_iter_at_mark(&marca2);
  
  presenta_jugadas::escribe_margenes (margen_nuevo,
                &textbuffer,
                start_iter.clone(),
                end_iter.clone());
                
  // hemos MARKado el movimiento completo
  // ahora vamos a resaltar solo el boton de la SAN
  let resul = gtk::TextIter::forward_search(&mut start_iter,
          " ",
          gtk::TextSearchFlags::VISIBLE_ONLY,
          None).unwrap();
  start_iter = resul.1.clone();
  let resul = gtk::TextIter::backward_search(&mut end_iter,
          " ",
          gtk::TextSearchFlags::VISIBLE_ONLY,
          None).unwrap();
  end_iter = resul.0.clone();
  
  textbuffer.apply_tag_by_name("fondo_amarillo", &start_iter, &end_iter);
  unsafe {
    tablero::ITERS_FONDO_TAG = Some((start_iter.clone(), end_iter));
    tablero::ITER_CURSOR = Some(start_iter);
  }
  visor_partida.show_all();
}


fn anade_jugada_mitad (movim: (String, String, String, String, String),
              vfen: String,
              visor_partida: &gtk::TextView,
              d_area: &gtk::DrawingArea) {
  
  let gl_jugada_actual: tablero::MoveT;
  let mut gl_arr_partida: Vec<Vec<tablero::MoveT>>;
  unsafe {
    let yyy = tablero::JUGADA_ACTUAL.clone();
    gl_jugada_actual = yyy.unwrap();
    gl_arr_partida = tablero::JUGADAS.clone().unwrap();
  }
  let indice: Vec<String> = gl_jugada_actual.idx_jug.split("Var").map(|s| s.to_string()).collect();
  let arr_idx: Vec<String> = indice[1].split("Mv").map(|s| s.to_string()).collect();
  let num_var = arr_idx[0].parse::<usize>().unwrap();
  let index = gl_arr_partida[num_var].iter().position(|r| r.idx_jug == gl_jugada_actual.idx_jug).unwrap();
  let jugada_siguiente = gl_arr_partida[num_var][index+1].clone();
  let margen_nuevo: i32 = gl_jugada_actual.profundidad.parse::<i32>().unwrap() + 1;
  
  /*
  TODO : CONTROL WHEN VARIATION IS ALREADY PLAYED
  */
  //creamos un array temporal para la nueva variante
  let mut arr_temp: Vec<tablero::MoveT> = Vec::new();
  let primer_elem = tablero::MoveT {
    idx_jug: format!("Var{}Mv{}", gl_arr_partida.len(), gl_jugada_actual.num_jug),
    san: "(".to_string(), 
    uci: "(".to_string(), 
    num_jug: gl_jugada_actual.num_jug.clone(), 
    turno: jugada_siguiente.turno, 
    fen: gl_jugada_actual.fen, 
    nag: "".to_string(), 
    comen: "".to_string(), 
    sub_var: [].to_vec(), 
    profundidad: (gl_jugada_actual.profundidad.parse::<i32>().unwrap() + 1).to_string(),
  };
  
  let jugada = tablero::MoveT {
    idx_jug: format!("Var{}Mv{}", gl_arr_partida.len(), (gl_jugada_actual.num_jug.parse::<i32>().unwrap() + 1)),
    san: movim.0, 
    uci: movim.1,
    num_jug: (gl_jugada_actual.num_jug.parse::<i32>().unwrap() + 1).to_string(), 
    turno: movim.2,
    fen: vfen, 
    nag: "".to_string(), 
    comen: "".to_string(), 
    sub_var: [].to_vec(), 
    profundidad: (gl_jugada_actual.profundidad.parse::<i32>().unwrap() + 1).to_string(),
  };
  
  let ult_elem = tablero::MoveT {
    idx_jug: format!("Var{}Mv{}", gl_arr_partida.len(), (gl_jugada_actual.num_jug.parse::<i32>().unwrap() + 2)),
    san: ")".to_string(), 
    uci: ")".to_string(),
    num_jug: (gl_jugada_actual.num_jug.parse::<i32>().unwrap() + 2).to_string(),
    turno: "".to_string(),
    fen: "".to_string(), 
    nag: "".to_string(), 
    comen: "".to_string(), 
    sub_var: [].to_vec(), 
    profundidad: (gl_jugada_actual.profundidad.parse::<i32>().unwrap() + 1).to_string(),
  };
  
  arr_temp.push(primer_elem);
  arr_temp.push(jugada.clone());
  arr_temp.push(ult_elem);
  
  // añadimos al array de la partida
  gl_arr_partida.push(arr_temp.clone());
  
  // añadimos el num. de array a la jugada padre
  let num_subvar = gl_arr_partida.len();
  gl_arr_partida[num_var][index+1].sub_var.push((num_subvar - 1) as usize);
  
  //let fen_inserta = gl_arr_partida[num_var][index+1].fen.clone();
  let textbuffer = visor_partida.get_buffer().expect("error al obtener el buffer 211");
  let mut iterador: gtk::TextIter;
  
  unsafe {
    tablero::JUGADA_ACTUAL = Some(jugada.clone());
    tablero::JUGADAS = Some(gl_arr_partida.clone());
    iterador = tablero::ITER_CURSOR.clone().expect("error linea 374 crea_movim");
  }
  
  // calculamos las variantes y subvariantes precedentes
  // para iterar sobre sus cierres ")"
  let (num_subvar, num_movims) = presenta_jugadas::calcula_parentesis (gl_arr_partida[num_var][index].clone());
  
  // cuando el movim tiene variantes
  if gl_jugada_actual.sub_var.len() > 0 {
    for _ in 0..=num_movims + 1 {
      let resul = gtk::TextIter::forward_search(&mut iterador,
            "\t",
            gtk::TextSearchFlags::VISIBLE_ONLY,
            None).unwrap();
      iterador = resul.1.clone();
    }
  }
  // cuando el movimiento no tiene variantes
  // y el siguiente tampoco
  if num_subvar == 0 && 
        gl_jugada_actual.sub_var.len() == 0 &&
        jugada_siguiente.sub_var.len() == 0  {
    
    // Vamos al final del movimiento completo que es un tabulador
    // y del de la jugadas siguiente
    for _ in 0..2 {
      let resul = gtk::TextIter::forward_search(&mut iterador,
            "\t",
            gtk::TextSearchFlags::VISIBLE_ONLY,
            None).unwrap();
      iterador = resul.1.clone();
    }
  }
  // cuando el movimiento no tiene variantes
  // y el siguiente si tiene variantes
  if num_subvar == 0 && 
        gl_jugada_actual.sub_var.len() == 0 &&
        jugada_siguiente.sub_var.len() > 0  {
    
    // Vamos al final del movimiento completo que es un tabulador
    // y del de la jugadas siguiente
    for _ in 0..2 {
      let resul = gtk::TextIter::forward_search(&mut iterador,
              "\t",
              gtk::TextSearchFlags::VISIBLE_ONLY,
              None).unwrap();
      iterador = resul.1.clone();
    }
    // como la nueva variante se inserta visualmente en la siguiente jugada
    // repetimos lo mismo usando el iterend
    if num_subvar > 0 {
      let resul_1 = gtk::TextIter::forward_search(&mut iterador,
            "\n",
            gtk::TextSearchFlags::VISIBLE_ONLY,
            None).unwrap();
      iterador = resul_1.1.clone();
    }
  }
  
  
  textbuffer.insert(&mut iterador, "\n");
  let marca1 = textbuffer.create_mark(Some("marca1"), &iterador, true).expect("error al crear marca");
  
  for elem in arr_temp {
    let mov = presenta_jugadas::crea_una_jugada(elem, &visor_partida, &d_area);
    
    let lbl_gtxt = mov.lbl_num.get_text().expect("error en gstring");
    if lbl_gtxt.as_str().len() > 0 {
      let ancla= textbuffer.create_child_anchor(&mut iterador)
            .expect("error en ancla");
      visor_partida.add_child_at_anchor(&mov.lbl_num, &ancla);
    }
    textbuffer.insert(&mut iterador, " " );
    
    let ancla1= textbuffer.create_child_anchor(&mut iterador)
          .expect("error en ancla");
    visor_partida.add_child_at_anchor(&mov.btn_san, &ancla1);
    textbuffer.insert(&mut iterador, " " );
    
    let ancla3= textbuffer.create_child_anchor(&mut iterador)
          .expect("error en ancla");
    visor_partida.add_child_at_anchor(&mov.lbl_comen, &ancla3);
    
    let txt_linea = format!("\t");
    textbuffer.insert(&mut iterador, &txt_linea );
    
  }
  
  let marca2 = textbuffer.create_mark(Some("marca2"), &iterador, true).expect("error al crear marca");
  if gl_arr_partida[num_var][index+1].sub_var.len() < 2 {
    textbuffer.insert(&mut iterador, "\n");
  }
  
  // añadimos los num. de jugada del tipo 1. ... cuando proceda
  
  //if gl_arr_partida[num_var][index].sub_var.len() > 0 {
    let div_fen: Vec<String> = gl_arr_partida[num_var][index+1]
          .fen.split(" ")
          .map(|s| s.to_string())
          .collect();
    let numero = div_fen[5].parse::<i32>().unwrap();
    if gl_arr_partida[num_var][index].turno == "w".to_string() &&
        num_var > 0{
      if gl_arr_partida[num_var][index+1].sub_var.len() == 1 &&
              gl_arr_partida[num_var][index+2].san != ")".to_string() {  //> 0 {
        //println!("477 --> {}", gl_arr_partida[num_var][index+1].sub_var.len());
        let txt_numju = format!("{}. ...", numero);
        let lbl_num = gtk::Label::new(Some(&txt_numju));
        let ancla= textbuffer.create_child_anchor(&mut iterador)
              .expect("error en ancla");
        visor_partida.add_child_at_anchor(&lbl_num, &ancla);
      }
    }
    if gl_arr_partida[num_var][index].turno == "b".to_string() &&
        num_var == 0{
      if gl_arr_partida[num_var][index+1].sub_var.len() == 1 {  //> 0 {
        //let (num_subvar, num_movims) = presenta_jugadas::calcula_parentesis (gl_arr_partida[num_var][index+1].clone());
        //println!("489 --> {}", num_subvar);
        let txt_numju = format!("{}. ...", numero);
        let lbl_num = gtk::Label::new(Some(&txt_numju));
        let ancla= textbuffer.create_child_anchor(&mut iterador)
              .expect("error en ancla");
        visor_partida.add_child_at_anchor(&lbl_num, &ancla);
      }
    }
  //}
  
  
  let mut start_iter = textbuffer.get_iter_at_mark(&marca1);
  let mut end_iter = textbuffer.get_iter_at_mark(&marca2);
  
  presenta_jugadas::escribe_margenes (margen_nuevo,
        &textbuffer,
        start_iter.clone(),
        end_iter.clone());
  
  
  let resul = gtk::TextIter::forward_search(&mut start_iter,
        "\t",
        gtk::TextSearchFlags::VISIBLE_ONLY,
        None).unwrap();  // -> Option<(textiter, textiter)>
  start_iter = resul.1;
  let mut resul = gtk::TextIter::forward_search(&mut start_iter,
        " ",
        gtk::TextSearchFlags::VISIBLE_ONLY,
        None).unwrap();
  start_iter = resul.1;
  
  for _ in 0..3 {
    resul = gtk::TextIter::backward_search(&mut end_iter,
        " ",
        gtk::TextSearchFlags::VISIBLE_ONLY,
        None).unwrap();
    end_iter = resul.0;
  }
  
  textbuffer.apply_tag_by_name("fondo_amarillo", &start_iter, &end_iter);
  unsafe {
    tablero::ITERS_FONDO_TAG = Some((start_iter.clone(), end_iter.clone()));
    tablero::ITER_CURSOR = Some(start_iter);
  }
  
  visor_partida.show_all();
  
}

