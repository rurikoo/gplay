use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use configparser::ini::Ini;
use prost::Message;

use googleplay_protobuf::{
    AndroidBuildProto, AndroidCheckinProto, DeviceConfigurationProto, DeviceFeature,
};

use wincode::{SchemaRead, SchemaWrite};
include!("src/snippets/device_properties.rs");

fn main() {
    if !Path::new("src/device_properties.bin").exists() {
        let mut config = Ini::new();
        config
            .read(fs::read_to_string("device.properties").unwrap())
            .unwrap();

        let mut device_properties_map: HashMap<String, EncodedDeviceProperties> = HashMap::new();
        for section in config.sections() {
            let encoded = DeviceProperties::parse(&config, &section).into_encoded();
            device_properties_map.insert(section, encoded);
        }

        let devices_encoded: Vec<u8> = wincode::serialize(&device_properties_map).unwrap();

        let mut file = File::create("src/device_properties.bin").unwrap();
        file.write_all(&devices_encoded).unwrap();
    }
}
