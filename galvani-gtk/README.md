# galvani-gtk

This is an experiment using a React Native-inspired syntax to build GTK UIs in Rust.

Currently, with this experiment, the sample Adwaita application goes from:

```rs
use adw::prelude::*;

use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};
use gtk::{Box, ListBox, Orientation, SelectionMode};

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    application.connect_activate(|app| {
        // ActionRows are only available in Adwaita
        let row = ActionRow::builder()
            .activatable(true)
            .title("Click me")
            .build();
        row.connect_activated(|_| {
            eprintln!("Clicked!");
        });

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(SelectionMode::None)
            // makes the list look nicer
            .css_classes(vec![String::from("boxed-list")])
            .build();
        list.append(&row);

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
        content.append(&HeaderBar::new());
        content.append(&list);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("First App")
            .default_width(350)
            // add content to window
            .content(&content)
            .build();
        window.show();
    });

    application.run();
}
```

To:

```rs
use adw::prelude::*;
use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};
use gtk::{glib::ExitCode, Box, ListBox, Orientation, SelectionMode};
use reactive_gtk::{build, widget};

fn main() -> ExitCode {
    build! {
        <Application
            application_id="com.example.FirstAdwaitaApp"
            self::connect_activate=|app| {
                build! {
                    <ApplicationWindow
                        application=app
                        title="First App"
                        default_width=350
                        // add content to window
                        content=&widget! {
                            // Combine the content in a box
                            <Box orientation=Orientation::Vertical>
                                // Adwaitas' ApplicationWindow does not include a HeaderBar
                                <HeaderBar />
                                <ListBox
                                    margin_top=32
                                    margin_end=32
                                    margin_bottom=32
                                    margin_start=32
                                    selection_mode=SelectionMode::None
                                    // makes the list look nicer
                                    css_classes=vec![String::from("boxed-list")]
                                >
                                    // ActionRows are only available in Adwaita
                                    <ActionRow
                                        activatable=true
                                        title="Click me"
                                        self::connect_activated=|_| {
                                            eprintln!("Clicked!");
                                        }
                                    />
                                </ListBox>
                            </Box>
                        }
                    />
                }.show();
            }
        />
    }
    .run()
}
```
