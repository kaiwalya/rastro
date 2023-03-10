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
pub mod enable_blob;

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

    #[serde(rename = "enableBLOB")]
    EnableBLOB(enable_blob::EnableBLOB),


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
            IncomingMsg::EnableBLOB(v) => Debug::fmt(v, f),


            IncomingMsg::Unparsed(v) => Debug::fmt(v, f),
        }
    }
}


#[cfg(test)]
mod test {
    use quick_xml::DeError;
    use crate::indi::blob::SetBlobVector;


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

    #[test]
    fn it_parses_blob() -> Result<(), DeError> {
        let xml = r#"
        <setBLOBVector device="CCD Simulator" name="CCD1" state="Ok" timeout="60" timestamp="2023-02-11T07:16:57">
            <oneBLOB name="CCD1" size="8640" format=".fits" len="8640">
                U0lNUExFICA9ICAgICAgICAgICAgICAgICAgICBUIC8gZmlsZSBkb2VzIGNvbmZvcm0gdG8gRklUUyBzdGFuZGFyZCAgICAgICAgICAgICBCSVRQSVggID0gICAgICAgICAgICAgICAgICAgMTYgLyBudW1iZXIgb2YgYml0cyBwZXIgZGF0YSBwaXhlbCAgICAgICAgICAgICAgICAgIE5BWElTICAgPSAgICAgICAgICAgICAgICAgICAgMiAvIG51bWJlciBvZiBkYXRhIGF4ZXMgICAgICAgICAgICAgICAgICAgICAgICAgICAgTkFYSVMxICA9ICAgICAgICAgICAgICAgICAgICA0IC8gbGVuZ3RoIG9mIGRhdGEgYXhpcyAxICAgICAgICAgICAgICAgICAgICAgICAgICBOQVhJUzIgID0gICAgICAgICAgICAgICAgICAgIDQgLyBsZW5ndGggb2YgZGF0YSBheGlzIDIgICAgICAgICAgICAgICAgICAgICAgICAgIEVYVEVORCAgPSAgICAgICAgICAgICAgICAgICAgVCAvIEZJVFMgZGF0YXNldCBtYXkgY29udGFpbiBleHRlbnNpb25zICAgICAgICAgICAgQ09NTUVOVCAgIEZJVFMgKEZsZXhpYmxlIEltYWdlIFRyYW5zcG9ydCBTeXN0ZW0pIGZvcm1hdCBpcyBkZWZpbmVkIGluICdBc3Ryb25vbXlDT01NRU5UICAgYW5kIEFzdHJvcGh5c2ljcycsIHZvbHVtZSAzNzYsIHBhZ2UgMzU5OyBiaWJjb2RlOiAyMDAxQSZBLi4uMzc2Li4zNTlIIEJaRVJPICAgPSAgICAgICAgICAgICAgICAzMjc2OCAvIG9mZnNldCBkYXRhIHJhbmdlIHRvIHRoYXQgb2YgdW5zaWduZWQgc2hvcnQgICAgQlNDQUxFICA9ICAgICAgICAgICAgICAgICAgICAxIC8gZGVmYXVsdCBzY2FsaW5nIGZhY3RvciAgICAgICAgICAgICAgICAgICAgICAgICBST1dPUkRFUj0gJ1RPUC1ET1dOJyAgICAgICAgICAgLyBSb3cgT3JkZXIgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIElOU1RSVU1FPSAnQ0NEIFNpbXVsYXRvcicgICAgICAvIENDRCBOYW1lICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgVEVMRVNDT1A9ICdUZWxlc2NvcGUgU2ltdWxhdG9yJyAvIFRlbGVzY29wZSBuYW1lICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICBPQlNFUlZFUj0gJ1Vua25vd24gJyAgICAgICAgICAgLyBPYnNlcnZlciBuYW1lICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIE9CSkVDVCAgPSAnVW5rbm93biAnICAgICAgICAgICAvIE9iamVjdCBuYW1lICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgRVhQVElNRSA9ICAgICAgICAgMS4wMDAwMDBFKzAwIC8gVG90YWwgRXhwb3N1cmUgVGltZSAocykgICAgICAgICAgICAgICAgICAgICAgICBDQ0QtVEVNUD0gICAgICAgICAgICAwLjAwMEUrMDAgLyBDQ0QgVGVtcGVyYXR1cmUgKENlbHNpdXMpICAgICAgICAgICAgICAgICAgICAgIFBJWFNJWkUxPSAgICAgICAgIDUuMjAwMDAwRSswMCAvIFBpeGVsIFNpemUgMSAobWljcm9ucykgICAgICAgICAgICAgICAgICAgICAgICAgUElYU0laRTI9ICAgICAgICAgNS4yMDAwMDBFKzAwIC8gUGl4ZWwgU2l6ZSAyIChtaWNyb25zKSAgICAgICAgICAgICAgICAgICAgICAgICBYQklOTklORz0gICAgICAgICAgICAgICAgICAgIDEgLyBCaW5uaW5nIGZhY3RvciBpbiB3aWR0aCAgICAgICAgICAgICAgICAgICAgICAgIFlCSU5OSU5HPSAgICAgICAgICAgICAgICAgICAgMSAvIEJpbm5pbmcgZmFjdG9yIGluIGhlaWdodCAgICAgICAgICAgICAgICAgICAgICAgWFBJWFNaICA9ICAgICAgICAgNS4yMDAwMDBFKzAwIC8gWCBiaW5uZWQgcGl4ZWwgc2l6ZSBpbiBtaWNyb25zICAgICAgICAgICAgICAgICBZUElYU1ogID0gICAgICAgICA1LjIwMDAwMEUrMDAgLyBZIGJpbm5lZCBwaXhlbCBzaXplIGluIG1pY3JvbnMgICAgICAgICAgICAgICAgIEZSQU1FICAgPSAnTGlnaHQgICAnICAgICAgICAgICAvIEZyYW1lIFR5cGUgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgSU1BR0VUWVA9ICdMaWdodCBGcmFtZScgICAgICAgIC8gRnJhbWUgVHlwZSAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICBGSUxURVIgID0gJ1JlZCAgICAgJyAgICAgICAgICAgLyBGaWx0ZXIgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIEZPQ0FMTEVOPSAgICAgICAgICAgIDkuMDAwRSswMiAvIEZvY2FsIExlbmd0aCAobW0pICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgQVBURElBICA9ICAgICAgICAgICAgMS4yMDBFKzAyIC8gVGVsZXNjb3BlIGRpYW1ldGVyIChtbSkgICAgICAgICAgICAgICAgICAgICAgICBST1RBVEFORz0gICAgICAgICAgICAwLjAwMEUrMDAgLyBSb3RhdG9yIGFuZ2xlIGluIGRlZ3JlZXMgICAgICAgICAgICAgICAgICAgICAgIFNDQUxFICAgPSAgICAgICAgIDEuMTkxOTU2RSswMCAvIGFyY3NlY3MgcGVyIHBpeGVsICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgU0lURUxBVCA9ICAgICAgICAgMy43NTYzMDZFKzAxIC8gTGF0aXR1ZGUgb2YgdGhlIGltYWdpbmcgc2l0ZSBpbiBkZWdyZWVzICAgICAgICBTSVRFTE9ORz0gICAgICAgIC0xLjIyMzIyOEUrMDIgLyBMb25naXR1ZGUgb2YgdGhlIGltYWdpbmcgc2l0ZSBpbiBkZWdyZWVzICAgICAgIEFJUk1BU1MgPSAgICAgICAgIDEuNjM4NDczRSswMCAvIEFpcm1hc3MgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgT0JKQ1RBWiA9ICAgICAgICAgNi40NjUwODZFLTAzIC8gQXppbXV0aCBvZiBjZW50ZXIgb2YgaW1hZ2UgaW4gRGVncmVlcyAgICAgICAgICBPQkpDVEFMVD0gICAgICAgICAzLjc1NjMzOUUrMDEgLyBBbHRpdHVkZSBvZiBjZW50ZXIgb2YgaW1hZ2UgaW4gRGVncmVlcyAgICAgICAgIE9CSkNUUkEgPSAnMjMgNTUgNTIuMjcnICAgICAgICAvIE9iamVjdCBKMjAwMCBSQSBpbiBIb3VycyAgICAgICAgICAgICAgICAgICAgICAgT0JKQ1RERUM9ICc4OSA1MiAwMC42NCcgICAgICAgIC8gT2JqZWN0IEoyMDAwIERFQyBpbiBEZWdyZWVzICAgICAgICAgICAgICAgICAgICBSQSAgICAgID0gICAgICAgICAzLjU4OTY3OEUrMDIgLyBPYmplY3QgSjIwMDAgUkEgaW4gRGVncmVlcyAgICAgICAgICAgICAgICAgICAgIERFQyAgICAgPSAgICAgICAgIDguOTg2Njg1RSswMSAvIE9iamVjdCBKMjAwMCBERUMgaW4gRGVncmVlcyAgICAgICAgICAgICAgICAgICAgUElFUlNJREU9ICdFQVNUICAgICcgICAgICAgICAgIC8gRWFzdCwgbG9va2luZyBXZXN0ICAgICAgICAgICAgICAgICAgICAgICAgICAgICBFUVVJTk9YID0gICAgICAgICAgICAgICAgIDIwMDAgLyBFcXVpbm94ICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIERBVEUtT0JTPSAnMjAyMy0wMi0xMVQwNzoxNjo1Ni4xNzknIC8gVVRDIHN0YXJ0IGRhdGUgb2Ygb2JzZXJ2YXRpb24gICAgICAgICAgICAgQ09NTUVOVCBHZW5lcmF0ZWQgYnkgSU5ESSAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICBHQUlOICAgID0gICAgICAgICAgICA5LjAwMEUrMDEgLyBHYWluICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIEVORCAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAggAOACIAHgAaAAIAJgAaACIABgACACYADgACAAoAAgAgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
            </oneBLOB>
        </setBLOBVector>
        "#;

        let el = quick_xml::de::from_str::<SetBlobVector>(xml)?;

        Ok(())
    }
}
