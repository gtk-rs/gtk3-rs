// Take a look at the license at the top of the repository in the LICENSE file.

use crate::keys::Key;
use crate::Keymap;
use crate::KeymapKey;
use crate::ModifierType;
use glib::translate::*;
use std::mem;
use std::ptr;

impl Keymap {
    /// Returns the keyvals bound to `hardware_keycode`.
    /// The Nth [KeymapKey](crate::KeymapKey) in `keys` is bound to the Nth
    /// keyval in `keyvals`. Free the returned arrays with `g_free`.
    /// When a keycode is pressed by the user, the keyval from
    /// this list of entries is selected by considering the effective
    /// keyboard group and level. See [Keymap::translate_keyboard_state](crate::Keymap::translate_keyboard_state).
    /// ## `hardware_keycode`
    /// a keycode
    ///
    /// # Returns
    ///
    /// [`true`] if there were any entries
    ///
    /// ## `keys`
    /// return
    ///  location for array of [KeymapKey](crate::KeymapKey), or [`None`]
    ///
    /// ## `keyvals`
    /// return
    ///  location for array of keyvals, or [`None`]
    #[doc(alias = "gdk_keymap_get_entries_for_keycode")]
    #[doc(alias = "get_entries_for_keycode")]
    pub fn entries_for_keycode(&self, hardware_keycode: u32) -> Vec<(KeymapKey, u32)> {
        unsafe {
            let mut keys = ptr::null_mut();
            let mut keyvals = ptr::null_mut();
            let mut n_entries = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_keymap_get_entries_for_keycode(
                self.to_glib_none().0,
                hardware_keycode,
                &mut keys,
                &mut keyvals,
                n_entries.as_mut_ptr(),
            ));
            if ret {
                let n_entries = n_entries.assume_init() as usize;
                let mut entries = Vec::with_capacity(n_entries);
                for i in 0..n_entries {
                    entries.push((from_glib_none(keys.add(i)), ptr::read(keyvals.add(i))));
                }
                glib::ffi::g_free(keys as *mut _);
                glib::ffi::g_free(keyvals as *mut _);
                entries
            } else {
                Vec::new()
            }
        }
    }

    /// Obtains a list of keycode/group/level combinations that will
    /// generate `keyval`. Groups and levels are two kinds of keyboard mode;
    /// in general, the level determines whether the top or bottom symbol
    /// on a key is used, and the group determines whether the left or
    /// right symbol is used. On US keyboards, the shift key changes the
    /// keyboard level, and there are no groups. A group switch key might
    /// convert a keyboard between Hebrew to English modes, for example.
    /// [EventKey](crate::EventKey) contains a `group` field that indicates the active
    /// keyboard group. The level is computed from the modifier mask.
    /// The returned array should be freed
    /// with `g_free`.
    /// ## `keyval`
    /// a keyval, such as `GDK_KEY_a`, `GDK_KEY_Up`, `GDK_KEY_Return`, etc.
    ///
    /// # Returns
    ///
    /// [`true`] if keys were found and returned
    ///
    /// ## `keys`
    /// return location
    ///  for an array of [KeymapKey](crate::KeymapKey)
    #[doc(alias = "gdk_keymap_get_entries_for_keyval")]
    #[doc(alias = "get_entries_for_keyval")]
    pub fn entries_for_keyval(&self, keyval: u32) -> Vec<KeymapKey> {
        unsafe {
            let mut keys = ptr::null_mut();
            let mut n_keys = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_keymap_get_entries_for_keyval(
                self.to_glib_none().0,
                keyval,
                &mut keys,
                n_keys.as_mut_ptr(),
            ));
            if ret {
                let n_keys = n_keys.assume_init() as usize;
                let mut r_keys = Vec::with_capacity(n_keys);
                for i in 0..n_keys {
                    r_keys.push(from_glib_none(keys.add(i)));
                }
                glib::ffi::g_free(keys as *mut _);
                r_keys
            } else {
                Vec::new()
            }
        }
    }

    /// Maps the non-virtual modifiers (i.e Mod2, Mod3, ...) which are set
    /// in `state` to the virtual modifiers (i.e. Super, Hyper and Meta) and
    /// set the corresponding bits in `state`.
    ///
    /// GDK already does this before delivering key events, but for
    /// compatibility reasons, it only sets the first virtual modifier
    /// it finds, whereas this function sets all matching virtual modifiers.
    ///
    /// This function is useful when matching key events against
    /// accelerators.
    /// ## `state`
    /// pointer to the modifier mask to change
    #[doc(alias = "gdk_keymap_add_virtual_modifiers")]
    pub fn add_virtual_modifiers(&self, state: &mut ModifierType) {
        unsafe {
            let mut s = state.into_glib();
            ffi::gdk_keymap_add_virtual_modifiers(self.to_glib_none().0, &mut s);
            *state = from_glib(s);
        }
    }

    /// Maps the virtual modifiers (i.e. Super, Hyper and Meta) which
    /// are set in `state` to their non-virtual counterparts (i.e. Mod2,
    /// Mod3,...) and set the corresponding bits in `state`.
    ///
    /// This function is useful when matching key events against
    /// accelerators.
    /// ## `state`
    /// pointer to the modifier state to map
    ///
    /// # Returns
    ///
    /// [`false`] if two virtual modifiers were mapped to the
    ///  same non-virtual modifier. Note that [`false`] is also returned
    ///  if a virtual modifier is mapped to a non-virtual modifier that
    ///  was already set in `state`.
    #[doc(alias = "gdk_keymap_map_virtual_modifiers")]
    pub fn map_virtual_modifiers(&self, state: &mut ModifierType) -> bool {
        unsafe {
            let mut s = state.into_glib();
            let ret = from_glib(ffi::gdk_keymap_map_virtual_modifiers(
                self.to_glib_none().0,
                &mut s,
            ));
            *state = from_glib(s);
            ret
        }
    }

    /// Looks up the keyval mapped to a keycode/group/level triplet.
    /// If no keyval is bound to `key`, returns 0. For normal user input,
    /// you want to use [Keymap::translate_keyboard_state](crate::Keymap::translate_keyboard_state) instead of
    /// this function, since the effective group/level may not be
    /// the same as the current keyboard state.
    /// ## `key`
    /// a [KeymapKey](crate::KeymapKey) with keycode, group, and level initialized
    ///
    /// # Returns
    ///
    /// a keyval, or 0 if none was mapped to the given `key`
    #[doc(alias = "gdk_keymap_lookup_key")]
    pub fn lookup_key(&self, key: &KeymapKey) -> Option<Key> {
        let key =
            unsafe { ffi::gdk_keymap_lookup_key(self.to_glib_none().0, key.to_glib_none().0) };
        if key != 0 {
            Some(Key::from(key))
        } else {
            None
        }
    }
}
