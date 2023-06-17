use yaserde::YaDeserialize;
use yaserde_derive::{YaDeserialize, YaSerialize};

use std::io::Read;

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

#[derive(Default, Debug, YaSerialize, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct Hole {
    #[yaserde(attribute)]
    pub timestamp: u64,
    #[yaserde(attribute)]
    pub length: u64,
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

impl VirtualBook {
    pub fn min_time(&self) -> Option<u64> {
        self.holes
            .holes
            .iter()
            .map(|h| h.timestamp + h.length)
            .reduce(|h, a| h.min(a))
    }

    pub fn max_time(&self) -> Option<u64> {
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
                    pipestopsetname: "default".into(),
                })
            })
            .collect();

        Self {
            holes: Holes { holes: vec![] },
            scale: Scale {
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
    no: u16,
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    delay: f32,
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    fixedlength: f32,
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    mididef: String,
}

#[derive(Default, Debug, YaSerialize, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct TrackRegisterControlStart {
    no: u16,
    pipestopsetname: String,
    pipestopnameinset: String,
}

#[derive(Default, Debug, YaSerialize, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct TrackRegisterControlReset {
    no: u16,
    resetpipestopsetname: String,
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
    pipestopsetname: String,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    pipestopnameinset: String,
    #[yaserde(child)]
    #[yaserde(
        prefix = "ns",
        namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
    )]
    resetpipestopsetname: String,
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
    tracks: Vec<Track>,
}

#[derive(Debug, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub enum Track {
    Unknown,
    TrackNoteDef(TrackNote),
    TrackDrum(TrackDrum),
    TrackRegisterControlStartDef(TrackRegisterControlStart),
    TrackRegisterControlResetDef(TrackRegisterControlReset),
}

impl YaDeserialize for Track {
    fn deserialize<R: std::io::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        let trackall = TrackALL::deserialize(reader)?;

        match trackall.xsitype.as_str() {
            "ns:TrackDrum" => Ok(Track::TrackDrum(TrackDrum {
                no: trackall.no,
                delay: trackall.delay,
                fixedlength: trackall.fixedlength,
                mididef: trackall.mididef,
            })),
            "ns:TrackNoteDef" => Ok(Track::TrackNoteDef(TrackNote {
                no: trackall.no,
                note: trackall.note,
                pipestopsetname: trackall.pipestopsetname,
            })),
            "ns:TrackRegisterControlStartDef" => Ok(Track::TrackRegisterControlStartDef(
                TrackRegisterControlStart {
                    no: trackall.no,
                    pipestopsetname: trackall.pipestopsetname,
                    pipestopnameinset: trackall.pipestopnameinset,
                },
            )),
            "ns:TrackRegisterControlResetDef" => Ok(Track::TrackRegisterControlResetDef(
                TrackRegisterControlReset {
                    no: trackall.no,
                    resetpipestopsetname: trackall.resetpipestopsetname,
                },
            )),

            e => Err(format!("error , unknown type {}", e)),
        }
    }
}

#[derive(Debug, YaSerialize, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "ns",
    namespace = "ns: http://barrelorgandiscovery.org/virtualbook/2016"
)]
pub struct TrackNote {
    no: u16,
    note: String,
    pipestopsetname: String,
}

pub fn read_book_stream(reader: &mut dyn Read) -> std::io::Result<VirtualBook> {
    let vb: VirtualBook = from_reader(reader).unwrap();
    // println!("{:?}", &vb);
    Ok(vb)
}

#[cfg(test)]
mod tests {

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
        // assert_eq!(holes.holes[0], Event::Pitch(Pitch { speed: 95, r#type: PitchType::FourSeam, outcome: PitchOutcome::Ball }));
    }

    #[test]
    fn read_file() -> std::io::Result<()> {
        use std::fs::File;
        use std::io::BufReader;

        let f = File::open("test_save.book")?;
        let reader = BufReader::new(f);
        let _vb: VirtualBook = from_reader(reader).unwrap();
        // println!("{:?}", &vb);
        Ok(())
    }
    #[test]
    fn perfs_reading() -> std::io::Result<()> {
        use microbench::{self, Options};
        let options = Options::default();
        microbench::bench(&options, "read_file_book", || read_file());
        Ok(())
    }
}
