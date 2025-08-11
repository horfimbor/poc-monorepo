pub mod civilisation;
pub mod planet;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Component {
    pub balise: String,
    pub id: String,
}
