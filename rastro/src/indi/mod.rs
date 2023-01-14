use std::fmt::{Debug, Display, Formatter};

pub mod common;
pub mod switch;
pub mod number;
pub mod text;
pub mod message;
pub mod light;
pub mod del;
pub mod blob;
pub mod connection;
pub mod get_properties;

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
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
    DelProperty(del::DelProperty),


    #[serde(rename = "getProperties")]
    GetProperties(get_properties::GetProperties),

    #[serde(
        //alias = "getProperties",
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

impl Display for IncomingMsg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IncomingMsg::Message(v) => Display::fmt(v, f),

            //Def
            IncomingMsg::DefSwitchVector(v) => Display::fmt(v, f),
            IncomingMsg::DefBlobVector(v) => Display::fmt(v, f),
            IncomingMsg::DefLightVector(v) => Display::fmt(v, f),
            IncomingMsg::DefTextVector(v) => Display::fmt(v, f),
            IncomingMsg::DefNumberVector(v) => Display::fmt(v, f),

            //Del
            IncomingMsg::DelProperty(v) => Display::fmt(v, f),

            //Set
            IncomingMsg::SetSwitchVector(v) => Display::fmt(v, f),
            IncomingMsg::SetBlobVector(v) => Display::fmt(v, f),
            IncomingMsg::SetTextVector(v) => Display::fmt(v, f),
            IncomingMsg::SetNumberVector(v) => Display::fmt(v, f),

            //One offs
            IncomingMsg::GetProperties(v) => Debug::fmt(v, f),

            IncomingMsg::Unparsed(v) => Debug::fmt(v, f),
        }
    }
}


#[cfg(test)]
mod test {
    use quick_xml::DeError;
    
    
    use crate::indi::switch::DefSwitchVector;

    #[test]
    fn it_parses() -> Result<(), DeError>{
        let xml = r#"
            <defSwitchVector device="Telescope Simulator" name="CONNECTION" label="Connection" group="Main Control" state="Idle" perm="rw" rule="OneOfMany" timeout="60" timestamp="2023-01-12T20:51:39">
                <defSwitch name="CONNECT" label="Connect">Off</defSwitch>
                <defSwitch name="DISCONNECT" label="Disconnect">On</defSwitch>
            </defSwitchVector>
        "#;

        /*

         */
        let el = quick_xml::de::from_str::<DefSwitchVector>(xml)?;
        assert_eq!(el.switches.len(), 2);
        Ok(())

    }
}
