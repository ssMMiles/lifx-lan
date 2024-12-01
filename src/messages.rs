#[cfg(feature = "no-std")]
use heapless::String;

use lifx_serialization::LifxPayload;

#[derive(LifxPayload, Debug, Clone)]
pub enum Message {
    #[packet_number(1)]
    Service { service: u8, port: u32 },
    // technically same as above, but new version?
    #[packet_number(3)]
    StateService { service: u8, port: u32 },
    #[packet_number(12)]
    GetMeshInfo {},
    #[packet_number(13)]
    MeshInfo {
        signal: f32,
        tx: u32,
        rx: u32,
        mcu_temperature: u16,
    },
    #[packet_number(15)]
    HostFirmware {
        build: u64,
        reserved_6: [u8; 8],
        version_minor: u16,
        version_major: u16,
    },
    #[packet_number(17)]
    WifiInfo {
        signal: f32,
        tx: u32,
        rx: u32,
        mcu_temperature: u16,
    },
    #[packet_number(19)]
    WifiFirmware {
        build: u64,
        reserved_6: [u8; 8],
        version_minor: u16,
        version_major: u16,
    },
    #[packet_number(22)]
    Power { level: u16 },
    #[packet_number(25)]
    Label {
        #[cfg(feature = "no-std")]
        label: String<32>,
        #[cfg(not(feature = "no-std"))]
        label: String,
    },
    #[packet_number(33)]
    Version {
        vendor: u32,
        product: u32,
        reserved_6: u32,
    },
    #[packet_number(35)]
    Info {
        time: u64,
        uptime: u64,
        downtime: u64,
    },
    #[packet_number(50)]
    Location {
        location: [u8; 16],
        #[cfg(feature = "no-std")]
        label: String<32>,
        #[cfg(not(feature = "no-std"))]
        label: String,
        updated_at: u64,
    },
    #[packet_number(53)]
    Group {
        group: [u8; 16],
        #[cfg(feature = "no-std")]
        label: String<32>,
        #[cfg(not(feature = "no-std"))]
        label: String,
        updated_at: u64,
    },
    #[packet_number(59)]
    EchoResponse { echoing: [u8; 64] },
    #[packet_number(223)]
    Unhandled { unhandled_type: u16 },
    #[packet_number(107)]
    LightState {
        hue: u16,
        saturation: u16,
        brightness: u16,
        kelvin: u16,
        reserved_6: [u8; 2],
        power: u16,
        #[cfg(feature = "no-std")]
        label: String<32>,
        #[cfg(not(feature = "no-std"))]
        label: String,
        reserved_7: [u8; 8],
    },
    #[packet_number(118)]
    LightPower { level: u16 },
    #[packet_number(121)]
    Infrared { brightness: u16 },
    #[packet_number(144)]
    HevCycle {
        duration_s: u32,
        remaining_s: u32,
        last_power: u8,
    },
    #[packet_number(147)]
    HevCycleConfig { indication: u8, duration_s: u32 },
    #[packet_number(149)]
    LastHevCycleResult { result: u8 },
    #[packet_number(2)]
    GetService,
    #[packet_number(14)]
    GetHostFirmware,
    #[packet_number(16)]
    GetWifiInfo,
    #[packet_number(18)]
    GetWifiFirmware,
    #[packet_number(20)]
    GetPower,
    #[packet_number(21)]
    SetPower { level: u16 },
    #[packet_number(23)]
    GetLabel,
    #[packet_number(24)]
    SetLabel {
        #[cfg(feature = "no-std")]
        label: String<32>,
        #[cfg(not(feature = "no-std"))]
        label: String,
    },
    #[packet_number(32)]
    GetVersion,
    #[packet_number(34)]
    GetInfo,
    #[packet_number(38)]
    SetReboot,
    #[packet_number(48)]
    GetLocation,
    #[packet_number(49)]
    SetLocation {
        location: [u8; 16],
        #[cfg(feature = "no-std")]
        label: String<32>,
        #[cfg(not(feature = "no-std"))]
        label: String,
        updated_at: u64,
    },
    #[packet_number(51)]
    GetGroup,
    #[packet_number(52)]
    SetGroup {
        group: [u8; 16],
        #[cfg(feature = "no-std")]
        label: String<32>,
        #[cfg(not(feature = "no-std"))]
        label: String,
        updated_at: u64,
    },
    #[packet_number(58)]
    EchoRequest { echoing: [u8; 64] },
    #[packet_number(101)]
    GetColor,
    #[packet_number(102)]
    SetColor {
        reserved_6: u8,
        hue: u16,
        saturation: u16,
        brightness: u16,
        kelvin: u16,
        duration_ms: u32,
    },
    #[packet_number(103)]
    SetWaveform {
        reserved_6: u8,
        transient: u8,
        hue: u16,
        saturation: u16,
        brightness: u16,
        kelvin: u16,
        period_ms: u32,
        cycles: f32,
        skew_ratio: i16,
        waveform: u8,
    },
    #[packet_number(116)]
    GetLightPower,
    #[packet_number(117)]
    SetLightPower { level: u16, duration_ms: u32 },
    #[packet_number(119)]
    SetWaveformOptional {
        reserved_6: u8,
        transient: u8,
        hue: u16,
        saturation: u16,
        brightness: u16,
        kelvin: u16,
        period_ms: u32,
        cycles: f32,
        skew_ratio: i16,
        waveform: u8,
        set_hue: u8,
        set_saturation: u8,
        set_brightness: u8,
        set_kelvin: u8,
    },
    #[packet_number(120)]
    GetInfrared,
    #[packet_number(122)]
    SetInfrared { brightness: u16 },
    #[packet_number(142)]
    GetHevCycle,
    #[packet_number(143)]
    SetHevCycle { duration_s: u32 },
    #[packet_number(145)]
    GetHevCycleConfiguration,
    #[packet_number(146)]
    SetHevCycleConfiguration { indication: u8, duration_s: u32 },
    #[packet_number(148)]
    GetLastHevCycleResult,

    #[packet_number(305)]
    SetAccessPoint {
        interface: u8, // 1 for access point, 2 for station

        #[cfg(feature = "no-std")]
        ssid: String<32>,
        #[cfg(feature = "no-std")]
        password: String<64>,

        #[cfg(not(feature = "no-std"))]
        #[size(32)]
        ssid: String,
        #[cfg(not(feature = "no-std"))]
        #[size(64)]
        password: String,

        protocol: u8, // docs below
    },
}

// enum INTERFACE : byte
// {
//   SOFT_AP = 1, // i.e. act as an access point
//   STATION = 2  // i.e. connect to an existing access point
// }

// enum SECURITY_PROTOCOL : byte
// {
//    OPEN           = 1,
//    WEP_PSK        = 2, // Not officially supported
//    WPA_TKIP_PSK   = 3,
//    WPA_AES_PSK    = 4,
//    WPA2_AES_PSK   = 5,
//    WPA2_TKIP_PSK  = 6,
//    WPA2_MIXED_PSK = 7
// }

pub enum WifiSignalQuality {
    High,
    Average,
    Low,
    VeryLow,
    None,
}

pub fn get_wifi_signal_quality(signal: f32) -> WifiSignalQuality {
    let rssi = (10.0 * libm::log10f(signal) + 0.5) as i32;

    if rssi < 0 || rssi == 200 {
        if rssi == 200 {
            WifiSignalQuality::None
        } else if rssi <= -80 {
            WifiSignalQuality::VeryLow
        } else if rssi <= -70 {
            WifiSignalQuality::Low
        } else if rssi <= -60 {
            WifiSignalQuality::Average
        } else {
            WifiSignalQuality::High
        }
    } else if rssi == 4 || rssi == 5 || rssi == 6 {
        WifiSignalQuality::VeryLow
    } else if rssi >= 7 && rssi <= 11 {
        WifiSignalQuality::Low
    } else if rssi >= 12 && rssi <= 16 {
        WifiSignalQuality::Average
    } else if rssi > 16 {
        WifiSignalQuality::High
    } else {
        WifiSignalQuality::None
    }
}
