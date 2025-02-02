use cursive::{utils::markup::markdown::parse, view::{Resizable, Scrollable}, views::{Dialog, SelectView, TextView}, Cursive, With};

use crate::utils::{docs::{self, PackageEntry}, package::Package};
use super::{ApplicationRoot, ApplicationState, RawPtr};

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
    Dialog::around(
      view
    )
    .title("Select package")
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
    Dialog::around(
      view
    )
    .title(unsafe { &*name })
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
    Dialog::around(
      view
    )
    .title(name)
    .dismiss_button("Close")
    .full_screen(),
  );
}

pub fn show_doc(c: &mut Cursive) {
  while let Some(_) = c.pop_layer() {}

  let page: &mut ApplicationState = c.user_data().unwrap();
  page.step = 4;

  let name = unsafe { &*page.to_open.as_ref().unwrap().0 };

  let pkg = &page.pkg.as_ref().unwrap();

  let doc = &pkg.doc;
  let doc = doc.get(name).unwrap();
  let doc = doc.get(unsafe { &*page.to_show_doc.as_ref().unwrap().0 }).unwrap();

  let parsed = parse(doc.replace("\n\n", "\n").replace("\n", "\n\n"));
  
  c.add_layer( 
    Dialog::around(
      TextView::new(parsed)
        .full_screen()
        .scrollable()
    )
    .title(name)
    .dismiss_button("Close")
    .full_screen(),
  );
}