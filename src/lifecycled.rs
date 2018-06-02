extern crate futures;

use self::futures::future::FutureResult;
use self::futures::Future;

pub struct LifecycleError {
    message: String,
}

impl LifecycleError {
    fn new(msg: String) -> LifecycleError {
        LifecycleError {
            message: msg
        }
    }
}

pub trait Lifecycled {
    fn initialize() -> FutureResult<(), LifecycleError>;
    fn destroy() -> FutureResult<(), LifecycleError>;
}

pub struct LifecyclePhase {
//    items: Vec<Future<Box<Lifecycled>, ()>>
}

impl LifecyclePhase {
    fn new() -> LifecyclePhase {
        LifecyclePhase {
//            items: vec![]
        }
    }
}
