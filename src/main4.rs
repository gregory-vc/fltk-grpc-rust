use fltk::{prelude::*, *};

const FILE: &[&str] = &[
    "13 11 3 1",
    "   c None",
    "x  c #ffffff",
    "@  c #202060",
    "   @@@@@@@   ",
    "  @xxxxxx@   ",
    " @xxxxxxx@   ",
    " @xxxxxxx@   ",
    " @xxxxxxx@   ",
    " @xxxxxxx@   ",
    " @xxxxxxx@   ",
    " @xxxxxxx@   ",
    " @xxxxxxx@   ",
    " @xxxxxxx@   ",
    " @@@@@@@@@   ",
];

fn main() {
    let icon = image::Pixmap::new(FILE).unwrap();

    let app_root = app::App::default();

    let mut win = window::Window::default()
        .with_size(300, 400)
        .center_screen();

    let mut tree = tree::Tree::default().size_of_parent().center_of_parent();
    tree.set_show_root(false);
    tree.set_item_draw_mode(tree::TreeItemDrawMode::LabelAndWidget);
    tree.set_widget_margin_left(0);

    tree.add("Route A");
    tree.add("Route A/Node 1");
    tree.add("Route A/Node 2");
    tree.add("Route B");
    tree.add("Route B/Node 1");

    let mut btn = frame::Frame::default().with_size(14, 14);
    btn.set_image(Some(icon));
    btn.handle(|_, evt| match evt {
        enums::Event::Push => {
            println!("button pressed");
            true
        }
        _ => false,
    });

    let mut item = tree::TreeItem::new(&tree, "Example Node");
    item.draw_item_content(|item, render| {
        let x = item.label_x();
        let y = item.label_y();
        let w = item.label_w();
        let h = item.label_h();
        let txt = match item.label() {
            Some(s) => s,
            None => String::new(),
        };
        let txt_len = draw::measure(&txt, false).0;

        if render {
            if item.is_selected() {
                draw::draw_rect_fill(x, y, w, h, enums::Color::DarkBlue);
                draw::set_draw_color(enums::Color::White);
            } else {
                draw::draw_rect_fill(x, y, w, h, item.label_bgcolor());
                draw::set_draw_color(item.label_fgcolor());
            }

            draw::draw_text2(&txt, x, y, w, h, enums::Align::Left);

            if let Some(mut wid) = item.try_widget() {
                wid.set_damage(true);
            }

            if let Some(mut wid) = item.try_widget() {
                // the widget needs to be manually positioned during each render
                let wx = (x + txt_len + 20).max(x + (w - 20));
                wid.resize(wx, wid.y(), 14, wid.h());
            }
        }

        // this returned value has little effect in this context
        x + txt_len
    });
    item.set_widget(&btn);

    tree.add_item("Route A/234234234", &item);

    win.end();
    win.make_resizable(true);
    win.show();

    app_root.run().unwrap();
}
