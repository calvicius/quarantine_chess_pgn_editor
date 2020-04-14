use gtk::*;

//mod uci;

use super::{tablero, uci};

pub fn activar_engine () {
    
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_border_width(5);
    window.set_default_size(950, 200);
    window.set_title("UCI interface");
    window.set_position(gtk::WindowPosition::CenterAlways);
    
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&vbox);
    
    let mut engine = uci::Engine::init("./engine/stockfish.exe");
    vbox.pack_start(&engine.scrolled_win, true, true, 0);
    
    // false space
    let lbl = gtk::Label::new(Some(""));
    vbox.pack_start(&lbl, false, false, 0);
    
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    vbox.pack_start(&hbox, false, false, 0);
    
    let lbl_depth = gtk::Label::new(Some("Calculation depth: "));
    hbox.pack_start(&lbl_depth, false, false, 0);
    let spin_depth = gtk::SpinButton::new_with_range(5.0, 100.0, 5.0);
    spin_depth.set_wrap(true);
    hbox.pack_start(&spin_depth, false, false, 0);
    
    let btn_depth = gtk::Button::new_with_label(" Analysis Board FEN and given depth ");
    gtk::WidgetExt::set_widget_name(&btn_depth, "btn_tablero");
    hbox.pack_end(&btn_depth, false, false, 5);
    
    let btn_infinite = gtk::Button::new_with_label(" Analysis infinite ");
    gtk::WidgetExt::set_widget_name(&btn_infinite, "btn_tablero");
    hbox.pack_end(&btn_infinite, false, false, 5);
    
    let btn_time = gtk::Button::new_with_label(" Analysis 500 ms ");
    gtk::WidgetExt::set_widget_name(&btn_time, "btn_tablero");
    hbox.pack_end(&btn_time, false, false, 5);
    
    window.show_all();
    
    engine.get_handshake();
    //engine.get_uci_options();
    // an example to change default engine options
    /* engine.set_options(b"setoption name MultiPV value 10\n"); */
    
    let is_ready_ok = engine.get_is_ready();
    /*
    if is_ready_ok.contains("readyok") {
      engine.set_initial_pos();
      engine.go_depth(spin_depth.get_value_as_int());
      
      let movs = "e2e4 c7c5";
      engine.set_initial_pos_with_moves(movs);
      engine.go_depth(spin_depth.get_value_as_int());
      //engine.go_infinite();
    }
    */
    let engine1 = engine.clone();
    let engine2 = engine.clone();
    let engine3 = engine.clone();
    let engine4 = engine.clone();
    {
      // Activate ESC key
      window.connect_key_press_event ( move |_widget, event| {
        let engine_clon = engine1.clone();
        if event.get_keyval() == gdk::enums::key::Escape {
          engine_clon.set_stop();
        }
        Inhibit(false)
      });
    }
    
    {
      let ready_ok_clon = is_ready_ok.clone();
      btn_depth.connect_clicked (move |_w| {
        let mut engine_clon = engine2.clone();
        if ready_ok_clon.contains("readyok") {
          
          let mut fen = String::new();
          {
            let obj_tab = tablero::VAR_TABLERO.read().unwrap();
            fen = obj_tab.fen_actual.clone();
          }
          let fen1 = fen.as_str();
          engine_clon.set_pos_fen (fen1);
          engine_clon.go_depth(spin_depth.get_value_as_int());
        }
      });
    }
    
    {
      let ready_ok_clon = is_ready_ok.clone();
      btn_infinite.connect_clicked (move |_w| {
        let mut engine_clon = engine3.clone();
        if ready_ok_clon.contains("readyok") {
          
          let mut fen = String::new();
          {
            let obj_tab = tablero::VAR_TABLERO.read().unwrap();
            fen = obj_tab.fen_actual.clone();
          }
          let fen1 = fen.as_str();
          engine_clon.set_pos_fen (fen1);
          engine_clon.go_infinite();
        }
      });
    }
    
    {
      let ready_ok_clon = is_ready_ok.clone();
      btn_time.connect_clicked (move |_w| {
        let mut engine_clon = engine4.clone();
        if ready_ok_clon.contains("readyok") {
          
          let mut fen = String::new();
          {
            let obj_tab = tablero::VAR_TABLERO.read().unwrap();
            fen = obj_tab.fen_actual.clone();
          }
          let fen1 = fen.as_str();
          engine_clon.set_pos_fen (fen1);
          engine_clon.go_by_time(500);
        }
      });
    }
}