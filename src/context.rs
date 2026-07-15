use crate::assets::{Assets, Options};

use geng::prelude::*;

#[derive(Clone)]
pub struct Context {
    pub geng: Geng,
    pub assets: Rc<Assets>,
    options: Rc<RefCell<Options>>,
}

impl Context {
    pub fn new(geng: Geng, assets: Rc<Assets>) -> Self {
        let options = Rc::new(RefCell::new(
            preferences::load(crate::OPTIONS_STORAGE).unwrap_or_default(),
        ));
        Self {
            geng,
            assets,
            options,
        }
    }

    pub fn get_options(&self) -> Options {
        self.options.borrow().clone()
    }

    #[allow(dead_code)]
    pub fn set_options(&self, options: Options) {
        let mut old = self.options.borrow_mut();
        if *old != options {
            preferences::save(crate::OPTIONS_STORAGE, &options);
            *old = options;
        }
    }
}
