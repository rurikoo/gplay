#[derive(Encode, Decode, Debug)]
struct EncodedDeviceProperties {
    pub device_configuration: Vec<u8>,
    pub android_checkin: Vec<u8>,
    pub extra_info: HashMap<String, String>,
}

#[allow(dead_code)]
impl EncodedDeviceProperties {
    pub fn new(
        device_configuration: Vec<u8>,
        android_checkin: Vec<u8>,
        extra_info: HashMap<String, String>,
    ) -> Self {
        Self {
            device_configuration,
            android_checkin,
            extra_info,
        }
    }

    pub fn into_decoded(self) -> DeviceProperties {
        DeviceProperties {
            device_configuration: DeviceConfigurationProto::decode(
                self.device_configuration.as_slice(),
            )
            .unwrap(),
            android_checkin: AndroidCheckinProto::decode(self.android_checkin.as_slice()).unwrap(),
            extra_info: self.extra_info,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct DeviceProperties {
    pub device_configuration: DeviceConfigurationProto,
    pub android_checkin: AndroidCheckinProto,
    pub extra_info: HashMap<String, String>,
}

#[allow(dead_code)]
impl DeviceProperties {
    pub fn new(
        device_configuration: DeviceConfigurationProto,
        android_checkin: AndroidCheckinProto,
        extra_info: HashMap<String, String>,
    ) -> Self {
        Self {
            device_configuration,
            android_checkin,
            extra_info,
        }
    }

    pub fn parse(config: &Ini, section: &str) -> Self {
        let mut extra_info = HashMap::new();
        extra_info.insert(
            "Build.ID".to_string(),
            config.get(section, "Build.ID").unwrap_or_default(),
        );
        extra_info.insert(
            "Vending.versionString".to_string(),
            config
                .get(section, "Vending.versionString")
                .unwrap_or_default(),
        );
        extra_info.insert(
            "Vending.version".to_string(),
            config.get(section, "Vending.version").unwrap_or_default(),
        );
        extra_info.insert(
            "Build.VERSION.RELEASE".to_string(),
            config
                .get(section, "Build.VERSION.RELEASE")
                .unwrap_or_default(),
        );
        if let Some(sim_operator) = config.get(section, "SimOperator") {
            extra_info.insert("SimOperator".to_string(), sim_operator);
        }

        let android_build = AndroidBuildProto {
            id: config.get(section, "Build.FINGERPRINT"),
            product: config.get(section, "Build.HARDWARE"),
            carrier: config.get(section, "Build.BRAND"),
            radio: config.get(section, "Build.RADIO"),
            bootloader: config.get(section, "Build.BOOTLOADER"),
            device: config.get(section, "Build.DEVICE"),
            sdk_version: config
                .getint(section, "Build.VERSION.SDK_INT")
                .unwrap()
                .map(|v| v as i32),
            model: config.get(section, "Build.MODEL"),
            manufacturer: config.get(section, "Build.MANUFACTURER"),
            build_product: config.get(section, "Build.PRODUCT"),
            client: config.get(section, "Client"),
            ota_installed: Some(false),
            google_services: config
                .getint(section, "GSF.version")
                .unwrap()
                .map(|v| v as i32),
            ..Default::default()
        };

        let android_checkin = AndroidCheckinProto {
            build: Some(android_build),
            last_checkin_msec: Some(0),
            cell_operator: config.get(section, "CellOperator"),
            sim_operator: config.get(section, "SimOperator"),
            roaming: config.get(section, "Roaming"),
            user_number: Some(0),
            ..Default::default()
        };

        let device_configuration = DeviceConfigurationProto {
            touch_screen: config
                .getint(section, "TouchScreen")
                .unwrap()
                .map(|v| v as i32),
            keyboard: config
                .getint(section, "Keyboard")
                .unwrap()
                .map(|v| v as i32),
            navigation: config
                .getint(section, "Navigation")
                .unwrap()
                .map(|v| v as i32),
            screen_layout: config
                .getint(section, "ScreenLayout")
                .unwrap()
                .map(|v| v as i32),
            has_hard_keyboard: config.getbool(section, "HasHardKeyboard").unwrap(),
            has_five_way_navigation: config.getbool(section, "HasFiveWayNavigation").unwrap(),
            screen_density: config
                .getint(section, "Screen.Density")
                .unwrap()
                .map(|v| v as i32),
            gl_es_version: config
                .getint(section, "GL.Version")
                .unwrap()
                .map(|v| v as i32),
            system_shared_library: config
                .get(section, "SharedLibraries")
                .unwrap()
                .split(",")
                .map(String::from)
                .collect(),
            system_available_feature: config
                .get(section, "Features")
                .unwrap()
                .split(",")
                .map(String::from)
                .collect(),
            native_platform: config
                .get(section, "Platforms")
                .unwrap_or_default()
                .split(",")
                .map(String::from)
                .collect(),
            screen_width: config
                .getint(section, "Screen.Width")
                .unwrap()
                .map(|v| v as i32),
            screen_height: config
                .getint(section, "Screen.Height")
                .unwrap()
                .map(|v| v as i32),
            system_supported_locale: config
                .get(section, "Locales")
                .unwrap()
                .split(",")
                .map(String::from)
                .collect(),
            gl_extension: config
                .get(section, "GL.Extensions")
                .unwrap()
                .split(",")
                .map(String::from)
                .collect(),
            device_feature: config
                .get(section, "Features")
                .unwrap()
                .split(",")
                .map(|s| DeviceFeature {
                    name: Some(s.to_string()),
                    value: Some(0),
                })
                .collect(),
            ..Default::default()
        };

        Self::new(device_configuration, android_checkin, extra_info)
    }

    pub fn into_encoded(self) -> EncodedDeviceProperties {
        EncodedDeviceProperties {
            device_configuration: self.device_configuration.encode_to_vec(),
            android_checkin: self.android_checkin.encode_to_vec(),
            extra_info: self.extra_info,
        }
    }
}
