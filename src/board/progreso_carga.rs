use gtk::*;

pub struct PopUp {
    pub win_popup: gtk::Window,
}


impl PopUp {
  pub fn init(etiq: &str) -> Self {
    let win_popup = gtk::Window::new(gtk::WindowType::Popup);
    win_popup.set_size_request(250, 140);
    win_popup.set_position(gtk::WindowPosition::CenterAlways);
    win_popup.set_border_width(20);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    win_popup.add(&vbox);
    
    let pb = gtk::ProgressBar::new();
    pb.set_text(Some(etiq));
    pb.set_show_text(true);
    pb.pulse();
    vbox.pack_start(&pb, false, false, 20);
    
    let tick = move || {
      pb.pulse();
      glib::Continue(true)
    };
    
    gtk::timeout_add(50, tick);  // 50 milsegundos
    
    win_popup.show_all();
    
    PopUp {
      win_popup
    }
  }
  
  pub fn destruye (self) {
    self.win_popup.destroy();
  }
}