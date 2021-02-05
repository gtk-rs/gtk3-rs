use std::cell::RefCell;
use std::marker::PhantomData;
use std::panic;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use glib::{clone, Downgrade, Object};

struct State {
    count: i32,
    started: bool,
}

impl State {
    fn new() -> Self {
        Self {
            count: 0,
            started: false,
        }
    }
}

#[test]
fn clone_and_references() {
    let state = Rc::new(RefCell::new(State::new()));
    let ref_state = &state;
    assert_eq!(ref_state.borrow().started, false);

    let closure = {
        clone!(@weak ref_state => move || {
            ref_state.borrow_mut().started = true;
        })
    };

    closure();
    assert_eq!(ref_state.borrow().started, true);
}

#[test]
fn subfields_renaming() {
    struct Foo {
        v: Rc<usize>,
    }

    impl Foo {
        fn foo(&self) {
            let state = Rc::new(RefCell::new(State::new()));

            let closure = clone!(@strong self.v as v, @weak state as hello => move |_| {
                println!("v: {}", v);
                hello.borrow_mut().started = true;
            });
            closure(2);
        }
    }

    Foo { v: Rc::new(0) }.foo();
}

#[test]
fn renaming() {
    let state = Rc::new(RefCell::new(State::new()));
    assert_eq!(state.borrow().started, false);

    let closure = {
        clone!(@weak state as hello => move || {
            hello.borrow_mut().started = true;
        })
    };

    closure();
    assert_eq!(state.borrow().started, true);
}

#[test]
fn clone_closure() {
    let state = Rc::new(RefCell::new(State::new()));
    assert_eq!(state.borrow().started, false);

    let closure = {
        clone!(@weak state => move || {
            state.borrow_mut().started = true;
        })
    };

    closure();

    assert_eq!(state.borrow().started, true);
    assert_eq!(state.borrow().count, 0);

    let closure = {
        let state2 = Rc::new(RefCell::new(State::new()));
        assert_eq!(state.borrow().started, true);

        clone!(@weak state, @strong state2 => move || {
            state.borrow_mut().count += 1;
            state.borrow_mut().started = true;
            state2.borrow_mut().started = true;
        })
    };

    closure();

    assert_eq!(state.borrow().count, 1);
    assert_eq!(state.borrow().started, true);
}

#[test]
fn clone_default_value() {
    let closure = {
        let state = Rc::new(RefCell::new(State::new()));
        clone!(@weak state => @default-return 42, move |_| {
            state.borrow_mut().started = true;
            10
        })
    };

    assert_eq!(42, closure(50));
}

#[test]
fn clone_panic() {
    let state = Arc::new(Mutex::new(State::new()));
    state.lock().expect("Failed to lock state mutex").count = 20;

    let closure = {
        let state2 = Arc::new(Mutex::new(State::new()));
        clone!(@weak state2, @strong state => @default-return panic!(), move |_| {
            state.lock().expect("Failed to lock state mutex").count = 21;
            state2.lock().expect("Failed to lock state2 mutex").started = true;
            10
        })
    };

    let result = panic::catch_unwind(|| {
        closure(50);
    });

    assert!(result.is_err());

    assert_eq!(state.lock().expect("Failed to lock state mutex").count, 20);
}

#[test]
fn clone_import_rename() {
    import_rename::test();
}

mod import_rename {
    use glib::clone as clone_g;

    #[allow(unused_macros)]
    macro_rules! clone {
        ($($anything:tt)*) => {
            |_, _| panic!("The clone! macro doesn't support renaming")
        };
    }

    #[allow(unused_variables)]
    pub fn test() {
        let n = 2;

        let closure: Box<dyn Fn(u32, u32)> = Box::new(clone_g!(
            @strong n
            => move |_, _|
            println!("The clone! macro does support renaming")
        ));

        closure(0, 0);
    }
}

#[test]
fn test_clone_macro_self_rename() {
    #[derive(Debug)]
    struct Foo {
        v: u8,
    }

    impl Foo {
        #[allow(dead_code)]
        fn foo(&self) {
            let closure = clone!(@strong self as this => move |_x| {
                println!("v: {:?}", this);
            });
            closure(0i8); // to prevent compiler error for unknown `x` type.
            let _ = clone!(@strong self as this => move || {
                println!("v: {:?}", this);
            });
            let closure = clone!(@strong self as this => move |_x| println!("v: {:?}", this));
            closure(0i8); // to prevent compiler error for unknown `x` type.
            let _ = clone!(@strong self as this => move || println!("v: {:?}", this));

            // Fields now!
            let closure = clone!(@strong self.v as v => move |_x| {
                println!("v: {:?}", v);
            });
            closure(0i8); // to prevent compiler error for unknown `x` type.
            let _ = clone!(@strong self.v as v => move || println!("v: {:?}", v));

            // With @default-panic
            let closure = clone!(@strong self.v as v => @default-panic, move |_x| {
                println!("v: {:?}", v);
            });
            closure(0i8); // to prevent compiler error for unknown `x` type.
            let _ = clone!(@strong self.v as v => @default-panic, move || println!("v: {:?}", v));

            // With @default-return
            let closure = clone!(@strong self.v as _v => @default-return true, move |_x| {
                false
            });
            closure(0i8); // to prevent compiler error for unknown `x` type.
            let _ = clone!(@strong self.v as _v => @default-return true, move || false);
        }
    }
}

#[test]
fn test_clone_macro_rename() {
    let v = Rc::new(1);

    let closure = clone!(@weak v as y => @default-panic, move |_x| {
        println!("v: {}", y);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v as y => @default-panic, move || println!("v: {}", y));

    let closure = clone!(@strong v as y => @default-panic, move |_x| {
        println!("v: {}", y);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@strong v as y => @default-panic, move || println!("v: {}", y));

    let closure = clone!(@weak v as y => move |_x| {
        println!("v: {}", y);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v as y => move || println!("v: {}", y));

    let closure = clone!(@strong v as y => move |_x| {
        println!("v: {}", y);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@strong v as y => move || println!("v: {}", y));

    let closure = clone!(@weak v as _y => @default-return true, move |_x| {
        false
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v as _y => @default-return true, move || false);

    let closure = clone!(@strong v as _y => @default-return true, move |_x| false);
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@strong v as _y => @default-return true, move || false);
}

#[test]
fn test_clone_macro_simple() {
    let v = Rc::new(1);

    let closure = clone!(@weak v => @default-panic, move |_x| {
        println!("v: {}", v);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v => @default-panic, move || println!("v: {}", v));

    let closure = clone!(@strong v => @default-panic, move |_x| {
        println!("v: {}", v);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@strong v => @default-panic, move || println!("v: {}", v));

    let closure = clone!(@weak v => move |_x| {
        println!("v: {}", v);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v => move || println!("v: {}", v));

    let closure = clone!(@strong v => move |_x| {
        println!("v: {}", v);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@strong v => move || println!("v: {}", v));

    let closure = clone!(@weak v => @default-return true, move |_x| {
        false
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v => @default-return true, move || false);

    let closure = clone!(@strong v => @default-return true, move |_x| false);
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@strong v => @default-return true, move || false);
}

#[test]
fn test_clone_macro_double_simple() {
    let v = Rc::new(1);
    let w = Rc::new(2);

    let closure = clone!(@weak v, @weak w => @default-panic, move |_x| {
        println!("v: {}, w: {}", v, w);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v, @weak w => @default-panic, move || println!("v: {}, w: {}", v, w));

    let closure = clone!(@strong v, @strong w => @default-panic, move |_x| {
        println!("v: {}, w: {}", v, w);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@strong v, @strong w => @default-panic, move || println!("v: {}, w: {}", v, w));

    let closure = clone!(@weak v, @weak w => move |_x| {
        println!("v: {}, w: {}", v, w);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v, @weak w => move || println!("v: {}, w: {}", v, w));

    let closure = clone!(@strong v, @strong w => move |_x| {
        println!("v: {}, w: {}", v, w);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@strong v, @strong w => move || println!("v: {}, w: {}", v, w));

    let closure = clone!(@weak v, @weak w => @default-return true, move |_x| {
        false
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v, @weak w => @default-return true, move || false);

    let closure = clone!(@strong v, @strong w => @default-return true, move |_x| false);
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@strong v, @strong w => @default-return true, move || false);
}

#[test]
fn test_clone_macro_double_rename() {
    let v = Rc::new(1);
    let w = Rc::new(2);

    let closure = clone!(@weak v as x, @weak w => @default-panic, move |_x| {
        println!("v: {}, w: {}", x, w);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v as x, @weak w => @default-panic, move || println!("v: {}, w: {}", x, w));

    let closure = clone!(@weak v, @weak w as x => @default-panic, move |_x| {
        println!("v: {}, w: {}", v, x);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v, @weak w as x => @default-panic, move || println!("v: {}, w: {}", v, x));

    let closure = clone!(@strong v as x, @strong w => @default-panic, move |_x| {
        println!("v: {}, w: {}", x, w);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ =
        clone!(@strong v as x, @strong w => @default-panic, move || println!("v: {}, w: {}", x, w));

    let closure = clone!(@strong v, @strong w as x => @default-panic, move |_x| {
        println!("v: {}, w: {}", v, x);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ =
        clone!(@strong v, @strong w as x => @default-panic, move || println!("v: {}, w: {}", v, x));

    let closure = clone!(@weak v as x, @weak w => move |_x| {
        println!("v: {}, w: {}", x, w);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v as x, @weak w => move || println!("v: {}, w: {}", x, w));

    let closure = clone!(@weak v, @weak w as x => move |_x| {
        println!("v: {}, w: {}", v, x);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v, @weak w as x => move || println!("v: {}, w: {}", v, x));

    let closure = clone!(@strong v as x, @strong w => move |_x| {
        println!("v: {}, w: {}", x, w);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@strong v as x, @strong w => move || println!("v: {}, w: {}", x, w));

    let closure = clone!(@strong v, @strong w as x => move |_x| {
        println!("v: {}, w: {}", v, x);
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@strong v, @strong w as x => move || println!("v: {}, w: {}", v, x));

    let closure = clone!(@weak v as _x, @weak w => @default-return true, move |_| {
        false
    });
    closure(0u8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v as _x, @weak w => @default-return true, move || false);

    let closure = clone!(@weak v, @weak w as _x => @default-return true, move |_| {
        false
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@weak v, @weak w as _x => @default-return true, move || false);

    let closure = clone!(@strong v as _x, @strong w => @default-return true, move |_| {
        false
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@strong v as _x, @strong w => @default-return true, move || false);

    let closure = clone!(@strong v, @strong w as _x => @default-return true, move |_| {
        false
    });
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(@strong v, @strong w as _x => @default-return true, move || false);
}

#[test]
fn test_clone_macro_typed_args() {
    let v = Rc::new(1);
    let w = Rc::new(2);

    let _closure = clone!(@weak v as x, @weak w => @default-panic, move |_x: i8| {
        println!("v: {}, w: {}", x, w);
    });

    let _closure = clone!(@weak v, @weak w as x => @default-panic, move |_x: i8| {
        println!("v: {}, w: {}", v, x);
    });

    let _closure = clone!(@strong v as x, @strong w => @default-panic, move |_x: i8| {
        println!("v: {}, w: {}", x, w);
    });

    let _closure = clone!(@strong v, @strong w as x => @default-panic, move |_x: i8| {
        println!("v: {}, w: {}", v, x);
    });

    let _closure = clone!(@weak v as x, @weak w => move |_x: i8| {
        println!("v: {}, w: {}", x, w);
    });

    let _closure = clone!(@weak v, @weak w as x => move |_x: i8| {
        println!("v: {}, w: {}", v, x);
    });

    let closure = clone!(@weak v, @weak w as x => move |_: i8, _| {
        println!("v: {}, w: {}", v, x);
    });
    closure(0, 'a');
}

#[test]
fn test_clone_macro_default_return() {
    let v = Rc::new(1);

    struct Foo(i32);

    let _closure = clone!(@weak v => @default-return Foo(0), move || Foo(1));

    #[allow(dead_code)]
    struct Bar {
        x: i32,
    }

    let _closure = clone!(@weak v => @default-return Bar { x: 0 }, move || Bar { x: 1 });

    #[allow(dead_code)]
    enum Enum {
        A,
        B(i32),
        C { x: i32 },
    }
    let _closure = clone!(@weak v => @default-return Enum::A, move || Enum::A);
    let _closure = clone!(@weak v => @default-return Enum::B(0), move || Enum::A);
    let _closure = clone!(@weak v => @default-return Enum::C { x: 0 }, move || Enum::A);
    let _closure = clone!(@weak v => @default-return { let x = 12; x + 2 }, move || 19);
}

#[test]
fn test_clone_macro_body() {
    let v = Rc::new(1);

    let _closure = clone!(@weak v => move || {
        ::std::thread::spawn(move || {
            for pos in 1..=10 {
                println!("{:?}", pos);
            }
        });
    });
}

#[test]
fn derive_downgrade() {
    #[derive(Downgrade)]
    pub struct NewType(Object);

    #[derive(Downgrade)]
    pub struct Struct {
        o1: Object,
        o2: std::rc::Rc<u32>,
    }

    #[derive(Downgrade)]
    pub enum Enum {
        None,
        Pair { x: Object, y: Object },
        Unit(),
        SingleUnnamed(Object),
        MultipleUnnamed(Object, Object, Object),
    }

    #[derive(Downgrade)]
    pub struct TypedWrapper<T>(Object, PhantomData<T>);

    #[derive(Downgrade)]
    pub enum TypedEnum<T> {
        This(Object, PhantomData<T>),
        That(Object, PhantomData<T>),
    }
}
