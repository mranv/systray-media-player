use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Menu, MenuItem};
use libappindicator::{AppIndicator, AppIndicatorStatus};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let app = Application::builder()
        .application_id("com.example.mediaplayer")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("Media Player");
    window.set_default_size(350, 70);

    let mut indicator = AppIndicator::new("mediaplayer", "applications-multimedia");
    indicator.set_status(AppIndicatorStatus::Active);

    let menu = Menu::new();

    let show_item = MenuItem::with_label("Show Media Player");
    menu.append(&show_item);

    let pause_item = MenuItem::with_label("Pause");
    menu.append(&pause_item);

    let stop_item = MenuItem::with_label("Stop");
    menu.append(&stop_item);

    let previous_item = MenuItem::with_label("Previous");
    menu.append(&previous_item);

    let next_item = MenuItem::with_label("Next");
    menu.append(&next_item);

    let record_item = MenuItem::with_label("Record");
    menu.append(&record_item);

    let speed_item = MenuItem::with_label("Speed");
    let speed_submenu = Menu::new();
    let speed_options = ["0.25x", "0.5x", "Normal", "1.25x", "1.5x", "2x"];
    for option in &speed_options {
        let sub_item = MenuItem::with_label(option);
        speed_submenu.append(&sub_item);
    }
    speed_item.set_submenu(Some(&speed_submenu));
    menu.append(&speed_item);

    let increase_volume_item = MenuItem::with_label("Increase Volume");
    menu.append(&increase_volume_item);

    let decrease_volume_item = MenuItem::with_label("Decrease Volume");
    menu.append(&decrease_volume_item);

    let mute_item = MenuItem::with_label("Mute");
    menu.append(&mute_item);

    let open_media_item = MenuItem::with_label("Open Media");
    menu.append(&open_media_item);

    let quit_item = MenuItem::with_label("Quit");
    menu.append(&quit_item);

    quit_item.connect_activate(move |_| {
        gtk::main_quit();
    });

    // Example of connecting an action to a menu item
    let window_weak = window.downgrade();
    show_item.connect_activate(move |_| {
        if let Some(window) = window_weak.upgrade() {
            window.present();
        }
    });

    // Connect other menu items to their respective actions
    pause_item.connect_activate(|_| {
        println!("Pause action triggered");
        // Add your pause logic here
    });

    stop_item.connect_activate(|_| {
        println!("Stop action triggered");
        // Add your stop logic here
    });

    previous_item.connect_activate(|_| {
        println!("Previous track action triggered");
        // Add your previous track logic here
    });

    next_item.connect_activate(|_| {
        println!("Next track action triggered");
        // Add your next track logic here
    });

    record_item.connect_activate(|_| {
        println!("Record action triggered");
        // Add your record logic here
    });

    increase_volume_item.connect_activate(|_| {
        println!("Increase volume action triggered");
        // Add your increase volume logic here
    });

    decrease_volume_item.connect_activate(|_| {
        println!("Decrease volume action triggered");
        // Add your decrease volume logic here
    });

    mute_item.connect_activate(|_| {
        println!("Mute action triggered");
        // Add your mute logic here
    });

    open_media_item.connect_activate(|_| {
        println!("Open media action triggered");
        // Add your open media logic here
    });

    menu.show_all();
    
    let menu_ref = Rc::new(RefCell::new(menu));
    indicator.set_menu(&mut *menu_ref.borrow_mut());

    window.show_all();
}