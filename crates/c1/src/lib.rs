pub use serde_json::{self, *};

// It will result in an error if we call `use c1::serde_json` in c2/src/lib.rs while avoiding `pub use serde_json::self;` as below
// pub use serde_json::*;
