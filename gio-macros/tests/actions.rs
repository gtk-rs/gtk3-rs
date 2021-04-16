use gio::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(glib::Downgrade)]
pub struct MyActionGroup {
    group: gio::SimpleActionGroup,
    log: Rc<RefCell<Vec<String>>>,
}

impl MyActionGroup {
    pub fn new() -> Self {
        let app = Self {
            group: gio::SimpleActionGroup::new(),
            log: Default::default(),
        };
        app.register_actions(&app.group);
        app
    }
}

impl std::default::Default for MyActionGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[gio::actions]
impl MyActionGroup {
    fn action1(&self) {
        self.log.borrow_mut().push("action1".to_string());
    }

    #[action(name = "action_second")]
    fn action2(&self) {
        self.log.borrow_mut().push("action2".to_string());
    }

    fn action3(&self, param: String) {
        self.log.borrow_mut().push(format!("action3: {}", param));
    }

    fn action4(&self, param: bool) {
        self.log.borrow_mut().push(format!("action4: {}", param));
    }

    fn action5(&self, (param1, param2, param3, param4): (String, bool, String, i32)) {
        self.log.borrow_mut().push(format!(
            "action5: {} {} {} {}",
            param1, param2, param3, param4
        ));
    }

    #[action(stateful, initial_state = false)]
    fn stateful_toggle(&self, state: bool) -> Option<bool> {
        self.log
            .borrow_mut()
            .push(format!("stateful_toggle: {}", state));
        Some(!state)
    }

    #[action(stateful)]
    fn stateful_toggle_default(&self, state: bool) -> Option<bool> {
        self.log
            .borrow_mut()
            .push(format!("stateful_toggle_default: {}", state));
        Some(!state)
    }

    #[action(stateful, initial_state = "()")]
    fn stateful_text(&self, state: String) -> Option<String> {
        self.log
            .borrow_mut()
            .push(format!("stateful_text: {}", state));
        Some(format!("({})", state))
    }

    #[action(stateful, initial_state = true)]
    fn stateful_toggle_with_parameter(&self, state: bool, mut param: String) -> Option<bool> {
        if state {
            param = param.chars().rev().collect();
        }
        self.log.borrow_mut().push(format!(
            "stateful_toggle_with_parameter: {} {}",
            state, param
        ));
        Some(!state)
    }

    #[action(change_state, initial_state = true)]
    fn change_state_with_inferred_parameter(&self, state: bool) -> Option<bool> {
        self.log
            .borrow_mut()
            .push(format!("change_state_with_inferred_parameter: {}", state));
        Some(state)
    }

    #[action(change_state, initial_state = true, no_parameter)]
    fn change_state_without_parameter(&self, state: bool) -> Option<bool> {
        self.log
            .borrow_mut()
            .push(format!("change_state_without_parameter: {}", state));
        Some(state)
    }
}

#[test]
fn test_actions() {
    let app = MyActionGroup::new();
    app.group.activate_action("action1", None);
    app.group.activate_action("action_second", None);
    app.group
        .activate_action("action3", Some(&"Hello".to_variant()));
    app.group
        .activate_action("action1", Some(&"Unexpected parameter".to_variant()));
    app.group.activate_action("action3", None); // Missing parameter
    app.group.activate_action("action1", None);
    app.group
        .activate_action("action4", Some(&false.to_variant()));
    app.group
        .activate_action("action5", Some(&("Hello", true, "World", 42).to_variant()));

    app.group.activate_action("stateful_toggle", None);
    app.group.activate_action("stateful_toggle", None);
    app.group.activate_action("stateful_toggle", None);

    app.group.activate_action("stateful_toggle_default", None);
    app.group.activate_action("stateful_toggle_default", None);
    app.group.activate_action("stateful_toggle_default", None);

    app.group.activate_action("stateful_text", None);
    app.group.activate_action("stateful_text", None);
    app.group.activate_action("stateful_text", None);

    app.group.activate_action(
        "stateful_toggle_with_parameter",
        Some(&"Hello".to_variant()),
    );
    app.group.activate_action(
        "stateful_toggle_with_parameter",
        Some(&"World".to_variant()),
    );
    app.group.activate_action(
        "stateful_toggle_with_parameter",
        Some(&"World".to_variant()),
    );
    app.group.activate_action(
        "stateful_toggle_with_parameter",
        Some(&"Hello".to_variant()),
    );
    app.group.activate_action(
        "change_state_with_inferred_parameter",
        Some(&false.to_variant()),
    );
    app.group.activate_action(
        "change_state_with_inferred_parameter",
        Some(&true.to_variant()),
    );
    app.group
        .activate_action("change_state_without_parameter", None);
    app.group
        .activate_action("change_state_without_parameter", None);
    app.group
        .activate_action("change_state_without_parameter", None);

    assert_eq!(
        app.log.borrow().as_ref(),
        vec![
            "action1",
            "action2",
            "action3: Hello",
            "action1",
            "action4: false",
            "action5: Hello true World 42",
            "stateful_toggle: false",
            "stateful_toggle: true",
            "stateful_toggle: false",
            "stateful_toggle_default: false",
            "stateful_toggle_default: true",
            "stateful_toggle_default: false",
            "stateful_text: ()",
            "stateful_text: (())",
            "stateful_text: ((()))",
            "stateful_toggle_with_parameter: true olleH",
            "stateful_toggle_with_parameter: false World",
            "stateful_toggle_with_parameter: true dlroW",
            "stateful_toggle_with_parameter: false Hello",
            "change_state_with_inferred_parameter: false",
            "change_state_with_inferred_parameter: true",
            "change_state_without_parameter: false",
            "change_state_without_parameter: true",
            "change_state_without_parameter: false",
        ]
    );
}
