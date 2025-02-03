use std::sync::LazyLock;

use cursive::{
  theme::{BaseColor, Color, PaletteColor},
  view::{Resizable, Scrollable},
  views::{Dialog, SelectView, TextView},
  Cursive, With,
};
use cursive_syntect::parse;
use syntect::{easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet};

use super::{home, ApplicationRoot, ApplicationState, RawPtr};
use crate::utils::{
  docs::{self, PackageEntry},
  package::Package,
};

pub fn select_pkg(c: &mut Cursive) {
  while let Some(_) = c.pop_layer() {}

  let page: &mut ApplicationState = c.user_data().unwrap();
  page.step = 1;

  let view = SelectView::new()
    .with(|c| {
      let pkgs = match page.root.unwrap() {
        ApplicationRoot::LeadHome => docs::lead_lib(),
        ApplicationRoot::Workspace => docs::lead_ws(),
      };

      for pkg in pkgs {
        c.add_item(pkg.display.clone(), pkg);
      }
    })
    .on_submit(|c: &mut Cursive, v: &PackageEntry| {
      c.pop_layer();

      let pkg = Package::new(v);

      c.with_user_data(move |c: &mut ApplicationState| {
        c.pkg = Some(pkg);
      });

      open_pkg(c);
    })
    .full_screen();

  c.add_layer(
    Dialog::around(view.scrollable())
      .title("Select package")
      .button("↰ Back", |siv| home(siv))
      .dismiss_button("Close")
      .full_screen(),
  );
}

pub fn open_pkg(c: &mut Cursive) {
  while let Some(_) = c.pop_layer() {}

  let page: &mut ApplicationState = c.user_data().unwrap();
  page.step = 2;

  let pkg = &page.pkg.as_ref().unwrap();

  let doc = &pkg.doc;
  let name = &pkg.name as *const _;

  let view = SelectView::new()
    .with(|c| {
      for (k, _) in doc {
        c.add_item(k, RawPtr(k as &str as *const str));
      }
    })
    .on_submit(|c: &mut Cursive, v: &RawPtr<str>| {
      let st = v.0;
      let state = c.user_data::<ApplicationState>().unwrap();
      state.to_open = Some(RawPtr(st));

      sel_method(c);
    })
    .full_screen();

  c.add_layer(
    Dialog::around(view.scrollable())
      .title(unsafe { &*name })
      .button("↰ Back", |siv| select_pkg(siv))
      .dismiss_button("Close")
      .full_screen(),
  );
}

pub fn sel_method(c: &mut Cursive) {
  while let Some(_) = c.pop_layer() {}

  let page: &mut ApplicationState = c.user_data().unwrap();
  page.step = 3;

  let name = unsafe { &*page.to_open.as_ref().unwrap().0 };

  let pkg = &page.pkg.as_ref().unwrap();

  let doc = &pkg.doc;
  let doc = doc.get(name).unwrap();

  let view = SelectView::new()
    .with(|c| {
      for (k, _) in doc {
        c.add_item(*k, *k);
      }
    })
    .on_submit(|c: &mut Cursive, v: &&str| {
      let page: &mut ApplicationState = c.user_data().unwrap();
      page.to_show_doc = Some(RawPtr(*v as _));

      show_doc(c);
    })
    .full_screen();

  c.add_layer(
    Dialog::around(view.scrollable())
      .title(name)
      .button("↰ Back", |siv| open_pkg(siv))
      .dismiss_button("Close")
      .full_screen(),
  );
}

static SYNTAX: LazyLock<SyntaxSet> = LazyLock::new(|| SyntaxSet::load_defaults_newlines());
static THEMES: LazyLock<ThemeSet> = LazyLock::new(|| ThemeSet::load_defaults());

pub fn show_doc(c: &mut Cursive) {
  while let Some(_) = c.pop_layer() {}

  let page: &mut ApplicationState = c.user_data().unwrap();
  page.step = 4;

  let name = unsafe { &*page.to_open.as_ref().unwrap().0 };

  let pkg = &page.pkg.as_ref().unwrap();

  let doc = &pkg.doc;

  let doc = doc.get(name).unwrap();
  let doc = doc
    .get(unsafe { &*page.to_show_doc.as_ref().unwrap().0 })
    .unwrap();

  // Important to make rust not mad
  let doc = doc as *const &str;
  let doc = unsafe { *doc };

  let syntax = SYNTAX.find_syntax_by_token("md").unwrap();
  let mut theme = THEMES.themes["InspiredGitHub"].clone();

  let (r, g, b, a) = match c.current_theme().palette[PaletteColor::View] {
    Color::TerminalDefault => (0, 0, 0, 0),
    Color::Rgb(r, g, b) | Color::RgbLowRes(r, g, b) => (r, g, b, 255),
    Color::Dark(c) | Color::Light(c) => match c {
      BaseColor::Black => (0, 0, 0, 0),
      BaseColor::Blue => (36, 114, 200, 255),
      BaseColor::Cyan => (17, 168, 205, 255),
      BaseColor::Green => (13, 188, 121, 255),
      BaseColor::Magenta => (188, 63, 188, 255),
      BaseColor::Red => (205, 49, 49, 255),
      BaseColor::White => (229, 229, 229, 255),
      BaseColor::Yellow => (229, 229, 16, 255),
    },
  };

  let color = theme.settings.background.as_mut().unwrap();

  color.a = a;
  color.r = r;
  color.g = g;
  color.b = b;

  let mut highlighter = HighlightLines::new(syntax, &theme);

  c.add_layer(
    Dialog::around(
      TextView::new(parse(doc, &mut highlighter, &*SYNTAX).unwrap())
        .full_screen()
        .scrollable(),
    )
    .title(name)
    .button("↰ Back", |siv| {
      let theme = siv.user_data::<ApplicationState>().unwrap().theme.clone();
      siv.set_theme(theme);
      sel_method(siv);
    })
    .button("Close", |siv| {
      let theme = siv.user_data::<ApplicationState>().unwrap().theme.clone();
      
      siv.set_theme(theme);
      siv.pop_layer();
    })
    .full_screen(),
  );
}
