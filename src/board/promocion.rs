use gtk::*;


pub static mut PROMOCION: char = 'Q';

pub struct Promocion {
  pub promocion: gtk::ComboBox,
}


impl Promocion {
  pub fn init () -> Self {
    let lista: Vec<(&str, &str)> = vec![
        ("Q", "Dama"),
        ("R", "Torre"),
        ("B", "Alfil"),
        ("N", "Caballo")
    ];
    // Setup combobox
    let combobox = gtk::ComboBox::new();
    let model = gtk::ListStore::new(&[glib::Type::String, glib::Type::String]);
    combobox.set_model(Some(&model));
    combobox.set_vexpand(false);
    combobox.set_hexpand(false);
    
    let cell_renderer0 = gtk::CellRendererText::new();
    // https://gtk-rs.org/docs/gtk/trait.CellLayoutExt.html#tymethod.pack_start
    combobox.pack_start(&cell_renderer0, false);
    combobox.add_attribute(&cell_renderer0, "text", 0);

    let cell_renderer1 = gtk::CellRendererText::new();
    combobox.pack_start(&cell_renderer1, false);
    combobox.add_attribute(&cell_renderer1, "text", 1);
    
    // poblamos con los datos
    for data in lista {
        let iter = model.append(); 
        //model.set(&iter, &[0, 1], &[&data.0, &data.1]);
        gtk::prelude::GtkListStoreExtManual::set(&model, &iter, &[0, 1], &[&data.0, &data.1]);
    }
    
    // Ahora los closures de señales
    {
      //let weak_model = model.downgrade();
      let weak_model = glib::object::ObjectExt::downgrade(&model);
      combobox.connect_changed ( move |widget| {
          let model = match weak_model.upgrade() {
              Some(model) => model,
              None => return,
          };
          let iter = widget.get_active_iter();
          let mut inicial: String;
          
          match iter {
              Some(iter) => {
                  let col0 = model.get_value(&iter, 0);
                  //let col1 = model.get_value(&iter, 1);
                  // necesitamos traducir estos valores para ser entendibles
                  inicial = col0.get::<String>().unwrap().unwrap(); // is Ok(Some("txt"))
                 },
              None => {inicial = "Q".to_string();},
          };
          
          unsafe {
            let ini: char = inicial.remove(0);
            PROMOCION = ini;
          }
          
      });
    }
    
    Promocion {
      promocion: combobox,
    }
  }
}