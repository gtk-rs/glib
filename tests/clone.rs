extern crate glib;

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
}
