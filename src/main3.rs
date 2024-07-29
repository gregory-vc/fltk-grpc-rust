use fltk::{prelude::*, *};

static COFACTOR: utils::oncelock::Lazy<i32> = utils::oncelock::Lazy::new(|| (app::font_size() as f64 * 2.0) as i32);

fn prep_tree(t: &mut tree::Tree) {
    if let Some(root) = t.next(&t.first().unwrap()) {
        if root.is_open() {
            let elems = root.children();
            t.resize(t.x(), t.y(), t.w(), (elems + 1) * *COFACTOR);
        } else {
            t.set_scrollbar_size(0);
            t.resize(t.x(), t.y(), t.w(), *COFACTOR);
        }
    } else {
        t.resize(t.x(), t.y(), t.w(), *COFACTOR);
    }
    app::redraw();
}

struct MyTree {
    t: tree::Tree,
}

impl MyTree {
    pub fn default() -> Self {
        let mut t = tree::Tree::default();
        t.set_show_root(false);
        t.set_callback(prep_tree);
        Self {
            t
        }
    }
    pub fn end(&mut self) {
        prep_tree(&mut self.t);
    }
}

widget_extends!(MyTree, tree::Tree, t);

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(800, 600);
    let mut row = group::Flex::default_fill().row();
    let mut scroll = group::Scroll::default();
    let mut col = group::Pack::default().with_type(group::PackType::Vertical);
    let mut t = MyTree::default();
    t.add("Source Control");
    for i in 1..4 {
        t.add(&format!("Source Control/Repo {}", i));
    }
    t.end();
    let mut t = MyTree::default();
    t.add("Commits");
    for i in 1..30 {
        t.add(&format!("Commits/Commit {}", i));
    }
    t.end();
    let mut t = MyTree::default();
    t.add("Branches");
    t.add("Branches/main");
    t.add("Branches/dev");
    t.end();
    col.end();
    scroll.end();
    scroll.resizable(&col);
    row.fixed(&scroll, 200);
    row.end();
    win.end();
    win.show_with_env_args();
    col.resize(scroll.x(), scroll.y(), scroll.w(), scroll.h());
    a.run().unwrap();
}