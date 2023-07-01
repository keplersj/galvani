use adw::prelude::*;
use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};
use gtk::{glib::ExitCode, Box, ListBox, Orientation, SelectionMode};
use reactive_gtk::{build, widget};

fn main() -> ExitCode {
    let application = build! {
        <Application
        application_id="com.example.FirstAdwaitaApp"
        self::connect_activate=|app| {
            let window = build! {
                <ApplicationWindow
                application=app
                title="First App"
                default_width=350
                // add content to window
                content=&widget! {
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
                }
                />
            };
            window.show();
        }
        />
    };

    application.run()
}
