use adw::prelude::*;

use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};
use gtk::{glib::ExitCode, Box, ListBox, Orientation, SelectionMode};
use reactive_gtk::widget;

fn main() -> ExitCode {
    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    application.connect_activate(|app| {
        let content = widget! {
            // Combine the content in a box
            <Box
                orientation=Orientation::Vertical>
                // Adwaitas' ApplicationWindow does not include a HeaderBar
                <HeaderBar />
                <ListBox
                    margin_top=32
                    margin_end=32
                    margin_bottom=32
                    margin_start=32
                    selection_mode=SelectionMode::None
                    // makes the list look nicer
                    css_classes=vec![String::from("boxed-list")]>
                    // ActionRows are only available in Adwaita
                    <ActionRow
                        activatable=true
                        title="Click me"
                        self::connect_activated=|_| {
                            eprintln!("Clicked!");
                        }/>
                </ListBox>
            </Box>
        };

        let window = ApplicationWindow::builder()
            .application(app)
            .title("First App")
            .default_width(350)
            // add content to window
            .content(&content)
            .build();
        window.show();
    });

    application.run()
}
