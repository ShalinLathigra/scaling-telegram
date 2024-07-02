mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
pub use bindings::Gl as InnerGl;
use std::{ops::Deref, rc::Rc};

#[derive(Clone)]
pub struct Gl {
    inner: Rc<bindings::Gl>,
}

impl Gl {
    pub fn load_with<F>(loadfn: F) -> Gl
        where F: FnMut(&'static str) -> *const types::GLvoid
    {
        Gl {
            inner: Rc::new(bindings::Gl::load_with(loadfn))
        }
    }
}

impl Deref for Gl {
    type Target = bindings::Gl;
    fn deref(&self) -> &bindings::Gl {
        &self.inner
    }
}

/*

So why did all of this work?
Gl is a custom wrapper around all of the gl function pointers.

inner + Deref trait allow it to be accessed without using gl::Gl
Furthermore, it uses Rc (Ref Counted) to avoid passing around the actual
value of the struct when needed (Program, shaders, etc. all need to interact w/ Gl libraries regularly)

Whenever something needs references to these functions, we provide it with a copy of Gl (Rc<Gl>)
that increments the # tracked instances

When that thing dies, tracked instances is decremented

Once all of them are dead, this will be deleted. (at the end of the program)

*/