//! A crate for loading data from the aseprite sprite editor. Should
//! go along well with the tiled crate, I hope!
//!
//! It does not load any actual images, just the metadata. Currently
//! it only loads aseprite's JSON export format, and only when
//! exported in a particular format that has all the options just
//! right. I've yet to find a use case that won't cover though.
//!
//! Automatically exporting a sprite to a given format is documented
//! here: https://www.aseprite.org/docs/cli/ The easy way to export in
//! the right format is to use a command such as `aseprite -b
//! boonga.ase --sheet boonga.png --format json-array --data
//! boonga.json`
//!
//! Otherwise you have to go to `file->export sprite sheet` and select
//! "array" rather than "hash".  Every.  Single.  Time.
//!
//! This has been tested to work with aseprite 1.1.6; newer or older
//! versions have not been tested.

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dimensions {
    pub w: u32,
    pub h: u32,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all="camelCase")]
pub struct Frame {
    pub filename: String,
    pub frame: Rect,
    pub rotated: bool,
    pub trimmed: bool,
    pub sprite_source_size: Rect,
    pub source_size: Dimensions,
    pub duration: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all="camelCase")]
pub enum Direction {
    Forward,
    Reverse,
    Pingpong,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
/// TODO: To be consistent with Aseprite this should be called `FrameTag`.
pub struct Frametag {
    pub name: String,
    pub from: u32,
    pub to: u32,
    pub direction: Direction,
}

// These are listed at:
// https://github.com/aseprite/aseprite/blob/2e3bbe2968da65fa8852ebb94464942bf9cb8870/src/doc/blend_mode.cpp#L17

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all="camelCase")]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    HslHue,
    HslSaturation,
    HslColor,
    HslLuminosity,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Cel {
    frame: u32,
    //opacity: u32, // TODO: This should be here, but isn't!
    color: String,
    data: Option<String>,

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged, rename_all="camelCase")]
pub enum Layer {
    Layer {
        name: String,
        group: Option<String>,
        opacity: u32,
        blend_mode: BlendMode,
        cels: Option<Vec<Cel>>,
        color: Option<String>,
        data: Option<String>,
    },
    Group {
        name: String,
        color: Option<String>,
        data: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Key {
    pub frame: u32,
    pub bounds: Rect,
    pub center: Option<Rect>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Slice {
    pub name: String,
    pub color: String,
    pub keys: Vec<Key>,
    pub data: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all="camelCase")]
pub struct Metadata {
    pub app: String,
    pub version: String,
    pub format: String,
    pub size: Dimensions,
    pub scale: String, // Surely this should be a number?
    pub frame_tags: Option<Vec<Frametag>>,
    pub layers: Option<Vec<Layer>>,
    pub slices: Option<Vec<Slice>>,
    pub image: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SpritesheetData {
    pub frames: Vec<Frame>,
    pub meta: Metadata,
}


#[cfg(test)]
mod tests {
    extern crate serde_json;

    const S: &'static str = r##"{ "frames": [
   {
    "filename": "boonga 0.ase",
    "frame": { "x": 1, "y": 1, "w": 18, "h": 18 },
    "rotated": false,
    "trimmed": false,
    "spriteSourceSize": { "x": 0, "y": 0, "w": 16, "h": 16 },
    "sourceSize": { "w": 16, "h": 16 },
    "duration": 250
   },
   {
    "filename": "boonga 1.ase",
    "frame": { "x": 20, "y": 1, "w": 18, "h": 18 },
    "rotated": false,
    "trimmed": false,
    "spriteSourceSize": { "x": 0, "y": 0, "w": 16, "h": 16 },
    "sourceSize": { "w": 16, "h": 16 },
    "duration": 250
   }
 ],
 "meta": {
  "app": "http://www.aseprite.org/",
  "version": "1.1.6-dev",
  "image": "boonga.png",
  "format": "RGBA8888",
  "size": { "w": 39, "h": 20 },
  "scale": "1",
  "frameTags": [
   { "name": "testtag", "from": 0, "to": 1, "direction": "forward" }
  ],
  "layers": [
   { "name": "Layer 1", "opacity": 255, "blendMode": "normal" }
  ]
 }
}
"##;


    const S_NO_META: &'static str = r##"{ "frames": [
   {
    "filename": "boonga 0.ase",
    "frame": { "x": 1, "y": 1, "w": 18, "h": 18 },
    "rotated": false,
    "trimmed": false,
    "spriteSourceSize": { "x": 0, "y": 0, "w": 16, "h": 16 },
    "sourceSize": { "w": 16, "h": 16 },
    "duration": 250
   },
   {
    "filename": "boonga 1.ase",
    "frame": { "x": 20, "y": 1, "w": 18, "h": 18 },
    "rotated": false,
    "trimmed": false,
    "spriteSourceSize": { "x": 0, "y": 0, "w": 16, "h": 16 },
    "sourceSize": { "w": 16, "h": 16 },
    "duration": 250
   }
 ],
 "meta": {
  "app": "http://www.aseprite.org/",
  "version": "1.1.6-dev",
  "image": "boonga.png",
  "format": "RGBA8888",
  "size": { "w": 39, "h": 20 },
  "scale": "1"
 }
}
"##;


    #[test]
    fn test_sprite_load_save() {
        let deserialized: super::SpritesheetData = serde_json::from_str(S).unwrap();

        let serialized = serde_json::to_string(&deserialized).unwrap();
        let deserialized_again: super::SpritesheetData = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, deserialized_again);
    }


    #[test]
    fn test_less_metadata() {
        let deserialized: super::SpritesheetData = serde_json::from_str(S_NO_META).unwrap();

        let serialized = serde_json::to_string(&deserialized).unwrap();
        let deserialized_again: super::SpritesheetData = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, deserialized_again);
    }

}
