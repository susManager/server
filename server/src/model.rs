use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Stoffe {
    AlteStoffe,
    NeueStoffe,
    ChemStoffe,
    DrugStoffe,
    KampfStoffe,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StoffListItem {
    pub name: String,
    pub blob: ByteArray,
}