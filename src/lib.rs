// Copyright 2016 `multipart` Crate Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate log;

extern crate mime;
extern crate mime_guess;
extern crate rand;
extern crate tempfile;

#[cfg(feature = "quick-error")]
#[macro_use]
extern crate quick_error;

extern crate safemem;

#[cfg(test)]
extern crate env_logger;

use rand::Rng;

/// Chain a series of results together, with or without previous results.
///
/// ```
/// #[macro_use] extern crate multipart;
///
/// fn try_add_one(val: u32) -> Result<u32, u32> {
///     if val < 5 {
///         Ok(val + 1)
///     } else {
///         Err(val)
///     }
/// }
///
/// fn main() {
///     let res = chain_result! {
///         try_add_one(1),
///         prev -> try_add_one(prev),
///         prev -> try_add_one(prev),
///         prev -> try_add_one(prev)
///     };
///
///     println!("{:?}", res);
/// }
///
/// ```
#[macro_export]
macro_rules! chain_result {
    ($first_expr:expr, $($try_expr:expr),*) => (
        $first_expr $(.and_then(|_| $try_expr))*
    );
    ($first_expr:expr, $($($arg:ident),+ -> $try_expr:expr),*) => (
        $first_expr $(.and_then(|$($arg),+| $try_expr))*
    );
}

pub mod server;

fn random_alphanumeric(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(len)
        .map(|c| c as char)
        .collect()
}
