// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use glib::subclass::prelude::*;

use glib::Cast;

use super::widget::WidgetImpl;
use crate::Container;
use crate::Widget;
use crate::WidgetPath;

pub trait ContainerImpl: ContainerImplExt + WidgetImpl {
    fn add(&self, container: &Self::Type, widget: &Widget) {
        self.parent_add(container, widget)
    }

    fn remove(&self, container: &Self::Type, widget: &Widget) {
        self.parent_remove(container, widget)
    }

    fn check_resize(&self, container: &Self::Type) {
        self.parent_check_resize(container)
    }

    fn set_focus_child(&self, container: &Self::Type, widget: Option<&Widget>) {
        self.parent_set_focus_child(container, widget)
    }

    fn child_type(&self, container: &Self::Type) -> glib::Type {
        self.parent_child_type(container)
    }

    #[doc(alias = "get_path_for_child")]
    fn path_for_child(&self, container: &Self::Type, widget: &Widget) -> WidgetPath {
        self.parent_path_for_child(container, widget)
    }

    fn forall(&self, container: &Self::Type, include_internals: bool, callback: &Callback) {
        self.parent_forall(container, include_internals, callback);
    }
}

pub trait ContainerImplExt: ObjectSubclass {
    fn parent_add(&self, container: &Self::Type, widget: &Widget);
    fn parent_remove(&self, container: &Self::Type, widget: &Widget);
    fn parent_check_resize(&self, container: &Self::Type);
    fn parent_set_focus_child(&self, container: &Self::Type, widget: Option<&Widget>);
    fn parent_child_type(&self, container: &Self::Type) -> glib::Type;
    fn parent_path_for_child(&self, container: &Self::Type, widget: &Widget) -> WidgetPath;
    fn parent_forall(&self, container: &Self::Type, include_internals: bool, callback: &Callback);
}

impl<T: ContainerImpl> ContainerImplExt for T {
    fn parent_add(&self, container: &Self::Type, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkContainerClass;
            if let Some(f) = (*parent_class).add {
                f(
                    container.unsafe_cast_ref::<Container>().to_glib_none().0,
                    widget.to_glib_none().0,
                )
            }
        }
    }

    fn parent_remove(&self, container: &Self::Type, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkContainerClass;
            if let Some(f) = (*parent_class).remove {
                f(
                    container.unsafe_cast_ref::<Container>().to_glib_none().0,
                    widget.to_glib_none().0,
                )
            }
        }
    }

    fn parent_check_resize(&self, container: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkContainerClass;
            if let Some(f) = (*parent_class).check_resize {
                f(container.unsafe_cast_ref::<Container>().to_glib_none().0)
            }
        }
    }

    fn parent_set_focus_child(&self, container: &Self::Type, widget: Option<&Widget>) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkContainerClass;
            if let Some(f) = (*parent_class).set_focus_child {
                f(
                    container.unsafe_cast_ref::<Container>().to_glib_none().0,
                    widget.to_glib_none().0,
                )
            }
        }
    }

    fn parent_child_type(&self, container: &Self::Type) -> glib::Type {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkContainerClass;
            if let Some(f) = (*parent_class).child_type {
                from_glib(f(container.unsafe_cast_ref::<Container>().to_glib_none().0))
            } else {
                glib::Type::UNIT
            }
        }
    }

    fn parent_path_for_child(&self, container: &Self::Type, widget: &Widget) -> WidgetPath {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkContainerClass;
            let f = (*parent_class)
                .get_path_for_child
                .expect("No parent class impl for \"get_path_for_child\"");
            from_glib_none(f(
                container.unsafe_cast_ref::<Container>().to_glib_none().0,
                widget.to_glib_none().0,
            ))
        }
    }

    fn parent_forall(&self, container: &Self::Type, include_internals: bool, callback: &Callback) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkContainerClass;
            if let Some(f) = (*parent_class).forall {
                f(
                    container.unsafe_cast_ref::<Container>().to_glib_none().0,
                    include_internals.into_glib(),
                    callback.callback,
                    callback.user_data,
                )
            }
        }
    }
}

unsafe impl<T: ContainerImpl> IsSubclassable<T> for Container {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        let klass = class.as_mut();
        klass.add = Some(container_add::<T>);
        klass.remove = Some(container_remove::<T>);
        klass.check_resize = Some(container_check_resize::<T>);
        klass.set_focus_child = Some(container_set_focus_child::<T>);
        klass.child_type = Some(container_child_type::<T>);
        klass.get_path_for_child = Some(container_get_path_for_child::<T>);
        klass.forall = Some(container_forall::<T>);
    }
}

unsafe extern "C" fn container_add<T: ContainerImpl>(
    ptr: *mut ffi::GtkContainer,
    wdgtptr: *mut ffi::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Container> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    imp.add(wrap.unsafe_cast_ref(), &widget)
}

unsafe extern "C" fn container_remove<T: ContainerImpl>(
    ptr: *mut ffi::GtkContainer,
    wdgtptr: *mut ffi::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Container> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    imp.remove(wrap.unsafe_cast_ref(), &widget)
}

unsafe extern "C" fn container_check_resize<T: ContainerImpl>(ptr: *mut ffi::GtkContainer) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Container> = from_glib_borrow(ptr);

    imp.check_resize(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn container_set_focus_child<T: ContainerImpl>(
    ptr: *mut ffi::GtkContainer,
    wdgtptr: *mut ffi::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Container> = from_glib_borrow(ptr);
    let widget: Borrowed<Option<Widget>> = from_glib_borrow(wdgtptr);

    imp.set_focus_child(wrap.unsafe_cast_ref(), widget.as_ref().as_ref())
}

unsafe extern "C" fn container_child_type<T: ContainerImpl>(
    ptr: *mut ffi::GtkContainer,
) -> glib::ffi::GType {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Container> = from_glib_borrow(ptr);

    imp.child_type(wrap.unsafe_cast_ref()).into_glib()
}

unsafe extern "C" fn container_get_path_for_child<T: ContainerImpl>(
    ptr: *mut ffi::GtkContainer,
    wdgtptr: *mut ffi::GtkWidget,
) -> *mut ffi::GtkWidgetPath {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Container> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    imp.path_for_child(wrap.unsafe_cast_ref(), &widget)
        .to_glib_none()
        .0
}

unsafe extern "C" fn container_forall<T: ObjectSubclass>(
    ptr: *mut ffi::GtkContainer,
    include_internals: glib::ffi::gboolean,
    callback: ffi::GtkCallback,
    user_data: glib::ffi::gpointer,
) where
    T: ContainerImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Container> = from_glib_borrow(ptr);
    let callback = Callback {
        callback,
        user_data,
    };

    imp.forall(
        wrap.unsafe_cast_ref(),
        from_glib(include_internals),
        &callback,
    )
}

#[derive(Debug)]
pub struct Callback {
    callback: ffi::GtkCallback,
    user_data: glib::ffi::gpointer,
}

impl Callback {
    pub fn call(&self, widget: &Widget) {
        unsafe {
            if let Some(callback) = self.callback {
                callback(widget.to_glib_none().0, self.user_data);
            }
        }
    }
}

pub unsafe trait ContainerClassSubclassExt: ClassStruct {
    #[doc(alias = "gtk_container_class_handle_border_width")]
    fn handle_border_width(&mut self) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkContainerClass;
            ffi::gtk_container_class_handle_border_width(widget_class);
        }
    }
}

unsafe impl<T: ClassStruct> ContainerClassSubclassExt for T where T::Type: ContainerImpl {}
