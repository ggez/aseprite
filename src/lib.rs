//! A crate for loading data from the aseprite sprite editor. Should
//! go along well with the tiled crate, I hope!
//!
//! It does not load any actual images, just the metadata. Currently
//! it only loads aseprite's JSON export format.  I've yet to find a use case
//! that won't cover though.
//!
//! Automatically exporting a sprite to a given format is documented
//! here: https://www.aseprite.org/docs/cli/ The easy way to export in
//! the right format is to use a command such as `aseprite -b
//! boonga.ase --sheet boonga.png --format json-array --data
//! boonga.json`
//!
//! Otherwise you have to go to `file->export sprite sheet`.
//!
//! This has been tested to work with aseprite 1.1.6 and 1.2.25; other
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
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dimensions {
    pub w: u32,
    pub h: u32,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl std::fmt::Debug for Color {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Self { r, g, b, a } = self;
        write!(fmt, "#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
    }
}

impl serde::Serialize for Color {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        format!("{:?}", self).serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Color {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        if !s.starts_with("#") {
            Err(serde::de::Error::custom("color doesn't start with '#'"))
        } else if !s.len() == 7 {
            Err(serde::de::Error::custom("color has wrong length"))
        } else {
            let r = u8::from_str_radix(&s[1..3], 16)
                .map_err(|_| serde::de::Error::custom("color has non-hex red component"))?;
            let g = u8::from_str_radix(&s[3..5], 16)
                .map_err(|_| serde::de::Error::custom("color has non-hex green component"))?;
            let b = u8::from_str_radix(&s[5..7], 16)
                .map_err(|_| serde::de::Error::custom("color has non-hex blue component"))?;
            let a = u8::from_str_radix(&s[7..9], 16)
                .map_err(|_| serde::de::Error::custom("color has non-hex alpha component"))?;
            Ok(Self { r, g, b, a })
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Frame {
    pub filename: String,
    #[serde(flatten)]
    pub data: FrameData,
}

impl std::ops::Deref for Frame {
    type Target = FrameData;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for Frame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FrameData {
    pub frame: Rect,
    pub rotated: bool,
    pub trimmed: bool,
    pub sprite_source_size: Rect,
    pub source_size: Dimensions,
    pub duration: u32,
}

fn deserialize_frames<'de, D: serde::Deserializer<'de>>(de: D) -> Result<Vec<Frame>, D::Error> {
    struct FramesVisitor;
    impl<'de> serde::de::Visitor<'de> for FramesVisitor {
        type Value = Vec<Frame>;
        fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.write_str("a json array or map")
        }

        fn visit_map<M: serde::de::MapAccess<'de>>(
            self,
            mut map: M,
        ) -> Result<Self::Value, M::Error> {
            let mut frames = Vec::new();
            while let Some(key) = map.next_key()? {
                frames.push(Frame {
                    filename: key,
                    data: map.next_value()?,
                });
            }
            Ok(frames)
        }

        fn visit_seq<S: serde::de::SeqAccess<'de>>(
            self,
            mut seq: S,
        ) -> Result<Self::Value, S::Error> {
            let mut frames = Vec::new();
            while let Some(frame) = seq.next_element()? {
                frames.push(frame);
            }
            Ok(frames)
        }
    }

    de.deserialize_any(FramesVisitor)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum Direction {
    Forward,
    Reverse,
    Pingpong,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Frametag {
    pub name: String,
    pub from: u32,
    pub to: u32,
    pub direction: Direction,
}

// These are listed at:
// https://github.com/aseprite/aseprite/blob/51b038ac024dd99902ab5b0c0d61524c48856b93/src/doc/blend_mode.cpp#L18-L37

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[derive(Default)]
pub enum BlendMode {
    #[default]
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
    Addition,
    Subtract,
    Divide,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Layer {
    pub name: String,
    pub group: Option<String>,
    #[serde(default)] // 0 / missing for groups - editor shows "0" greyed out
    pub opacity: u32,
    // 0 / missing for groups - editor shows "Normal" greyed out
    pub blend_mode: BlendMode,
    pub color: Option<Color>,
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Slice {
    pub name: String,
    pub color: Color,
    pub data: Option<String>,
    pub keys: Vec<SliceKey>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SliceKey {
    pub frame: u32,
    pub bounds: Rect,
    pub pivot: Option<Point>,
    pub center: Option<Rect>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Metadata {
    pub app: String,
    pub version: String,
    pub format: String,
    pub size: Dimensions,
    pub scale: String, // Surely this should be a number?
    pub frame_tags: Vec<Frametag>,
    #[serde(default)]
    pub layers: Vec<Layer>,
    pub image: Option<String>,
    #[serde(default)]
    pub slices: Vec<Slice>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpritesheetData {
    #[serde(deserialize_with = "deserialize_frames")]
    pub frames: Vec<Frame>,
    pub meta: Metadata,
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    const S: &str = r##"{ "frames": [
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

    const S_NO_META: &str = r##"{ "frames": [
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

    #[test]
    fn test_aseprite_test_data() {
        use super::SpritesheetData;
        use std::convert::*;

        for file in aseprite_test_data::FileSet::list() {
            let basic_json: SpritesheetData = serde_json::from_slice(file.basic_json)
                .unwrap_or_else(|e| {
                    panic!(
                        "{}/basic/{}.json: failed to deserialize: {}",
                        file.version, file.name, e
                    )
                });
            let array_json: SpritesheetData = serde_json::from_slice(file.array_json)
                .unwrap_or_else(|e| {
                    panic!(
                        "{}/array/{}.json: failed to deserialize: {}",
                        file.version, file.name, e
                    )
                });
            let hash_json: SpritesheetData =
                serde_json::from_slice(file.hash_json).unwrap_or_else(|e| {
                    panic!(
                        "{}/hash/{}.json: failed to deserialize: {}",
                        file.version, file.name, e
                    )
                });

            for (i, json) in [&basic_json, &array_json, &hash_json]
                .iter()
                .cloned()
                .enumerate()
            {
                let is_basic = i == 0;
                assert_eq!(file.n_frames, json.frames.len());
                assert_eq!(
                    if is_basic { 0 } else { file.n_layers },
                    json.meta.layers.len()
                );
                assert_eq!(
                    if is_basic { 0 } else { file.n_slices },
                    json.meta.slices.len()
                );
            }

            for (png_name, png) in [
                ("basic", file.basic_png),
                ("array", file.array_png),
                ("hash", file.hash_png),
            ]
            .iter()
            .copied()
            {
                let path = format!("data/{}/{}/{}.png", file.version, png_name, file.name);

                let (png_info, mut reader) = png::Decoder::new(std::io::Cursor::new(png))
                    .read_info()
                    .unwrap_or_else(|e| panic!("{}: error decoding info: {}", path, e));
                let mut frame = Vec::new();
                frame.resize(png_info.buffer_size(), 0);
                reader
                    .next_frame(&mut frame)
                    .unwrap_or_else(|e| panic!("{}: error decoding frame: {}", path, e));

                let png_color_profile = png_color_profile(png);
                assert_eq!(
                    png_color_profile, file.png_color_profile,
                    "{}: decoded with ColorProfile::{:?} but expected ColorProfile::{:?}",
                    path, png_color_profile, file.png_color_profile
                );
                assert_eq!(
                    png_info.width,
                    file.size[0] * file.n_frames as u32,
                    "{}: expected {}x{} but got {}x{}",
                    path,
                    png_info.width,
                    png_info.height,
                    file.size[0],
                    file.size[1]
                );
                assert_eq!(
                    png_info.height, file.size[1],
                    "{}: expected {}x{} but got {}x{}",
                    path, png_info.width, png_info.height, file.size[0], file.size[1]
                );
                assert_eq!(
                    png_info.bit_depth,
                    png::BitDepth::Eight,
                    "{}: expected 8BPP",
                    path
                );
                assert_eq!(
                    png_info.color_type,
                    png::ColorType::RGBA,
                    "{}: expected RGBA",
                    path
                );

                if let Some(file_pixels) = file.pixels {
                    assert_eq!(
                        file_pixels.len(),
                        frame.len() / 4,
                        "{}: expected {} pixels ({}x{}) but decoded {} pixels from png",
                        path,
                        file_pixels.len(),
                        file.size[0],
                        file.size[1],
                        frame.len() / 4
                    );
                    let png_pixels = frame.chunks_exact(4).map(|p| {
                        aseprite_test_data::RGBA(u32::from_be_bytes(p.try_into().unwrap()))
                    });
                    for ((i, expected), actual) in
                        file_pixels.iter().copied().enumerate().zip(png_pixels)
                    {
                        let w = png_info.width as usize;
                        assert_eq!(
                            expected,
                            actual,
                            "{}: pixel ({},{}): expected {:?} but got {:?}",
                            path,
                            i % w,
                            i / w,
                            expected,
                            actual
                        );
                    }
                }
            }
        }
    }

    fn png_color_profile(png: &[u8]) -> aseprite_test_data::PngColorProfile {
        let mut discard = Vec::new();
        let mut decoder = png::StreamingDecoder::new();
        let mut has_srgb = false;
        let mut has_iccp = false;
        let mut rest = png;
        while !rest.is_empty() {
            let (next, decoded) = decoder.update(rest, &mut discard).unwrap();
            rest = &rest[next..];

            const SRGB: png::chunk::ChunkType = *b"sRGB"; // https://en.wikipedia.org/wiki/SRGB
            const ICCP: png::chunk::ChunkType = *b"iCCP"; // International Color Consortium Profile
            match decoded {
                png::Decoded::PartialChunk(SRGB) => has_srgb = true,
                png::Decoded::PartialChunk(ICCP) => has_iccp = true,
                _ => {}
            }
        }

        match (has_srgb, has_iccp) {
            (true, _) => aseprite_test_data::PngColorProfile::SRGB,
            (false, true) => aseprite_test_data::PngColorProfile::Other,
            (false, false) => aseprite_test_data::PngColorProfile::None,
        }
    }

    #[test]
    fn test_aseprite_test_data_complex() {
        use super::{BlendMode, Dimensions, Direction, Point, Rect, SpritesheetData};

        let complex = aseprite_test_data::FileSet::complex_1_2_25();
        let array: SpritesheetData = serde_json::from_slice(complex.array_json).unwrap();
        let basic: SpritesheetData = serde_json::from_slice(complex.basic_json).unwrap();
        let hash: SpritesheetData = serde_json::from_slice(complex.hash_json).unwrap();

        macro_rules! assert_fields_eq {
            ( $expected:expr, $($field:tt)* ) => {
                let expected = $expected;
                assert_eq!(basic. $($field)*, expected);
                assert_eq!(array. $($field)*, expected);
                assert_eq!(hash.  $($field)*, expected);
            };
        }
        assert_fields_eq!(
            Some("complex.aseprite.png"),
            meta.image.as_ref().map(|s| s.as_str())
        );
        assert_fields_eq!("I8", meta.format);
        assert_fields_eq!(Dimensions { w: 72, h: 8 }, meta.size);

        // Frames

        assert_fields_eq!(9, frames.len());

        for (((i, basic), array), hash) in basic
            .frames
            .iter()
            .enumerate()
            .zip(array.frames.iter())
            .zip(hash.frames.iter())
        {
            macro_rules! assert_fields_eq {
                ( $expected:expr, $($field:tt)* ) => {
                    let expected = $expected;
                    assert_eq!(basic. $($field)*, expected);
                    assert_eq!(array. $($field)*, expected);
                    assert_eq!(hash.  $($field)*, expected);
                };
            }

            assert_fields_eq!(format!("complex {}.aseprite", i), filename);
            assert_fields_eq!(
                Rect {
                    x: (i * 8) as u32,
                    y: 0,
                    w: 8,
                    h: 8
                },
                frame
            );
            assert_fields_eq!(false, rotated);
            assert_fields_eq!(false, trimmed);
            assert_fields_eq!(
                Rect {
                    x: 0,
                    y: 0,
                    w: 8,
                    h: 8
                },
                sprite_source_size
            );
            assert_fields_eq!(Dimensions { w: 8, h: 8 }, source_size);
            assert_fields_eq!((100 * (i + 1)) as u32, duration);
        }

        // frameTags

        let expected = [
            // name,     from, to, direction,           color
            ("start", 0, 2, Direction::Forward, ""),
            ("forward", 0, 1, Direction::Forward, ""),
            ("ping-pong", 2, 3, Direction::Pingpong, ""),
            ("reverse", 4, 5, Direction::Reverse, ""),
            ("end", 6, 8, Direction::Forward, ""),
            ("red", 6, 7, Direction::Forward, "#fe5b59ff"),
        ];

        assert_eq!(0, basic.meta.frame_tags.len());
        assert_eq!(expected.len(), array.meta.frame_tags.len());
        assert_eq!(expected.len(), hash.meta.frame_tags.len());

        for (((name, from, to, dir, color), array), hash) in expected
            .iter()
            .copied()
            .zip(array.meta.frame_tags.iter())
            .zip(hash.meta.frame_tags.iter())
        {
            assert_eq!(name, array.name);
            assert_eq!(name, hash.name);

            assert_eq!(from, array.from);
            assert_eq!(from, hash.from);

            assert_eq!(to, array.to);
            assert_eq!(to, hash.to);

            assert_eq!(dir, array.direction);
            assert_eq!(dir, hash.direction);

            let _ = color; // currently the JSON format doesn't seem to expose frameTags colors
        }

        // layers

        let expected = [
            // name,                group,      opacity, blend_mode,                color,       data
            (
                "Mode Layers",
                "",
                0,
                BlendMode::Normal,
                "#6acd5bff",
                "Mode Layers User Data",
            ),
            (
                "Layer Normal",
                "Mode Layers",
                255,
                BlendMode::Normal,
                "",
                "",
            ),
            (
                "Layer Darken",
                "Mode Layers",
                255,
                BlendMode::Darken,
                "",
                "",
            ),
            (
                "Layer Multiply",
                "Mode Layers",
                255,
                BlendMode::Multiply,
                "",
                "",
            ),
            (
                "Layer Color Burn",
                "Mode Layers",
                255,
                BlendMode::ColorBurn,
                "",
                "",
            ),
            (
                "Layer Lighten",
                "Mode Layers",
                255,
                BlendMode::Lighten,
                "",
                "",
            ),
            (
                "Layer Screen",
                "Mode Layers",
                255,
                BlendMode::Screen,
                "",
                "",
            ),
            (
                "Layer Color Dodge",
                "Mode Layers",
                255,
                BlendMode::ColorDodge,
                "",
                "",
            ),
            (
                "Layer Addition",
                "Mode Layers",
                255,
                BlendMode::Addition,
                "",
                "",
            ),
            (
                "Layer Overlay",
                "Mode Layers",
                255,
                BlendMode::Overlay,
                "",
                "",
            ),
            (
                "Layer Soft Light",
                "Mode Layers",
                255,
                BlendMode::SoftLight,
                "",
                "",
            ),
            (
                "Layer Hard Light",
                "Mode Layers",
                255,
                BlendMode::HardLight,
                "",
                "",
            ),
            (
                "Layer Difference",
                "Mode Layers",
                255,
                BlendMode::Difference,
                "",
                "",
            ),
            (
                "Layer Exclusion",
                "Mode Layers",
                255,
                BlendMode::Exclusion,
                "",
                "",
            ),
            (
                "Layer Subtract",
                "Mode Layers",
                255,
                BlendMode::Subtract,
                "",
                "",
            ),
            (
                "Layer Divide",
                "Mode Layers",
                255,
                BlendMode::Divide,
                "",
                "",
            ),
            ("Layer Hue", "Mode Layers", 255, BlendMode::HslHue, "", ""),
            (
                "Layer Saturation",
                "Mode Layers",
                255,
                BlendMode::HslSaturation,
                "",
                "",
            ),
            (
                "Layer Color",
                "Mode Layers",
                255,
                BlendMode::HslColor,
                "",
                "",
            ),
            (
                "Layer Luminosity",
                "Mode Layers",
                255,
                BlendMode::HslLuminosity,
                "",
                "",
            ),
            ("Layer Opacity 127", "", 127, BlendMode::Normal, "", ""),
            ("Layer Locked", "", 255, BlendMode::Normal, "", ""),
            (
                "Layer User Data",
                "",
                255,
                BlendMode::Normal,
                "#f7a547ff",
                "Orange Layer",
            ),
            ("Layer Linked Cels", "", 255, BlendMode::Normal, "", ""),
            ("Layer Even Cels", "", 255, BlendMode::Normal, "", ""),
        ];

        assert_eq!(0, basic.meta.layers.len());
        assert_eq!(expected.len(), array.meta.layers.len());
        assert_eq!(expected.len(), hash.meta.layers.len());

        for (((name, group, opacity, blend_mode, color, data), array), hash) in expected
            .iter()
            .copied()
            .zip(array.meta.layers.iter())
            .zip(hash.meta.layers.iter())
        {
            let group = if group.is_empty() { None } else { Some(group) };
            let color = if color.is_empty() {
                None
            } else {
                Some(String::from(color))
            };
            let data = if data.is_empty() { None } else { Some(data) };

            assert_eq!(name, array.name);
            assert_eq!(name, hash.name);

            assert_eq!(group, array.group.as_deref());
            assert_eq!(group, hash.group.as_deref());

            assert_eq!(opacity, array.opacity);
            assert_eq!(opacity, hash.opacity);

            assert_eq!(blend_mode, array.blend_mode);
            assert_eq!(blend_mode, hash.blend_mode);

            assert_eq!(color, array.color.as_ref().map(|c| format!("{:?}", c)));
            assert_eq!(color, hash.color.as_ref().map(|c| format!("{:?}", c)));

            assert_eq!(data, array.data.as_deref());
            assert_eq!(data, hash.data.as_deref());
        }

        // slices

        let expected = [
            // name,            color,       data,                      frame, bounds,    pivot,        center
            (
                "Top Right Pivot",
                "#0000ffff",
                None,
                0,
                [5, 1, 2, 2],
                Some([6, 2]),
                None,
            ),
            (
                "9 Slice",
                "#0000ffff",
                None,
                0,
                [1, 1, 6, 6],
                None,
                Some([2, 2, 2, 2]),
            ),
            (
                "Top Left",
                "#6acd5bff",
                Some("Top Left User Data"),
                0,
                [1, 1, 2, 2],
                None,
                None,
            ),
        ];

        assert_eq!(0, basic.meta.slices.len());
        assert_eq!(expected.len(), array.meta.slices.len());
        assert_eq!(expected.len(), hash.meta.slices.len());

        for (((name, color, data, frame, bounds, pivot, center), array), hash) in expected
            .iter()
            .copied()
            .zip(array.meta.slices.iter())
            .zip(hash.meta.slices.iter())
        {
            let bounds = Some(bounds)
                .map(|[x, y, w, h]| Rect { x, y, w, h })
                .unwrap();
            let pivot = pivot.map(|[x, y]| Point { x, y });
            let center = center.map(|[x, y, w, h]| Rect { x, y, w, h });

            assert_eq!(name, array.name);
            assert_eq!(name, hash.name);

            assert_eq!(color, format!("{:?}", array.color));
            assert_eq!(color, format!("{:?}", hash.color));

            assert_eq!(data, array.data.as_deref());
            assert_eq!(data, hash.data.as_deref());

            assert_eq!(1, array.keys.len());
            assert_eq!(1, hash.keys.len());

            assert_eq!(frame, array.keys[0].frame);
            assert_eq!(frame, hash.keys[0].frame);

            assert_eq!(bounds, array.keys[0].bounds);
            assert_eq!(bounds, hash.keys[0].bounds);

            assert_eq!(pivot, array.keys[0].pivot);
            assert_eq!(pivot, hash.keys[0].pivot);

            assert_eq!(center, array.keys[0].center);
            assert_eq!(center, hash.keys[0].center);
        }
    }
}
