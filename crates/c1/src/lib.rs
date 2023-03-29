pub use serde_json::{self, *};

// It will result in an error if we use `use c1::serde_json` in c2/src/lib.rs while avoiding `pub use serde_json::self;` like belwo
// pub use serde_json::*;
