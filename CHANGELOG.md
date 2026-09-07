# Change Log

## [Unreleased]

### Features

- Several new classes and interfaces can now be subclassed:
  - `gtk::CellEditable`
  - `gtk::CellLayout`
  - `gtk::Editable`
  - `gtk::Grid`
  - `gtk::ListStore`
  - `gtk::Orientable`
  - `gtk::Scrollable`
  - `gtk::TreeDragDest`
  - `gtk::TreeDragSource`
  - `gtk::TreeModelFilter`
  - `gtk::TreeModelSort`
  - `gtk::TreeStore`
- New methods and signals:
  - `gtk::MessageDialog::with_markup()`, a constructor whose message is
    interpreted as Pango markup
  - `gtk::Clipboard::connect_owner_change()`
  - `gtk::EntryExt::connect_toggle_direction()` and
    `gtk::EntryExt::emit_toggle_direction()`
  - `atk::AtkObjectExt::accessible_help_text()`,
    `set_accessible_help_text()`,
    `connect_accessible_help_text_notify()`, and
    `connect_accessible_id_notify()`
  - `accessible_id()` and `accessible_help_text()` on the builders for
    `gtk::HeaderBarAccessible`, `gtk::PlugAccessible`, and
    `gtk::SocketAccessible`
- Builders are now generated for `gtk::EventControllerKey`,
  `gtk::EventControllerMotion`, `gtk::EventControllerScroll`,
  `gtk::FileChooserWidgetAccessible`, `gtk::GestureStylus`,
  `gtk::Settings`, and `gtk::ShortcutLabel`.
- New feature flags:
  - `atk`'s `v2_52` adds `atk::AtkObjectExt::help_text()`,
    `set_help_text()`, `connect_attribute_changed()`, and
    `atk::DocumentExt::connect_document_attribute_changed()`.
  - `gtk`'s `gio_v2_80` adds a `version()` method to
    `gtk::Application`'s builder.
  - `gtk`'s `v3_22` enables `gtk::FileChooserExtManual::add_choice()`,
    which was previously unreachable because no crate defined the
    feature.
- `atk::Role` has a new `Switch` variant.
- The hand-written `*ExtManual` and subclass `*ImplExt` traits are no
  longer sealed with a private `sealed::Sealed` supertrait, matching
  the generated `*Ext` traits.  They still cannot be implemented
  downstream, as the blanket impls already prevent it.  `*ImplExt`
  traits are now declared as `FooImplExt: FooImpl`, so their
  `parent_*()` methods are reached through the blanket impl rather than
  as supertrait methods.  Code that uses `gtk::subclass::prelude::*` is
  unaffected, but code that imports subclassing traits individually
  must also import `FooImplExt` to call `parent_*()`.
- Subclassing support for the following have been enhanced:
  - `gtk::Container`: `ContainerImpl` can now declare and implement
    child properties:
      - `child_properties()`
      - `child_property()`
      - `set_child_property()`
  - `gtk::TreeView`: this used to be subclassable, but no vfuncs were
    overridable; now all are.
  - `gtk::Widget`: `WidgetImpl` gained overrides for many
    previously-missing vfuncs:
      - `can_activate_accel()`
      - `event()`
      - `focus()`
      - `focus_in_event()`
      - `focus_out_event()`
      - `grab_broken_event()`
      - `grab_focus()`
      - `grab_notify()`
      - `hide()`
      - `hierarchy_changed()`
      - `key_press_event()`
      - `key_release_event()`
      - `keynav_failed()`
      - `map_event()`
      - `mnemonic_activate()`
      - `move_focus()`
      - `parent_set()`
      - `popup_menu()`
      - `preferred_height_and_baseline_for_width()`
      - `property_notify_event()`
      - `proximity_in_event()`
      - `proximity_out_event()`
      - `query_tooltip()`
      - `queue_draw_region()`
      - `screen_changed()`
      - `selection_clear_event()`
      - `selection_get()`
      - `selection_notify_event()`
      - `selection_received()`
      - `selection_request_event()`
      - `show()`
      - `show_all()`
      - `show_help()`
      - `state_flags_changed()`
      - `style_updated()`
      - `touch_event()`
      - `unmap_event()`
      - `visibility_notify_event()`
    - `WidgetClassSubclassExt::set_accessible_type()` and
      `WidgetClassSubclassExt::set_accessible_role()` allow a subclass
      to declare the accessibility type and role it presents.
    - `WidgetClassSubclassExt::enable_baseline_support()` opts a
      subclass into baseline support.  Subclasses must call this in
      order for `WidgetImpl::preferred_height_and_baseline_for_width()`
      to be used instead of the non-baseline vfuncs.

### Breaking Changes

- All crates depend on v0.22 of the `gtk-rs-core` crates.  Applications
  using `gtk3-rs` 0.18 or below must update any references to those
  crates.
- `atk::Role::PushButton` is now `atk::Role::Button`, following the
  rename in ATK itself.  The value is unchanged, and `atk-sys` still
  exports `ATK_ROLE_PUSH_BUTTON` alongside `ATK_ROLE_BUTTON`.
- The `LastDefined` variants have been removed from `atk::RelationType`,
  `atk::Role`, `atk::TextAttribute`, and `atk::ValueType`, along with
  the corresponding `ATK_*_LAST_DEFINED` constants in `atk-sys`.  These
  are sentinels marking the end of the value range, and aren't useful in
  Rust code.
- Subclasses of `gtk::Stack` must now implement
  `gtk::subclass::prelude::StackImpl`.  Its `IsSubclassable` impl was
  bounded on `ContainerImpl`, so implementing `StackImpl` was documented
  but not actually enforced.
- Every subclassing `Impl` trait now requires the implementing type to
  name the matching class in its `glib::wrapper!` `@extends` list.  For
  example, `ContainerImpl` is now declared as `ContainerImpl: WidgetImpl
  - ObjectSubclass<Type: IsA<Container>>`.  A subclass whose `@extends`
    list skips an ancestor still compiled before, but no longer does;
    add the missing types to it.
- `gtk::Application`'s builder no longer has an `action_group()` method.
  GIO now marks the `action-group` property deprecated since 2.32, which
  is below the GIO version `gtk3-rs` requires, so `gir` no longer
  generates it.  Use the `gio::prelude::ActionMapExt` methods on the
  built `gtk::Application` instead.
- `gtk::RecentInfo::added()`, `gtk::RecentInfo::modified()`, and
  `gtk::RecentInfo::visited()` now return `libc::time_t` instead of
  `libc::c_long`, as does the last element of the tuple returned by
  `gtk::RecentInfo::application_info()`.  The two are the same type on
  64-bit Linux and on macOS, but differ on Windows and on 32-bit targets
  with a 64-bit `time_t`, where the previous signature was wrong.
- `gtk::Widget::can_activate_accel()` now takes a `SignalId` for the
  `signal_id` argument instead of `u32`.
- The closure passed to `gtk::Widget::connect_can_activate_accel()` now
  takes a `SignalId` for the `signal_id` argument instead of a `u32`.
- `can_activate_accel()` and `connect_can_activate_accel()` have moved
  from `gtk::prelude::WidgetExt` to `gtk::prelude::WidgetExtManual`.
  Code using `gtk::prelude::*` is unaffected, but code that imports
  `WidgetExt` by name, or that calls either method through it, must be
  updated.
- `gtk::WindowExt::type_()` has been removed, because it awkwardly
  conflicts with `glib::ObjectExt::type_()`.  Use
  `gtk::WindowExt::window_type()` instead.
- The hand-written `MessageDialogExt` has been renamed to
  `MessageDialogExtManual`, and its methods now match the C functions
  they call: `set_secondary_markup()` is now
  `format_secondary_markup()`, and `set_secondary_text()` is now
  `format_secondary_text()`.  It previously shared its name with the
  generated `MessageDialogExt` while being re-exported as
  `gtk::MessageDialogExt` instead of through the prelude, so importing
  it silently replaced the generated trait.  Both are now in the
  prelude.  Note that `format_secondary_text()` also clears
  `secondary-use-markup`, which the generated `set_secondary_text()`
  property setter does not.
- The opaque class and interface type aliases in `atk-sys`, `gdk-sys`,
  and `gdkx11-sys` are no longer pointers.  For example,
  `GdkFrameClockClass` is now `_GdkFrameClockClass` rather than `*mut
  _GdkFrameClockClass`.
- `gdkx11-sys`: the optional `cairo` feature is gone.  `cairo-sys` is
  now an unconditional dependency, and `v3_24_2` no longer enables it.
