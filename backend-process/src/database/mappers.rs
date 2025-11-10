use crate::database::models::{ImageMapping, InputMapping};
use enigo::Key;
use firmware_api::inputs::InputActions;
use rusqlite::Row;
use std::io::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct InputMappingStorageFormat {
    pub input_id: u8,
    pub actions: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImageMappingStorageFormat {
    pub display_zone: u8,
    pub image_path: String,
}

impl From<ImageMapping> for ImageMappingStorageFormat {
    fn from(mapping: ImageMapping) -> Self {
        Self {
            display_zone: mapping.display_zone.into(),
            image_path: mapping.image_path,
        }
    }
}

impl TryFrom<ImageMappingStorageFormat> for ImageMapping {
    type Error = Error;
    fn try_from(storage_format: ImageMappingStorageFormat) -> Result<Self, Self::Error> {
        Ok(ImageMapping {
            display_zone: storage_format.display_zone.try_into()?,
            image_path: storage_format.image_path,
        })
    }
}

impl TryFrom<InputMapping> for InputMappingStorageFormat {
    type Error = String;

    fn try_from(input: InputMapping) -> Result<Self, Self::Error> {
        let actions = ron::to_string(&input.actions()).map_err(|e| e.to_string())?;

        Ok(Self {
            input_id: input.input().into(),
            actions,
        })
    }
}

impl TryFrom<InputMappingStorageFormat> for InputMapping {
    type Error = String;
    fn try_from(input: InputMappingStorageFormat) -> Result<Self, Self::Error> {
        let deserialized_actions: Vec<Key> =
            ron::from_str(&input.actions).map_err(|e| e.to_string())?;

        Ok(InputMapping::new(
            InputActions::from(input.input_id),
            deserialized_actions,
        ))
    }
}
impl TryFrom<&Row<'_>> for InputMapping {
    type Error = String;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let input_id: u8 = row.get(0).map_err(|e| e.to_string())?;
        let actions: String = row.get(1).map_err(|e| e.to_string())?;

        InputMapping::try_from(InputMappingStorageFormat { input_id, actions })
    }
}

impl TryFrom<&Row<'_>> for ImageMapping {
    type Error = String;
    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let display_zone: u8 = row.get(0).map_err(|e| e.to_string())?;
        let image_path: String = row.get(1).map_err(|e| e.to_string())?;

        ImageMapping::try_from(ImageMappingStorageFormat {
            display_zone,
            image_path,
        })
        .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use firmware_api::display_zones::DisplayZones;

    #[test]
    fn converts_in_memory_input_mapping_to_storage_format() {
        let rust = InputMapping::new(InputActions::from(8), vec![Key::Add, Key::Backspace]);
        assert_eq!(
            InputMappingStorageFormat::try_from(rust).unwrap(),
            InputMappingStorageFormat {
                input_id: 8,
                actions: String::from("[Add,Backspace]")
            }
        )
    }

    #[test]
    fn converts_in_memory_image_mapping_to_image_mapping() {
        let rust = ImageMapping {
            display_zone: DisplayZones::Button4,
            image_path: String::from("/foo/bar/baz.png"),
        };

        assert_eq!(
            ImageMappingStorageFormat::from(rust),
            ImageMappingStorageFormat {
                display_zone: 14,
                image_path: String::from("/foo/bar/baz.png")
            }
        )
    }
}
