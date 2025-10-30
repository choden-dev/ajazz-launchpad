use crate::database::models::InputMapping;
use enigo::Key;
use firmware_api::inputs::InputActions;
use rusqlite::Row;

#[derive(Debug, PartialEq, Clone)]
pub struct InputMappingStorageFormat {
    pub input_id: u8,
    pub actions: String,
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
