use cursive::{
  event::Key, menu::Tree, theme::Theme, view::{Resizable, Scrollable}, views::{Dialog, SelectView, TextContent, TextView}, Cursive, CursiveExt
};
use select::{open_pkg, sel_method, select_pkg, show_doc};

use crate::utils::package::Package;

mod select;

pub struct RawPtr<T: ?Sized>(*const T);

unsafe impl<T: ?Sized> Send for RawPtr<T> {}
unsafe impl<T: ?Sized> Sync for RawPtr<T> {}

pub struct ApplicationState {
  step: u8,
  root: Option<ApplicationRoot>,
  pkg: Option<Package>,
  to_open: Option<RawPtr<str>>,
  to_show_doc: Option<RawPtr<str>>,
  theme: Theme,
}

#[derive(Debug, Clone, Copy)]
pub enum ApplicationRoot {
  LeadHome,
  Workspace,
}

pub fn run_cursive() {
  let mut siv = Cursive::new();

  siv.set_user_data(ApplicationState {
    step: 0,
    root: None,
    pkg: None,
    to_open: None,
    to_show_doc: None,
    theme: Theme::retro(),
  });

  siv.set_autohide_menu(false);

  siv
    .menubar()
    .add_subtree(
      "📦 Lead Docs",
      Tree::new()
        .leaf(format!("🖥 {}", env!("CARGO_PKG_VERSION")), |_| {})
        .leaf("🛈 About", |c| {
          c.add_layer(
          Dialog::new()
            .content(
            TextView::new(
                format!("Lead Docs TUI\n{}\n\nMade with ♥ by the Lead Language team\n\n© The Lead Programming Language 2025\nhttps://leadlang.github.io", env!("CARGO_PKG_VERSION"))
              )
              .center()
              .scrollable()
            )
            .dismiss_button("Ok"),
          );
        })
        .delimiter()
        .leaf("x Quit", |c| {
          c.add_layer(
            Dialog::new()
              .content(TextView::new_with_content(TextContent::new(
                "Are you sure you want to close?",
              )))
              .dismiss_button("No")
              .button("Yes", |c| c.quit()),
          );
        })
    )
    .add_delimiter()
    .add_leaf("⌂ Home", |c| {
      while let Some(_) = c.pop_layer() {}
      home(c);
    })
    .add_leaf("↰ Back", |c| {
      while let Some(_) = c.pop_layer() {}

      let data = c.user_data::<ApplicationState>().unwrap();

      match data.step {
        0 => {
          c.add_layer(
            Dialog::around(TextView::new("Cannot go back! You're on the first page."))
              .dismiss_button("Ok"),
          );
        }
        1 => home(c),
        2 => select_pkg(c),
        3 => open_pkg(c),
        4 => sel_method(c),
        5 => show_doc(c),
        _ => {}
      }
    })
    .add_leaf("x Quit", |c| {
      c.add_layer(
        Dialog::new()
          .content(TextView::new_with_content(TextContent::new(
            "Are you sure you want to close?",
          )))
          .dismiss_button("No")
          .button("Yes", |c| c.quit()),
      );
    })
    // .add_subtree(
    //   "☁ Theme",
    //   Tree::new()
    //     .leaf("☈ Default", |c| {
    //       c.set_theme(Theme::retro());
    //       c.user_data::<ApplicationState>().unwrap().theme = Theme::retro();
    //     })
    //     .leaf("☀ Bicolor Terminal", |c| {
    //       c.set_theme(Theme::terminal_default());
    //       c.user_data::<ApplicationState>().unwrap().theme = Theme::terminal_default();
    //     }),
    // )
    .add_delimiter()
    .add_leaf("<select>", |_| {});

  home(&mut siv);

  siv.add_global_callback(Key::Tab, |s| s.select_menubar());
  siv.run();
}

pub fn home(siv: &mut Cursive) {
  while let Some(_) = siv.pop_layer() {}

  let len = siv.menubar().len();
  siv.menubar().remove(len - 1);
  siv.menubar().add_leaf("<select>", |_| {});

  siv.user_data::<ApplicationState>().unwrap().step = 0;

  let handle = |c: &mut Cursive, root: ApplicationRoot| {
    while let Some(_) = c.pop_layer() {}

    let len = c.menubar().len();
    c.menubar().remove(len - 1);

    c.menubar().add_leaf(
      match root {
        ApplicationRoot::LeadHome => "📚 Lead Default",
        ApplicationRoot::Workspace => "📦 Workspace",
      },
      |_| {},
    );

    c.with_user_data(move |c: &mut ApplicationState| {
      c.root = Some(root);
    });

    select::select_pkg(c);
  };

  siv.add_layer(
    Dialog::around(
      SelectView::new()
        .item("📚 Lead Default", ApplicationRoot::LeadHome)
        .item("📦 Workspace", ApplicationRoot::Workspace)
        .on_submit(move |c, v| {
          handle(c, *v);
        })
        .scrollable()
        .fixed_size((20, 6)),
    )
    .title("Select")
    .dismiss_button("Close"),
  );
}
