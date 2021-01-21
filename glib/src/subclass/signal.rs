// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use crate::Closure;
use crate::SignalFlags;
use crate::Type;
use crate::Value;

use std::fmt;
use std::ptr;
use std::sync::Mutex;

/// Builder for signals.
#[allow(clippy::type_complexity)]
pub struct SignalBuilder<'a> {
    name: &'a str,
    flags: SignalFlags,
    arg_types: &'a [Type],
    ret_type: Type,
    class_handler: Option<
        Box<dyn Fn(&SignalClassHandlerToken, &[Value]) -> Option<Value> + Send + Sync + 'static>,
    >,
    accumulator: Option<
        Box<dyn Fn(&SignalInvocationHint, &mut Value, &Value) -> bool + Send + Sync + 'static>,
    >,
}

/// Signal metadata.
pub struct Signal {
    name: String,
    flags: SignalFlags,
    arg_types: Vec<Type>,
    ret_type: Type,
    registration: Mutex<SignalRegistration>,
}

/// Token passed to signal class handlers.
pub struct SignalClassHandlerToken(pub(super) *mut gobject_ffi::GTypeInstance);

impl fmt::Debug for SignalClassHandlerToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_tuple("SignalClassHandlerToken")
            .field(&unsafe { crate::Object::from_glib_borrow(self.0 as *mut gobject_ffi::GObject) })
            .finish()
    }
}

/// Signal invocation hint passed to signal accumulators.
#[repr(transparent)]
pub struct SignalInvocationHint(gobject_ffi::GSignalInvocationHint);

impl SignalInvocationHint {
    pub fn detail(&self) -> crate::Quark {
        unsafe { from_glib(self.0.detail) }
    }

    pub fn run_type(&self) -> SignalFlags {
        unsafe { from_glib(self.0.run_type) }
    }
}

impl fmt::Debug for SignalInvocationHint {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("SignalInvocationHint")
            .field("detail", &self.detail())
            .field("run_type", &self.run_type())
            .finish()
    }
}

/// Signal ID.
#[derive(Debug, Clone, Copy)]
pub struct SignalId(pub(super) Type, pub(super) u32);

#[allow(clippy::type_complexity)]
enum SignalRegistration {
    Unregistered {
        class_handler: Option<
            Box<
                dyn Fn(&SignalClassHandlerToken, &[Value]) -> Option<Value> + Send + Sync + 'static,
            >,
        >,
        accumulator: Option<
            Box<dyn Fn(&SignalInvocationHint, &mut Value, &Value) -> bool + Send + Sync + 'static>,
        >,
    },
    Registered {
        type_: Type,
        signal_id: u32,
    },
}

impl<'a> SignalBuilder<'a> {
    /// Run the signal class handler in the first emission stage.
    pub fn run_first(mut self) -> Self {
        self.flags |= SignalFlags::RUN_FIRST;
        self
    }

    /// Run the signal class handler in the third emission stage.
    pub fn run_last(mut self) -> Self {
        self.flags |= SignalFlags::RUN_LAST;
        self
    }

    /// Run the signal class handler in the last emission stage.
    pub fn run_cleanup(mut self) -> Self {
        self.flags |= SignalFlags::RUN_CLEANUP;
        self
    }

    /// Signals being emitted for an object while currently being in emission for this very object
    /// will not be emitted recursively, but instead cause the first emission to be restarted.
    pub fn no_recurse(mut self) -> Self {
        self.flags |= SignalFlags::NO_RECURSE;
        self
    }

    /// This signal supports "::detail" appendices to the signal name upon handler connections and
    /// emissions.
    pub fn detailed(mut self) -> Self {
        self.flags |= SignalFlags::DETAILED;
        self
    }

    /// Action signals are signals that may freely be emitted on alive objects from user code.
    pub fn action(mut self) -> Self {
        self.flags |= SignalFlags::ACTION;
        self
    }

    /// No emissions hooks are supported for this signal.
    pub fn no_hooks(mut self) -> Self {
        self.flags |= SignalFlags::NO_HOOKS;
        self
    }

    /// Varargs signal emission will always collect the arguments, even if there are no signal
    /// handlers connected.
    pub fn must_collect(mut self) -> Self {
        self.flags |= SignalFlags::MUST_COLLECT;
        self
    }

    /// The signal is deprecated and will be removed in a future version.
    pub fn deprecated(mut self) -> Self {
        self.flags |= SignalFlags::DEPRECATED;
        self
    }

    /// Explicitly set all flags.
    ///
    /// This overrides previously set flags on this builder.
    pub fn flags(mut self, flags: SignalFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Class handler for this signal.
    pub fn class_handler<
        F: Fn(&SignalClassHandlerToken, &[Value]) -> Option<Value> + Send + Sync + 'static,
    >(
        mut self,
        func: F,
    ) -> Self {
        self.class_handler = Some(Box::new(func));
        self
    }

    /// Accumulator for the return values of the signal.
    ///
    /// This is called if multiple signal handlers are connected to the signal for accumulating the
    /// return values into a single value.
    pub fn accumulator<
        F: Fn(&SignalInvocationHint, &mut Value, &Value) -> bool + Send + Sync + 'static,
    >(
        mut self,
        func: F,
    ) -> Self {
        self.accumulator = Some(Box::new(func));
        self
    }

    /// Build the signal.
    ///
    /// This does not register the signal yet, which only happens as part of object type
    /// registration.
    pub fn build(self) -> Signal {
        let flags = if self.flags
            & (SignalFlags::RUN_FIRST | SignalFlags::RUN_LAST | SignalFlags::RUN_CLEANUP)
            == SignalFlags::empty()
        {
            self.flags | SignalFlags::RUN_LAST
        } else {
            self.flags
        };

        Signal {
            name: String::from(self.name),
            flags,
            arg_types: Vec::from(self.arg_types),
            ret_type: self.ret_type,
            registration: Mutex::new(SignalRegistration::Unregistered {
                class_handler: self.class_handler,
                accumulator: self.accumulator,
            }),
        }
    }
}

impl Signal {
    /// Create a new builder for a signal.
    pub fn builder<'a>(name: &'a str, arg_types: &'a [Type], ret_type: Type) -> SignalBuilder<'a> {
        SignalBuilder {
            name,
            arg_types,
            ret_type,
            flags: SignalFlags::empty(),
            class_handler: None,
            accumulator: None,
        }
    }

    /// Name of the signal.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Flags of the signal.
    pub fn flags(&self) -> SignalFlags {
        self.flags
    }

    /// Argument types of the signal.
    pub fn arg_types(&self) -> &[Type] {
        &self.arg_types
    }

    /// Return type of the signal.
    pub fn ret_type(&self) -> Type {
        self.ret_type
    }

    /// Signal ID.
    ///
    /// This will panic if called before the signal was registered.
    pub fn signal_id(&self) -> SignalId {
        match &*self.registration.lock().unwrap() {
            SignalRegistration::Unregistered { .. } => panic!("Signal not registered yet"),
            SignalRegistration::Registered { type_, signal_id } => SignalId(*type_, *signal_id),
        }
    }

    /// Type this signal was registered for.
    ///
    /// This will panic if called before the signal was registered.
    pub fn type_(&self) -> Type {
        match &*self.registration.lock().unwrap() {
            SignalRegistration::Unregistered { .. } => panic!("Signal not registered yet"),
            SignalRegistration::Registered { type_, .. } => *type_,
        }
    }

    pub(super) fn register(&self, type_: Type) {
        let mut registration = self.registration.lock().unwrap();

        let (class_handler, accumulator) = match &mut *registration {
            SignalRegistration::Unregistered {
                class_handler,
                accumulator,
            } => (class_handler.take(), accumulator.take()),
            SignalRegistration::Registered { .. } => unreachable!(),
        };

        let arg_types = self
            .arg_types
            .iter()
            .map(ToGlib::to_glib)
            .collect::<Vec<_>>();

        let class_handler = class_handler.map(|class_handler| {
            Closure::new(move |values| unsafe {
                let instance = gobject_ffi::g_value_get_object(values[0].to_glib_none().0);
                class_handler(&SignalClassHandlerToken(instance as *mut _), values)
            })
        });

        unsafe extern "C" fn accumulator_trampoline(
            ihint: *mut gobject_ffi::GSignalInvocationHint,
            return_accu: *mut gobject_ffi::GValue,
            handler_return: *const gobject_ffi::GValue,
            data: ffi::gpointer,
        ) -> ffi::gboolean {
            let accumulator = &**(data
                as *const *const (dyn Fn(&SignalInvocationHint, &mut Value, &Value) -> bool
                     + Send
                     + Sync
                     + 'static));
            accumulator(
                &SignalInvocationHint(*ihint),
                &mut *(return_accu as *mut Value),
                &*(handler_return as *const Value),
            )
            .to_glib()
        }

        let (accumulator, accumulator_trampoline) = if let Some(accumulator) = accumulator {
            (
                Box::into_raw(Box::new(accumulator)),
                Some::<unsafe extern "C" fn(_, _, _, _) -> _>(accumulator_trampoline),
            )
        } else {
            (ptr::null_mut(), None)
        };

        let signal_id = unsafe {
            gobject_ffi::g_signal_newv(
                self.name.to_glib_none().0,
                type_.to_glib(),
                self.flags.to_glib(),
                class_handler.to_glib_none().0,
                accumulator_trampoline,
                accumulator as ffi::gpointer,
                None,
                self.ret_type.to_glib(),
                arg_types.len() as u32,
                arg_types.as_ptr() as *mut _,
            )
        };

        *registration = SignalRegistration::Registered { type_, signal_id };
    }
}
