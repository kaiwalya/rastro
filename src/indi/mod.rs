use crate::indi::del::DelProperty;

pub mod common;
pub mod switch;
pub mod text;
pub mod number;
pub mod light;
pub mod blob;
pub mod message;
pub mod connection;
pub mod del;

#[derive(Debug, serde::Deserialize, PartialEq)]
pub enum IncomingMsg {
    #[serde(rename = "defSwitchVector")]
    DefSwitchVector(switch::DefSwitchVector),
    #[serde(rename = "setSwitchVector")]
    SetSwitchVector(switch::SetSwitchVector),

    #[serde(rename = "defTextVector")]
    DefTextVector(text::DefTextVector),
    #[serde(rename = "setTextVector")]
    SetTextVector(text::SetTextVector),


    #[serde(rename = "defNumberVector")]
    DefNumberVector(number::DefNumberVector),
    #[serde(rename = "setNumberVector")]
    SetNumberVector(number::SetNumberVector),


    #[serde(rename = "defLightVector")]
    DefLightVector(light::DefLightVector),

    #[serde(rename = "defBLOBVector")]
    DefBlobVector(blob::DefBlobVector),
    #[serde(rename = "setBLOBVector")]
    SetBlobVector(blob::SetBlobVector),

    #[serde(rename = "message")]
    Message(message::Message),

    #[serde(rename = "delProperty")]
    DelProperty(DelProperty),

    #[serde(
        alias = "getProperties",
        alias = "newNumberVector",
        alias = "newLightVector",
        alias = "newSwitchVector",
        alias = "newTextVector",
        //alias = "delProperty",
        //alias = "message"
    )]
    Unparsed(std::collections::BTreeMap<String, String>),
    //Unparsed(String)
}
