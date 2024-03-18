use std::io;

mod game;
mod gui;

use game::{create_players, num_of_players, play_round, GameState};

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gui::build_ui;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()

    //play();
}

fn play() {
    //Logic -- to be moved an refactored later
    let num_players = num_of_players();
    println!("Playing with {} players", num_players);

    let mut players = create_players(num_players);

    // Games of 1s
    for i in 0..=num_players - 1 {
        println!("Game of 1 - {} of {}", i + 1, num_players);
        play_round(&mut players, 1, i as usize);
        for player in &players {
            println!(
                "Player: {}, Score: {}, Bid: {}, Hands: {}",
                &player.name, &player.score, &player.bid, &player.hands
            );
        }
    }
    // Games of 2 to 7s
    for i in 0..=5 {
        println!("Game of {}", i + 2);
        play_round(&mut players, i + 2, (i % num_players) as usize);
        for player in &players {
            println!(
                "Player: {}, Score: {}, Bid: {}, Hands: {}",
                &player.name, &player.score, &player.bid, &player.hands
            );
        }
    }
    // Games of 8s
    for i in 0..=num_players - 1 {
        println!("Game of 8 - {} of {}", i + 1, num_players);
        play_round(&mut players, 8, i as usize);
        for player in &players {
            println!(
                "Player: {}, Score: {}, Bid: {}, Hands: {}",
                &player.name, &player.score, &player.bid, &player.hands
            );
        }
    }
    // Games of 7 to 2s
    for i in (0..=5).rev() {
        println!("Game of {}", i + 2);
        play_round(&mut players, i + 2, (i % num_players) as usize);
        for player in &players {
            println!(
                "Player: {}, Score: {}, Bid: {}, Hands: {}",
                &player.name, &player.score, &player.bid, &player.hands
            );
        }
    }
    // Games of 1s
    for i in 0..=num_players - 1 {
        println!("Game of 1 - {} of {}", i + 1, num_players);
        play_round(&mut players, 8, i as usize);
        for player in &players {
            println!(
                "Player: {}, Score: {}, Bid: {}, Hands: {}",
                &player.name, &player.score, &player.bid, &player.hands
            );
        }
    }
}
