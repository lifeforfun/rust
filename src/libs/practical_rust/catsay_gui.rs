extern crate cursive;

use cursive::event::Key;
use cursive::traits::Identifiable;
use cursive::views::{Checkbox, Dialog, EditView, ListView, TextView};
use cursive::Cursive; // for .with_id() and .call_on_id()

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

pub fn test() {
    tui();
}
