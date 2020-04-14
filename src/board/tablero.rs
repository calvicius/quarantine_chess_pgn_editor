/*
extern crate gtk;
extern crate cairo;
extern crate gdk;
extern crate gdk_pixbuf;
#[macro_use]
extern crate lazy_static;
extern crate mut_static;    // https://github.com/tyleo/mut_static
*/

use gtk::*;
use gdk::prelude::*;
//use gdk::ContextExt;
use std::mem;
use std::ops::DerefMut;
use mut_static::MutStatic;
use std::collections::HashMap;


use super::ajedrez;
//use super::parser;
use super::{presenta_jugadas, crea_piezas, tablero_interno,
    crea_movim, botones_tablero, progreso_carga, promocion,
    graba_pgn, ayuda, uci_interface, cabecera};


/*************************************
/ variables globales del tablero     *
**************************************/
const FILAS: i32 = 8;
const COLUMNAS: i32 = 8;
pub const Y_EJE: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const X_EJE: [i32; 8] = [1,2,3,4,5,6,7,8];
const XY_FLIPPED: [[&str; 8]; 8] = [
  ["h8", "g8", "f8", "e8", "d8", "c8", "b8", "a8"],
  ["h7", "g7", "f7", "e7", "d7", "c7", "b7", "a7"],
  ["h6", "g6", "f6", "e6", "d6", "c6", "b6", "a6"],
  ["h5", "g5", "f5", "e5", "d5", "c5", "b5", "a5"],
  ["h4", "g4", "f4", "e4", "d4", "c4", "b4", "a4"],
  ["h3", "g3", "f3", "e3", "d3", "c3", "b3", "a3"],
  ["h2", "g2", "f2", "e2", "d2", "c2", "b2", "a2"],
  ["h1", "g1", "f1", "e1", "d1", "c1", "b1", "a1"]
];


pub struct VariablesTablero {
  color1: (f64, f64, f64),
  color2: (f64, f64, f64),
  dim_square: f64,
  dir_piezas: String,
  //fen_inicial: String,
  pub fen_actual: String,
  pub tablero_interno: HashMap<String, String>,
  pub flipped: bool,
  casilla_origen: String,
  raton_x: f64,
  raton_y: f64,
  pub turno: i16,
}


lazy_static! {
    pub static ref VAR_TABLERO: MutStatic<VariablesTablero> = MutStatic::new();
}

lazy_static! {
    static ref TABLERO_INVERTIDO: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("a1", "h8");
        m.insert("b1", "g8");
        m.insert("c1", "f8");
        m.insert("d1", "e8");
        m.insert("e1", "d8");
        m.insert("f1", "c8");
        m.insert("g1", "b8");
        m.insert("h1", "a8");
        
        m.insert("a2", "h7");
        m.insert("b2", "g7");
        m.insert("c2", "f7");
        m.insert("d2", "e7");
        m.insert("e2", "d7");
        m.insert("f2", "c7");
        m.insert("g2", "b7");
        m.insert("h2", "a7");
        
        m.insert("a3", "h6");
        m.insert("b3", "g6");
        m.insert("c3", "f6");
        m.insert("d3", "e6");
        m.insert("e3", "d6");
        m.insert("f3", "c6");
        m.insert("g3", "b6");
        m.insert("h3", "a6");
        
        m.insert("a4", "h5");
        m.insert("b4", "g5");
        m.insert("c4", "f5");
        m.insert("d4", "e5");
        m.insert("e4", "d5");
        m.insert("f4", "c5");
        m.insert("g4", "b5");
        m.insert("h4", "a5");
        
        m.insert("a5", "h4");
        m.insert("b5", "g4");
        m.insert("c5", "f4");
        m.insert("d5", "e4");
        m.insert("e5", "d4");
        m.insert("f5", "c4");
        m.insert("g5", "b4");
        m.insert("h5", "a4");
        
        m.insert("a6", "h3");
        m.insert("b6", "g3");
        m.insert("c6", "f3");
        m.insert("d6", "e3");
        m.insert("e6", "d3");
        m.insert("f6", "c3");
        m.insert("g6", "b3");
        m.insert("h6", "a3");
        
        m.insert("a7", "h2");
        m.insert("b7", "g2");
        m.insert("c7", "f2");
        m.insert("d7", "e2");
        m.insert("e7", "d2");
        m.insert("f7", "c2");
        m.insert("g7", "b2");
        m.insert("h7", "a2");
        
        m.insert("a8", "h1");
        m.insert("b8", "g1");
        m.insert("c8", "f1");
        m.insert("d8", "e1");
        m.insert("e8", "d1");
        m.insert("f8", "c1");
        m.insert("g8", "b1");
        m.insert("h8", "a1");
        
        m
    };
}

/*************************************
/ Fin variables globales del tablero *
**************************************/

/*************************************
/ variables globales de las jugadas  *
**************************************/
#[derive(Clone)]
pub struct Cabecera {
  pub event: String,
  pub site: String,
  pub date: String,
  pub round: String,
  pub white: String,
  pub black: String,
  pub result: String,
  pub eco: String,
  pub white_elo: String,
  pub black_elo: String,
  pub fen: String,
}

pub static mut CABECERA: Option<Cabecera> = None;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MoveT {
    pub idx_jug: String,
    pub san: String,
    pub uci: String,
    pub num_jug: String,
    pub turno: String, 
    pub fen: String,
    pub nag: String,
    pub comen: String,
    pub sub_var: Vec<usize>,        //las eventuales variantes que cuelgen de este movim.
    pub profundidad: String,        //esto es a la hora de imprimir (margenes, saltos de linea, etc...)
}

#[derive(Clone)]
pub struct Movim {
  pub move_t: MoveT,
  pub lbl_num: gtk::Label,
  pub btn_san: gtk::Button, // san + nag
  //pub lbl_nag: gtk::Label,
  pub lbl_comen: gtk::Label,
}

impl Movim {
  pub fn mod_btn_san (&mut self, boton: gtk::Button) {
    self.btn_san = boton;
  }
}

#[derive(Clone)]
pub struct Coordenadas {
  x: i32,
  y: i32,
  pub iter_cursor: gtk::TextIter,
}

pub static mut JUGADAS: Option<Vec<Vec<MoveT>>> = None;
pub static mut JUGADA_ACTUAL: Option<MoveT> = None;
pub static mut MOVIMS_TEXTVIEW: Option<Vec<Movim>> = None;
pub static mut COORDENADAS: Option<Coordenadas> = None;
pub static mut ITER_CURSOR: Option<gtk::TextIter> = None;
pub static mut ITERS_FONDO_TAG: Option<(gtk::TextIter, gtk::TextIter)> = None;


/*****************************************
/ fin variables globales de las jugadas  *
******************************************/

pub const CSS: &'static str = "
#lbl_jugada {
  font-size: 14px;
  padding: 0px 0px;
  color: #222222;
}

#lbl_comen {
  font-size: 14px;
  padding: 0px 0px;
  color: #999999;
}

textview text {
  color: #222222;
  background-color: #fefefe;
  font-family: \"Courier New\";
  font-size: 1px;
}

entry {
  min-height: 22px;
  padding: 0 6px;
  margin-right: 15px;
}

#btn_jugada {
  font-size: 14px;
  font-weight: normal;
  transition: all 200ms ease-out;
  min-width:0px;
  min-height:0px;
  padding: 0px 0px;
  
  border-width: 0px;
  border-style: hidden;
  color: #222222;
  background-image: none;
  box-shadow: none;
}

#btn_jugada_par {
  font-size: 14px;
  font-weight: normal;
  transition: all 200ms ease-out;
  min-width:0px;
  min-height:0px;
  padding: 0px 0px;
  
  border-width: 0px;
  border-style: hidden;
  color: darkblue;
  background-image: none;
  box-shadow: none;
}

#btn_jugada_impar {
  font-size: 14px;
  font-weight: normal;
  transition: all 200ms ease-out;
  min-width:0px;
  min-height:0px;
  padding: 0px 0px;
  
  border-width: 0px;
  border-style: hidden;
  color: darkgreen;
  background-image: none;
  box-shadow: none;
}

#btn_jugada:hover{
  background-image: none;
  color: black;
  background: yellow;
}
#btn_jugada:focus {
  background-image: none;
  color: black;
  background: yellow;
}

#btn_jugada_par:hover{
  background-image: none;
  color: black;
  background: yellow;
}
#btn_jugada_par:focus {
  background-image: none;
  color: black;
  background: yellow;
}

#btn_jugada_impar:hover{
  background-image: none;
  color: black;
  background: yellow;
}
#btn_jugada_impar:focus {
  background-image: none;
  color: black;
  background: yellow;
}


menu menuitem:hover {
  background-color: rgba(59, 70, 78, 0.7);
}

#btn_tablero {
  min-height: 24px;
  min-width: 16px;
  padding: 4px 8px;
  border-radius: 2px;
}

button {
  font-size: 14px;
  transition: all 200ms ease-out;
  min-width:0px;
  min-height:0px;
  padding: 0px 0px;
}

label {
  font-size: 14px;
  transition: all 200ms ease-out;
  min-width:0px;
  min-height:0px;
  padding: 0px 0px;
}

#lbl_fen {
  font-size: 12px;
  color: #999999;
}

progressbar {
  font-size: 12px;
}

combobox arrow {
  -gtk-icon-source: -gtk-icontheme(\"pan-down-symbolic\");
  min-height: 16px;
  min-width: 16px;
}

combobox decoration {
  transition: none;
}


combobox button.combo cellview:dir(ltr) {
  margin-left: 0px;
}

combobox button.combo cellview:dir(rtl) {
  margin-right: 0px;
}

combobox menu {
  padding: 2px 0;
}

combobox menu menuitem {
  min-height: 16px; /* 24px */
  padding: 0 8px;
}

combobox menu menuitem:hover {
  background-color: rgba(59, 70, 78, 0.7);
}

combobox menu > arrow.top {
  margin-top: -2px;
}

combobox menu > arrow.bottom {
  margin-top: 4px;
  margin-bottom: -6px;
}

combobox > .linked:not(.vertical) > entry:not(:only-child) {
  border-radius: 2px 2px 0 0;
}

combobox > .linked:not(.vertical) > entry:not(:only-child):first-child {
  margin-right: -32px;
  padding-right: 32px;
}

combobox > .linked:not(.vertical) > entry:not(:only-child):last-child {
  margin-left: -32px;
  padding-left: 32px;
}

combobox > .linked:not(.vertical) > button:not(:only-child) {
  min-height: 16px;
  min-width: 16px;
  margin: 4px;
  padding: 4px;
  border-radius: 9999px;
}

button.combo:only-child {
  border-radius: 2px 2px 0 0;
  font-weight: normal;
  transition: all 75ms cubic-bezier(0, 0, 0.2, 1), border-image 225ms cubic-bezier(0, 0, 0.2, 1);
  border-image: radial-gradient(circle closest-corner at center calc(100% - 1px), #d87777 0%, transparent 0%) 0 0 0/0 0 0px;
  box-shadow: inset 0 0 0 9999px transparent, inset 0 -1px rgba(153, 153, 153, 0.3);
  background-color: rgba(153, 153, 153, 0.04);
  color: #999999;
}

button.combo:only-child:focus {
  box-shadow: inset 0 0 0 9999px alpha(currentColor, 0.08), inset 0 -1px rgba(153, 153, 153, 0.3);
}

button.combo:only-child:hover,
button.combo:only-child:drop(active) {
  box-shadow: inset 0 0 0 9999px alpha(currentColor, 0.08), inset 0 -1px rgba(153, 153, 153, 0.3);
}

button.combo:only-child:checked {
  border-image: radial-gradient(circle closest-corner at center calc(100% - 1px), #d87777 100%, transparent 0%) 0 0 2/0 0 2px;
}

button.combo:only-child:disabled {
  box-shadow: inset 0 0 0 9999px transparent, inset 0 -1px rgba(153, 153, 153, 0.12);
  background-color: rgba(153, 153, 153, 0.04);
  color: rgba(153, 153, 153, 0.5);
}
";


pub struct Tablero {
  // window: gtk::Window,
}

impl Tablero {
  // el constructor de la Aplicacion
  pub fn init(jugadas: String)   {
    // Cargamos el CSS.
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(CSS.as_bytes())
        .expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );
    
    let carga = progreso_carga::PopUp::init("Cargando partida ...");
    
    while gtk::events_pending() {
      gtk::main_iteration();
    }
  
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_size_request(950, 550);
    window.set_title("Editor de partidas de Ajedrez");
    window.set_border_width(10);
    
    window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });
    
    // parseamos las jugadas del pgn
    while gtk::events_pending() {
      gtk::main_iteration();
    }
    presenta_jugadas::crea_partida (jugadas);
    
    while gtk::events_pending() {
      gtk::main_iteration();
    }
    let fen_inicial: String;
    unsafe {
      let arr = JUGADAS.clone().unwrap();
      fen_inicial = arr[0][0].fen.clone();
      JUGADA_ACTUAL = Some(arr[0][0].clone());
    }
    
    // Variables del Tablero
    let volteado = false;
    let mut board = ajedrez::Tablero::init();
    let _fen_valida = ajedrez::set_fen(fen_inicial.as_str(), &mut board);
    let fen_actual = ajedrez::get_fen(&mut board);
    let grafico = ajedrez::tablero_grafico(&mut board);
    let tablero_interno = tablero_interno::procesa_notacion(grafico, volteado);
    
    while gtk::events_pending() {
      gtk::main_iteration();
    }
    // inicializamos las variables globales del tablero_grafico
    let mut obj_tab: mut_static::ForceSomeRwLockWriteGuard<VariablesTablero>;
    let correcto = VAR_TABLERO.set( VariablesTablero {
      color2: (221.0 / 255.0, 184.0 / 255.0, 140.0 / 255.0),
      color1: (166.0 / 255.0, 109.0 / 255.0, 79.0 / 255.0),
      dim_square: 45.0,
      dir_piezas: "./piezas/Merida96/".to_string(),
      //fen_inicial: fen_inicial.to_string(),
      fen_actual: fen_actual.clone(),
      tablero_interno: tablero_interno.clone(),
      flipped : volteado,
      casilla_origen: "999".to_string(),
      raton_x: 0.0,
      raton_y: 0.0,
      turno: board.to_move,
    });
    
    // a cada llamada a este modulo nos encontramos que VAR_TABLERO
    // puede estar ya inicilizado y arroja un error al hacer .set()
    // entonces la desreferenciamos y colocamos en el mismo sitio_piezas
    // de memoria la reinicializacion
    match correcto {
      Ok(_correcto) => {obj_tab = VAR_TABLERO.write().unwrap();},
      Err(_err) => {
        obj_tab = VAR_TABLERO.write().unwrap();
        
        mem::replace(obj_tab.deref_mut(), VariablesTablero {
          color2: (221.0 / 255.0, 184.0 / 255.0, 140.0 / 255.0),
          color1: (166.0 / 255.0, 109.0 / 255.0, 79.0 / 255.0),
          dim_square: 45.0,
          dir_piezas: "./piezas/Merida96/".to_string(),
          //fen_inicial: fen_inicial.to_string(),
          fen_actual: fen_actual.clone(),
          tablero_interno: tablero_interno.clone(),
          flipped : volteado,
          casilla_origen: "99".to_string(),
          raton_x: 0.0,
          raton_y: 0.0,
          turno: board.to_move,
        });
      },
    };
    
    // una vez inicializado lo usamos
    while gtk::events_pending() {
      gtk::main_iteration();
    }
    let lista_piezas = crea_piezas::crea_lista_piezas(obj_tab.dir_piezas.clone());
    
    while gtk::events_pending() {
      gtk::main_iteration();
    }
    
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    // el box horizontal de cabecera
    let hbox_cabecera = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    vbox.pack_start(&hbox_cabecera, false, false, 0);
    let lbl_cab = gtk::Label::new(None);
    hbox_cabecera.pack_start(&lbl_cab, false, false, 0);
    let lbl_sep = gtk::Label::new(Some("   "));
    hbox_cabecera.pack_start(&lbl_sep, false, false, 0);
    let btn_cab = gtk::Button::new_with_label(" Editar ");
    gtk::WidgetExt::set_tooltip_markup(&btn_cab, Some("Edita la cabecera del PGN.\nPendiente"));
    hbox_cabecera.pack_start(&btn_cab, false, false, 0);
    let lbl_fen = gtk::Label::new(None);
    gtk::WidgetExt::set_widget_name(&lbl_fen, "lbl_fen");
    hbox_cabecera.pack_end(&lbl_fen, false, false, 0);
    
    // el paned
    let vp = gtk::Paned::new(Orientation::Horizontal);
    vp.set_position(430); // la posicion del divisor de los paneles
    
    vbox.pack_start(&vp, true, true, 0);
    
    let frm = gtk::AspectFrame::new(None, 0.0, 0.0, 5.0, true);
    let area_tablero = gtk::DrawingArea::new();
    gtk::prelude::WidgetExtManual::add_events(&area_tablero, 
                gdk::EventMask::POINTER_MOTION_MASK | 
                gdk::EventMask::BUTTON_PRESS_MASK | 
                gdk::EventMask::BUTTON_RELEASE_MASK);
    
    frm.add(&area_tablero);
    
    let vbox_tablero = gtk::Box::new(gtk::Orientation::Vertical, 10);
    vbox_tablero.set_hexpand(true);
    vbox_tablero.pack_start(&frm, true, true, 10);
    
    while gtk::events_pending() {
      gtk::main_iteration();
    }
    
    // Y aqui la Actionbar 
		let action_bar = gtk::ActionBar::new ();
    // botones lado izquierdo
    let btn_voltea = gtk::Button::new_from_icon_name (Some("object-flip-vertical"), gtk::IconSize::SmallToolbar);
    gtk::WidgetExt::set_widget_name(&btn_voltea, "btn_tablero");
    gtk::WidgetExt::set_tooltip_markup(&btn_voltea, Some("Gira el tablero"));
		action_bar.pack_start (&btn_voltea);
    
    // botones lado derecho
    let btn_borra_jug = gtk::Button::new_from_icon_name (Some("user-trash"), gtk::IconSize::SmallToolbar);
    gtk::WidgetExt::set_widget_name(&btn_borra_jug, "btn_tablero");
    gtk::WidgetExt::set_tooltip_markup(&btn_borra_jug, Some("Borra jugadas restantes"));
		action_bar.pack_end (&btn_borra_jug);
    let btn_borra_var = gtk::Button::new_from_icon_name (Some("user-trash-full"), gtk::IconSize::SmallToolbar);
    gtk::WidgetExt::set_widget_name(&btn_borra_var, "btn_tablero");
    gtk::WidgetExt::set_tooltip_markup(&btn_borra_var, Some("Borra variante actual"));
		action_bar.pack_end (&btn_borra_var);
    
    // botones centrales en la action_bar
    let secondary_box = gtk::Box::new (gtk::Orientation::Horizontal, 0);
		
		let btn_primera = gtk::Button::new_from_icon_name (Some("media-skip-backward"), gtk::IconSize::SmallToolbar);
    gtk::WidgetExt::set_widget_name(&btn_primera, "btn_tablero");
    gtk::WidgetExt::set_tooltip_markup(&btn_primera, Some("Ir a la primera jugada de la variante"));
		secondary_box.pack_start (&btn_primera, true, true, 0);
		let btn_anterior = gtk::Button::new_from_icon_name (Some("media-seek-backward"), gtk::IconSize::SmallToolbar);
    gtk::WidgetExt::set_widget_name(&btn_anterior, "btn_tablero");
    gtk::WidgetExt::set_tooltip_markup(&btn_anterior, Some("Ir a la jugada anterior de la variante"));
		secondary_box.pack_start (&btn_anterior, true, true, 0);
		let btn_siguiente = gtk::Button::new_from_icon_name (Some("media-seek-forward"), gtk::IconSize::SmallToolbar);
    gtk::WidgetExt::set_widget_name(&btn_siguiente, "btn_tablero");
    gtk::WidgetExt::set_tooltip_markup(&btn_siguiente, Some("Ir a la jugada siguiente de la variante"));
		secondary_box.pack_start (&btn_siguiente, true, true, 0);
		let btn_ultima = gtk::Button::new_from_icon_name (Some("media-skip-forward"), gtk::IconSize::SmallToolbar);
    gtk::WidgetExt::set_tooltip_markup(&btn_ultima, Some("Ir a la ultima jugada de la variante"));
    gtk::WidgetExt::set_widget_name(&btn_ultima, "btn_tablero");
		secondary_box.pack_start (&btn_ultima, true, true, 0);
        
    action_bar.set_center_widget (Some(&secondary_box));
    vbox_tablero.pack_start (&action_bar, false, false, 0);
    
    vp.add1(&vbox_tablero);
    
    while gtk::events_pending() {
      gtk::main_iteration();
    }
    // =======================================
    
    let cabecera: String;
    unsafe {
      let c = CABECERA.clone().unwrap();
      cabecera = format!("{}  vs  {}  -  {}", c.white, c.black, c.result);
    }
    lbl_cab.set_text(&cabecera);
    lbl_fen.set_text(&obj_tab.fen_actual);
    
    let vbox_partida = gtk::Box::new(gtk::Orientation::Vertical, 10);
    
    // =======================================
    
    while gtk::events_pending() {
      gtk::main_iteration();
    }
    // ahora el texto de la partida y la ventana con scroll
    let v_ajuste = gtk::Adjustment::new(0.0, 0.0, 530.0, 34.0, 306.0, 340.0);
    let scrolledwindow = gtk::ScrolledWindow::new(
          gtk::NONE_ADJUSTMENT, Some(&v_ajuste));
    scrolledwindow.set_policy(
          gtk::PolicyType::Never, gtk::PolicyType::Always);
    
    let visor_partida = gtk::TextView::new();
    visor_partida.set_wrap_mode(gtk::WrapMode::Word);
    let textbuffer = visor_partida.get_buffer().unwrap();
    
    // el tamaño del tabulador para separar cada movimiento en pixeles
    // num_jugada \s SAN+NAG \s comen \n
    let mut tabs = pango::TabArray::new(1, true);
    tabs.set_tab (0, pango::TabAlign::Left, 1);
    visor_partida.set_tabs(&tabs);
    
    let arr_jugadas = presenta_jugadas::crea_texto_partida (
            &visor_partida, &area_tablero);
    
    while gtk::events_pending() {
      gtk::main_iteration();
    }
    let tabla_tags = textbuffer.get_tag_table().expect("error al obtener la tabla de tags");
    // creamos los tags y escribimos la partida
    presenta_jugadas::crea_tags_margenes (&tabla_tags);
    presenta_jugadas::escribe_jugadas (arr_jugadas, &textbuffer, &visor_partida, &area_tablero);
    
    while gtk::events_pending() {
      gtk::main_iteration();
    }
    // un closure para cambiar el cursor del visor de partida
    visor_partida.connect_motion_notify_event (move |widget, eventmotion| {
      let display = gdk::Display::get_default()
              .expect("error en display");
      let gcursor = gdk::Cursor::new_for_display(&display, 
              gdk::CursorType::Hand1);
      let gwindow = gtk::TextViewExt::get_window(widget, 
              gtk::TextWindowType::Text)
              .expect("error en gwindow");
      gdk::WindowExt::set_cursor(&gwindow, Some(&gcursor));
      
      // ahora obtenemos las coords y el iter a cada coordenadas
      let (ex, ey) = gdk::EventMotion::get_position(eventmotion);
      let (x, y) = widget.window_to_buffer_coords (gtk::TextWindowType::Widget, ex as i32, ey as i32);
      if let Some(text_iter) = widget.get_iter_at_location (x, y) {
        let coords = Coordenadas {
          x: x,
          y: y,
          iter_cursor: text_iter,
        };
        unsafe {
          COORDENADAS = Some(coords);
        }
      }
      
      Inhibit(false)
    });
    
    scrolledwindow.set_hexpand(true);
    scrolledwindow.set_border_width(10);
    scrolledwindow.add(&visor_partida);
    vbox_partida.pack_start(&scrolledwindow, true, true, 0);
    
    // el box horizontal debajo del textview
    let hbox_pie = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    vbox_partida.pack_start(&hbox_pie, false, false, 0);
    let lbl = gtk::Label::new(Some("  Promoción peón: "));
    hbox_pie.pack_start(&lbl, false, false, 0);
    let corona = promocion::Promocion::init();
    hbox_pie.pack_start(&corona.promocion, false, false, 0);
    
    let btn_exit = gtk::Button::new_from_icon_name (Some("application-exit"), gtk::IconSize::SmallToolbar);
    gtk::WidgetExt::set_tooltip_markup(&btn_exit, Some("Salir"));
    hbox_pie.pack_end(&btn_exit, false, false, 2);
    
    let btn_help = gtk::Button::new_from_icon_name (Some("dialog-question"), gtk::IconSize::SmallToolbar);
    gtk::WidgetExt::set_tooltip_markup(&btn_help, Some("Ayuda"));
    hbox_pie.pack_end(&btn_help, false, false, 2);
    
    let btn_engine = gtk::Button::new_from_icon_name (Some("applications-engineering"), gtk::IconSize::SmallToolbar);
    gtk::WidgetExt::set_tooltip_markup(&btn_engine, Some("Arrancar Motor"));
    hbox_pie.pack_end(&btn_engine, false, false, 2);
    
    let btn_salv_pgn = gtk::Button::new_from_icon_name (Some("document-save-as"), gtk::IconSize::SmallToolbar);
    gtk::WidgetExt::set_tooltip_markup(&btn_salv_pgn, Some("Grabar partida como PGN"));
    hbox_pie.pack_end(&btn_salv_pgn, false, false, 2);
    
    
    vp.add2(&vbox_partida);
    window.add(&vbox);
    
    while gtk::events_pending() {
      gtk::main_iteration();
    }
    
    // el dibujo del tablero
    area_tablero.connect_draw ( move |widget, ctx| {
      let mut obj_tab = VAR_TABLERO.write().unwrap();
      lbl_fen.set_text(&obj_tab.fen_actual);
      let mut color = obj_tab.color2;
      obj_tab.dim_square = (widget.get_allocated_width() / 8) as f64;
      // el padding
      let leftover_space = widget.get_allocated_width() as f64 - 
            obj_tab.dim_square * 8.0;
      let padding = leftover_space / 2.0;
      cairo::Context::translate(ctx, padding as f64, padding as f64);
      
      // el tablero
      for r in 0..FILAS {
        if color == obj_tab.color2 {
          color = obj_tab.color1;
        } else { color = obj_tab.color2; }
        for c in 0..COLUMNAS {
          let x1 = c as f64 * obj_tab.dim_square;
          let y1 = (7-r) as f64 * obj_tab.dim_square;
          ctx.set_source_rgb(color.0, color.1, color.2);
          ctx.rectangle(x1, y1, obj_tab.dim_square, obj_tab.dim_square);
          ctx.fill();
          if color == obj_tab.color2 {
            color = obj_tab.color1;
          } else { color = obj_tab.color2; }
        }
      }
      
      // las piezas
      for (xycoord, valor) in &obj_tab.tablero_interno {
        let (x, y) = tablero_interno::num_notacion(xycoord);
        let x0 = (y as f64 * obj_tab.dim_square) + 
              (obj_tab.dim_square/16.0);
        let y0 = ((7-x) as f64 * obj_tab.dim_square) + 
              (obj_tab.dim_square/16.0);
        
        let pieza = lista_piezas.get(valor)
              .expect("error al obtener la pieza");
        let pixbuf = pieza.scale_simple (
              (obj_tab.dim_square * 0.90) as i32,
              (obj_tab.dim_square * 0.90) as i32,
              gdk_pixbuf::InterpType::Bilinear
              ).expect("error al escalar pixbuf");
        let _sr1 = ctx.set_source_pixbuf(&pixbuf, x0, y0);
        
        if obj_tab.casilla_origen == "999".to_string() {
          ctx.paint();
        }
        if obj_tab.flipped {
          if obj_tab.casilla_origen != TABLERO_INVERTIDO.get(xycoord.as_str()).unwrap().to_string() {
            ctx.paint();
          }
        }
        else if !obj_tab.flipped {
          if obj_tab.casilla_origen != *xycoord {
            ctx.paint();
          }
        }
      }
       
      
      if obj_tab.casilla_origen != "999".to_string() {
        let mut pieza_interna = obj_tab.tablero_interno.get(&obj_tab.casilla_origen);
        
        if obj_tab.flipped {
          let flipada = TABLERO_INVERTIDO.get(&obj_tab.casilla_origen.as_str()).unwrap().to_string();
          pieza_interna = obj_tab.tablero_interno.get(&flipada);
        }
        
        match pieza_interna {
          Some(pieza_interna) => {
            let pieza = lista_piezas.get(pieza_interna)
                  .expect("error al obtener la pieza");
            let pixbuf = pieza.scale_simple (
                  (obj_tab.dim_square * 0.90) as i32,
                  (obj_tab.dim_square * 0.90) as i32,
                  gdk_pixbuf::InterpType::Bilinear
                  ).expect("error al escalar pixbuf");
            //
            let _sr1 = ctx.set_source_pixbuf(&pixbuf, obj_tab.raton_x - (obj_tab.dim_square/2.0), 
                  obj_tab.raton_y - (obj_tab.dim_square/2.0));
            ctx.paint();
          }
          None => {}
        };
      }
      
      Inhibit(false)
    });
    
    while gtk::events_pending() {
      gtk::main_iteration();
    }
    // **** los closures de los botones del drag-drop en el tablero ******
    {
      area_tablero.connect_button_press_event ( move |widget, event| {
        on_pieza_presionada(widget, event);
        Inhibit(false)
      });
      
      area_tablero.connect_motion_notify_event ( move |widget, event| {
        on_pieza_moviendo(widget, event);
        Inhibit(false)
      });
      
      let weak_visor_partida = visor_partida.downgrade();
      area_tablero.connect_button_release_event(move |widget, event| {
        let visor_partida = match weak_visor_partida.upgrade() {
          Some(visor_partida) => visor_partida,
          None => return Inhibit(true),
        };
        
        on_pieza_soltada(widget, event, &visor_partida);
        Inhibit(false)
      });
    }
    // ******** fin de los closures del drg-drop del ratón *********
    
    while gtk::events_pending() {
      gtk::main_iteration();
    }
    // ******** los closures de los botones del tablero **************
    {
      let weak_area_tablero = area_tablero.downgrade();
      btn_voltea.connect_clicked (move |w| {
        let area_tablero = match weak_area_tablero.upgrade() {
          Some(area_tablero) => area_tablero,
          None => return,
        };
        botones_tablero::gira_tablero(w, &area_tablero);
      });
    }
    {
      let weak_area_tablero = area_tablero.downgrade();
      let weak_visor_partida = visor_partida.downgrade();
      btn_anterior.connect_clicked (move |w| {
        let area_tablero = match weak_area_tablero.upgrade() {
          Some(area_tablero) => area_tablero,
          None => return,
        };
        let visor_partida = match weak_visor_partida.upgrade() {
          Some(visor_partida) => visor_partida,
          None => return,
        };
        botones_tablero::jugada_anterior(w, &area_tablero, &visor_partida);
      });
    }
    {
      let weak_area_tablero = area_tablero.downgrade();
      let weak_visor_partida = visor_partida.downgrade();
      btn_primera.connect_clicked (move |w| {
        let area_tablero = match weak_area_tablero.upgrade() {
          Some(area_tablero) => area_tablero,
          None => return,
        };
        let visor_partida = match weak_visor_partida.upgrade() {
          Some(visor_partida) => visor_partida,
          None => return,
        };
        botones_tablero::jugada_primera(w, &area_tablero, &visor_partida);
      });
    }
    {
      let weak_area_tablero = area_tablero.downgrade();
      let weak_visor_partida = visor_partida.downgrade();
      btn_siguiente.connect_clicked (move |w| {
        let area_tablero = match weak_area_tablero.upgrade() {
          Some(area_tablero) => area_tablero,
          None => return,
        };
        let visor_partida = match weak_visor_partida.upgrade() {
          Some(visor_partida) => visor_partida,
          None => return,
        };
        botones_tablero::jugada_siguiente(w, &area_tablero, &visor_partida);
      });
    }
    {
      let weak_area_tablero = area_tablero.downgrade();
      let weak_visor_partida = visor_partida.downgrade();
      btn_ultima.connect_clicked (move |w| {
        let area_tablero = match weak_area_tablero.upgrade() {
          Some(area_tablero) => area_tablero,
          None => return,
        };
        let visor_partida = match weak_visor_partida.upgrade() {
          Some(visor_partida) => visor_partida,
          None => return,
        };
        botones_tablero::jugada_ultima(w, &area_tablero, &visor_partida);
      });
    }
    {
      let weak_area_tablero = area_tablero.downgrade();
      let weak_visor_partida = visor_partida.downgrade();
      btn_borra_jug.connect_clicked (move |w| {
        let area_tablero = match weak_area_tablero.upgrade() {
          Some(area_tablero) => area_tablero,
          None => return,
        };
        let visor_partida = match weak_visor_partida.upgrade() {
          Some(visor_partida) => visor_partida,
          None => return,
        };
        botones_tablero::borra_jugadas(w, &area_tablero, &visor_partida);
      });
    }
    {
      let weak_area_tablero = area_tablero.downgrade();
      let weak_visor_partida = visor_partida.downgrade();
      btn_borra_var.connect_clicked (move |w| {
        let area_tablero = match weak_area_tablero.upgrade() {
          Some(area_tablero) => area_tablero,
          None => return,
        };
        let visor_partida = match weak_visor_partida.upgrade() {
          Some(visor_partida) => visor_partida,
          None => return,
        };
        botones_tablero::borra_variante(w, &area_tablero, &visor_partida);
      });
    
    }
    // ******** fin de los closures de los botones del tablero *******
    
    // ******** el resto de los botones ******************************
    btn_salv_pgn.connect_clicked ( move |_| {
      let fich_string = graba_pgn::elige_fichero();
      match fich_string {
        Some (fichero) => {
          graba_pgn::graba_fichero (fichero.as_str());
        },
        None => graba_pgn::alerta(" No se ha elegido nombre de fichero "),
      }
    });
    
    btn_exit.connect_clicked ( move |_| {
      main_quit();
      
    });
    
    btn_engine.connect_clicked( move |_| {
        //dialog_engine::lee_motor();
        uci_interface::activar_engine();
    });
    
    btn_help.connect_clicked( move |_| {
        ayuda::ayuda_simple();
    });
    
    
    btn_cab.connect_clicked( move |_| {
        cabecera::modif_cabecera();
        let cabecera: String;
        unsafe {
          let c = CABECERA.clone().unwrap();
          cabecera = format!("{}  vs  {}  -  {}", c.white, c.black, c.result);
        }
        lbl_cab.set_text(&cabecera);
        lbl_cab.show();
    });
    
    
    
    // ******** fin de los closures **********************************
    
    while gtk::events_pending() {
      gtk::main_iteration();
    }
    window.show_all();
    carga.destruye();
  }
}



/*
  Las funciones de los closures de los botones del ratón
  drag-drop
*/

fn on_pieza_presionada(_widget: &gtk::DrawingArea, 
                event: &gdk::EventButton) {
  
  let e = event;
  /* Comienza el arrastre de una pieza */
  // para averiguar la casilla de inicio
  let mut obj_tab = VAR_TABLERO.write().unwrap();
  let col_tamano = obj_tab.dim_square;
  let fila_tamano = obj_tab.dim_square;
  let flipped = obj_tab.flipped;
  if e.get_button() == 1 {
    let (x, y) = event.get_position();
    
    let seleccionada_columna = (x / col_tamano) as usize;
    let seleccionada_fila = 7 - (y / fila_tamano) as usize;
    
    let pos = tablero_interno::alfa_notacion((seleccionada_fila, seleccionada_columna));
    
    //averiguamos la pieza y color que esta en tablero[pos] --> pos es la casilla b8, c7, etc.
    //si el turno (w o b) coincide con el color de la pieza (mays, minusculas), iniciamos el movimiento
    //if pos in obj_tab.tablero_interno {
    if obj_tab.tablero_interno.contains_key(&pos) {
      let es_mayus = obj_tab.tablero_interno.get(&pos).unwrap().as_str().chars().nth(0).unwrap().is_uppercase();
      // turno =1 juegan blancas; mayusculas=true es una pieza blanca
      // turno =-1 juegan negras; mayusculas=false es una pieza negra
      if (obj_tab.turno == 1 && es_mayus) || (obj_tab.turno == -1 && !es_mayus) {
        if flipped {
          obj_tab.casilla_origen = XY_FLIPPED[seleccionada_fila][seleccionada_columna].to_string();
        }
        else {
          obj_tab.casilla_origen = pos;
        }
        obj_tab.raton_x = x;
        obj_tab.raton_y = y;
      }
      else {
        obj_tab.casilla_origen = "999".to_string();
        obj_tab.raton_x = 0.0;
        obj_tab.raton_y = 0.0;
      }
    }
    else {
      // usamos "999" como casilla inexistente o sin pieza
      obj_tab.casilla_origen = "999".to_string();
      obj_tab.raton_x = 0.0;
      obj_tab.raton_y = 0.0;
    }
  }
}

fn on_pieza_moviendo (widget: &gtk::DrawingArea, 
                event: &gdk::EventMotion) {
  
  let mut obj_tab = VAR_TABLERO.write().unwrap();
  
  if obj_tab.casilla_origen == "999".to_string() {
    obj_tab.raton_x = 0.0;
    obj_tab.raton_y = 0.0;
  }
  else {
    let e = event;
    let (x, y) = gdk::EventMotion::get_position(e); // -> (f64, f64)
    
    obj_tab.raton_x = x;
    obj_tab.raton_y = y;
    
    widget.queue_draw();
  }
}


fn on_pieza_soltada(widget: &gtk::DrawingArea, 
                event: &gdk::EventButton,
                visor_partida: &gtk::TextView) {
  
  /* Final del arrastre de la pieza */
  let mut obj_tab = VAR_TABLERO.write().unwrap();
  // reseteamos la informacion del arrastre
  let casilla_origen = obj_tab.casilla_origen.clone();
  obj_tab.casilla_origen = "999".to_string();
  obj_tab.raton_x = 0.0;
  obj_tab.raton_y = 0.0;
  
  // ahora obtenemos la casilla destino
  let col_tamano = obj_tab.dim_square;
  let fila_tamano = obj_tab.dim_square;
  let (x, y) = event.get_position();
  let seleccionada_columna = (x / col_tamano) as usize;
  let seleccionada_fila = 7 - (y / fila_tamano) as usize;
  
  let casilla_destino: String;
  if obj_tab.flipped {
    casilla_destino = XY_FLIPPED[seleccionada_fila][seleccionada_columna].to_string();
  }
  else {
    casilla_destino = tablero_interno::alfa_notacion((seleccionada_fila, seleccionada_columna));
  }
  
  let mut board = ajedrez::Tablero::init();
  let fen = obj_tab.fen_actual.clone();
  let fen_anterior = fen.as_str();
  let fen_valida = ajedrez::set_fen(fen_anterior, &mut board);
  
  if fen_valida {
    // obtenemos la pieza de promocion del peon como uns str
    let car: char;
    unsafe {
      car = promocion::PROMOCION;
    }
    let mut b = [0; 1];
    let carstr: &str = car.encode_utf8(&mut b);
    
    // realizamos el movimiento
    let movim = (casilla_origen.as_str(), casilla_destino.as_str(), carstr);		// ("e2", "e4", "");
    let result = ajedrez::mueve_algebra(&mut board, movim);
    
    if result.0 != "None" {
      let vfen = ajedrez::get_fen(&mut board);
      
      obj_tab.fen_actual = vfen.clone();
      obj_tab.turno = board.to_move;
      
      let grafico = ajedrez::tablero_grafico(&mut board);
      let tablero_interno = tablero_interno::procesa_notacion(grafico, obj_tab.flipped);
      
      obj_tab.tablero_interno = tablero_interno;
      widget.queue_draw();
      // aqui hay que manejar la SAN y añadirlo a la lista
      visor_partida.grab_focus();
      crea_movim::jugada_tablero(result, vfen, visor_partida, widget);
    }
    else {
      widget.queue_draw();
    }
  }
  else {
    widget.queue_draw();
  }
}
