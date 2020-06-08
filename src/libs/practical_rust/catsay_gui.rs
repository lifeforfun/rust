extern crate cursive;
extern crate gio;
extern crate gtk;

use cursive::event::Key;
use cursive::traits::Identifiable;
use cursive::views::{Checkbox, Dialog, EditView, ListView, TextView};
use cursive::Cursive; // for .with_id() and .call_on_id()
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Image, Label, Orientation};
use std::env;

// wrap all form fields value in one struct so we can pass around easily
struct CatsayOptions<'a> {
    message: &'a str,
    dead: bool,
}

fn tui_input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Please fill out the form for the cat")
            // setting the title
            .content(
                ListView::new()
                    .child("Message", EditView::new().with_id("message"))
                    .child("Dead?", Checkbox::new().with_id("dead")),
            )
            .button("OK", |s| {
                let message = s
                    .call_on_id("message", |t: &mut EditView| t.get_content())
                    .unwrap();
                let is_dead = s
                    .call_on_id("dead", |t: &mut Checkbox| t.is_checked())
                    .unwrap();
                let options = CatsayOptions {
                    message: &message,
                    dead: is_dead,
                };
                tui_result_step(s, &options);
            }),
    );
}

fn tui_result_step(siv: &mut Cursive, options: &CatsayOptions) {
    let eye = if options.dead { "x" } else { "o" };
    let cat_text = format!(
        "{msg}
             \\
              \\
                /\\_/\\
               ( {eye} {eye} )
               =( I )=
            ",
        msg = options.message,
        eye = eye
    );
    siv.pop_layer();
    siv.add_layer(
        Dialog::around(TextView::new(cat_text))
            .title("The cat says...")
            .button("OK", |s| s.quit()),
    );
}

fn tui() {
    let mut siv: Cursive = Cursive::default();
    // let cat_text = "Meow!
    //  \\
    //   \\
    //     /\\_/\\
    //    ( o o )
    //    =( I )=
    // ";
    // // Declaring the app layout
    // siv.add_layer(
    //     Dialog::around(TextView::new(cat_text))
    //         .button("OK", |s|s.quit())
    // );
    tui_input_step(&mut siv);
    siv.run();
}

fn gui()
{
    let app = Application::new(
        "com.shinglyu.catsay-gui",
        gio::ApplicationFlags::empty()
    ).expect("Failed to initialize GTK.");

    app.connect_startup(|app|{
        let window = ApplicationWindow::new(app);
        window.set_title("Catsay");
        window.set_default_size(350, 70);

        let layout_box = Box::new(Orientation::Vertical, 0);
        let label = Label::new("Meow!

            \\
             \\
        ");
        layout_box.add(&label);
        let cat_image = Image::new_from_file("./meow.jpeg");
        layout_box.add(&cat_image);

        window.add(&layout_box);

        window.connect_delete_event(|win,_|{
            win.destroy();
            // Don't prevent default behavior (i.e. close)
            Inhibit(false)
        });
        window.show_all();
    });

    app.connect_activate(|_|{});
    app.run(&env::args().collect::<Vec<_>>());
}

fn gui_glade()
{
    let application = gtk::Application::new(
        "com.shinglyu.catsay-gui-glade",
        Default::default()
    ).expect("Failed to initialize GTK");
    application.connect_activate(|app|{
        let glade_src = include_str!("layout.glade");
        let builder = gtk::Builder::new_from_string(glade_src);
        let window:gtk::Window = builder
            .get_object("applicationwindow1").unwrap();
        window.set_application(app);

        let message_input: gtk::Entry = builder
            .get_object("message_input").unwrap();
        let button:gtk::Button = builder
            .get_object("generate_btn").unwrap();
        let message_output:gtk::Label = builder
            .get_object("message_output").unwrap();
        let image_output:gtk::Image = builder
            .get_object("image_output").unwrap();
        let image_output_clone = image_output.clone();

        button.connect_clicked(move|_|{
            message_output.set_text(&format!(
                "{}
                \\
                 \\
                ",
                message_input.get_text().unwrap().as_str()
            ));
            image_output_clone.show();
        });

        window.show_all();
        image_output.hide();
    });
    application.run(&env::args().collect::<Vec<_>>());
}

pub fn test() {
//    tui();
//     gui();
    gui_glade();
}
