extern crate gdk;
extern crate gdk_sys;

#[test]
fn check_event() {
    gdk::init();
    let base_ev = gdk::Event::new(gdk::EventType::KeyPress);
    let mut ev: gdk::EventKey = base_ev.downcast().unwrap();
    ev.as_mut().keyval = *gdk::keys::constants::A;

    let keyval = gdk::keyval_to_unicode(*ev.get_keyval());
    let keyval2 = ev.get_keyval().to_unicode();

    assert_eq!(keyval, Some('A'));
    assert_eq!(keyval, keyval2);

    let name = gdk::keyval_name(*ev.get_keyval());
    let name2 = ev.get_keyval().name();

    assert_eq!(name, Some("A".into()));
    assert_eq!(name, name2);
}
