/// Same as [`try!`] but to be used in functions that return `Box<dyn Future>` instead of `Result`.
///
/// Useful when calling synchronous (but cheap enough) functions in async code.
#[macro_export]
macro_rules! tryf {
    ($e:expr) => {
        match $e {
            Ok(value) => value,
            Err(e) => {
                return Box::new(::futures01::future::err(::std::convert::From::from(e)))
                    as Box<dyn ::futures01::future::Future<Item = _, Error = _>>;
            }
        }
    };
}

/// Declare a closure that clone specific values before moving them into their bodies. Mainly useful
/// when using combinator functions such as [`Future::and_then`](futures01::Future::and_then) or
/// [`Future::map`](futures01::Future::map).
#[macro_export]
macro_rules! clone {
    ($($n:ident),+ , || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );

    ($($n:ident),+ , |$($p:pat),*| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$($p),*| $body
        }
    );

    ($($n:ident),+ , |$($p:ident : $t:ty),*| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$($p : $t),*| $body
        }
    );
}
