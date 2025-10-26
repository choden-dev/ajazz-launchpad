## `firmware-api` workspace

This workspace deals with writing and reading commands from the launchpad.

### How the messages were figured out

The protocols between the devices were extracted through the work done in the [StreamDock-Device-SDK] repo. More
specifically:

- [`Ghidra`] to decompile [`libtransport.so`] to see what each of the messages was
- [`WireShark`] and [`USBPcap`] to observe messaging
- [`hidapitester`] for initial prototyping and verification
- [StreamDock-Device-SDK's example python usage] to see the intended behavior

### Example usage

See the `/examples` folder for some use cases. For a very basic one (from `examples/simple-connect-and-read.rs`)

_Note you can run any of the examples with the following command:_

```shell
cargo run --example simple_connect_and_read
```

```rust
use firmware_api::device::{Device, FunctionHandler, HidDeviceWrapper};
use hidapi::HidApi;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let hid_api = HidApi::new().unwrap_or_else(|e| panic!("Failed to initialize HID API: {}", e));

    let hid_device = hid_api
        .open(0x0300, 0x3004)
        .unwrap_or_else(|e| panic!("Failed to open device: {}", e));

    let device = Device::new(
        HidDeviceWrapper::new(hid_device),
        FunctionHandler::new(|action| println!("{:?}", action)),
    );

    device
        .refresh()
        .unwrap_or_else(|e| panic!("Failed to refresh device: {}", e));

    loop {
        device
            .read_input()
            .unwrap_or_else(|e| println!("Failed to read input: {}", e));
        sleep(Duration::from_millis(500));
    }
}
```

You can see in the above example the usage revolves around the `Device` struct and its implementation.

[`Ghidra`]: https://github.com/NationalSecurityAgency/ghidra

[`WireShark`]: https://www.wireshark.org/download.html

[`USBPcap`]: https://desowin.org/usbpcap/

[`libtransport.so`]: https://github.com/MiraboxSpace/StreamDock-Device-SDK/blob/6174312894d1275b163beb7c073332baa3f9a660/Python-Linux-SDK/src/StreamDock/Transport/libtransport.so

[StreamDock-Device-SDK]: https://github.com/MiraboxSpace/StreamDock-Device-SDK/tree/6174312894d1275b163beb7c073332baa3f9a660

[StreamDock-Device-SDK's example python usage]: https://github.com/MiraboxSpace/StreamDock-Device-SDK/blob/6174312894d1275b163beb7c073332baa3f9a660/Python-Linux-SDK/src/main.py