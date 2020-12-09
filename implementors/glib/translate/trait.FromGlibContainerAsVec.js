(function() {var implementors = {};
implementors["gdk"] = [{"text":"impl FromGlibContainerAsVec&lt;*mut _GdkAtom, *mut *mut _GdkAtom&gt; for Atom","synthetic":false,"types":[]},{"text":"impl FromGlibContainerAsVec&lt;GdkTimeCoord, *mut GdkTimeCoord&gt; for TimeCoord","synthetic":false,"types":[]}];
implementors["glib"] = [];
implementors["gtk"] = [{"text":"impl FromGlibContainerAsVec&lt;GtkPageRange, *mut GtkPageRange&gt; for PageRange","synthetic":false,"types":[]}];
implementors["pango"] = [{"text":"impl FromGlibContainerAsVec&lt;*mut PangoGlyphInfo, *mut PangoGlyphInfo&gt; for GlyphInfo","synthetic":false,"types":[]},{"text":"impl FromGlibContainerAsVec&lt;*mut PangoGlyphInfo, *const PangoGlyphInfo&gt; for GlyphInfo","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()