use crate::database::mappers::{ImageMappingStorageFormat, InputMappingStorageFormat};
use crate::database::models::{ImageMapping, InputMapping};
use crate::database::sqlite::SqLite;
use rusqlite::Connection;
use std::io::{Error, ErrorKind};

pub struct Operations {
    database: SqLite,
}

impl Operations {
    pub fn new(db: SqLite) -> Self {
        let instance = Operations { database: db };
        instance.create_input_mapping_table().ok();
        instance.create_image_mapping_table().ok();

        instance
    }

    fn open_connection(&self) -> Result<&Connection, String> {
        self.database
            .connection()
            .ok_or(String::from("Operations not initialized"))
    }

    fn create_input_mapping_table(&self) -> Result<(), String> {
        pub const CREATE_INPUT_MAPPING_TABLE: &str = "
            CREATE TABLE IF NOT EXISTS input_mapping (
            button_id INTEGER PRIMARY KEY,
            actions TEXT NOT NULL
        )";

        self.open_connection()?
            .execute(CREATE_INPUT_MAPPING_TABLE, ())
            .ok();

        Ok(())
    }

    fn create_image_mapping_table(&self) -> Result<(), String> {
        pub const CREATE_IMAGE_MAPPING_TABLE: &str = "
            CREATE TABLE IF NOT EXISTS image_mapping (
            display_zone_id INTEGER PRIMARY KEY,
            image_path TEXT NOT NULL
        )";

        self.open_connection()?
            .execute(CREATE_IMAGE_MAPPING_TABLE, ())
            .ok();

        Ok(())
    }

    pub fn set_mapping_for_input(&self, input_mapping: InputMapping) -> Result<usize, String> {
        let input_mapping: InputMappingStorageFormat = input_mapping.try_into()?;
        const SET_INPUT_MAPPING: &str = "INSERT INTO input_mapping (button_id, actions) VALUES (?1, ?2) \
                                            ON CONFLICT(button_id) DO UPDATE SET actions=?2";

        self.open_connection()?
            .execute(
                SET_INPUT_MAPPING,
                (&input_mapping.input_id, &input_mapping.actions),
            )
            .map_err(|e| e.to_string())
    }

    pub fn set_image_for_display_zone(&self, image_mapping: ImageMapping) -> Result<usize, Error> {
        let input_mapping: ImageMappingStorageFormat = image_mapping
            .try_into()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;
        const SET_INPUT_MAPPING: &str = "INSERT INTO image_mapping (display_zone_id, image_path) VALUES (?1, ?2) \
                                            ON CONFLICT(display_zone_id) DO UPDATE SET image_path=?2";

        self.open_connection()
            .map_err(|e| Error::new(ErrorKind::ConnectionRefused, e))?
            .execute(
                SET_INPUT_MAPPING,
                (&input_mapping.display_zone, &input_mapping.image_path),
            )
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }

    #[allow(dead_code)]
    pub fn get_all_image_mappings(&self) -> Result<Vec<ImageMapping>, String> {
        const GET_ALL_IMAGE_MAPPINGS: &str = "SELECT * FROM image_mapping";

        let conn = self.open_connection()?;
        let mut stmt = conn
            .prepare(GET_ALL_IMAGE_MAPPINGS)
            .map_err(|e| e.to_string())?;

        let row_iter = stmt
            .query_map([], |row| Ok(ImageMapping::try_from(row)))
            .map_err(|e| e.to_string())?;

        row_iter
            .map(|row_result| {
                row_result
                    .map_err(|e| e.to_string())
                    .and_then(|mapping_result| {
                        mapping_result.map_err(|e| format!("Conversion error: {}", e))
                    })
            })
            .collect()
    }

    pub fn get_all_input_mappings(&self) -> Result<Vec<InputMapping>, String> {
        const GET_ALL_INPUT_MAPPINGS: &str = "SELECT * FROM input_mapping";

        let conn = self.open_connection()?;
        let mut stmt = conn
            .prepare(GET_ALL_INPUT_MAPPINGS)
            .map_err(|e| e.to_string())?;

        let row_iter = stmt
            .query_map([], |row| Ok(InputMapping::try_from(row)))
            .map_err(|e| e.to_string())?;

        row_iter
            .map(|row_result| {
                row_result
                    .map_err(|e| e.to_string())
                    .and_then(|mapping_result| {
                        mapping_result.map_err(|e| format!("Conversion error: {}", e))
                    })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enigo::Key;
    use firmware_api::display_zones::DisplayZones;
    use firmware_api::inputs::InputActions;
    use firmware_api::inputs::buttons::ButtonActions::{Button1Pressed, Button4Pressed};

    #[test]
    fn allows_setting_input_mappings() {
        let sqlite = SqLite::new(false);
        let operations = Operations::new(sqlite.unwrap());

        let to_add = &[
            InputMapping::new(InputActions::Button(Button4Pressed), vec![Key::Option]),
            InputMapping::new(InputActions::Button(Button1Pressed), vec![Key::Backspace]),
        ];

        operations.create_input_mapping_table().unwrap();

        to_add.iter().for_each(|item| {
            operations.set_mapping_for_input(item.clone()).unwrap();
        });

        let all_rows = operations.get_all_input_mappings().unwrap();

        assert_eq!(all_rows.len(), 2);
        to_add.iter().for_each(|item| {
            assert!(all_rows.contains(item));
        });

        // Replace the button 4 bindings
        operations
            .set_mapping_for_input(InputMapping::new(
                InputActions::Button(Button4Pressed),
                vec![Key::Add, Key::Backspace],
            ))
            .unwrap();

        let new_rows = operations.get_all_input_mappings().unwrap();

        assert_eq!(new_rows.len(), 2);
        // Doesn't contain an old version of button 4
        assert!(!new_rows.contains(&to_add[0]));

        // Should still contain binding to button 1
        assert!(new_rows.contains(&to_add[1]));
    }

    #[test]
    fn allows_setting_display_zone_images() {
        let sqlite = SqLite::new(false);
        let operations = Operations::new(sqlite.unwrap());

        let to_add = &[
            ImageMapping {
                display_zone: DisplayZones::Touchscreen3,
                image_path: String::from("foo.jpg"),
            },
            ImageMapping {
                display_zone: DisplayZones::Button3,
                image_path: String::from("fat.jpg"),
            },
        ];

        operations.create_input_mapping_table().unwrap();

        for item in to_add.iter() {
            operations.set_image_for_display_zone(item.clone()).unwrap();
        }

        let all_rows = operations.get_all_image_mappings().unwrap();

        assert_eq!(all_rows.len(), 2);
        to_add.iter().for_each(|item| {
            assert!(all_rows.contains(item));
        });

        // Replace the touchscreen 3 bindings
        operations
            .set_image_for_display_zone(ImageMapping {
                display_zone: DisplayZones::Touchscreen3,
                image_path: String::from("231.jpg"),
            })
            .unwrap();

        let new_rows = operations.get_all_image_mappings().unwrap();

        assert_eq!(new_rows.len(), 2);
        // Doesn't contain an old version of button 4
        assert!(!new_rows.contains(&to_add[0]));
        // And instead contains the new version
        assert!(new_rows.contains(&ImageMapping {
            display_zone: DisplayZones::Touchscreen3,
            image_path: String::from("231.jpg"),
        }));

        // Should still contain binding to button 1
        assert!(new_rows.contains(&to_add[1]));
    }
}
