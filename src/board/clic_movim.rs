use gtk::*;
use gtk::prelude::*;

use tablero;
use super::presenta_jugadas;


pub fn clic_movim (movim: tablero::MoveT,
          widget: &gtk::Button,
          evento: &gdk::EventButton,
          visor: &gtk::TextView,
          d_area: &gtk::DrawingArea) -> bool {
  
  let num_btn = evento.get_button();
  if num_btn == 2 {
    return true;
  }
  // boton izquierdo
  if num_btn == 1 {
    let iterador: gtk::TextIter;
    unsafe {
      let gl_arr_partida = tablero::JUGADAS.clone().unwrap();
      let indice: Vec<String> = movim.idx_jug.split("Var").map(|s| s.to_string()).collect();
      let arr_idx: Vec<String> = indice[1].split("Mv").map(|s| s.to_string()).collect();
      let num_var = arr_idx[0].parse::<usize>().unwrap();
      let index = gl_arr_partida[num_var].iter().position(|r| r.idx_jug == movim.idx_jug).unwrap();
      
      tablero::JUGADA_ACTUAL = Some(gl_arr_partida[num_var][index].clone());
      let temp = tablero::COORDENADAS.clone().unwrap();
      iterador = temp.iter_cursor.clone();
      tablero::ITER_CURSOR = Some(iterador);
      
      // vamos a limpiar los tagas con color de fondo_amarillo
      let textbuffer = gtk::TextViewExt::get_buffer(visor)
            .expect("error al obtener textbuffer clic_movim");
      {
        let iters = tablero::ITERS_FONDO_TAG.clone();
        match iters {
          Some(iters) => {
              textbuffer.remove_tag_by_name("fondo_amarillo", &iters.0, &iters.1);
            },
          None => {},
        };
      }
    }
    
    let arg = movim.fen.clone();
    presenta_jugadas::muestra_arg_link(&arg, &d_area);
    return false;
  }
  // boton derecho
  if num_btn == 3 {
    //let mut iterador: gtk::TextIter;
    let num_var: usize;
    let index: usize;
    let marca1tag: gtk::TextMark;
    let marca2tag: gtk::TextMark;
    unsafe {
      let gl_arr_partida = tablero::JUGADAS.clone().unwrap();
      let indice: Vec<String> = movim.idx_jug.split("Var").map(|s| s.to_string()).collect();
      let arr_idx: Vec<String> = indice[1].split("Mv").map(|s| s.to_string()).collect();
      num_var = arr_idx[0].parse::<usize>().unwrap();
      index = gl_arr_partida[num_var].iter().position(|r| r.idx_jug == movim.idx_jug).unwrap();
      
      tablero::JUGADA_ACTUAL = Some(gl_arr_partida[num_var][index].clone());
      let temp = tablero::COORDENADAS.clone().unwrap();
      let iterador = temp.iter_cursor.clone();
      let mut iter_clon = iterador.clone();
      tablero::ITER_CURSOR = Some(iterador);
      // vamos a limpiar los tagas con color de fondo_amarillo
      let textbuffer = gtk::TextViewExt::get_buffer(visor)
            .expect("error al obtener textbuffer clic_movim");
      {
        let iters = tablero::ITERS_FONDO_TAG.clone();
        match iters {
          Some(iters) => {
              textbuffer.remove_tag_by_name("fondo_amarillo", &iters.0, &iters.1);
            },
          None => {},
        };
      }
      // ahora ponemos el fondo amarillo cuando pulsamos con boton derecho
      let resul_ant = gtk::TextIter::backward_search(&mut iter_clon,
          " ",
          gtk::TextSearchFlags::VISIBLE_ONLY,
          None).unwrap();  // -> Option<(textiter, textiter)>
      let resul_post = gtk::TextIter::forward_search(&mut iter_clon,
          " ",
          gtk::TextSearchFlags::VISIBLE_ONLY,
          None).unwrap();
      
      marca1tag = textbuffer.create_mark(Some("marca1tag"), &resul_ant.1, true)
            .expect("error al crear marca1tag");
      marca2tag = textbuffer.create_mark(Some("marca2tag"), &resul_post.0, true)
            .expect("error al crear marca2tag");
      textbuffer.apply_tag_by_name("fondo_amarillo", &resul_ant.1, &resul_post.0);
      tablero::ITERS_FONDO_TAG = Some((resul_ant.1, resul_post.0));
    }
    
    let arg = movim.fen.clone();
    presenta_jugadas::muestra_arg_link(&arg, &d_area);
    
    crea_menu (widget, evento, &visor, num_var, index, marca1tag, marca2tag);
    
    return false;
  }
  
  false
}


fn crea_menu (btn: &gtk::Button,
              _evento: &gdk::EventButton, 
              visor: &gtk::TextView,
              num_var: usize,
              index: usize,
              marca1tag: gtk::TextMark,
              marca2tag: gtk::TextMark) {
 
  let menu = gtk::Menu::new();
  let comen = gtk::MenuItem::new_with_label("Comentario posterior ...");
  menu.append(&comen);
  menu.append(&gtk::SeparatorMenuItem::new());
  
  let calidad = gtk::MenuItem::new_with_label("!, ?, ...");
    let sub_calidad = gtk::Menu::new();
    calidad.set_submenu(Some(&sub_calidad));
    let calidad_ninguno = gtk::MenuItem::new_with_label("(Ninguno)");
    let calidad_buena = gtk::MenuItem::new_with_label("Buena jugada - !");
    let calidad_mala = gtk::MenuItem::new_with_label("Mala jugada - ?");
    let calidad_muy_buena = gtk::MenuItem::new_with_label("Muy buena jugada - \u{203C}");
    let calidad_muy_mala = gtk::MenuItem::new_with_label("Muy mala jugada - \u{2047}");
    let calidad_interesante = gtk::MenuItem::new_with_label("Jugada interesante - \u{2049}");
    let calidad_dudosa = gtk::MenuItem::new_with_label("Jugada dudosa - \u{2048}");
    let calidad_forzada = gtk::MenuItem::new_with_label("Jugada forzada - \u{25A1}");
    let calidad_zugzwang = gtk::MenuItem::new_with_label("Zugzwang - \u{2A00}");
    sub_calidad.append(&calidad_ninguno);
    sub_calidad.append(&calidad_buena);
    sub_calidad.append(&calidad_mala);
    sub_calidad.append(&calidad_muy_buena);
    sub_calidad.append(&calidad_muy_mala);
    sub_calidad.append(&calidad_interesante);
    sub_calidad.append(&calidad_dudosa);
    sub_calidad.append(&calidad_forzada);
    sub_calidad.append(&calidad_zugzwang);
  
  let evalua = gtk::MenuItem::new_with_label("\u{2213}, =, ...");
    let sub_evalua = gtk::Menu::new();
    evalua.set_submenu(Some(&sub_evalua));
    let evalua_ninguno = gtk::MenuItem::new_with_label("(Ninguno)");
    let evalua_igualdad = gtk::MenuItem::new_with_label("Igualdad - =");
    let evalua_confuso = gtk::MenuItem::new_with_label("Poco claro - \u{221E}");
    let evalua_ligera_blanca = gtk::MenuItem::new_with_label("Ligera ventaja blanca - \u{2A72}");
    let evalua_ligera_negra = gtk::MenuItem::new_with_label("Ligera ventaja negra - \u{2A71}");
    let evalua_moderada_blanca = gtk::MenuItem::new_with_label("Moderada ventaja blanca - \u{00B1}");
    let evalua_moderada_negra = gtk::MenuItem::new_with_label("Moderada ventaja negra - \u{2213}");
    let evalua_decisiva_blanca = gtk::MenuItem::new_with_label("Decisiva ventaja blanca - +-");
    let evalua_decisiva_negra = gtk::MenuItem::new_with_label("Decisiva ventaja negra - -+");
    let evalua_compensacion = gtk::MenuItem::new_with_label("Compensación - =/\u{221E}");
    let evalua_ataque = gtk::MenuItem::new_with_label("Ataque - \u{2191}");
    let evalua_iniciativa = gtk::MenuItem::new_with_label("Iniciativa - \u{2192}");
    let evalua_contrajuego = gtk::MenuItem::new_with_label("Contrajuego - \u{21C6}");
    let evalua_desarrollo = gtk::MenuItem::new_with_label("Ventaja de desarrollo - \u{27F3}");
    let evalua_novedad = gtk::MenuItem::new_with_label("Novedad - N");
    let evalua_tiempo = gtk::MenuItem::new_with_label("Apuro de tiempo - \u{1F540}");  //
    sub_evalua.append(&evalua_ninguno);
    sub_evalua.append(&evalua_igualdad);
    sub_evalua.append(&evalua_confuso);
    sub_evalua.append(&evalua_ligera_blanca);
    sub_evalua.append(&evalua_ligera_negra);
    sub_evalua.append(&evalua_moderada_blanca);
    sub_evalua.append(&evalua_moderada_negra);
    sub_evalua.append(&evalua_decisiva_blanca);
    sub_evalua.append(&evalua_decisiva_negra);
    sub_evalua.append(&evalua_compensacion);
    sub_evalua.append(&evalua_ataque);
    sub_evalua.append(&evalua_iniciativa);
    sub_evalua.append(&evalua_contrajuego);
    sub_evalua.append(&evalua_desarrollo);
    sub_evalua.append(&evalua_novedad);
    sub_evalua.append(&evalua_tiempo);
  
  let otros = gtk::MenuItem::new_with_label("Otros ...");
    let sub_otros = gtk::Menu::new();
    otros.set_submenu(Some(&sub_otros));
    let otros_ninguno = gtk::MenuItem::new_with_label("(Ninguno)");
    let otros_editorial = gtk::MenuItem::new_with_label("Comentario Editorial - RR");
    let otros_mejor = gtk::MenuItem::new_with_label("Es mejor - \u{2313}");
    let otros_peor = gtk::MenuItem::new_with_label("Es peor - \u{2264}");
    let otros_idea = gtk::MenuItem::new_with_label("Con la idea - \u{2206}");
    let otros_contra = gtk::MenuItem::new_with_label("Dirigido contra - \u{2207}");
    sub_otros.append(&otros_ninguno);
    sub_otros.append(&otros_editorial);
    sub_otros.append(&otros_mejor);
    sub_otros.append(&otros_peor);
    sub_otros.append(&otros_idea);
    sub_otros.append(&otros_contra);
    
  //let close = MenuItem::new_with_label("Close");                        // on_close
  
  menu.append(&calidad);
  menu.append(&evalua);
  menu.append(&otros);
  menu.append(&gtk::SeparatorMenuItem::new());
  
  menu.show_all();
  // https://gtk-rs.org/docs/gtk/prelude/trait.GtkMenuExtManual.html#tymethod.popup_easy
  menu.popup_easy(1, 3);
  
  // los closures
  {
    //let ev_clon = evento.clone();
    let weak_visor = glib::object::ObjectExt::downgrade(visor);
    comen.connect_activate( move |_m| {
      let visor = match weak_visor.upgrade() {
                  Some(visor) => visor,
                  None => return,
              };
      let mut gl_arr_partida: Vec<Vec<tablero::MoveT>>;
      unsafe {
        gl_arr_partida = tablero::JUGADAS.clone().unwrap();
      }
      let texto = gl_arr_partida[num_var][index].comen.clone();
      let txt_cambiado = modifica_comen (texto);
      let buffer = visor.get_buffer().expect("error en get_buffer");
      let mut iter: gtk::TextIter;
      unsafe {
        let temp = tablero::COORDENADAS.clone().unwrap();
        iter = temp.iter_cursor.clone();
      }
      let marca = buffer.create_mark(Some("marca"), &iter, true).expect("error al crear marca");
      
      // el inicio de la etiqueta para borrarla
      let resul_1 = gtk::TextIter::forward_search(&mut iter,
          " ",
          gtk::TextSearchFlags::VISIBLE_ONLY,
          None).unwrap();
      iter = resul_1.1.clone();
      
      let mut resul_1 = gtk::TextIter::forward_search(&mut iter,
          "\t",
          gtk::TextSearchFlags::VISIBLE_ONLY,
          None).unwrap();
          
      buffer.delete(&mut iter, &mut resul_1.0);
      
      // ahora la regeneramos
      let mut resul_2 = gtk::TextIter::backward_search(&mut resul_1.0,
          " ",
          gtk::TextSearchFlags::VISIBLE_ONLY,
          None).unwrap();
      
      let palabras = txt_cambiado.split(" ");
      let items: Vec<&str> = palabras.collect();
      for elem in items {
        let ele = format!("{} ", elem);
        let lbl_comen = gtk::Label::new(Some(ele.as_str()));
        gtk::WidgetExt::set_widget_name(&lbl_comen, "lbl_comen");
        let ancla = buffer.create_child_anchor(&mut resul_2.1)
            .expect("error en ancla");
        visor.add_child_at_anchor(&lbl_comen, &ancla);
        lbl_comen.show();
      }
      
      //visor.show_all();
      
      unsafe {
        gl_arr_partida[num_var][index].comen = txt_cambiado;
        tablero::JUGADAS = Some(gl_arr_partida);
        
        let iterador = buffer.get_iter_at_mark(&marca);
        tablero::ITER_CURSOR = Some(iterador);
        
        let iter1tag = buffer.get_iter_at_mark(&marca1tag);
        let iter2tag = buffer.get_iter_at_mark(&marca2tag);
        tablero::ITERS_FONDO_TAG = Some((iter1tag, iter2tag));
      }
    });
  }
  
  // bloque calificacion de jugada
  {
    let btn_clon = btn.clone();
    calidad_ninguno.connect_activate( move |_m| {
      let nag = "$0".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    calidad_buena.connect_activate( move |_m| {
      let nag = "$1".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    calidad_mala.connect_activate( move |_m| {
      let nag = "$2".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    calidad_muy_buena.connect_activate( move |_m| {
      let nag = "$3".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    calidad_muy_mala.connect_activate( move |_m| {
      let nag = "$4".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    calidad_interesante.connect_activate( move |_m| {
      let nag = "$5".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    calidad_dudosa.connect_activate( move |_m| {
      let nag = "$6".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    calidad_forzada.connect_activate( move |_m| {
      let nag = "$7".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    calidad_zugzwang.connect_activate( move |_m| {
      let nag = "$22".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  
  // bloque valoracion/evaluacion de jugada
  {
    let btn_clon = btn.clone();
    evalua_ninguno.connect_activate( move |_m| {
      let nag = "$0".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_igualdad.connect_activate( move |_m| {
      let nag = "$10".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_confuso.connect_activate( move |_m| {
      let nag = "$13".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_ligera_blanca.connect_activate( move |_m| {
      let nag = "$14".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_ligera_negra.connect_activate( move |_m| {
      let nag = "$15".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_moderada_blanca.connect_activate( move |_m| {
      let nag = "$16".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_moderada_negra.connect_activate( move |_m| {
      let nag = "$17".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_decisiva_blanca.connect_activate( move |_m| {
      let nag = "$18".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_decisiva_negra.connect_activate( move |_m| {
      let nag = "$19".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_compensacion.connect_activate( move |_m| {
      let nag = "$44".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_ataque.connect_activate( move |_m| {
      let nag = "$40".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_iniciativa.connect_activate( move |_m| {
      let nag = "$36".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_contrajuego.connect_activate( move |_m| {
      let nag = "$132".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_desarrollo.connect_activate( move |_m| {
      let nag = "$32".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_novedad.connect_activate( move |_m| {
      let nag = "$146".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    evalua_tiempo.connect_activate( move |_m| {
      let nag = "$136".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  
  // bloque otras nag de jugada
  {
    let btn_clon = btn.clone();
    otros_ninguno.connect_activate( move |_m| {
      let nag = "$0".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    otros_editorial.connect_activate( move |_m| {
      let nag = "$145".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    otros_mejor.connect_activate( move |_m| {
      let nag = "$142".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    otros_peor.connect_activate( move |_m| {
      let nag = "$143".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    otros_idea.connect_activate( move |_m| {
      let nag = "$140".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
  {
    let btn_clon = btn.clone();
    otros_contra.connect_activate( move |_m| {
      let nag = "$141".to_string();
      tramita_menu(nag, num_var, index, &btn_clon);
    });
  }
}



fn tramita_menu (nag: String,
          num_var: usize,
          index: usize,
          btn: &gtk::Button) {
  
  let mut gl_arr_partida: Vec<Vec<tablero::MoveT>>;
  unsafe {
    gl_arr_partida = tablero::JUGADAS.clone().unwrap();
  }
  
  if nag == "$0".to_string() {
    gl_arr_partida[num_var][index].nag = "".to_string();
    btn.set_label(&gl_arr_partida[num_var][index].san);
    unsafe {
      tablero::JUGADAS = Some(gl_arr_partida);
    }
  }
  else {
    gl_arr_partida[num_var][index].nag = format!("{}{}", gl_arr_partida[num_var][index].nag, nag);
    let txt_nag = presenta_jugadas::lee_nag (gl_arr_partida[num_var][index].nag.clone());
    let txt_btn = format!("{} {}", gl_arr_partida[num_var][index].san, txt_nag);
    btn.set_label(&txt_btn);
    unsafe {
      tablero::JUGADAS = Some(gl_arr_partida);
    }
  }
}


fn modifica_comen (mut texto: String) -> String {
  let dialog = gtk::Dialog::new_with_buttons(
                        Some("Comentario jugada"),
                        None::<&Window>,   // es el parent
                        gtk::DialogFlags::MODAL,
                        &[("Ok", gtk::ResponseType::Ok),
                        ("Cancelar", gtk::ResponseType::Cancel)]
                    );
  dialog.set_size_request(350, 200);
  dialog.set_position(gtk::WindowPosition::CenterAlways);
  let top_area = dialog.get_content_area(); // -> Box
  let vista_comen = gtk::TextView::new();
  top_area.pack_start(&vista_comen, true, true, 3);
  let buffer_comen = vista_comen.get_buffer().expect("error en get_buffer");
  
  if texto.len() > 0 {
    let _ = texto.remove(0);
    let _ = texto.remove(texto.len() - 1);
  }
  buffer_comen.set_text(&texto);
  
  let mut texto_retorno = texto;
  dialog.show_all();
  let result = dialog.run();
  if result == gtk::ResponseType::Ok.into() {
    let inicio = buffer_comen.get_start_iter();
    let fin = buffer_comen.get_end_iter();
    let gstring = buffer_comen.get_text(&inicio, &fin, true)
              .expect("error al obtener el gstring");
    let txtstr = gstring.as_str();
    // reemplazamos caracteres no compatibles
    let mut temp = txtstr.replace("{", "[");
    temp = temp.replace("}", "]");
    temp = temp.replace("\"", "'");
    temp = temp.replace("\n", " ");
    texto_retorno = format!("{}{}{}", "{", temp, "}");
  }
  
  dialog.destroy();
  texto_retorno
}