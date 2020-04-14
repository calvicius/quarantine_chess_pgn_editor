use gtk::*;

use super::tablero;


pub fn modif_cabecera () {
    let mut cabecera: tablero::Cabecera;
    unsafe {
        if tablero::CABECERA.is_none() {
            alerta("No hay datos en la cabecera del PGN");
            return
        }
        let cab = tablero::CABECERA.clone().unwrap();
        cabecera = cab;
    }
    
    let dialog = gtk::Dialog::new_with_buttons(
                    Some("Modificar datos de la partida PGN"),
                    None::<&Window>,   // es el parent
                    gtk::DialogFlags::MODAL,
                    &[("Grabar", gtk::ResponseType::Ok), ("Cancelar", gtk::ResponseType::Close)]
                );
    dialog.set_position(gtk::WindowPosition::CenterAlways);
    let top_area = dialog.get_content_area(); // -> Box
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    top_area.pack_start(&vbox, false, true, 10);
    
    let table = gtk::Grid::new();
    table.set_row_spacing(3);
  
    // TODO: function to automate all of this
    let mut fila = 0;
    let lbl_event = gtk::Label::new(Some("Site"));
    let entry_event = gtk::Entry::new();
    entry_event.set_text(&cabecera.event);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_event, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_event, 1, fila, 1, 1);
    
    fila = 1;
    let lbl_date = gtk::Label::new(Some("Date"));
    let entry_date = gtk::Entry::new();
    entry_date.set_text(&cabecera.date);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_date, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_date, 1, fila, 1, 1);
    
    fila = 2;
    let lbl_round = gtk::Label::new(Some("Round"));
    let entry_round = gtk::Entry::new();
    entry_round.set_text(&cabecera.round);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_round, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_round, 1, fila, 1, 1);
    
    fila = 3;
    let lbl_white = gtk::Label::new(Some("White"));
    let entry_white = gtk::Entry::new();
    entry_white.set_text(&cabecera.white);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_white, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_white, 1, fila, 1, 1);
    
    fila = 4;
    let lbl_black = gtk::Label::new(Some("Black"));
    let entry_black = gtk::Entry::new();
    entry_black.set_text(&cabecera.black);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_black, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_black, 1, fila, 1, 1);
    
    fila = 5;
    let lbl_result = gtk::Label::new(Some("Result"));
    let entry_result = gtk::Entry::new();
    entry_result.set_text(&cabecera.result);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_result, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_result, 1, fila, 1, 1);
    
    fila = 6;
    let lbl_eco = gtk::Label::new(Some("ECO"));
    let entry_eco = gtk::Entry::new();
    entry_eco.set_text(&cabecera.eco);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_eco, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_eco, 1, fila, 1, 1);
    
    fila = 7;
    let lbl_w_elo = gtk::Label::new(Some("White ELO"));
    let entry_w_elo = gtk::Entry::new();
    entry_w_elo.set_text(&cabecera.white_elo);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_w_elo, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_w_elo, 1, fila, 1, 1);
    
    fila = 8;
    let lbl_b_elo = gtk::Label::new(Some("Blacl ELO"));
    let entry_b_elo = gtk::Entry::new();
    entry_b_elo.set_text(&cabecera.black_elo);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_b_elo, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_b_elo, 1, fila, 1, 1);
    
    vbox.pack_start(&table, true, true, 20);
    
    dialog.show_all();
    let result = dialog.run();
    if result == gtk::ResponseType::Ok.into() {
        unsafe {
            cabecera.event = entry_event.get_text().unwrap().to_string();   //.as_str()
            cabecera.site = entry_event.get_text().unwrap().to_string();
            cabecera.date = entry_date.get_text().unwrap().to_string();
            cabecera.round = entry_round.get_text().unwrap().to_string();
            cabecera.white = entry_white.get_text().unwrap().to_string();
            cabecera.black = entry_black.get_text().unwrap().to_string();
            cabecera.result = entry_result.get_text().unwrap().to_string();
            cabecera.eco = entry_eco.get_text().unwrap().to_string();
            cabecera.white_elo = entry_w_elo.get_text().unwrap().to_string();
            cabecera.black_elo = entry_b_elo.get_text().unwrap().to_string();
            
            tablero::CABECERA = Some(cabecera);
        }
    }
    
    dialog.destroy();
    
    
}


 fn alerta(msg: &str) { 
    let dialog = gtk::Dialog::new_with_buttons(
                    Some("Alerta"),
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
    
    let lbl_msg = gtk::Label::new(Some(msg));
    hbox.pack_start(&lbl_msg, true, true, 3);
    
    dialog.show_all();
    dialog.run();
    dialog.destroy();
}