use gtk::glib::GString;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Entry, Grid, Label};

pub fn build_ui(app: &Application) {
    // Grid
    let grid = Grid::builder()
        .visible(true)
        .column_spacing(12)
        .visible(true)
        .row_spacing(5)
        .build();
    // Player Name Entry
    let player_entry = Entry::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    grid.attach(&player_entry, 0, 0, 1, 1);
    // Create a button with label and margins
    let button = Button::builder()
        .label("Add Player")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    grid.attach(&button, 1, 0, 1, 1);

    let player1_label = Label::new(None);
    grid.attach(&player1_label, 0, 1, 15, 1);
    let player2_label = Label::new(None);
    grid.attach(&player2_label, 1, 1, 15, 1);
    let player3_label = Label::new(None);
    grid.attach(&player3_label, 2, 1, 15, 1);
    let player4_label = Label::new(None);
    grid.attach(&player4_label, 3, 1, 15, 1);

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        let text = &player_entry.text();
        player1_label.set_text(text);
        player2_label.set_text(text);
        player3_label.set_text(text);
        player4_label.set_text(text);
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&grid)
        .build();

    // Present window
    window.present();
}

