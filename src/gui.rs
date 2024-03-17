extern crate gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, Grid, Label};

pub fn run_gui() {
    // Application
    let app = Application::builder()
        .application_id("com.rinvel0.whistscore")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Whist Score Keeper")
            .default_width(400)
            .default_height(300)
            .build();

        let grid = Grid::builder()
            .margin(10)
            .column_spacing(5)
            .row_spacing(5)
            .build();

        let entry = Entry::builder().build();
        grid.attach(&entry, 0, 0, 1, 1);

        let button = Button::with_label("Draw Table");
        grid.attach(&button, 1, 0, 1, 1);

        // Create a label to display the table
        let table_label = Label::new(None);
        grid.attach(&table_label, 0, 1, 2, 1);

        // Connect button click event
        button.connect_clicked(move |_| {
            // Get numbers from entry
            let numbers_str = entry.text().trim().to_string();
            let numbers: Vec<&str> = numbers_str.split_whitespace().collect();

            // Create a table string
            let mut table_str = String::new();
            for num in &numbers {
                table_str.push_str(&format!("{}\n", num));
            }

            // Set the table text
            table_label.set_text(&table_str);
        });

        window.set_child(Some(&grid));
        window.show_all();
    });

    app.run();
}
