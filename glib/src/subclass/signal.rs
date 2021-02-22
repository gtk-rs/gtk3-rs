// Take a look at the license at the top of the repository in the LICENSE file.

use smallvec::SmallVec;

use crate::translate::*;
use crate::Closure;
use crate::SignalFlags;
use crate::Type;
use crate::Value;

use std::ptr;
use std::sync::Mutex;
use std::{fmt, num::NonZeroU32};

/// Builder for signals.
#[allow(clippy::type_complexity)]
pub struct SignalBuilder<'a> {
    name: &'a str,
    flags: SignalFlags,
    param_types: &'a [SignalType],
    return_type: SignalType,
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
    param_types: Vec<SignalType>,
    return_type: SignalType,
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

/// In-depth information of a specific signal
pub struct SignalQuery(gobject_ffi::GSignalQuery);

impl SignalQuery {
    /// The name of the signal.
    pub fn signal_name<'a>(&self) -> &'a str {
        unsafe {
            let ptr = self.0.signal_name;
            std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
        }
    }

    /// The ID of the signal.
    pub fn signal_id(&self) -> SignalId {
        unsafe { SignalId::from_glib(self.0.signal_id) }
    }

    /// The instance type this signal can be emitted for.
    pub fn type_(&self) -> Type {
        unsafe { from_glib(self.0.itype) }
    }

    /// The signal flags.
    pub fn flags(&self) -> SignalFlags {
        unsafe { from_glib(self.0.signal_flags) }
    }

    /// The return type for the user callback.
    pub fn return_type(&self) -> SignalType {
        unsafe { from_glib(self.0.return_type) }
    }

    /// The number of parameters the user callback takes.
    pub fn n_params(&self) -> u32 {
        self.0.n_params
    }

    /// The parameters for the user callback.
    pub fn param_types(&self) -> SmallVec<[SignalType; 10]> {
        unsafe {
            let types = self.0.param_types;
            FromGlibContainerAsVec::from_glib_none_num_as_vec(types, self.n_params() as usize)
                .into_iter()
                .collect::<smallvec::SmallVec<[_; 10]>>()
        }
    }
}

impl fmt::Debug for SignalQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("SignalQuery")
            .field("signal_name", &self.signal_name())
            .field("type", &self.type_())
            .field("flags", &self.flags())
            .field("return_type", &self.return_type())
            .field("param_types", &self.param_types())
            .finish()
    }
}
/// Signal ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignalId(NonZeroU32);

impl SignalId {
    /// Create a new Signal Identifier.
    ///
    /// # Safety
    ///
    /// The caller has to ensure it's a valid signal identifier.
    pub unsafe fn new(id: NonZeroU32) -> Self {
        Self(id)
    }

    #[doc(alias = "g_signal_parse_name")]
    pub fn parse_name(name: &str, type_: Type, force_detail: bool) -> Option<(Self, crate::Quark)> {
        let mut signal_id = std::mem::MaybeUninit::uninit();
        let mut detail_quark = std::mem::MaybeUninit::uninit();
        unsafe {
            let found: bool = from_glib(gobject_ffi::g_signal_parse_name(
                name.to_glib_none().0,
                type_.to_glib(),
                signal_id.as_mut_ptr(),
                detail_quark.as_mut_ptr(),
                force_detail.to_glib(),
            ));

            if found {
                Some((
                    from_glib(signal_id.assume_init()),
                    crate::Quark::from_glib(detail_quark.assume_init()),
                ))
            } else {
                None
            }
        }
    }

    /// Find a SignalId by it's `name` and the `type` it connects to.
    #[doc(alias = "g_signal_lookup")]
    pub fn lookup(name: &str, type_: Type) -> Option<Self> {
        unsafe {
            let signal_id = gobject_ffi::g_signal_lookup(name.to_glib_none().0, type_.to_glib());
            if signal_id == 0 {
                None
            } else {
                Some(Self::new(NonZeroU32::new_unchecked(signal_id)))
            }
        }
    }

    /// Queries more in-depth information about the current signal.
    #[doc(alias = "g_signal_query")]
    pub fn query(&self) -> SignalQuery {
        unsafe {
            let mut query_ptr = std::mem::MaybeUninit::uninit();
            gobject_ffi::g_signal_query(self.to_glib(), query_ptr.as_mut_ptr());
            let query = query_ptr.assume_init();
            assert_ne!(query.signal_id, 0);
            SignalQuery(query)
        }
    }

    /// Find the signal name.
    #[doc(alias = "g_signal_name")]
    pub fn name<'a>(&self) -> &'a str {
        unsafe {
            let ptr = gobject_ffi::g_signal_name(self.to_glib());
            std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}

#[doc(hidden)]
impl FromGlib<u32> for SignalId {
    unsafe fn from_glib(signal_id: u32) -> Self {
        assert_ne!(signal_id, 0);
        Self::new(NonZeroU32::new_unchecked(signal_id))
    }
}

#[doc(hidden)]
impl ToGlib for SignalId {
    type GlibType = u32;

    fn to_glib(&self) -> u32 {
        self.0.into()
    }
}

#[derive(Copy, Clone, Hash)]
pub struct SignalType(ffi::GType);

impl SignalType {
    pub fn with_static_scope(type_: Type) -> Self {
        Self(type_.to_glib() | gobject_ffi::G_TYPE_FLAG_RESERVED_ID_BIT)
    }

    pub fn static_scope(&self) -> bool {
        (self.0 & gobject_ffi::G_TYPE_FLAG_RESERVED_ID_BIT) != 0
    }

    pub fn type_(&self) -> Type {
        (*self).into()
    }
}

impl From<Type> for SignalType {
    fn from(type_: Type) -> Self {
        Self(type_.to_glib())
    }
}

impl From<SignalType> for Type {
    fn from(type_: SignalType) -> Self {
        // Remove the extra-bit used for G_SIGNAL_TYPE_STATIC_SCOPE
        let type_ = type_.0 & (!gobject_ffi::G_TYPE_FLAG_RESERVED_ID_BIT);
        unsafe { from_glib(type_) }
    }
}

impl PartialEq<Type> for SignalType {
    fn eq(&self, other: &Type) -> bool {
        let type_: Type = (*self).into();
        type_.eq(other)
    }
}

impl std::fmt::Debug for SignalType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let type_: Type = (*self).into();
        f.debug_struct("SignalType")
            .field("name", &type_.name())
            .field("static_scope", &self.static_scope())
            .finish()
    }
}

impl std::fmt::Display for SignalType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let type_: Type = (*self).into();
        f.debug_struct("SignalType")
            .field("name", &type_.name())
            .field("static_scope", &self.static_scope())
            .finish()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GType> for SignalType {
    unsafe fn from_glib(type_: ffi::GType) -> Self {
        Self(type_)
    }
}

#[doc(hidden)]
impl ToGlib for SignalType {
    type GlibType = ffi::GType;

    fn to_glib(&self) -> ffi::GType {
        self.0
    }
}

impl FromGlibContainerAsVec<Type, *const ffi::GType> for SignalType {
    unsafe fn from_glib_none_num_as_vec(ptr: *const ffi::GType, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib(*ptr.add(i)));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(_: *const ffi::GType, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }

    unsafe fn from_glib_full_num_as_vec(_: *const ffi::GType, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }
}

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
        signal_id: SignalId,
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
            param_types: self.param_types.to_vec(),
            return_type: self.return_type,
            registration: Mutex::new(SignalRegistration::Unregistered {
                class_handler: self.class_handler,
                accumulator: self.accumulator,
            }),
        }
    }
}

impl Signal {
    /// Create a new builder for a signal.
    pub fn builder<'a>(
        name: &'a str,
        param_types: &'a [SignalType],
        return_type: SignalType,
    ) -> SignalBuilder<'a> {
        SignalBuilder {
            name,
            param_types,
            return_type,
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

    /// Parameter types of the signal.
    pub fn param_types(&self) -> &[SignalType] {
        &self.param_types
    }

    /// Return type of the signal.
    pub fn return_type(&self) -> SignalType {
        self.return_type
    }

    /// Signal ID.
    ///
    /// This will panic if called before the signal was registered.
    pub fn signal_id(&self) -> SignalId {
        match &*self.registration.lock().unwrap() {
            SignalRegistration::Unregistered { .. } => panic!("Signal not registered yet"),
            SignalRegistration::Registered { signal_id, .. } => *signal_id,
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

        let param_types = self
            .param_types
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

        unsafe {
            let signal_id = gobject_ffi::g_signal_newv(
                self.name.to_glib_none().0,
                type_.to_glib(),
                self.flags.to_glib(),
                class_handler.to_glib_none().0,
                accumulator_trampoline,
                accumulator as ffi::gpointer,
                None,
                self.return_type.to_glib(),
                param_types.len() as u32,
                param_types.as_ptr() as *mut _,
            );
            *registration = SignalRegistration::Registered {
                type_,
                signal_id: SignalId::from_glib(signal_id),
            };
        }
    }
}
