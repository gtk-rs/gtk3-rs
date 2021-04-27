#[derive(glib::Downgrade)]
pub struct MyWidget(gtk::Widget);

#[gtk3_macros::builder_handlers]
impl MyWidget {
    fn handler0(&self) {}

    fn handler1(&self, _p: u32) {}

    fn handler2(&self, _x: f64, _y: f64) {}

    fn handler3(&self, x: f64, y: f64) -> f64 {
        x + y
    }

    // fn handler4(&self, x: f64, y: f64) -> Option<f64> {
    //     if x >= 0.0 && y >= 0.0 {
    //         Some(x + y)
    //     } else {
    //         None
    //     }
    // }
}

// Generated code
/*
impl MyWidget {
    #[allow(clippy)]
    fn get_handler(
        &self,
        signal: &str,
    ) -> Option<Box<dyn Fn(&[glib::Value]) -> Option<glib::Value> + 'static>> {
        match signal {
            "handler0" => Some({
                #[allow(unused_variables)]
                Box::new(
                    glib::clone!(@weak self as this => move |values: &[glib::Value]| {
                        this.handler0();
                        None
                    }),
                )
            }),
            "handler1" => Some({
                #[allow(unused_variables)]
                Box::new(
                    glib::clone!(@weak self as this => move |values: &[glib::Value]| {
                        this.handler1(
                            match values[0usize].get_some() {
                                Ok(value) => value,
                                Err(error) => {
                                    glib::g_critical!(
                                        "builder handler",
                                        "Handler {} expects an argument of type {} but received `{:?}`: {}.",
                                        "handler1", stringify! (u32),
                                        values[0usize], error
                                    );
                                    return None;
                                },
                            }
                        );
                        None
                    }),
                )
            }),
            "handler2" => Some({
                #[allow(unused_variables)]
                Box::new(
                    glib::clone!(@weak self as this => move |values: &[glib::Value]| {
                        this.handler2(
                            match values[0usize].get_some() {
                                Ok(value) => value,
                                Err(error) => {
                                    glib::g_critical!(
                                        "builder handler",
                                        "Handler {} expects an argument of type {} but received `{:?}`: {}.",
                                        "handler2",
                                        stringify!(f64),
                                        values[0usize],
                                        error
                                    );
                                    return None;
                                },
                            },
                            match values[1usize].get_some() {
                                Ok(value) => value,
                                Err(error) => {
                                    glib::g_critical!(
                                        "builder handler",
                                        "Handler {} expects an argument of type {} but received `{:?}`: {}.",
                                        "handler2",
                                        stringify!(f64),
                                        values[1usize],
                                        error
                                    );
                                    return None;
                                },
                            }
                        );
                        None
                    }),
                )
            }),
            "handler3" => Some({
                #[allow(unused_variables)]
                Box::new(
                    glib::clone!(@weak self as this => move |values: &[glib::Value]| {
                        let result = this.handler3(
                            match values[0usize].get_some() {
                                Ok(value) => value,
                                Err(error) => {
                                    glib::g_critical!(
                                        "builder handler",
                                        "Handler {} expects an argument of type {} but received `{:?}`: {}.",
                                        "handler3",
                                        stringify!(f64),
                                        values[0usize],
                                        error
                                    );
                                    return None;
                                },
                            },
                            match values[1usize].get_some() {
                                Ok(value) => value,
                                Err(error) => {
                                    glib::g_critical!(
                                        "builder handler",
                                        "Handler {} expects an argument of type {} but received `{:?}`: {}.",
                                        "handler3",
                                        stringify!(f64),
                                        values[1usize],
                                        error
                                    );
                                    return None;
                                },
                            }
                        );
                        Some(glib::value::ToValue::to_value(&result))
                    }),
                )
            }),
            _ => None,
        }
    }
}
*/
