use bitflags::bitflags;

/// Response data types ([Assigned Numbers] Section 2.3).
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    enum_iterator::Sequence,
    num_enum::IntoPrimitive,
    num_enum::TryFromPrimitive,
)]
#[non_exhaustive]
#[repr(u8)]
pub(super) enum ResponseDataType {
    Flags = 0x01,                             // [CSS] Part A, Section 1.3
    IncompleteServiceClass16 = 0x02,          // [CSS] Part A, Section 1.1
    CompleteServiceClass16 = 0x03,            // [CSS] Part A, Section 1.1
    IncompleteServiceClass32 = 0x04,          // [CSS] Part A, Section 1.1
    CompleteServiceClass32 = 0x05,            // [CSS] Part A, Section 1.1
    IncompleteServiceClass128 = 0x06,         // [CSS] Part A, Section 1.1
    CompleteServiceClass128 = 0x07,           // [CSS] Part A, Section 1.1
    ShortLocalName = 0x08,                    // [CSS] Part A, Section 1.2
    CompleteLocalName = 0x09,                 // [CSS] Part A, Section 1.2
    TxPower = 0x0A,                           // [CSS] Part A, Section 1.5
    ClassOfDevice = 0x0D,                     // [CSS] Part A, Section 1.6
    SspHashC192 = 0x0E,                       // [CSS] Part A, Section 1.6
    SspRandR192 = 0x0F,                       // [CSS] Part A, Section 1.6
    SmTkValue = 0x10,                         // [CSS] Part A, Section 1.8
    SmOobFlags = 0x11,                        // [CSS] Part A, Section 1.7
    PeripheralConnectionIntervalRange = 0x12, // [CSS] Part A, Section 1.9
    ServiceSolicitation16 = 0x14,             // [CSS] Part A, Section 1.10
    ServiceSolicitation128 = 0x15,            // [CSS] Part A, Section 1.10
    ServiceData16 = 0x16,                     // [CSS] Part A, Section 1.11
    PublicTargetAddress = 0x17,               // [CSS] Part A, Section 1.13
    RandomTargetAddress = 0x18,               // [CSS] Part A, Section 1.14
    Appearance = 0x19,                        // [CSS] Part A, Section 1.12
    AdvInterval = 0x1A,                       // [CSS] Part A, Section 1.15
    LeDeviceAddr = 0x1B,                      // [CSS] Part A, Section 1.16
    LeRole = 0x1C,                            // [CSS] Part A, Section 1.17
    SspHashC256 = 0x1D,                       // [CSS] Part A, Section 1.6
    SspRandR256 = 0x1E,                       // [CSS] Part A, Section 1.6
    ServiceSolicitation32 = 0x1F,             // [CSS] Part A, Section 1.10
    ServiceData32 = 0x20,                     // [CSS] Part A, Section 1.11
    ServiceData128 = 0x21,                    // [CSS] Part A, Section 1.11
    LeScConfirmValue = 0x22,                  // [CSS] Part A, Section 1.6
    LeScRandValue = 0x23,                     // [CSS] Part A, Section 1.6
    Uri = 0x24,                               // [CSS] Part A, Section 1.18
    LeSupportedFeatures = 0x27,               // [CSS] Part A, Section 1.19
    ChannelMapUpdateIndication = 0x28,        // [CSS] Part A, Section 1.20
    BigInfo = 0x2C,                           // [CSS] Part A, Section 1.21
    BroadcastCode = 0x2D,                     // [CSS] Part A, Section 1.22
    AdvIntervalLong = 0x2F,                   // [CSS] Part A, Section 1.15
    ManufacturerData = 0xFF,                  // [CSS] Part A, Section 1.4
}

bitflags! {
    /// Advertising response data flags ([CSS] Part A, Section 1.3).
    #[derive(Default)]
    #[repr(transparent)]
    pub struct AdvFlag: u8 {
        /// LE Limited Discoverable Mode.
        const LE_LIMITED = 1 << 0;
        /// LE General Discoverable Mode.
        const LE_GENERAL = 1 << 1;
        /// BR/EDR Not Supported. Bit 37 of LMP Feature Mask Definitions (Page
        /// 0).
        const NO_BREDR = 1 << 2;
        /// Simultaneous LE and BR/EDR to Same Device Capable (Controller). Bit
        /// 49 of LMP Feature Mask Definitions (Page 0).
        const LE_BREDR_CONTROLLER = 1 << 3;
        /// Simultaneous LE and BR/EDR to Same Device Capable (Host). Bit 66 of
        /// LMP Feature Mask Definitions (Page 1). Deprecated in CSS v10.
        const LE_BREDR_HOST = 1 << 4;
    }
}

/// Device appearance ([Assigned Numbers] Section 2.6.3).
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    num_enum::FromPrimitive,
    num_enum::IntoPrimitive,
)]
#[non_exhaustive]
#[repr(u16)]
pub enum Appearance {
    #[default]
    GenericUnknown = 0x0000,
    GenericPhone = 0x0040,
    GenericComputer = 0x0080,
    DesktopWorkstation = 0x0081,
    ServerclassComputer = 0x0082,
    Laptop = 0x0083,
    HandheldPcPdaClamshell = 0x0084,
    PalmsizePcPda = 0x0085,
    WearableComputer = 0x0086,
    Tablet = 0x0087,
    DockingStation = 0x0088,
    AllInOne = 0x0089,
    BladeServer = 0x008A,
    Convertible = 0x008B,
    Detachable = 0x008C,
    IoTGateway = 0x008D,
    MiniPc = 0x008E,
    StickPc = 0x008F,
    GenericWatch = 0x00C0,
    SportsWatch = 0x00C1,
    Smartwatch = 0x00C2,
    GenericClock = 0x0100,
    GenericDisplay = 0x0140,
    GenericRemoteControl = 0x0180,
    GenericEyeglasses = 0x01C0,
    GenericTag = 0x0200,
    GenericKeyring = 0x0240,
    GenericMediaPlayer = 0x0280,
    GenericBarcodeScanner = 0x02C0,
    GenericThermometer = 0x0300,
    EarThermometer = 0x0301,
    GenericHeartRateSensor = 0x0340,
    HeartRateBelt = 0x0341,
    GenericBloodPressure = 0x0380,
    ArmBloodPressure = 0x0381,
    WristBloodPressure = 0x0382,
    GenericHumanInterfaceDevice = 0x03C0,
    Keyboard = 0x03C1,
    Mouse = 0x03C2,
    Joystick = 0x03C3,
    Gamepad = 0x03C4,
    DigitizerTablet = 0x03C5,
    CardReader = 0x03C6,
    DigitalPen = 0x03C7,
    BarcodeScanner = 0x03C8,
    Touchpad = 0x03C9,
    PresentationRemote = 0x03CA,
    GenericGlucoseMeter = 0x0400,
    GenericRunningWalkingSensor = 0x0440,
    InShoeRunningWalkingSensor = 0x0441,
    OnShoeRunningWalkingSensor = 0x0442,
    OnHipRunningWalkingSensor = 0x0443,
    GenericCycling = 0x0480,
    CyclingComputer = 0x0481,
    SpeedSensor = 0x0482,
    CadenceSensor = 0x0483,
    PowerSensor = 0x0484,
    SpeedAndCadenceSensor = 0x0485,
    GenericControlDevice = 0x04C0,
    Switch = 0x04C1,
    Multiswitch = 0x04C2,
    Button = 0x04C3,
    Slider = 0x04C4,
    RotarySwitch = 0x04C5,
    TouchPanel = 0x04C6,
    SingleSwitch = 0x04C7,
    DoubleSwitch = 0x04C8,
    TripleSwitch = 0x04C9,
    BatterySwitch = 0x04CA,
    EnergyHarvestingSwitch = 0x04CB,
    PushButton = 0x04CC,
    GenericNetworkDevice = 0x0500,
    AccessPoint = 0x0501,
    MeshDevice = 0x0502,
    MeshNetworkProxy = 0x0503,
    GenericSensor = 0x0540,
    MotionSensor = 0x0541,
    AirQualitySensor = 0x0542,
    TemperatureSensor = 0x0543,
    HumiditySensor = 0x0544,
    LeakSensor = 0x0545,
    SmokeSensor = 0x0546,
    OccupancySensor = 0x0547,
    ContactSensor = 0x0548,
    CarbonMonoxideSensor = 0x0549,
    CarbonDioxideSensor = 0x054A,
    AmbientLightSensor = 0x054B,
    EnergySensor = 0x054C,
    ColorLightSensor = 0x054D,
    RainSensor = 0x054E,
    FireSensor = 0x054F,
    WindSensor = 0x0550,
    ProximitySensor = 0x0551,
    MultiSensor = 0x0552,
    FlushMountedSensor = 0x0553,
    CeilingMountedSensor = 0x0554,
    WallMountedSensor = 0x0555,
    Multisensor = 0x0556,
    EnergyMeter = 0x0557,
    FlameDetector = 0x0558,
    VehicleTirePressureSensor = 0x0559,
    GenericLightFixtures = 0x0580,
    WallLight = 0x0581,
    CeilingLight = 0x0582,
    FloorLight = 0x0583,
    CabinetLight = 0x0584,
    DeskLight = 0x0585,
    TrofferLight = 0x0586,
    PendantLight = 0x0587,
    IngroundLight = 0x0588,
    FloodLight = 0x0589,
    UnderwaterLight = 0x058A,
    BollardWithLight = 0x058B,
    PathwayLight = 0x058C,
    GardenLight = 0x058D,
    PoletopLight = 0x058E,
    Spotlight = 0x058F,
    LinearLight = 0x0590,
    StreetLight = 0x0591,
    ShelvesLight = 0x0592,
    BayLight = 0x0593,
    EmergencyExitLight = 0x0594,
    LightController = 0x0595,
    LightDriver = 0x0596,
    Bulb = 0x0597,
    LowbayLight = 0x0598,
    HighbayLight = 0x0599,
    GenericFan = 0x05C0,
    CeilingFan = 0x05C1,
    AxialFan = 0x05C2,
    ExhaustFan = 0x05C3,
    PedestalFan = 0x05C4,
    DeskFan = 0x05C5,
    WallFan = 0x05C6,
    GenericHvac = 0x0600,
    HvacThermostat = 0x0601,
    HvacHumidifier = 0x0602,
    HvacDehumidifier = 0x0603,
    HvacHeater = 0x0604,
    HvacRadiator = 0x0605,
    HvacBoiler = 0x0606,
    HvacHeatPump = 0x0607,
    HvacInfraredHeater = 0x0608,
    HvacRadiantPanelHeater = 0x0609,
    HvacFanHeater = 0x060A,
    HvacAirCurtain = 0x060B,
    GenericAirConditioning = 0x0640,
    GenericHumidifier = 0x0680,
    GenericHeating = 0x06C0,
    HeatingRadiator = 0x06C1,
    HeatingBoiler = 0x06C2,
    HeatingHeatPump = 0x06C3,
    HeatingInfraredHeater = 0x06C4,
    HeatingRadiantPanelHeater = 0x06C5,
    HeatingFanHeater = 0x06C6,
    HeatingAirCurtain = 0x06C7,
    GenericAccessControl = 0x0700,
    AccessDoor = 0x0701,
    GarageDoor = 0x0702,
    EmergencyExitDoor = 0x0703,
    AccessLock = 0x0704,
    Elevator = 0x0705,
    Window = 0x0706,
    EntranceGate = 0x0707,
    DoorLock = 0x0708,
    Locker = 0x0709,
    GenericMotorizedDevice = 0x0740,
    MotorizedGate = 0x0741,
    Awning = 0x0742,
    BlindsOrShades = 0x0743,
    Curtains = 0x0744,
    Screen = 0x0745,
    GenericPowerDevice = 0x0780,
    PowerOutlet = 0x0781,
    PowerStrip = 0x0782,
    Plug = 0x0783,
    PowerSupply = 0x0784,
    LedDriver = 0x0785,
    FluorescentLampGear = 0x0786,
    HidLampGear = 0x0787,
    ChargeCase = 0x0788,
    PowerBank = 0x0789,
    GenericLightSource = 0x07C0,
    IncandescentLightBulb = 0x07C1,
    LedLamp = 0x07C2,
    HidLamp = 0x07C3,
    FluorescentLamp = 0x07C4,
    LedArray = 0x07C5,
    MultiColorLedArray = 0x07C6,
    LowVoltageHalogen = 0x07C7,
    Oled = 0x07C8,
    GenericWindowCovering = 0x0800,
    WindowShades = 0x0801,
    WindowBlinds = 0x0802,
    WindowAwning = 0x0803,
    WindowCurtain = 0x0804,
    ExteriorShutter = 0x0805,
    ExteriorScreen = 0x0806,
    GenericAudioSink = 0x0840,
    StandaloneSpeaker = 0x0841,
    Soundbar = 0x0842,
    BookshelfSpeaker = 0x0843,
    StandmountedSpeaker = 0x0844,
    Speakerphone = 0x0845,
    GenericAudioSource = 0x0880,
    Microphone = 0x0881,
    Alarm = 0x0882,
    Bell = 0x0883,
    Horn = 0x0884,
    BroadcastingDevice = 0x0885,
    ServiceDesk = 0x0886,
    Kiosk = 0x0887,
    BroadcastingRoom = 0x0888,
    Auditorium = 0x0889,
    GenericMotorizedVehicle = 0x08C0,
    Car = 0x08C1,
    LargeGoodsVehicle = 0x08C2,
    TwoWheeledVehicle = 0x08C3,
    Motorbike = 0x08C4,
    Scooter = 0x08C5,
    Moped = 0x08C6,
    ThreeWheeledVehicle = 0x08C7,
    LightVehicle = 0x08C8,
    QuadBike = 0x08C9,
    Minibus = 0x08CA,
    Bus = 0x08CB,
    Trolley = 0x08CC,
    AgriculturalVehicle = 0x08CD,
    CamperOrCaravan = 0x08CE,
    RecreationalVehicleOrMotorHome = 0x08CF,
    GenericDomesticAppliance = 0x0900,
    Refrigerator = 0x0901,
    Freezer = 0x0902,
    Oven = 0x0903,
    Microwave = 0x0904,
    Toaster = 0x0905,
    WashingMachine = 0x0906,
    Dryer = 0x0907,
    CoffeeMaker = 0x0908,
    ClothesIron = 0x0909,
    CurlingIron = 0x090A,
    HairDryer = 0x090B,
    VacuumCleaner = 0x090C,
    RoboticVacuumCleaner = 0x090D,
    RiceCooker = 0x090E,
    ClothesSteamer = 0x090F,
    GenericWearableAudioDevice = 0x0940,
    Earbud = 0x0941,
    Headset = 0x0942,
    Headphones = 0x0943,
    NeckBand = 0x0944,
    GenericAircraft = 0x0980,
    LightAircraft = 0x0981,
    Microlight = 0x0982,
    Paraglider = 0x0983,
    LargePassengerAircraft = 0x0984,
    GenericAvEquipment = 0x09C0,
    Amplifier = 0x09C1,
    Receiver = 0x09C2,
    Radio = 0x09C3,
    Tuner = 0x09C4,
    Turntable = 0x09C5,
    CdPlayer = 0x09C6,
    DvdPlayer = 0x09C7,
    BlurayPlayer = 0x09C8,
    OpticalDiscPlayer = 0x09C9,
    SetTopBox = 0x09CA,
    GenericDisplayEquipment = 0x0A00,
    Television = 0x0A01,
    Monitor = 0x0A02,
    Projector = 0x0A03,
    GenericHearingAid = 0x0A40,
    InearHearingAid = 0x0A41,
    BehindearHearingAid = 0x0A42,
    CochlearImplant = 0x0A43,
    GenericGaming = 0x0A80,
    HomeVideoGameConsole = 0x0A81,
    PortableHandheldConsole = 0x0A82,
    GenericSignage = 0x0AC0,
    DigitalSignage = 0x0AC1,
    ElectronicLabel = 0x0AC2,
    GenericPulseOximeter = 0x0C40,
    FingertipPulseOximeter = 0x0C41,
    WristWornPulseOximeter = 0x0C42,
    GenericWeightScale = 0x0C80,
    GenericPersonalMobilityDevice = 0x0CC0,
    PoweredWheelchair = 0x0CC1,
    MobilityScooter = 0x0CC2,
    GenericContinuousGlucoseMonitor = 0x0D00,
    GenericInsulinPump = 0x0D40,
    DurableInsulinPump = 0x0D41,
    PatchInsulinPump = 0x0D44,
    InsulinPen = 0x0D48,
    GenericMedicationDelivery = 0x0D80,
    GenericOutdoorSportsActivity = 0x1440,
    LocationDisplay = 0x1441,
    LocationAndNavigationDisplay = 0x1442,
    LocationPod = 0x1443,
    LocationAndNavigationPod = 0x1444,
}

impl Appearance {
    /// Returns the generic appearance category
    /// ([Assigned Numbers] Section 2.6.2).
    #[inline]
    #[must_use]
    pub fn category(self) -> Self {
        Self::from(u16::from(self) & (u16::MAX << 6))
    }
}
