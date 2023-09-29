#![allow(dead_code)]

pub enum InventoryType {
    Categorized(CategorizedInventoryKind),
    Unique,
}

pub enum CategorizedInventoryKind {
    Device(DeviceKind),
    Part(DeviceKind, PartKind),
    Accessory,
}

pub enum DeviceKind {
    Phone(PhoneModel),
    Tablet(TabletModel),
    Console(ConsoleModel),
}

pub enum PhoneModel {
    Apple(ApplePhone),
    Samsung(SamsungPhone),
    Google(GooglePhone),
    Motorola(MotorolaPhone),
    // TODO: Support more devices (other models and device types)
    Other,
}

pub enum TabletModel {
    Apple(AppleTablet),
    Samsung(SamsungTablet),
    Other,
}

pub enum ConsoleModel {
    PlayStation(PlayStationConsole),
    Xbox(XboxConsole),
    Nintendo(NintendoConsole),
}

pub enum ApplePhone {
    Iphone15ProMax,
    Iphone15Pro,
    Iphone15Plus,
    Iphone15,
    Iphone14ProMax,
    Iphone14Pro,
    Iphone14Plus,
    Iphone14,
    Iphone13ProMax,
    Iphone13Pro,
    Iphone13Mini,
    Iphone13,
    Iphone12ProMax,
    Iphone12Pro,
    Iphone12Mini,
    Iphone12,
    Iphone11ProMax,
    Iphone11Pro,
    Iphone11,
    IphoneXSMax,
    IphoneXS,
    IphoneXR,
    IphoneX,
    Iphone8Plus,
    Iphone8,
    Iphone7Plus,
    Iphone7,
    Iphone6SPlus,
    Iphone6S,
    Iphone6Plus,
    Iphone6,
    Iphone5S,
    Iphone5C,
    Iphone5,
    Iphone4S,
    Iphone4,
    Iphone3GS,
    Iphone3G,
    Iphone1,
    IphoneSE1,
    IphoneSE2,
    IphoneSE3,
}

pub enum AppleTablet {
    // TODO: Add models
}

pub enum SamsungPhone {
    // TODO: Add models
}

pub enum SamsungTablet {
    // TODO: Add models
}

pub enum GooglePhone {
    // TODO: Add models
}

pub enum MotorolaPhone {
    // TODO: Add models
}

pub enum PlayStationConsole {
    PlayStation5Disc,
    PlayStation5Digital,
    PlayStation4Pro,
    PlayStation4Slim,
    PlayStation4,
    PlayStation3Slim,
    PlayStation3SuperSlim,
    PlayStation3,
    PlayStation2Slim,
    PlayStation2,
    PlayStation1,
    PlayStation,
    PlayStationVita,
    PlayStationPortable,
    PlayStationClassic,
}

pub enum XboxConsole {
    XboxSeriesX,
    XboxSeriesS,
    XboxOneX,
    XboxOneS,
    XboxOne,
    Xbox360E,
    Xbox360S,
    Xbox360,
    Xbox,
}

pub enum NintendoConsole {
    Switch,
    WiiU,
    Wii,
    GameCube,
    Nintendo64,
    SuperNintendoEntertainmentSystem,
    NintendoEntertainmentSystem,
}

// ? How do we handle sub-types here, like different types of rear cameras?
pub enum PartKind {
    Screen,
    Battery,
    Backglass,
    Frame,
    FrontCamera,
    RearCamera,
    LensCover,
    ChargePort,
    Other,
}

pub enum AccessoryKind {
    Case(PhoneModel),
    // TODO: Find a better way to model multi-compatibility
    ScreenProtector(Vec<PhoneModel>),
    // ? Is it necessary to have concrete types for this or should we just use a custom item system?
    Charger(ChargerType),
}

pub enum ChargerType {
    Block(ChargerBlock),
    Cable(ChargerCable),
}

pub struct ChargerBlock {
    connectors: Vec<Connector>,
}

pub struct ChargerCable {
    end_1: Connector,
    end_2: Connector,
}

pub enum Connector {
    UsbTypeA,
    UsbTypeC,
    UsbMicro,
    Lightning,
}
