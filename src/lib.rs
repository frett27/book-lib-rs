use yaserde::YaDeserialize;
use yaserde_derive::{YaDeserialize, YaSerialize};

use std::{error::Error, io::Read};

use yaserde::de::from_reader;

#[derive(Default, Debug, YaSerialize, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct Holes {
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    #[yaserde(rename = "hole")]
    pub holes: Vec<Hole>,
}

#[derive(Default, Debug, YaSerialize, YaDeserialize, PartialEq, Clone)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct Hole {
    #[yaserde(attribute)]
    pub timestamp: i64,
    #[yaserde(attribute)]
    pub length: i64,
    #[yaserde(attribute)]
    pub track: u16,
}

#[derive(Default, Debug, YaSerialize, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct VirtualBook {
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub holes: Holes,
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub scale: Scale,
}

#[profiling::all_functions]
impl VirtualBook {
    pub fn min_time(&self) -> Option<i64> {
        self.holes
            .holes
            .iter()
            .map(|h| h.timestamp)
            .reduce(|h, a| h.min(a))
    }

    pub fn max_time(&self) -> Option<i64> {
        self.holes
            .holes
            .iter()
            .map(|h| h.timestamp + h.length)
            .reduce(|h, a| h.max(a))
    }

    pub fn midi_scale() -> Self {
        let r = 0..128;
        let tracks = r
            .map(|i| {
                Track::TrackNoteDef(TrackNote {
                    no: i,
                    note: "".into(),
                    pipestopsetname: Some("default".into()),
                })
            })
            .collect();

        Self {
            holes: Holes { holes: vec![] },
            scale: Scale {
                name: "".into(),
                definition: ScaleDefinition {
                    speed: 60.0,
                    width: 128.0,
                    firsttrackdistance: 0.5,
                    intertrackdistance: 1.0,
                    defaulttrackheight: 1.0,
                    tracks: Tracks { tracks },
                    scaletype: "".into(),
                    tracknb: 128,
                    bookmovefromrighttoleft: false,
                    ispreferredviewinverted: false,
                },
            },
        }
    }
}

#[derive(Default, Debug, YaSerialize, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct Scale {
    #[yaserde(attribute)]
    pub name: String,
    pub definition: ScaleDefinition,
}

#[derive(Default, Debug, YaSerialize, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct ScaleDefinition {
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub speed: f32,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub width: f32,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub firsttrackdistance: f32,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub intertrackdistance: f32,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub defaulttrackheight: f32,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub tracks: Tracks,

    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub scaletype: String,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub tracknb: u16,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub bookmovefromrighttoleft: bool,

    #[yaserde(child, default = false)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub ispreferredviewinverted: bool,
}

#[derive(Default, Debug, YaSerialize, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct TrackDrum {
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub no: u16,
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub delay: f32,
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub fixedlength: f32,
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pub mididef: String,
}

#[derive(Default, Debug, YaSerialize, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct TrackRegisterControlStart {
    pub no: u16,
    pub pipestopsetname: String,
    pub pipestopnameinset: String,
}

#[derive(Default, Debug, YaSerialize, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct TrackRegisterControlReset {
    pub no: u16,
    pub resetpipestopsetname: String,
}

#[derive(Default, Debug, YaSerialize, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct TrackALL {
    #[yaserde(attribute)]
    #[yaserde(rename = "type")]
    #[yaserde(
        prefix = "xsi",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance"
    )]
    xsitype: String,

    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    no: u16,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    delay: f32,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    fixedlength: f32,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    mididef: String,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pipestopsetname: NullableString,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pipestopnameinset: NullableString,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    resetpipestopsetname: NullableString,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    note: String,
}

#[derive(Default, Debug, YaSerialize, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct Tracks {
    #[yaserde(rename = "track")]
    pub tracks: Vec<Track>,
}

#[derive(Default, Debug, YaSerialize, PartialEq)]
pub struct NullableString {
    pub inner_string: Option<String>,
}

impl YaDeserialize for NullableString {
    #[allow(unused_assignments)]
    fn deserialize<R: std::io::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        let mut result = NullableString { inner_string: None };
        let mut last_characters: Option<String> = None;

        loop {
            let mut e = reader.peek()?.to_owned();
            match e {
                xml::reader::XmlEvent::EndElement { .. } => {
                    result.inner_string = last_characters;
                    break;
                }
                xml::reader::XmlEvent::Characters(characters) => {
                    last_characters = Some(characters);
                }
                xml::reader::XmlEvent::EndDocument => {
                    return Err("error while reaing NullableString, reached end of document".into());
                }
                _ => {}
            }
            e = reader.next_event()?;
        }
        Ok(result)
    }
}

#[derive(Debug, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub enum Track {
    Unknown,
    TrackNoteDef(TrackNote),
    TrackDrumDef(TrackDrum),
    TrackRegisterControlStartDef(TrackRegisterControlStart),
    TrackRegisterControlResetDef(TrackRegisterControlReset),
}

impl YaDeserialize for Track {
    fn deserialize<R: std::io::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        let trackall = TrackALL::deserialize(reader)?;

        match trackall.xsitype.as_str() {
            "ns:TrackDrum" => Ok(Track::TrackDrumDef(TrackDrum {
                no: trackall.no,
                delay: trackall.delay,
                fixedlength: trackall.fixedlength,
                mididef: trackall.mididef,
            })),
            "ns:TrackNoteDef" => Ok(Track::TrackNoteDef(TrackNote {
                no: trackall.no,
                note: trackall.note,
                pipestopsetname: trackall.pipestopsetname.inner_string,
            })),
            "ns:TrackRegisterControlStartDef" => Ok(Track::TrackRegisterControlStartDef(
                TrackRegisterControlStart {
                    no: trackall.no,
                    pipestopsetname: trackall.pipestopsetname.inner_string.unwrap(),
                    pipestopnameinset: trackall.pipestopnameinset.inner_string.unwrap(),
                },
            )),
            "ns:TrackRegisterControlResetDef" => Ok(Track::TrackRegisterControlResetDef(
                TrackRegisterControlReset {
                    no: trackall.no,
                    resetpipestopsetname: trackall.resetpipestopsetname.inner_string.unwrap(),
                },
            )),

            e => Err(format!("error , unknown type {:?}", e)),
        }
    }
}

#[derive(Debug, YaSerialize, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct TrackNote {
    pub no: u16,
    pub note: String,
    pub pipestopsetname: Option<String>,
}

/// read the virtual book from
pub fn read_book_stream(reader: &mut dyn Read) -> Result<VirtualBook, Box<dyn Error>> {
    let vb: VirtualBook = from_reader(reader)?;
    Ok(vb)
}

#[cfg(test)]
mod tests {

    use std::{ffi::OsString, fs::File, io::BufReader, path::PathBuf};

    use super::*;

    #[test]
    fn read_virtualbook() {
        let document = r#"
        <ns:virtualBook title=""
        xmlns:ns="http://barrelorgandiscovery.org/virtualbook/2016">
        <ns:metadata>
          <ns:Author xsi:nil="true"
            xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"/>
          <ns:Arranger xsi:nil="true"
            xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"/>
          <ns:Genre xsi:nil="true"
            xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"/>
          <ns:DesignedInstrumentName>52 Limonaire Fournier</ns:DesignedInstrumentName>
          <ns:Description xsi:nil="true"
            xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"/>
        </ns:metadata>
        <ns:scale name="52 Limonaire">
          <ns:infos>
            <ns:contact xsi:nil="true"
              xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"/>
            <ns:state>COMPLETED</ns:state>
            <ns:description>47 - chef
      44 - touche arret</ns:description>
          </ns:infos>
          <ns:definition>
            <ns:speed>60.0</ns:speed>
            <ns:width>200.0</ns:width>
            <ns:firsttrackdistance>8.5</ns:firsttrackdistance>
            <ns:intertrackdistance>3.5</ns:intertrackdistance>
            <ns:defaulttrackheight>3.0</ns:defaulttrackheight>
            <ns:tracks>
                <ns:track xsi:type="ns:TrackDrum"
                    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                    <ns:no>10</ns:no>
                    <ns:delay>6.0</ns:delay>
                    <ns:fixedlength>6.0</ns:fixedlength>
                    <ns:mididef>ACOUSTIC_BASS_DRUM</ns:mididef>
                </ns:track>
            </ns:tracks>
            <ns:pipestopsets>
              <ns:name>BASSE</ns:name>
              <ns:pipestop>
                <ns:name>BOURDON</ns:name>
                <ns:isPartOfRegister>false</ns:isPartOfRegister>
              </ns:pipestop>
              <ns:pipestop>
                <ns:name>TROMBONE</ns:name>
                <ns:isPartOfRegister>true</ns:isPartOfRegister>
              </ns:pipestop>
            </ns:pipestopsets>
            <ns:pipestopsets>
              <ns:name>ACCOMPAGNEMENT</ns:name>
              <ns:pipestop>
                <ns:name>VIOLONCELLE</ns:name>
                <ns:isPartOfRegister>false</ns:isPartOfRegister>
              </ns:pipestop>
              <ns:pipestop>
                <ns:name>BOURDON</ns:name>
                <ns:isPartOfRegister>false</ns:isPartOfRegister>
              </ns:pipestop>
            </ns:pipestopsets>
            <ns:pipestopsets>
              <ns:name>CHANT</ns:name>
              <ns:pipestop>
                <ns:name>VIOLONS</ns:name>
                <ns:isPartOfRegister>true</ns:isPartOfRegister>
              </ns:pipestop>
              <ns:pipestop>
                <ns:name>CLARINETTE</ns:name>
                <ns:isPartOfRegister>true</ns:isPartOfRegister>
              </ns:pipestop>
              <ns:pipestop>
                <ns:name>FLUTE</ns:name>
                <ns:isPartOfRegister>true</ns:isPartOfRegister>
              </ns:pipestop>
            </ns:pipestopsets>
            <ns:scaletype>default</ns:scaletype>
            <ns:tracknb>52</ns:tracknb>
            <ns:bookmovefromrighttoleft>true</ns:bookmovefromrighttoleft>
          </ns:definition>
        </ns:scale>
        <ns:holes>
          <ns:hole timestamp="1554824" length="223684" track="12"/>
          <ns:hole timestamp="1697982" length="165526" track="15"/>
          <ns:hole timestamp="1921665" length="371317" track="33"/>
          <ns:hole timestamp="3031140" length="322105" track="35"/>
          <ns:hole timestamp="3089298" length="1337631" track="51"/>
          <ns:hole timestamp="3415877" length="357894" track="26"/>
          <ns:hole timestamp="3469561" length="308684" track="40"/>
          <ns:hole timestamp="3733508" length="599474" track="48"/>
          <ns:hole timestamp="3760350" length="997632" track="41"/>
          <ns:hole timestamp="3930350" length="313158" track="27"/>
          <ns:hole timestamp="4019824" length="577105" track="46"/>
          <ns:hole timestamp="4216666" length="380263" track="9"/>
          <ns:hole timestamp="4480614" length="796315" track="18"/>
          <ns:hole timestamp="4525350" length="662106" track="14"/>
          <ns:hole timestamp="4766929" length="277369" track="12"/>
        </ns:holes>
      </ns:virtualBook>
    "#;
        use yaserde::de::from_str;

        let vb: VirtualBook = from_str(document).unwrap();
        println!("{:?}", vb);
    }
    #[test]
    fn read_holes() {
        let document = r#"
      <ns:holes xmlns:ns="http://barrelorgandiscovery.org/virtualbook/2016">
        <ns:hole timestamp="1554824" length="223684" track="12"/>
        <ns:hole timestamp="1697982" length="165526" track="15"/>
        <ns:hole timestamp="1921665" length="371317" track="33"/>
        <ns:hole timestamp="3031140" length="322105" track="35"/>
        <ns:hole timestamp="3089298" length="1337631" track="51"/>
        <ns:hole timestamp="3415877" length="357894" track="26"/>
        <ns:hole timestamp="3469561" length="308684" track="40"/>
        <ns:hole timestamp="3733508" length="599474" track="48"/>
        <ns:hole timestamp="3760350" length="997632" track="41"/>
        <ns:hole timestamp="3930350" length="313158" track="27"/>
        <ns:hole timestamp="4019824" length="577105" track="46"/>
        <ns:hole timestamp="4216666" length="380263" track="9"/>
        <ns:hole timestamp="4480614" length="796315" track="18"/>
        <ns:hole timestamp="4525350" length="662106" track="14"/>
        <ns:hole timestamp="4766929" length="277369" track="12"/>
      </ns:holes>"#;

        use yaserde::de::from_str;

        let holes: Holes = from_str(document).unwrap();
        println!("{:?}", holes);
        assert!(holes.holes.len() == 15);
    }

    #[test]
    fn read_file() -> Result<(), Box<dyn Error>> {
        use std::fs::File;
        use std::io::BufReader;

        let f = File::open("test_save.book")?;
        let reader = BufReader::new(f);
        let _vb: VirtualBook = from_reader(reader)?;
        // println!("{:?}", &vb);

        assert!(_vb.scale.name == "52 Limonaire");

        Ok(())
    }

    #[test]
    fn full_book_regression_and_fuzzy_tests() -> Result<(), Box<dyn Error>> {
        let mut failed_elements: Vec<(OsString, Box<dyn Error>)> = vec![];

        let book_testimonial = PathBuf::from("book_testimonial");
        let files_iterator = std::fs::read_dir(&book_testimonial)?;
        for dir_entry in files_iterator.flatten() {
            let filename = dir_entry.file_name();
            if filename
                .to_ascii_lowercase()
                .to_string_lossy()
                .ends_with(".book")
            {
                let f = File::open(dir_entry.path())?;
                println!("reading {:?}", f);

                let mut reader = BufReader::new(f);
                let vb_result = read_book_stream(&mut reader);
                if let Err(e) = vb_result {
                    failed_elements.push((filename.clone(), e));
                    println!("tests failed on {:?}", &filename);
                }
            }
        }

        if !failed_elements.is_empty() {
            panic!("failed_elements : {:?}", &failed_elements);
        }

        Ok(())
    }

    #[test]
    fn perfs_reading() -> Result<(), Box<dyn Error>> {
        use microbench::{self, Options};
        let options = Options::default();
        microbench::bench(&options, "read_file_book", || read_file().unwrap());
        Ok(())
    }
}
