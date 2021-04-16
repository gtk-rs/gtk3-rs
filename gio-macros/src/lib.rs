// Take a look at the license at the top of the repository in the LICENSE file.

extern crate proc_macro;

mod actions;

use darling::FromMeta;
use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ItemImpl};

/// Macro for creating [`gio::Action`]s and registering them in a given
/// [`gio::ActionMap`]. It generates a method `register_actions` with
/// a following signature:
///
/// ```rust
/// # struct X; impl X {
/// fn register_actions<AM: glib::object::IsA<gio::ActionMap>>(&self, map: &AM)
/// # {} }
/// ```
///
/// ## Name of an action
///
/// Name of an action is the same as the name of a function. This can be
/// changed by an annotation.
///
/// ```rust
/// # use gio::ActionMapExt;
/// # #[derive(glib::Downgrade)]
/// # pub struct X;
/// # #[gio_macros::actions]
/// # impl X {
/// // Name of the action is "play".
/// fn play(&self) {
/// }
///
/// // Name of the action is "pause".
/// #[action(name = "pause")]
/// fn pause_handler(&self) {
/// }
/// # }
/// ```
///
/// ## Stateful action
///
/// An action may have a state. A handler need to be annotated accordingly.
/// A handler must have a following signature:
///
/// ```rust,ignore
/// #[action(stateful, initial_state = <initial-value>)]
/// fn action_handler(&self, state: StateType) -> Option<StateType>
/// ```
///
/// `StateType` must implement [`glib::variant::StaticVariantType`]. Returning
/// `Some(value)` from such handler triggers a change of an action's state.
///
/// An `initial_state` annotation may be omited. In this case `StateType` must
/// implement [`std::default::Default`], which is used to initialize an action.
///
/// ## Parameter
///
/// An action may have a parameter. In this case a handler need to have one of
/// the following signatures:
///
/// ```rust,ignore
/// // Stateless action
/// fn action_handler(&self, parameter: ParameterType)
///
/// // Stateful action
/// #[action(stateful)]
/// fn action_handler(&self, state: StateType, parameter: ParameterType) -> Option<StateType>
/// ```
///
/// `ParameterType` must also implement [`glib::variant::StaticVariantType`].
///
/// ## Change state handler
///
/// A handler may be annotated as `change_state`. In this case it will be bound
/// to a `change-state` signal of an action (in opposite to an `activate`
/// signal used by default). In this case a signature should look like this:
///
/// ```rust,ignore
/// // Stateful action
/// #[action(stateful, change_state)]
/// fn change_state_action_handler(&self, state: StateType) -> Option<StateType>
/// ```
///
/// If you need an action with both `activate` and `change-state` handlers, just
/// create two methods and annotate them with the same action name.
///
/// ## Defaults
///
/// A `stateful` annotation may be omited if `initial_state` or `change_state` is
/// specified.
///
/// # Example
/// ```rust
/// use gio::prelude::*;
///
/// #[derive(glib::Downgrade)]
/// pub struct MyApplication(gio::Application);
///
/// impl MyApplication {
///     pub fn new() -> Self {
///         let app = Self(gio::Application::new(
///             Some("com.example.MyApplication"),
///             gio::ApplicationFlags::FLAGS_NONE,
///         ));
///         app.register_actions(&app.0);
///         app
///     }
/// }
///
/// #[gio::actions]
/// impl MyApplication {
///     fn action1(&self) {
///         // handle "action1"
///     }
///
///     // Explicitely specify action name
///     #[action(name = "action_second")]
///     fn action2(&self) {
///         // handle "action_second"
///     }
///
///     // Action with a parameter
///     fn action3(&self, param: String) {
///         // handle "action3"
///     }
///
///     // Stateful action with a specified initial state
///     #[action(stateful, initial_state = false)]
///     fn stateful_toggle(&self, state: bool) -> Option<bool> {
///         // handle action
///         Some(!state) // return new state
///     }
///
///     // Stateful action with a default initial state (`false`)
///     #[action(stateful)]
///     fn stateful_toggle_default(&self, state: bool) -> Option<bool> {
///         Some(!state) // return new state
///     }
///
///     // Stateful action with a state of `String` type
///     #[action(stateful, initial_state = "")]
///     fn stateful_text(&self, state: String) -> Option<String> {
///         if state.len() >= 10 {
///             None // do not change state
///         } else {
///             Some(state + "!") // change state
///         }
///     }
///
///     // Stateful action with a `String` parameter
///     #[action(stateful, initial_state = true)]
///     fn stateful_toggle_with_parameter(&self, state: bool, param: String) -> Option<bool> {
///         // Do not change the state of the action
///         None
///     }
///
///     // Stateful action with a handler for a `change-state` signal
///     #[action(stateful, initial_state = 0.0_f64, change_state)]
///     fn volume(&self, value: f64) -> Option<f64> {
///         if value >= 0.0 && value <= 10.0 {
///             // accept new state
///             Some(value)
///         } else {
///             // reject
///             None
///         }
///     }
///
///     // Stateful action with a handler for a `change-state` signal and a parameter of the same type.
///     // `change_state` also implies `stateful`, so it may be omitted.
///     #[action(initial_state = true, change_state)]
///     fn pause(&self, paused: bool) -> Option<bool> {
///         Some(paused)
///     }
///
///     // Stateful action with a handler for a `change-state` signal and without a parameter.
///     #[action(change_state, initial_state = true, no_parameter)]
///     fn pause_no_param(&self, paused: bool) -> Option<bool> {
///         Some(paused)
///     }
/// }
/// ```
///
/// [`gio::Action`]: struct.Action.html
/// [`gio::ActionMap`]: struct.ActionMap.html
/// [`glib::variant::StaticVariantType`]: ../glib/variant/trait.StaticVariantType.html
#[proc_macro_attribute]
pub fn actions(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let attribute_args = parse_macro_input!(args as AttributeArgs);
    let attrs = match actions::ActionImplAttributes::from_list(&attribute_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };
    actions::actions(attrs, input).unwrap_or_else(|err| err)
}
