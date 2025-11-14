use crate::database::mappers::{ImageMappingStorageFormat, InputMappingStorageFormat};
use crate::database::models::{ImageMapping, InputMapping};
use crate::database::sqlite::SqLite;
use firmware_api::display_zones::DisplayZones;
use rusqlite::fallible_streaming_iterator::FallibleStreamingIterator;
use rusqlite::{Connection, params};
use std::io::{Error, ErrorKind};

pub struct Operations {
    database: SqLite,
}

impl Operations {
    pub fn new(db: SqLite) -> Self {
        let instance = Operations { database: db };
        instance
            .create_input_mapping_table()
            .expect("Failed to create input_mapping table");
        instance
            .create_image_mapping_table()
            .expect("Failed to create image_mapping table");
        instance
            .create_config_mapping_table()
            .expect("Failed to create config_mapping table");

        instance
    }

    fn open_connection(&self) -> Result<&Connection, String> {
        self.database
            .connection()
            .ok_or(String::from("Operations not initialized"))
    }

    fn create_input_mapping_table(&self) -> Result<(), String> {
        const CREATE_INPUT_MAPPING_TABLE: &str = "
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
        const CREATE_IMAGE_MAPPING_TABLE: &str = "
            CREATE TABLE IF NOT EXISTS image_mapping (
            display_zone_id INTEGER PRIMARY KEY,
            image_path TEXT NOT NULL
        )";

        self.open_connection()?
            .execute(CREATE_IMAGE_MAPPING_TABLE, ())
            .ok();

        Ok(())
    }

    fn create_config_mapping_table(&self) -> Result<(), String> {
        // Extend this with any other params if required
        const CREATE_CONFIG_MAPPING_TABLE: &str = "
            CREATE TABLE IF NOT EXISTS config_mapping (
            id INTEGER PRIMARY KEY DEFAULT 1 CHECK (id = 1),
            brightness INTEGER)
        ";

        self.open_connection()?
            .execute(CREATE_CONFIG_MAPPING_TABLE, ())
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
        let input_mapping: ImageMappingStorageFormat = image_mapping.into();
        const SET_INPUT_MAPPING: &str = "INSERT INTO image_mapping (display_zone_id, image_path) VALUES (?1, ?2) \
                                            ON CONFLICT(display_zone_id) DO UPDATE SET image_path=?2";

        self.open_connection()
            .map_err(|e| Error::new(ErrorKind::ConnectionRefused, e))?
            .execute(
                SET_INPUT_MAPPING,
                (&input_mapping.display_zone, &input_mapping.image_path),
            )
            .map_err(Error::other)
    }

    pub fn clear_image_for_display_zone(
        &self,
        display_zones: DisplayZones,
    ) -> Result<usize, Error> {
        const REMOVE_IMAGE_FOR_DISPLAY_ZONES: &str =
            "DELETE FROM image_mapping WHERE display_zone_id = ?";

        let int_value = u8::from(display_zones);

        self.open_connection()
            .map_err(|e| Error::new(ErrorKind::ConnectionRefused, e))?
            .execute(REMOVE_IMAGE_FOR_DISPLAY_ZONES, params![int_value])
            .map_err(Error::other)
    }

    pub fn set_brightness(&self, brightness: u8) -> Result<usize, Error> {
        const SET_BRIGHTNESS: &str = "INSERT INTO config_mapping (id, brightness) VALUES (1, ?1)\
                                        ON CONFLICT(id) DO UPDATE SET brightness=?1";
        self.open_connection()
            .map_err(|e| Error::new(ErrorKind::ConnectionRefused, e))?
            .execute(SET_BRIGHTNESS, params![brightness])
            .map_err(Error::other)
    }

    pub fn get_stored_brightness(&self) -> Result<Option<u8>, Error> {
        const GET_BRIGHTNESS_VALUE: &str = "SELECT brightness FROM config_mapping WHERE id = 1";

        let conn = self
            .open_connection()
            .map_err(|e| Error::new(ErrorKind::ConnectionRefused, e))?;
        let mut stmt = conn.prepare(GET_BRIGHTNESS_VALUE).map_err(Error::other)?;

        let mut rows = stmt.query(params![]).map_err(Error::other)?;
        let singleton_row = rows.nth(0).map_err(Error::other)?;
        if let Some(row) = singleton_row {
            let brightness: u8 = row.get(0).map_err(Error::other)?;
            return Ok(Some(brightness));
        }

        Ok(None)
    }

    pub fn clear_all_display_zone_images(&self) -> Result<usize, Error> {
        const CLEAR_ALL_DISPLAY_ZONE_IMAGES: &str = "DELETE FROM image_mapping";

        self.open_connection()
            .map_err(|e| Error::new(ErrorKind::ConnectionRefused, e))?
            .execute(CLEAR_ALL_DISPLAY_ZONE_IMAGES, ())
            .map_err(Error::other)
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
    fn allows_setting_and_clearing_display_zone_images() {
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

        // Test single clear
        operations
            .clear_image_for_display_zone(DisplayZones::Touchscreen3)
            .unwrap();

        let new_rows = operations.get_all_image_mappings().unwrap();
        assert_eq!(new_rows.len(), 1);
    }

    #[test]
    fn allows_clearing_all_display_zone_images() {
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

        operations.clear_all_display_zone_images().unwrap();
        let new_rows = operations.get_all_image_mappings().unwrap();

        assert_eq!(new_rows.len(), 0);
    }

    #[test]
    fn allows_setting_brightness_value() {
        let sqlite = SqLite::new(false);
        let operations = Operations::new(sqlite.unwrap());

        operations.set_brightness(69).unwrap();

        let brightness = operations.get_stored_brightness().unwrap().unwrap();

        assert_eq!(brightness, 69);

        operations.set_brightness(20).unwrap();
        let brightness = operations.get_stored_brightness().unwrap().unwrap();

        assert_eq!(brightness, 20);
    }
}
