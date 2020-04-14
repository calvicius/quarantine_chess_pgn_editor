use gtk::*;
use glib;

use tablero;
use super::ajedrez;
use super::parser;
use super::tablero_interno;
use super::clic_movim;
use super::misc;


// cuando no hay un pgn o es vacío creamos
// la estructura interna de la partida
pub fn crea_partida (pgn_txt: String) {
  if pgn_txt.len() == 0 {
    let cabeza = tablero::Cabecera {
      event: "?".to_string(),
      site: "?".to_string(),
      date: "????.??.??".to_string(),
      round: "?".to_string(),
      white: "?".to_string(),
      black: "?".to_string(),
      result: "*".to_string(),
      eco: "???".to_string(),
      white_elo: "0".to_string(),
      black_elo: "0".to_string(),
      fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
    };
    
    let mut varin: Vec<tablero::MoveT> = Vec::new();
    // movimiento ficticio que configuramos con los datos de inicio de partida
    let mut movim = tablero::MoveT{
      idx_jug: "Var0Mv0".to_string(),
      san: "".to_string(),
      uci: "".to_string(),
      num_jug: "0".to_string(),
      turno: "".to_string(),
      fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
      nag: "".to_string(),
      comen: "".to_string(),
      sub_var: [].to_vec(),
      profundidad: "0".to_string(),
    };
    
    varin.push(movim);
    
    // movimiento ficticio para recoger el resultado e indicar el final de la partida
    movim = tablero::MoveT{
      idx_jug: "Var0Mv1".to_string(),
      san:"*".to_string(),
      uci: "*".to_string(),
      num_jug: "1".to_string(),
      turno: "".to_string(),
      fen: "".to_string(),
      nag: "".to_string(),
      comen: "".to_string(),
      sub_var: [].to_vec(),
      profundidad: "0".to_string()
    };
    varin.push(movim);
    let mut jugadas: Vec<Vec<tablero::MoveT>> = Vec::new();
    jugadas.push(varin);
    
    unsafe {
      tablero::CABECERA = Some(cabeza);
      tablero::JUGADAS = Some(jugadas);
    }
    
  }
  else {
    // calculamos cuanto tiempo tarda
    //let antes = misc::get_time_ms();
    let partida = parser::procesa_jugadas(pgn_txt);
    
    let cabeza = tablero::Cabecera {
      event: partida[0].cabecera[0].clone(),
      site: partida[0].cabecera[1].clone(),
      date: partida[0].cabecera[2].clone(),
      round: partida[0].cabecera[3].clone(),
      white: partida[0].cabecera[4].clone(),
      black: partida[0].cabecera[5].clone(),
      result: partida[0].cabecera[6].clone(),
      eco: partida[0].cabecera[7].clone(),
      white_elo: partida[0].cabecera[8].clone(),
      black_elo: partida[0].cabecera[9].clone(),
      fen: partida[0].cabecera[10].clone(),
    };
    
    let partida_json: String = partida[0].partida_json.clone();
    
    unsafe {
      tablero::CABECERA = Some(cabeza);
      //usamos el json para no depender de variables/estructuras de otros modulos
      tablero::JUGADAS = Some(serde_json::from_str(&partida_json).unwrap());
    }
    //let despues = misc::get_time_ms();
    //println!("Tiempo de parser; milisegundos: {}", (despues-antes));
  }
}

// creamos los tags de los márgenes de las jugadas
pub fn crea_tags_margenes (tabla_tags: &gtk::TextTagTable) {
  let tag_margen_0 = gtk::TextTag::new(Some("margen0"));
  tag_margen_0.set_property_left_margin(2);
  let mut _tag_correcto = tabla_tags.add(&tag_margen_0);  // -> bool
  
  let tag_margen_1 = gtk::TextTag::new(Some("margen1"));
  tag_margen_1.set_property_left_margin(20);
  let mut _tag_correcto = tabla_tags.add(&tag_margen_1);
  
  let tag_margen_2 = gtk::TextTag::new(Some("margen2"));
  tag_margen_2.set_property_left_margin(40);
  let mut _tag_correcto = tabla_tags.add(&tag_margen_2);
  
  let tag_margen_3 = gtk::TextTag::new(Some("margen3"));
  tag_margen_3.set_property_left_margin(60);
  let mut _tag_correcto = tabla_tags.add(&tag_margen_3);
  
  let tag_margen_4 = gtk::TextTag::new(Some("margen4"));
  tag_margen_4.set_property_left_margin(80);
  let mut _tag_correcto = tabla_tags.add(&tag_margen_4);
  
  let tag_margen_5 = gtk::TextTag::new(Some("margen5"));
  tag_margen_5.set_property_left_margin(100);
  let mut _tag_correcto = tabla_tags.add(&tag_margen_5);
  
  let tag_margen_6 = gtk::TextTag::new(Some("margen6"));
  tag_margen_6.set_property_left_margin(115);
  let mut _tag_correcto = tabla_tags.add(&tag_margen_6);
  
  let tag_margen_7 = gtk::TextTag::new(Some("margen7"));
  tag_margen_7.set_property_left_margin(130);
  let mut _tag_correcto = tabla_tags.add(&tag_margen_7);
  
  let tag_margen_8 = gtk::TextTag::new(Some("margen8"));
  tag_margen_8.set_property_left_margin(145);
  let mut _tag_correcto = tabla_tags.add(&tag_margen_8);
  
  let tag_margen_9 = gtk::TextTag::new(Some("margen9"));
  tag_margen_9.set_property_left_margin(160);
  let mut _tag_correcto = tabla_tags.add(&tag_margen_9);
  
  let tag_margen_10 = gtk::TextTag::new(Some("margen10"));
  tag_margen_10.set_property_left_margin(175);
  let mut _tag_correcto = tabla_tags.add(&tag_margen_10);
  
  let fondo_amarillo = gtk::TextTag::new(Some("fondo_amarillo"));
  fondo_amarillo.set_property_background(Some("#F4FA58")); // 
  let mut _tag_correcto = tabla_tags.add(&fondo_amarillo);  // -> bool
}


pub fn escribe_margenes (margin: i32,
                textbuffer: &gtk::TextBuffer,
                start_iter: gtk::TextIter,
                end_iter: gtk::TextIter) {
  match margin {
      0 =>  textbuffer.apply_tag_by_name("margen0", &start_iter, &end_iter),
      1 =>  textbuffer.apply_tag_by_name("margen1", &start_iter, &end_iter),
      2 =>  textbuffer.apply_tag_by_name("margen2", &start_iter, &end_iter),
      3 =>  textbuffer.apply_tag_by_name("margen3", &start_iter, &end_iter),
      4 =>  textbuffer.apply_tag_by_name("margen4", &start_iter, &end_iter),
      5 =>  textbuffer.apply_tag_by_name("margen5", &start_iter, &end_iter),
      6 =>  textbuffer.apply_tag_by_name("margen6", &start_iter, &end_iter),
      7 =>  textbuffer.apply_tag_by_name("margen7", &start_iter, &end_iter),
      8 =>  textbuffer.apply_tag_by_name("margen8", &start_iter, &end_iter),
      9 =>  textbuffer.apply_tag_by_name("margen9", &start_iter, &end_iter),
      10 => textbuffer.apply_tag_by_name("margen10", &start_iter, &end_iter),
      _ =>  textbuffer.apply_tag_by_name("margen10", &start_iter, &end_iter),
    };
}
/**********************************************
*  Creamos las funciones que procesan el      *
*  array de jugadas y lo presenta en pantalla *
*  formateada                                 *
***********************************************/
pub fn escribe_jugadas (arr_jugadas: Vec<tablero::Movim>, 
          textbuffer: &gtk::TextBuffer,
          visor_partida: &gtk::TextView,
          d_area: &gtk::DrawingArea) {
  
  let mut margin: i32 = 0;
  visor_partida.set_left_margin(margin);
  visor_partida.set_editable(true);
  let mut iter_inicio = textbuffer.get_start_iter();
  textbuffer.insert(&mut iter_inicio, "\t" );
        
  for mut elem in arr_jugadas.clone() {
    let nuevo_margen: i32 = elem.move_t.profundidad.parse::<i32>().unwrap();
    let etiq = elem.move_t.san.clone();
    
    if margin != nuevo_margen {
      if etiq == "(".to_string() {
        let mut iter = textbuffer.get_end_iter();
        let txt_linea = format!("\n");
        textbuffer.insert(&mut iter, &txt_linea );
      }
      margin = nuevo_margen;
    }
    
    let btn_san = crea_link(elem.move_t.clone(), &visor_partida, &d_area);
    elem.mod_btn_san(btn_san);
    
    let mut iter = textbuffer.get_end_iter();
    let marca1 = textbuffer.create_mark(Some("marca1"), &iter, true)
            .expect("error al crear marca");
    let lbl_gtxt = elem.lbl_num.get_text().expect("error en gstring");
    if lbl_gtxt.as_str().len() > 0 {
      let ancla= textbuffer.create_child_anchor(&mut iter)
            .expect("error en ancla");
      visor_partida.add_child_at_anchor(&elem.lbl_num, &ancla);
    }
    let mut iter = textbuffer.get_end_iter();
    textbuffer.insert(&mut iter, " " );
    
    let mut iter = textbuffer.get_end_iter();
    let ancla1= textbuffer.create_child_anchor(&mut iter)
          .expect("error en ancla");
    visor_partida.add_child_at_anchor(&elem.btn_san, &ancla1);
    let mut iter = textbuffer.get_end_iter();
    textbuffer.insert(&mut iter, " " );
    
    let mut iter = textbuffer.get_end_iter();
    let splited: Vec<&str> = elem.move_t.comen.split(" ").collect();
    if splited.len() == 1 {
      let ancla3= textbuffer.create_child_anchor(&mut iter)
          .expect("error en ancla");
      visor_partida.add_child_at_anchor(&elem.lbl_comen, &ancla3);
    }
    else {
      for el in splited {
        let tx = format!("{} ", el);
        let lbl = gtk::Label::new(Some(&tx));
        gtk::WidgetExt::set_widget_name(&lbl, "lbl_comen");
        let ancla3= textbuffer.create_child_anchor(&mut iter)
            .expect("error en ancla");
        visor_partida.add_child_at_anchor(&lbl, &ancla3);
      }
    }
    
    let mut end_iter = textbuffer.get_end_iter();
    let txt_linea = format!("\t");
    textbuffer.insert(&mut end_iter, &txt_linea );
    let marca2 = textbuffer.create_mark(Some("marca2"), &end_iter, true).expect("error al crear marca");
    
    let start_iter = textbuffer.get_iter_at_mark(&marca1);
    let end_iter = textbuffer.get_iter_at_mark(&marca2);
    
    escribe_margenes (margin,
                &textbuffer,
                start_iter,
                end_iter);
    
    
    if margin == nuevo_margen && etiq == ")".to_string() {
      let mut iter = textbuffer.get_end_iter();
      let txt_linea = format!("\n");
      textbuffer.insert(&mut iter, &txt_linea );
    }
  }
                                  
  let mut iter = textbuffer.get_end_iter();
  let txt_linea = format!("\n\n");
  textbuffer.insert(&mut iter, &txt_linea );
        
  visor_partida.set_editable(false);
}


pub fn crea_texto_partida (visor: &gtk::TextView,
          d_area: &gtk::DrawingArea) -> Vec<tablero::Movim> {
  
  let mut arr_jugadas: Vec<tablero::Movim> = Vec::new();
  let partida: Vec<Vec<tablero::MoveT>>;
  unsafe {
    partida = tablero::JUGADAS.clone().unwrap();
  }
  for i in 1..partida[0].len() {
    let num: f32 = partida[0][i].num_jug.parse::<f32>().unwrap();
    let num_jugada = (num / 2.0).ceil();
    
    let txt_numju: String;
    if arr_jugadas.len() == 0 {
      if partida[0][i].turno == "w".to_string(){
        txt_numju = format!("{}.", num_jugada);
      }
      else if partida[0][i].turno == "b".to_string() {
        txt_numju = format!("{}. ...", num_jugada);
      }
      else {
        txt_numju = "".to_string();
      }
    }
    else {
      // vemos si el btn o lbl anterior contiene ')' o '}'
      let lbl_anterior = arr_jugadas[arr_jugadas.len()-1]
              .btn_san.get_label()
              .expect("error al obtener etiqueta boton san");  // GString
      let btn_anterior = lbl_anterior.as_str();
      
      let lbl_anterior = arr_jugadas[arr_jugadas.len()-1]
              .lbl_comen.get_text()
              .expect("error al obtener etiqueta label comen");  // GString
      let lbl_anterior = lbl_anterior.as_str();
      
      if partida[0][i].turno == "w".to_string(){
        txt_numju = format!("{}.", num_jugada);
      }
      else if partida[0][i].turno == "b".to_string() &&
              (btn_anterior.contains(")") || lbl_anterior.contains("}")) {
        txt_numju = format!("{}. ...", num_jugada);
      }
      else {
        txt_numju = format!("");
      }
    }
    
    let lbl_jugada = gtk::Label::new(Some(""));
    if txt_numju != "".to_string() {
      lbl_jugada.set_label(&txt_numju);
    }
    gtk::WidgetExt::set_widget_name(&lbl_jugada, "lbl_jugada");
    
    // ahora los nags
    let nag = lee_nag (partida[0][i].nag.clone());
    let texto_btn = format!("{}{}", partida[0][i].san.clone(), nag);
    let btn_san = gtk::Button::new_with_label(&texto_btn);
    gtk::WidgetExt::set_widget_name(&btn_san, "btn_jugada");
    
    // ahora los comentarios
    let lbl_comen = gtk::Label::new(Some(""));
    if partida[0][i].comen != "".to_string() {
      lbl_comen.set_label(&partida[0][i].comen); //= gtk::Label::new(Some(&partida[0][i].comen));
    }
    gtk::WidgetExt::set_widget_name(&lbl_comen, "lbl_comen");
    
    let movim = tablero::Movim {
      move_t: partida[0][i].clone(),
      lbl_num: lbl_jugada,
      btn_san: btn_san,
      lbl_comen: lbl_comen,
    };
    arr_jugadas.push(movim);
    
    for j in 0..partida[0][i].sub_var.len(){
      let mut arr_var = crea_texto_var(partida[0][i].sub_var[j], 
                  partida.clone(),
                  &visor,
                  d_area);
      arr_jugadas.append(&mut arr_var);
    }
    
  }
  
  unsafe {
    tablero::MOVIMS_TEXTVIEW = Some(arr_jugadas.clone());
  }
  arr_jugadas
}


fn crea_texto_var (num_var: usize, 
            partida: Vec<Vec<tablero::MoveT>>,
            visor: &gtk::TextView,
            d_area: &gtk::DrawingArea) -> Vec<tablero::Movim> {
  while gtk::events_pending() {
    gtk::main_iteration();
  }
  let mut idx = num_var.clone();
  let mut arr_jugadas: Vec<tablero::Movim> = Vec::new();
  let mut v_fen: String;
  let mut arr_fen: Vec<String>;
  let mut turno: String;
  
  /* para controlar la numeracion de la primera jugada de cada variante */
  let mut anadida_jug = false;
    
  // ponemos el primer elemento que es san = (
  let lbl_jugada = gtk::Label::new(Some(""));
  gtk::WidgetExt::set_widget_name(&lbl_jugada, "lbl_jugada");
  let nag = lee_nag (partida[idx][0].nag.clone());
  let texto_btn = format!("{}{}", partida[idx][0].san.clone(), nag);
  let btn_san = gtk::Button::new_with_label(&texto_btn);
  gtk::WidgetExt::set_widget_name(&btn_san, "btn_jugada");
  
  let lbl_comen = gtk::Label::new(Some(""));
  if partida[idx][0].comen != "".to_string() {
    lbl_comen.set_label(&partida[idx][0].comen);
  }
  gtk::WidgetExt::set_widget_name(&lbl_comen, "lbl_comen");
  
  let movim = tablero::Movim {
    move_t: partida[idx][0].clone(),
    lbl_num: lbl_jugada,
    btn_san: btn_san,
    lbl_comen: lbl_comen,
  };
  arr_jugadas.push(movim);
    
  for i in 1..partida[idx].len(){
    v_fen = partida[idx][i].fen.clone();
    
    // es la ultima jugada, la que coniene el san = ')'
    if v_fen.len() == 0 {
      // ponemos el primer elemento que es san = )
      let lbl_jugada = gtk::Label::new(Some(""));
      gtk::WidgetExt::set_widget_name(&lbl_jugada, "lbl_jugada");
      
      let nag = lee_nag (partida[idx][i].nag.clone());
      let texto_btn = format!("{}{}", partida[idx][i].san, nag);
      let btn_san = gtk::Button::new_with_label(&texto_btn);
      gtk::WidgetExt::set_widget_name(&btn_san, "btn_jugada");
      
      let lbl_comen = gtk::Label::new(Some(""));
      if partida[idx][i].comen != "".to_string() {
        lbl_comen.set_label(&partida[idx][i].comen);
      }
      gtk::WidgetExt::set_widget_name(&lbl_comen, "lbl_comen");
      
      let movim = tablero::Movim {
        move_t: partida[idx][i].clone(),
        lbl_num: lbl_jugada,
        btn_san: btn_san,
        lbl_comen: lbl_comen,
      };
      arr_jugadas.push(movim);
    }
    else {
      arr_fen = v_fen.split(" ").map(|s| s.to_string()).collect();
      let num_jug = arr_fen[5].clone();
      turno = arr_fen[1].clone();
      
      let num_jugada: i32 = num_jug.parse::<i32>().unwrap();
      
      let mut txt_numju: String = "".to_string();
      
      if !anadida_jug {
        if turno == "b".to_string() {
            txt_numju = format!("{}.", num_jugada);
        } else if turno == "w".to_string() {
            txt_numju = format!("{}. ...", num_jugada-1);
        }
        anadida_jug = true;
      }
      else if arr_jugadas.len() > 1 {
        // vemos si el btn o lbl anterior contiene ')' o '}' o '('
        let lbl_anterior = arr_jugadas[arr_jugadas.len()-1]
                .btn_san.get_label()
                .expect("error al obtener etiqueta boton san");  // GString
        let btn_anterior = lbl_anterior.as_str();
        
        let lbl_anterior = arr_jugadas[arr_jugadas.len()-1]
                .lbl_comen.get_text()
                .expect("error al obtener etiqueta label comen");
        let lbl_anterior = lbl_anterior.as_str();
        
        if turno == "b".to_string(){ //han jugado las blancas
            txt_numju = format!("{}.", num_jugada);
        }
        else if turno == "w".to_string() && 
                (btn_anterior.contains(")") || 
                lbl_anterior.contains("}") || 
                btn_anterior.contains("(")) {
            txt_numju = format!("{}. ...", num_jugada-1);
        }
      }
      else {
        txt_numju = format!("");
      }
      
      let lbl_jugada = gtk::Label::new(Some(&txt_numju));
      gtk::WidgetExt::set_widget_name(&lbl_jugada, "lbl_jugada");
      
      let nag = lee_nag (partida[idx][i].nag.clone());
      let texto_btn = format!("{}{}", partida[idx][i].san, nag);
      let btn_san = gtk::Button::new_with_label(&texto_btn);
      gtk::WidgetExt::set_widget_name(&btn_san, "btn_jugada");
      
      let lbl_comen = gtk::Label::new(Some(""));
      if partida[idx][i].comen != "".to_string() {
        lbl_comen.set_label(&partida[idx][i].comen);
      }
      gtk::WidgetExt::set_widget_name(&lbl_comen, "lbl_comen");
      
      let movim = tablero::Movim {
        move_t: partida[idx][i].clone(),
        lbl_num: lbl_jugada,
        btn_san: btn_san,
        lbl_comen: lbl_comen,
      };
      
      arr_jugadas.push(movim);
    }
    let idx_viejo = idx;
    for j in 0..partida[idx][i].sub_var.len(){
      let mut arr_var = crea_texto_var(partida[idx][i].sub_var[j], 
                  partida.clone(),
                  &visor,
                  d_area);
      arr_jugadas.append(&mut arr_var);
    }
    idx = idx_viejo;
  }
  
  arr_jugadas
}


pub fn crea_una_jugada (elem: tablero::MoveT,
                  visor_partida: &gtk::TextView,
                  d_area: &gtk::DrawingArea) -> tablero::Movim {
  
  let movim: tablero::Movim;
  if elem.san == "(".to_string() || 
            elem.san == ")".to_string() ||
            elem.san == "0-1".to_string() || 
            elem.san == "1-0".to_string() || 
            elem.san == "1/2-1/2".to_string() || 
            elem.san == "*".to_string() {
    
    let lbl_jugada = gtk::Label::new(None);
    gtk::WidgetExt::set_widget_name(&lbl_jugada, "lbl_jugada");
    
    let btn_san = crea_link(elem.clone(), &visor_partida, &d_area);
    //gtk::WidgetExt::set_widget_name(&btn_san, "btn_jugada");
    
    let lbl_comen = gtk::Label::new(None);
    if elem.comen != "" {
      lbl_comen.set_label(&elem.comen);
    }
    gtk::WidgetExt::set_widget_name(&lbl_comen, "lbl_comen");
    
    movim = tablero::Movim {
      move_t: elem.clone(),
      lbl_num: lbl_jugada,
      btn_san: btn_san,
      lbl_comen: lbl_comen,
    };
  }
  
  else {
    let v_fen = elem.fen.clone();
    let arr_fen: Vec<String> = v_fen
            .split(" ")
            .map(|s| s.to_string())
            .collect();
    let num_jug = arr_fen[5].clone();
    let turno = arr_fen[1].clone();
    
    let num_jugada: i32 = num_jug.parse::<i32>().unwrap();
    let mut txt_numju = "".to_string();
    
    if turno == "b".to_string() {
        txt_numju = format!("{}.", num_jugada);
    } else if turno == "w".to_string() {
        txt_numju = format!("{}. ...", num_jugada-1);
    }
    
    let lbl_jugada = gtk::Label::new(Some(&txt_numju));
    gtk::WidgetExt::set_widget_name(&lbl_jugada, "lbl_jugada");
    
    let btn_san = crea_link(elem.clone(), &visor_partida, &d_area);
    //gtk::WidgetExt::set_widget_name(&btn_san, "btn_jugada");
    
    let lbl_comen = gtk::Label::new(None);
    if elem.comen != "".to_string() {
      lbl_comen.set_label(&elem.comen);
    }
    gtk::WidgetExt::set_widget_name(&lbl_comen, "lbl_comen");
    
    movim = tablero::Movim {
      move_t: elem.clone(),
      lbl_num: lbl_jugada,
      btn_san: btn_san,
      lbl_comen: lbl_comen,
    };
  }
  
  movim
}


pub fn crea_una_jugada_final (num_subvar: i32, 
                  elem: tablero::MoveT,
                  visor_partida: &gtk::TextView,
                  d_area: &gtk::DrawingArea) -> tablero::Movim {
  
  let movim: tablero::Movim;
  if elem.san == "(".to_string() || 
            elem.san == ")".to_string() ||
            elem.san == "0-1".to_string() || 
            elem.san == "1-0".to_string() || 
            elem.san == "1/2-1/2".to_string() || 
            elem.san == "*".to_string() {
    let lbl_jugada = gtk::Label::new(None);
    gtk::WidgetExt::set_widget_name(&lbl_jugada, "lbl_jugada");
    
    let btn_san = crea_link(elem.clone(), &visor_partida, &d_area);
    //gtk::WidgetExt::set_widget_name(&btn_san, "btn_jugada");
    
    let lbl_comen = gtk::Label::new(None);
    if elem.comen != "" {
      lbl_comen.set_label(&elem.comen);
    }
    gtk::WidgetExt::set_widget_name(&lbl_comen, "lbl_comen");
    
    movim = tablero::Movim {
      move_t: elem.clone(),
      lbl_num: lbl_jugada,
      btn_san: btn_san,
      lbl_comen: lbl_comen,
    };
  }
  
  else {
    let v_fen = elem.fen.clone();
    let arr_fen: Vec<String> = v_fen.split(" ").map(|s| s.to_string()).collect();
    let num_jug = arr_fen[5].clone();
    let turno = arr_fen[1].clone();
    
    let num_jugada: i32 = num_jug.parse::<i32>().unwrap();
    let mut txt_numju = "".to_string();
    
    if turno == "b".to_string() {
        txt_numju = format!("{}.", num_jugada);
    } else if turno == "w".to_string() && num_subvar !=0 {
        txt_numju = format!("{}. ...", num_jugada-1);
    }
    
    let lbl_jugada = gtk::Label::new(Some(&txt_numju));
    gtk::WidgetExt::set_widget_name(&lbl_jugada, "lbl_jugada");
    
    let btn_san = crea_link(elem.clone(), &visor_partida, &d_area);
    //gtk::WidgetExt::set_widget_name(&btn_san, "btn_jugada");
    
    let lbl_comen = gtk::Label::new(None);
    if elem.comen != "".to_string() {
      lbl_comen.set_label(&elem.comen);
    }
    gtk::WidgetExt::set_widget_name(&lbl_comen, "lbl_comen");
    
    movim = tablero::Movim {
      move_t: elem.clone(),
      lbl_num: lbl_jugada,
      btn_san: btn_san,
      lbl_comen: lbl_comen,
    };
  }
  
  movim
}



fn crea_link <'a>(movim: tablero::MoveT,
          visor: &gtk::TextView,
          d_area: &gtk::DrawingArea) -> gtk::Button {
  
  let nag = lee_nag (movim.nag.clone());
  let texto_btn: String;
  if nag.len() > 0 {
    texto_btn = format!("{} {}", movim.san, nag);
  }
  else {
    texto_btn = format!("{}", movim.san);
  }
  let btn_jugada = gtk::Button::new_with_label(&texto_btn);
  btn_jugada.set_focus_on_click(true);
  
  // para colorear la variante: azul o verde / impar - par
  let profun = movim.profundidad.parse::<i32>().unwrap();
  if profun == 0 {
    gtk::WidgetExt::set_widget_name(&btn_jugada, "btn_jugada");
  }
  else {
    if profun & 2 == 0 {
      gtk::WidgetExt::set_widget_name(&btn_jugada, "btn_jugada_par");
    }
    else {
      gtk::WidgetExt::set_widget_name(&btn_jugada, "btn_jugada_impar");
    }
  }
  
  
  let weak_visor = glib::object::ObjectExt::downgrade(visor);
  let weak_area = glib::object::ObjectExt::downgrade(d_area);
  
  btn_jugada.connect_button_press_event ( move |w, event| {
    let d_area = match weak_area.upgrade() {
                Some(d_area) => d_area,
                None => return Inhibit(true),
            };
    let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return Inhibit(true),
            };
    
    let movim_clon = movim.clone();
    let resul = clic_movim::clic_movim(movim_clon, w, event, &visor, &d_area);
    Inhibit(resul)
  });
  
  btn_jugada
}


pub fn muestra_arg_link (arg: &str, d_area: &gtk::DrawingArea) {
  let mut obj_tab = tablero::VAR_TABLERO.write().unwrap();
  
  let mut board = ajedrez::Tablero::init();
  let fen_valida = ajedrez::set_fen(arg, &mut board);
  obj_tab.turno = board.to_move;
  if fen_valida {
    let grafico = ajedrez::tablero_grafico(&mut board);
    let tablero_interno = tablero_interno::procesa_notacion(grafico, obj_tab.flipped);
    
    obj_tab.fen_actual = arg.to_string();
    obj_tab.tablero_interno = tablero_interno;
  }
  d_area.queue_draw();
}


/***********************************************************
*  Creamos los NAGs                                        *
*  https://en.wikipedia.org/wiki/Numeric_Annotation_Glyphs *
*  https://en.wikipedia.org/wiki/Chess_annotation_symbols  *
************************************************************/

pub fn lee_nag (cadena: String) -> String {
  let nag: [&str; 251] = [
    "", 
    "!", "?", "\u{203C}", "\u{2047}", "\u{2049}", "\u{2048}", "\u{25A1}", "", "", "=",      // nag [1] to [10]
    "", "", "\u{221E}", "\u{2A72}", "\u{2A71}", "\u{00B1}", "\u{2213}", "+-", "-+", "",     // nag [11] to [20]
    "", "\u{2A00}", "\u{2A00}", "", "", "", "", "", "", "",                                 // nag [21] to [30]
    "", "\u{27F3}", "\u{27F3}", "", "", "\u{2192}", "\u{2192}", "", "", "\u{2191}",         // nag [31] to [40]
    "\u{2191}", "", "", "=/\u{221E}", "=/\u{221E}", "=/\u{221E}", "=/\u{221E}", "", "", "", // nag [41] to [50]
    "", "", "", "", "", "", "", "", "", "",                                                 // nag [51] to [60]
    "", "", "", "", "", "", "", "", "", "",                                             // nag [61] to [70]
    "", "", "", "", "", "", "", "", "", "",                                             // nag [71] to [80]
    "", "", "", "", "", "", "", "", "", "",                                             // nag [81] to [90]
    "", "", "", "", "", "", "", "", "", "",                                             // nag [91] to [100]
    "", "", "", "", "", "", "", "", "", "",                                             // nag [101] to [110]
    "", "", "", "", "", "", "", "", "", "",                                             // nag [111] to [120]
    "", "", "", "", "", "", "", "", "", "",                                             // nag [121] to [130]
    "", "\u{21C6}", "\u{21C6}", "", "", "\u{2A01}", "\u{2A01}", "\u{2A01}", "\u{2A01}", "\u{2206}",             // nag [131] to [140]
    "\u{25BD}", "\u{2313}", "\u{2264}", "", "RR", "N", "", "", "", "",                  // Non standard nag [141] to [150]
    "", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [151] to [160]
    "", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [161] to [170]
    "", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [171] to [180]
    "", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [181] to [190]
    "", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [191] to [200]
    "", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [201] to [210]
    "", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [211] to [220]
    "", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [221] to [230]
    "", "", "", "", "", "", "", "\u{25CB}", "\u{21D4}", "\u{21D7}",                     // Non standard nag [231] to [240]
    "", "\u{27EB}", "\u{27EA}", "\u{2715}", "\u{22A5}", "", "", "", "", "",             // Non standard nag [241] to [250]
  ];
  
  let items = cadena.split("$");
  let num_nag: Vec<&str> = items.collect();
  
  if num_nag.len() > 1 {
    let mut retorno: String = "".to_string();
    
    for ii in 1..num_nag.len() {
      let indice: usize = num_nag[ii].parse::<usize>().unwrap();
      retorno = format!("{}{}", retorno, nag[indice]);
    }
    return retorno;
  }
  else {
    return "".to_string();
  }
  
}


static mut PARENTESIS: i32 = 0;
static mut MOVIMIENTOS: usize = 0;
pub fn calcula_parentesis (movim: tablero::MoveT) -> (i32, i32) {
  let mut variantes: Vec<Vec<tablero::MoveT>>;
  for i in 0..movim.sub_var.len() {
    unsafe {
      variantes = tablero::JUGADAS.clone().unwrap();
      MOVIMIENTOS += variantes[movim.sub_var[i].clone()].len();
    }
    let _ = calc_paren_var (movim.sub_var[i].clone(), &variantes);
    unsafe {
      PARENTESIS += 1;
    }
  }
  let num_variantes: i32;
  let num_movims: i32;
  unsafe {
    num_variantes = PARENTESIS;
    num_movims = MOVIMIENTOS as i32;
    PARENTESIS = 0;
    MOVIMIENTOS = 0;
  }
  
  (num_variantes, num_movims)
}

fn calc_paren_var ( num_var: usize,
              variantes: &Vec<Vec<tablero::MoveT>>) {
  
  for elem in variantes[num_var].clone() {
    if elem.sub_var.len() > 0 {
      for j in 0..elem.sub_var.len() {
        unsafe {
          MOVIMIENTOS += variantes[elem.sub_var[j].clone()].len();
          PARENTESIS += 1;
        }
        let _ = calc_paren_var(elem.sub_var[j], &variantes);
      }
    }
  }
}