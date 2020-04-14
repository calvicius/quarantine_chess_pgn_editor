use gtk::*;

pub fn ayuda_simple () {
    let dialog = gtk::Dialog::new_with_buttons(
                        Some("Ayuda"),
                        None::<&Window>,   // es el parent
                        gtk::DialogFlags::MODAL,
                        &[(&"OK", gtk::ResponseType::Ok)]
                    );
    dialog.set_position(gtk::WindowPosition::CenterAlways);
    let top_area = dialog.get_content_area(); // -> Box
    let hbox = gtk::Box::new(gtk::Orientation::Vertical, 3);
    top_area.pack_start(&hbox, false, true, 3);
    
    let stock = gtk::Image::new_from_icon_name(Some("dialog-information-symbolic"), gtk::IconSize::Dialog);
    hbox.pack_start(&stock, true, true, 3);
    
    let lbl_msg = gtk::Label::new(Some("1.- Las piezas en el tablero se mueven haciendo drag & drop"));
    hbox.pack_start(&lbl_msg, true, true, 3);
    let lbl_msg = gtk::Label::new(Some("2.- Una vez movida la pieza, se actualiza en la ventana de la derecha el movimiento SAN"));
    hbox.pack_start(&lbl_msg, true, true, 3);
    let lbl_msg = gtk::Label::new(Some(".  3.- Pulsando con el boton izquierdo en la entana derecha se reproduce el movimiento en el tablero  ."));
    hbox.pack_start(&lbl_msg, true, true, 3);
    let lbl_msg = gtk::Label::new(Some("3.- Pulsando con el boton derecho se pueden modificar las NAG y los comentarios"));
    hbox.pack_start(&lbl_msg, true, true, 3);
    let lbl_msg = gtk::Label::new(Some("4.- Está pendiente la edición de todos los datos de la cabecera del PGN"));
    hbox.pack_start(&lbl_msg, true, true, 3);
    
    dialog.show_all();
    dialog.run();
    dialog.destroy();
    
}