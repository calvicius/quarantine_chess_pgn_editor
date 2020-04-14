use gtk::*;

pub fn lee_motor () {
    let dialog = gtk::Dialog::new_with_buttons(
                        Some("Engine Comunication Dialog"),
                        None::<&Window>,   // es el parent
                        gtk::DialogFlags::MODAL,
                        &[(&"OK", gtk::ResponseType::Ok)]
                    );
    dialog.set_position(gtk::WindowPosition::CenterAlways);
    let top_area = dialog.get_content_area(); // -> Box
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 3);
    top_area.pack_start(&hbox, false, true, 3);
    
    let stock = gtk::Image::new_from_icon_name(Some("dialog-warning-symbolic"), gtk::IconSize::Dialog);
    hbox.pack_start(&stock, true, true, 3);
    
    let lbl_msg = gtk::Label::new(Some("Diálogo con el motor está pendiente Ver:"));
    hbox.pack_start(&lbl_msg, true, true, 3);
    let lbl_msg = gtk::Label::new(Some("https://github.com/calvicius/rs_gtk_chess_uci_interpreter"));
    hbox.pack_start(&lbl_msg, true, true, 3);
    
    dialog.show_all();
    dialog.run();
    dialog.destroy();
    
}