// Example originally taken from https://github.com/subgraph/gtk-layer-shell-rs/blob/master/examples/example.rs
// This is analogous to:
// https://github.com/wmww/gtk-layer-shell/blob/master/example/example.c

// Currently this example is broken and needs to be fixed

extern crate gio;
extern crate gtk;
extern crate gtk_layer_shell;

use gtk::prelude::*;

use std::env::args;

fn activate(application: &gtk::Application) {
    // Create a normal GTK window however you like
    let window = gtk::ApplicationWindow::new(application);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Before the window is first realized, set it up to be a layer surface
    gtk_layer_shell::init_for_window(&window);

    // Order above normal windows
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

    // Push other windows out of the way
    gtk_layer_shell::auto_exclusive_zone_enable(&window);

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Left, 40);
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Right, 40);
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Top, 20);
    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, false);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Bottom, true);

    // Set up a widget
    let label = gtk::Label::new(Some(""));
    label.set_markup("<span font_desc=\"20.0\">GTK Layer Shell example!</span>");
    window.add(&label);
    window.set_border_width(12);
    window.show_all()
}

fn main() {
    let application = gtk::Application::new(Some("com.gtk-layer-example"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        activate(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
