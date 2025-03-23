use std::{collections::HashMap, sync::LazyLock};

use cursive::{
  theme::{BaseColor, Color, PaletteColor},
  view::{Resizable, Scrollable},
  views::{Dialog, SelectView, TextView},
  Cursive, With,
};
use cursive_syntect::parse;
use syntect::{easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet};

use super::{home, ApplicationRoot, ApplicationState, RawPtr, TypeOfAction};
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

      select_rt_or_fn(c);
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

pub fn select_rt_or_fn(c: &mut Cursive) {
  while let Some(_) = c.pop_layer() {}

  let page: &mut ApplicationState = c.user_data().unwrap();
  page.step = 2;

  let handle = |c: &mut Cursive, r#type: TypeOfAction| {
    while let Some(_) = c.pop_layer() {}

    c.with_user_data(move |c: &mut ApplicationState| {
      c.r#type = Some(r#type);
    });

    open_pkg(c);
  };

  c.add_layer(
    Dialog::around(
      SelectView::new()
        .item("📚 Functions", TypeOfAction::Function)
        .item("📦 Runtime Values", TypeOfAction::RuntimeValue)
        .on_submit(move |c, v| {
          handle(c, *v);
        })
        .scrollable()
        .fixed_size((20, 6)),
    )
    .title("Select")
    .button("↰ Back", |siv| select_pkg(siv))
    .dismiss_button("Close"),
  );
}

fn iterate<'a>(app: *const ApplicationState) -> Box<dyn Iterator<Item = ((String, String), &'a HashMap<&'a str, &'a str>)>> {
  let app = unsafe { &*app };

  let rt = app.pkg.as_ref().unwrap();

  match app.r#type.as_ref().unwrap() {
    TypeOfAction::Function => Box::new(rt.doc.iter().map(|(k, v)| ((k.to_owned(), k.to_owned()), v))),
    TypeOfAction::RuntimeValue => {
      Box::new(
        rt.runtimes
          .iter()
          .map(|(k1, (k2, data))| 
            ((k1.to_string(), format!("{k2} / {k1}")), data)
          )
      )
    }
  }
}

pub fn open_pkg(c: &mut Cursive) {
  while let Some(_) = c.pop_layer() {}

  let page: &mut ApplicationState = c.user_data().unwrap();
  page.step = 3;

  let pkg = iterate(page);

  let name = &page.pkg.as_ref().unwrap().name as *const _;

  let view = SelectView::new()
    .with(|c| {
      for ((k1, k2), _) in pkg {
        c.add_item(k2, k1);
      }
    })
    .on_submit(|c: &mut Cursive, v: &String| {
      let state = c.user_data::<ApplicationState>().unwrap();
      state.to_open = Some(v.clone());

      sel_method(c);
    })
    .full_screen();

  c.add_layer(
    Dialog::around(view.scrollable())
      .title(unsafe { &*name })
      .button("↰ Back", |siv| select_rt_or_fn(siv))
      .dismiss_button("Close")
      .full_screen(),
  );
}

pub fn sel_method(c: &mut Cursive) {
  while let Some(_) = c.pop_layer() {}

  let page: &mut ApplicationState = unsafe { &mut *(c as *mut _) as &mut Cursive }.user_data().unwrap();
  page.step = 5;

  let name = &*page.to_open.as_ref().unwrap() as &str;

  let doc = match page.r#type.as_ref().unwrap() {
    TypeOfAction::RuntimeValue => {
      page.pkg.as_ref().unwrap().runtimes.get(name).and_then(|x| Some(&x.1))
    },
    TypeOfAction::Function => page.pkg.as_ref().unwrap().doc.get(name),
  }.unwrap();

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

  let page: &mut ApplicationState = unsafe { &mut *(c as *mut _) as &mut Cursive }.user_data().unwrap();
  page.step = 6;

  let name = &*page.to_open.as_ref().unwrap() as &str;

  let doc = match page.r#type.as_ref().unwrap() {
    TypeOfAction::RuntimeValue => page.pkg.as_ref().unwrap().runtimes.get(name).and_then(|x| Some(&x.1)),
    TypeOfAction::Function => page.pkg.as_ref().unwrap().doc.get(name),
  }.unwrap();

  let doc = doc.get(unsafe { &*page.to_show_doc.as_ref().unwrap().0 as &str }).unwrap();

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
