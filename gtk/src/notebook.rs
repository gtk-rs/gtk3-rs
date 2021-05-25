// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Notebook;
use crate::Widget;
use glib::translate::*;
use glib::IsA;
use libc::c_int;

pub trait NotebookExtManual: 'static {
    /// Appends a page to `self`.
    /// ## `child`
    /// the [Widget](crate::Widget) to use as the contents of the page
    /// ## `tab_label`
    /// the [Widget](crate::Widget) to be used as the label
    ///  for the page, or [`None`] to use the default label, “page N”
    ///
    /// # Returns
    ///
    /// the index (starting from 0) of the appended
    ///  page in the notebook, or -1 if function fails
    #[doc(alias = "gtk_notebook_append_page")]
    fn append_page<T: IsA<Widget>, U: IsA<Widget>>(&self, child: &T, tab_label: Option<&U>) -> u32;

    /// Appends a page to `self`, specifying the widget to use as the
    /// label in the popup menu.
    /// ## `child`
    /// the [Widget](crate::Widget) to use as the contents of the page
    /// ## `tab_label`
    /// the [Widget](crate::Widget) to be used as the label
    ///  for the page, or [`None`] to use the default label, “page N”
    /// ## `menu_label`
    /// the widget to use as a label for the
    ///  page-switch menu, if that is enabled. If [`None`], and `tab_label`
    ///  is a [Label](crate::Label) or [`None`], then the menu label will be a newly
    ///  created label with the same text as `tab_label`; if `tab_label`
    ///  is not a [Label](crate::Label), `menu_label` must be specified if the
    ///  page-switch menu is to be used.
    ///
    /// # Returns
    ///
    /// the index (starting from 0) of the appended
    ///  page in the notebook, or -1 if function fails
    #[doc(alias = "gtk_notebook_append_page_menu")]
    fn append_page_menu<T, U, V>(
        &self,
        child: &T,
        tab_label: Option<&U>,
        menu_label: Option<&V>,
    ) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
        V: IsA<Widget>;

    /// Returns the page number of the current page.
    ///
    /// # Returns
    ///
    /// the index (starting from 0) of the current
    ///  page in the notebook. If the notebook has no pages,
    ///  then -1 will be returned.
    #[doc(alias = "gtk_notebook_get_current_page")]
    #[doc(alias = "get_current_page")]
    fn current_page(&self) -> Option<u32>;

    /// Gets the number of pages in a notebook.
    ///
    /// # Returns
    ///
    /// the number of pages in the notebook
    #[doc(alias = "gtk_notebook_get_n_pages")]
    #[doc(alias = "get_n_pages")]
    fn n_pages(&self) -> u32;

    /// Returns the child widget contained in page number `page_num`.
    /// ## `page_num`
    /// the index of a page in the notebook, or -1
    ///  to get the last page
    ///
    /// # Returns
    ///
    /// the child widget, or [`None`] if `page_num`
    /// is out of bounds
    #[doc(alias = "gtk_notebook_get_nth_page")]
    #[doc(alias = "get_nth_page")]
    fn nth_page(&self, page_num: Option<u32>) -> Option<Widget>;

    /// Insert a page into `self` at the given position.
    /// ## `child`
    /// the [Widget](crate::Widget) to use as the contents of the page
    /// ## `tab_label`
    /// the [Widget](crate::Widget) to be used as the label
    ///  for the page, or [`None`] to use the default label, “page N”
    /// ## `position`
    /// the index (starting at 0) at which to insert the page,
    ///  or -1 to append the page after all other pages
    ///
    /// # Returns
    ///
    /// the index (starting from 0) of the inserted
    ///  page in the notebook, or -1 if function fails
    #[doc(alias = "gtk_notebook_insert_page")]
    fn insert_page<T, U>(&self, child: &T, tab_label: Option<&U>, position: Option<u32>) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>;

    /// Insert a page into `self` at the given position, specifying
    /// the widget to use as the label in the popup menu.
    /// ## `child`
    /// the [Widget](crate::Widget) to use as the contents of the page
    /// ## `tab_label`
    /// the [Widget](crate::Widget) to be used as the label
    ///  for the page, or [`None`] to use the default label, “page N”
    /// ## `menu_label`
    /// the widget to use as a label for the
    ///  page-switch menu, if that is enabled. If [`None`], and `tab_label`
    ///  is a [Label](crate::Label) or [`None`], then the menu label will be a newly
    ///  created label with the same text as `tab_label`; if `tab_label`
    ///  is not a [Label](crate::Label), `menu_label` must be specified if the
    ///  page-switch menu is to be used.
    /// ## `position`
    /// the index (starting at 0) at which to insert the page,
    ///  or -1 to append the page after all other pages.
    ///
    /// # Returns
    ///
    /// the index (starting from 0) of the inserted
    ///  page in the notebook
    #[doc(alias = "gtk_notebook_insert_page_menu")]
    fn insert_page_menu<T, U, V>(
        &self,
        child: &T,
        tab_label: Option<&U>,
        menu_label: Option<&V>,
        position: Option<u32>,
    ) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
        V: IsA<Widget>;

    /// Finds the index of the page which contains the given child
    /// widget.
    /// ## `child`
    /// a [Widget](crate::Widget)
    ///
    /// # Returns
    ///
    /// the index of the page containing `child`, or
    ///  -1 if `child` is not in the notebook
    #[doc(alias = "gtk_notebook_page_num")]
    fn page_num<T: IsA<Widget>>(&self, child: &T) -> Option<u32>;

    /// Prepends a page to `self`.
    /// ## `child`
    /// the [Widget](crate::Widget) to use as the contents of the page
    /// ## `tab_label`
    /// the [Widget](crate::Widget) to be used as the label
    ///  for the page, or [`None`] to use the default label, “page N”
    ///
    /// # Returns
    ///
    /// the index (starting from 0) of the prepended
    ///  page in the notebook, or -1 if function fails
    #[doc(alias = "gtk_notebook_prepend_page")]
    fn prepend_page<T, U>(&self, child: &T, tab_label: Option<&U>) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>;

    /// Prepends a page to `self`, specifying the widget to use as the
    /// label in the popup menu.
    /// ## `child`
    /// the [Widget](crate::Widget) to use as the contents of the page
    /// ## `tab_label`
    /// the [Widget](crate::Widget) to be used as the label
    ///  for the page, or [`None`] to use the default label, “page N”
    /// ## `menu_label`
    /// the widget to use as a label for the
    ///  page-switch menu, if that is enabled. If [`None`], and `tab_label`
    ///  is a [Label](crate::Label) or [`None`], then the menu label will be a newly
    ///  created label with the same text as `tab_label`; if `tab_label`
    ///  is not a [Label](crate::Label), `menu_label` must be specified if the
    ///  page-switch menu is to be used.
    ///
    /// # Returns
    ///
    /// the index (starting from 0) of the prepended
    ///  page in the notebook, or -1 if function fails
    #[doc(alias = "gtk_notebook_prepend_page_menu")]
    fn prepend_page_menu<T, U, V>(
        &self,
        child: &T,
        tab_label: Option<&U>,
        menu_label: Option<&V>,
    ) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
        V: IsA<Widget>;

    /// Removes a page from the notebook given its index
    /// in the notebook.
    /// ## `page_num`
    /// the index of a notebook page, starting
    ///  from 0. If -1, the last page will be removed.
    #[doc(alias = "gtk_notebook_remove_page")]
    fn remove_page(&self, page_num: Option<u32>);

    /// Reorders the page containing `child`, so that it appears in position
    /// `position`. If `position` is greater than or equal to the number of
    /// children in the list or negative, `child` will be moved to the end
    /// of the list.
    /// ## `child`
    /// the child to move
    /// ## `position`
    /// the new position, or -1 to move to the end
    #[doc(alias = "gtk_notebook_reorder_child")]
    fn reorder_child<T: IsA<Widget>>(&self, child: &T, position: Option<u32>);

    /// Switches to the page number `page_num`.
    ///
    /// Note that due to historical reasons, GtkNotebook refuses
    /// to switch to a page unless the child widget is visible.
    /// Therefore, it is recommended to show child widgets before
    /// adding them to a notebook.
    /// ## `page_num`
    /// index of the page to switch to, starting from 0.
    ///  If negative, the last page will be used. If greater
    ///  than the number of pages in the notebook, nothing
    ///  will be done.
    #[doc(alias = "gtk_notebook_set_current_page")]
    fn set_current_page(&self, page_num: Option<u32>);
}

impl<O: IsA<Notebook>> NotebookExtManual for O {
    fn append_page<T: IsA<Widget>, U: IsA<Widget>>(&self, child: &T, tab_label: Option<&U>) -> u32 {
        unsafe {
            let ret = ffi::gtk_notebook_append_page(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
            );
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn append_page_menu<T, U, V>(
        &self,
        child: &T,
        tab_label: Option<&U>,
        menu_label: Option<&V>,
    ) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
        V: IsA<Widget>,
    {
        unsafe {
            let ret = ffi::gtk_notebook_append_page_menu(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
                menu_label.map(|p| p.as_ref()).to_glib_none().0,
            );
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn current_page(&self) -> Option<u32> {
        unsafe {
            let ret = ffi::gtk_notebook_get_current_page(self.as_ref().to_glib_none().0);
            if ret >= 0 {
                Some(ret as u32)
            } else {
                None
            }
        }
    }

    fn n_pages(&self) -> u32 {
        unsafe {
            let ret = ffi::gtk_notebook_get_n_pages(self.as_ref().to_glib_none().0);
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn nth_page(&self, page_num: Option<u32>) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_nth_page(
                self.as_ref().to_glib_none().0,
                page_num.map_or(-1, |n| n as c_int),
            ))
        }
    }

    fn insert_page<T, U>(&self, child: &T, tab_label: Option<&U>, position: Option<u32>) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
    {
        unsafe {
            let ret = ffi::gtk_notebook_insert_page(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
                position.map_or(-1, |n| n as c_int),
            );
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn insert_page_menu<T, U, V>(
        &self,
        child: &T,
        tab_label: Option<&U>,
        menu_label: Option<&V>,
        position: Option<u32>,
    ) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
        V: IsA<Widget>,
    {
        unsafe {
            let ret = ffi::gtk_notebook_insert_page_menu(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
                menu_label.map(|p| p.as_ref()).to_glib_none().0,
                position.map_or(-1, |n| n as c_int),
            );
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn page_num<T: IsA<Widget>>(&self, child: &T) -> Option<u32> {
        unsafe {
            let ret = ffi::gtk_notebook_page_num(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
            if ret >= 0 {
                Some(ret as u32)
            } else {
                None
            }
        }
    }

    fn prepend_page<T, U>(&self, child: &T, tab_label: Option<&U>) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
    {
        unsafe {
            let ret = ffi::gtk_notebook_prepend_page(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
            );
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn prepend_page_menu<T, U, V>(
        &self,
        child: &T,
        tab_label: Option<&U>,
        menu_label: Option<&V>,
    ) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
        V: IsA<Widget>,
    {
        unsafe {
            let ret = ffi::gtk_notebook_prepend_page_menu(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
                menu_label.map(|p| p.as_ref()).to_glib_none().0,
            );
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn remove_page(&self, page_num: Option<u32>) {
        unsafe {
            ffi::gtk_notebook_remove_page(
                self.as_ref().to_glib_none().0,
                page_num.map_or(-1, |n| n as c_int),
            );
        }
    }

    fn reorder_child<T: IsA<Widget>>(&self, child: &T, position: Option<u32>) {
        unsafe {
            ffi::gtk_notebook_reorder_child(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position.map_or(-1, |n| n as c_int),
            );
        }
    }

    fn set_current_page(&self, page_num: Option<u32>) {
        unsafe {
            ffi::gtk_notebook_set_current_page(
                self.as_ref().to_glib_none().0,
                page_num.map_or(-1, |n| n as c_int),
            );
        }
    }
}
