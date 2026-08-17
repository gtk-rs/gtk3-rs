(function() {
    var type_impls = Object.fromEntries([["atk_sys",[]],["gdk_sys",[]],["gdk_x11_sys",[]],["gtk_sys",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[14,15,19,15]}