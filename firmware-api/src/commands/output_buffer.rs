use crate::common::ByteArray;

pub const BUFFER_SIZE_513: usize = 513;

/// Generates a fixed-size byte-array containing a message.
///
/// Note that the lower addresses will be populated first
///
/// * `N` - The length of the buffer, the launchpad typically uses 513 (0x201) bytes for the most part but some commands require 1025 (0x401) bytes
/// * `message` - byte array to overwrite the lower bytes (will contain your intended message)
pub fn create_output_buffer<const N: usize>(message: &[u8]) -> ByteArray<N> {
    let mut buffer = [0u8; N];
    let length = message.len().min(N);

    buffer[..length].copy_from_slice(&message[..length]);
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fills_in_lower_bytes_happy_path() {
        let output = create_output_buffer::<513>(&[0x09, 0x90]);
        let mut to_check = [0; 513];

        to_check[0] = 0x09;
        to_check[1] = 0x90;

        assert_eq!(output, to_check);
    }
    #[test]
    fn truncates_oversized_message() {
        let message = [99u8; 6969];
        let output = create_output_buffer::<10>(&message);

        assert_eq!(output, message[0..10]);
    }
}
