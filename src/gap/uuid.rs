#![allow(clippy::use_self)]

use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::num::{NonZeroU128, NonZeroU16};

use crate::gatt;

const SHIFT: u32 = u128::BITS - u32::BITS;
const BASE: u128 = 0x00000000_0000_1000_8000_00805F9B34FB;
const MASK_16: u128 = !((u16::MAX as u128) << SHIFT);
const MASK_32: u128 = !((u32::MAX as u128) << SHIFT);

/// 16-, 32-, or 128-bit UUID ([Vol 3] Part B, Section 2.5.1).
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Uuid(NonZeroU128);

impl Uuid {
    /// Creates a UUID from a `u128`.
    #[inline]
    #[must_use]
    pub const fn new(v: u128) -> Option<Self> {
        match NonZeroU128::new(v) {
            Some(nz) => Some(Self(nz)),
            None => None,
        }
    }

    /// Creates a UUID from a `u128` without checking whether the value is
    /// non-zero.
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(v: u128) -> Self {
        Self(NonZeroU128::new_unchecked(v))
    }

    /// Returns a [`Uuid16`] representation or [`None`] if the UUID is not an
    /// assigned 16-bit UUID.
    #[inline]
    #[must_use]
    pub fn as_uuid16(self) -> Option<Uuid16> {
        self.as_u16().map(Uuid16::sig)
    }

    /// Converts an assigned 16-bit Bluetooth SIG UUID to `u16`. This is
    /// mutually exclusive with `as_u32` and `as_u128`.
    #[inline]
    #[must_use]
    pub fn as_u16(self) -> Option<u16> {
        #[allow(clippy::cast_possible_truncation)]
        let v = (self.0.get() >> SHIFT) as u16;
        (self.0.get() & MASK_16 == BASE && v > 0).then_some(v)
    }

    /// Converts an assigned 32-bit Bluetooth SIG UUID to `u32`. This is
    /// mutually exclusive with `as_u16` and `as_u128`.
    #[inline]
    #[must_use]
    pub fn as_u32(self) -> Option<u32> {
        let v = (self.0.get() >> SHIFT) as u32;
        (self.0.get() & MASK_32 == BASE && v > u32::from(u16::MAX)).then_some(v)
    }

    /// Converts an unassigned UUID to `u128`. This is mutually exclusive with
    /// `as_u16` and `as_u32`.
    #[inline]
    #[must_use]
    pub fn as_u128(self) -> Option<u128> {
        (self.0.get() & MASK_32 != BASE).then_some(self.0.get())
    }
}

impl From<Uuid16> for Uuid {
    #[inline]
    fn from(u: Uuid16) -> Self {
        u.as_uuid()
    }
}

impl Debug for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        #[allow(clippy::cast_possible_truncation)]
        if let Some(v) = self.as_u16() {
            write!(f, "{v:#06X}")
        } else if let Some(v) = self.as_u32() {
            write!(f, "{v:#010X}")
        } else {
            let v = self.0.get();
            write!(
                f,
                "{:08X}-{:04X}-{:04X}-{:04X}-{:012X}",
                (v >> 96) as u32,
                (v >> 80) as u16,
                (v >> 64) as u16,
                (v >> 48) as u16,
                (v & ((1 << 48) - 1)) as u64
            )
        }
    }
}

impl Display for Uuid {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO: Translate
        Debug::fmt(self, f)
    }
}

impl From<Uuid> for u128 {
    #[inline]
    fn from(u: Uuid) -> Self {
        u.0.get()
    }
}

/// 16-bit Bluetooth SIG UUID.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Uuid16(NonZeroU16);

impl Uuid16 {
    /// Creates a 16-bit SIG UUID from a `u16`.
    #[inline]
    #[must_use]
    pub const fn new(v: u16) -> Option<Self> {
        match NonZeroU16::new(v) {
            Some(nz) => Some(Self(nz)),
            None => None,
        }
    }

    /// Creates an assigned 16-bit SIG UUID from a `u16`.
    ///
    /// # Panics
    ///
    /// Panics if v is 0.
    #[inline(always)]
    #[must_use]
    pub const fn sig(v: u16) -> Self {
        assert!(v != 0);
        // SAFETY: v != 0
        Self(unsafe { NonZeroU16::new_unchecked(v) })
    }

    /// Returns 128-bit UUID representation.
    #[inline]
    #[must_use]
    pub const fn as_uuid(self) -> Uuid {
        // TODO: Use NonZeroU128::from() when it is const
        // SAFETY: Always non-zero
        unsafe { Uuid::new_unchecked((self.0.get() as u128) << SHIFT | BASE) }
    }

    /// Returns the raw 16-bit UUID value.
    #[inline]
    #[must_use]
    pub(crate) const fn raw(self) -> u16 {
        self.0.get()
    }
}

impl Debug for Uuid16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#06X}", self.0.get())
    }
}

impl Display for Uuid16 {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Uuid16 {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_uuid().hash(state);
    }
}

impl From<Uuid16> for u16 {
    #[inline]
    fn from(u: Uuid16) -> Self {
        u.raw()
    }
}

/// Provides implementations for converting a `u16` SIG UUID enum into [`Uuid`]
/// and [`Uuid16`].
macro_rules! sig_enum {
    ($($t:ty)*) => {$(
        impl $t {
            /// Returns the `Uuid16` representation of the variant.
            #[inline]
            #[must_use]
            pub const fn uuid16(self) -> Uuid16 {
                Uuid16::sig(self as _)
            }
        }

        impl From<$t> for Uuid {
            #[inline]
            fn from(v: $t) -> Self {
                v.uuid16().as_uuid()
            }
        }

        impl From<$t> for Uuid16 {
            #[inline]
            fn from(v: $t) -> Self {
                v.uuid16()
            }
        }
    )*}
}
sig_enum! { ServiceClassId GattServiceId DescriptorId CharacteristicId }

/// SDP service class identifiers ([Assigned Numbers] Section 3.3).
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, num_enum::IntoPrimitive, strum::Display,
)]
#[non_exhaustive]
#[repr(u16)]
pub enum ServiceClassId {
    ServiceDiscoveryServer = 0x1000,
    BrowseGroupDescriptor = 0x1001,
    SerialPort = 0x1101,
    LanAccessUsingPpp = 0x1102,
    DialupNetworking = 0x1103,
    IrMcSync = 0x1104,
    ObexObjectPush = 0x1105,
    ObexFileTransfer = 0x1106,
    IrMcSyncCommand = 0x1107,
    Headset = 0x1108,
    CordlessTelephony = 0x1109,
    AudioSource = 0x110A,
    AudioSink = 0x110B,
    AvRemoteControlTarget = 0x110C,
    AvRemoteControl = 0x110E,
    AvRemoteControlController = 0x110F,
    Intercom = 0x1110,
    Fax = 0x1111,
    HeadsetAudioGateway = 0x1112,
    Wap = 0x1113,
    WapClient = 0x1114,
    Panu = 0x1115,
    Nap = 0x1116,
    Gn = 0x1117,
    DirectPrinting = 0x1118,
    ReferencePrinting = 0x1119,
    ImagingResponder = 0x111B,
    ImagingAutomaticArchive = 0x111C,
    ImagingReferencedObjects = 0x111D,
    Handsfree = 0x111E,
    HandsfreeAudioGateway = 0x111F,
    DirectPrintingReferenceObjectsService = 0x1120,
    ReflectedUi = 0x1121,
    PrintingStatus = 0x1123,
    HumanInterfaceDeviceService = 0x1124,
    HcrPrint = 0x1126,
    HcrScan = 0x1127,
    CommonIsdnAccess = 0x1128,
    SimAccess = 0x112D,
    PhonebookAccessPce = 0x112E,
    PhonebookAccessPse = 0x112F,
    HeadsetHs = 0x1131,
    MessageAccessServer = 0x1132,
    MessageNotificationServer = 0x1133,
    GnssServer = 0x1136,
    ThreeDDisplay = 0x1137,
    ThreeDGlasses = 0x1138,
    MpsScUuid = 0x113B,
    CtnAccessService = 0x113C,
    CtnNotificationService = 0x113D,
    PnPInformation = 0x1200,
    GenericNetworking = 0x1201,
    GenericFileTransfer = 0x1202,
    GenericAudio = 0x1203,
    GenericTelephony = 0x1204,
    UpnpService = 0x1205,
    UpnpIpService = 0x1206,
    EsdpUpnpIpPan = 0x1300,
    EsdpUpnpIpLap = 0x1301,
    EsdpUpnpL2Cap = 0x1302,
    VideoSource = 0x1303,
    VideoSink = 0x1304,
    HdpSource = 0x1401,
    HdpSink = 0x1402,
}

/// GATT Services ([Assigned Numbers] Section 3.4).
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, num_enum::IntoPrimitive, strum::Display,
)]
#[non_exhaustive]
#[repr(u16)]
pub enum GattServiceId {
    GenericAccess = 0x1800,
    GenericAttribute = 0x1801,
    ImmediateAlert = 0x1802,
    LinkLoss = 0x1803,
    TxPower = 0x1804,
    CurrentTime = 0x1805,
    ReferenceTimeUpdate = 0x1806,
    NextDstChange = 0x1807,
    Glucose = 0x1808,
    HealthThermometer = 0x1809,
    DeviceInformation = 0x180A,
    HeartRate = 0x180D,
    PhoneAlertStatus = 0x180E,
    Battery = 0x180F,
    BloodPressure = 0x1810,
    AlertNotification = 0x1811,
    HumanInterfaceDevice = 0x1812,
    ScanParameters = 0x1813,
    RunningSpeedAndCadence = 0x1814,
    AutomationIo = 0x1815,
    CyclingSpeedAndCadence = 0x1816,
    CyclingPower = 0x1818,
    LocationAndNavigation = 0x1819,
    EnvironmentalSensing = 0x181A,
    BodyComposition = 0x181B,
    UserData = 0x181C,
    WeightScale = 0x181D,
    BondManagement = 0x181E,
    ContinuousGlucoseMonitoring = 0x181F,
    InternetProtocolSupport = 0x1820,
    IndoorPositioning = 0x1821,
    PulseOximeter = 0x1822,
    HttpProxy = 0x1823,
    TransportDiscovery = 0x1824,
    ObjectTransfer = 0x1825,
    FitnessMachine = 0x1826,
    MeshProvisioning = 0x1827,
    MeshProxy = 0x1828,
    ReconnectionConfiguration = 0x1829,
    InsulinDelivery = 0x183A,
    BinarySensor = 0x183B,
    EmergencyConfiguration = 0x183C,
    AuthorizationControl = 0x183D,
    PhysicalActivityMonitor = 0x183E,
    AudioInputControl = 0x1843,
    VolumeControl = 0x1844,
    VolumeOffsetControl = 0x1845,
    CoordinatedSetIdentification = 0x1846,
    DeviceTime = 0x1847,
    MediaControl = 0x1848,
    GenericMediaControl = 0x1849,
    ConstantToneExtension = 0x184A,
    TelephoneBearer = 0x184B,
    GenericTelephoneBearer = 0x184C,
    MicrophoneControl = 0x184D,
    AudioStreamControl = 0x184E,
    BroadcastAudioScan = 0x184F,
    PublishedAudioCapabilities = 0x1850,
    BasicAudioAnnouncement = 0x1851,
    BroadcastAudioAnnouncement = 0x1852,
    CommonAudio = 0x1853,
    HearingAid = 0x1854,
    Tmas = 0x1855,
}

/// Descriptors ([Assigned Numbers] Section 3.7).
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, num_enum::IntoPrimitive, strum::Display,
)]
#[non_exhaustive]
#[repr(u16)]
pub enum DescriptorId {
    CharacteristicExtendedProperties = gatt::Type::CHARACTERISTIC_EXTENDED_PROPERTIES.raw(),
    CharacteristicUserDescription = gatt::Type::CHARACTERISTIC_USER_DESCRIPTION.raw(),
    ClientCharacteristicConfiguration = gatt::Type::CLIENT_CHARACTERISTIC_CONFIGURATION.raw(),
    ServerCharacteristicConfiguration = gatt::Type::SERVER_CHARACTERISTIC_CONFIGURATION.raw(),
    CharacteristicPresentationFormat = gatt::Type::CHARACTERISTIC_PRESENTATION_FORMAT.raw(),
    CharacteristicAggregateFormat = gatt::Type::CHARACTERISTIC_AGGREGATE_FORMAT.raw(),
    ValidRange = 0x2906,
    ExternalReportReference = 0x2907,
    ReportReference = 0x2908,
    NumberOfDigitals = 0x2909,
    ValueTriggerSetting = 0x290A,
    EnvironmentalSensingConfiguration = 0x290B,
    EnvironmentalSensingMeasurement = 0x290C,
    EnvironmentalSensingTriggerSetting = 0x290D,
    TimeTriggerSetting = 0x290E,
    CompleteBredrTransportBlockData = 0x290F,
}

/// Characteristics ([Assigned Numbers] Section 3.8).
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, num_enum::IntoPrimitive, strum::Display,
)]
#[non_exhaustive]
#[repr(u16)]
pub enum CharacteristicId {
    DeviceName = 0x2A00,
    Appearance = 0x2A01,
    PeripheralPrivacyFlag = 0x2A02,
    ReconnectionAddress = 0x2A03,
    PeripheralPreferredConnectionParameters = 0x2A04,
    ServiceChanged = 0x2A05,
    AlertLevel = 0x2A06,
    TxPowerLevel = 0x2A07,
    DateTime = 0x2A08,
    DayOfWeek = 0x2A09,
    DayDateTime = 0x2A0A,
    ExactTime256 = 0x2A0C,
    DstOffset = 0x2A0D,
    TimeZone = 0x2A0E,
    LocalTimeInformation = 0x2A0F,
    TimeWithDst = 0x2A11,
    TimeAccuracy = 0x2A12,
    TimeSource = 0x2A13,
    ReferenceTimeInformation = 0x2A14,
    TimeUpdateControlPoint = 0x2A16,
    TimeUpdateState = 0x2A17,
    GlucoseMeasurement = 0x2A18,
    BatteryLevel = 0x2A19,
    TemperatureMeasurement = 0x2A1C,
    TemperatureType = 0x2A1D,
    IntermediateTemperature = 0x2A1E,
    MeasurementInterval = 0x2A21,
    BootKeyboardInputReport = 0x2A22,
    SystemId = 0x2A23,
    ModelNumberString = 0x2A24,
    SerialNumberString = 0x2A25,
    FirmwareRevisionString = 0x2A26,
    HardwareRevisionString = 0x2A27,
    SoftwareRevisionString = 0x2A28,
    ManufacturerNameString = 0x2A29,
    IeeeRegulatoryCertificationDataList = 0x2A2A,
    CurrentTime = 0x2A2B,
    MagneticDeclination = 0x2A2C,
    ScanRefresh = 0x2A31,
    BootKeyboardOutputReport = 0x2A32,
    BootMouseInputReport = 0x2A33,
    GlucoseMeasurementContext = 0x2A34,
    BloodPressureMeasurement = 0x2A35,
    IntermediateCuffPressure = 0x2A36,
    HeartRateMeasurement = 0x2A37,
    BodySensorLocation = 0x2A38,
    HeartRateControlPoint = 0x2A39,
    AlertStatus = 0x2A3F,
    RingerControlPoint = 0x2A40,
    RingerSetting = 0x2A41,
    AlertCategoryIdBitMask = 0x2A42,
    AlertCategoryId = 0x2A43,
    AlertNotificationControlPoint = 0x2A44,
    UnreadAlertStatus = 0x2A45,
    NewAlert = 0x2A46,
    SupportedNewAlertCategory = 0x2A47,
    SupportedUnreadAlertCategory = 0x2A48,
    BloodPressureFeature = 0x2A49,
    HidInformation = 0x2A4A,
    ReportMap = 0x2A4B,
    HidControlPoint = 0x2A4C,
    Report = 0x2A4D,
    ProtocolMode = 0x2A4E,
    ScanIntervalWindow = 0x2A4F,
    PnpId = 0x2A50,
    GlucoseFeature = 0x2A51,
    RecordAccessControlPoint = 0x2A52,
    RscMeasurement = 0x2A53,
    RscFeature = 0x2A54,
    ScControlPoint = 0x2A55,
    Aggregate = 0x2A5A,
    CscMeasurement = 0x2A5B,
    CscFeature = 0x2A5C,
    SensorLocation = 0x2A5D,
    PlxSpotCheckMeasurement = 0x2A5E,
    PlxContinuousMeasurement = 0x2A5F,
    PlxFeatures = 0x2A60,
    CyclingPowerMeasurement = 0x2A63,
    CyclingPowerVector = 0x2A64,
    CyclingPowerFeature = 0x2A65,
    CyclingPowerControlPoint = 0x2A66,
    LocationAndSpeed = 0x2A67,
    Navigation = 0x2A68,
    PositionQuality = 0x2A69,
    LnFeature = 0x2A6A,
    LnControlPoint = 0x2A6B,
    Elevation = 0x2A6C,
    Pressure = 0x2A6D,
    Temperature = 0x2A6E,
    Humidity = 0x2A6F,
    TrueWindSpeed = 0x2A70,
    TrueWindDirection = 0x2A71,
    ApparentWindSpeed = 0x2A72,
    ApparentWindDirection = 0x2A73,
    GustFactor = 0x2A74,
    PollenConcentration = 0x2A75,
    UvIndex = 0x2A76,
    Irradiance = 0x2A77,
    Rainfall = 0x2A78,
    WindChill = 0x2A79,
    HeatIndex = 0x2A7A,
    DewPoint = 0x2A7B,
    DescriptorValueChanged = 0x2A7D,
    AerobicHeartRateLowerLimit = 0x2A7E,
    AerobicThreshold = 0x2A7F,
    Age = 0x2A80,
    AnaerobicHeartRateLowerLimit = 0x2A81,
    AnaerobicHeartRateUpperLimit = 0x2A82,
    AnaerobicThreshold = 0x2A83,
    AerobicHeartRateUpperLimit = 0x2A84,
    DateOfBirth = 0x2A85,
    DateOfThresholdAssessment = 0x2A86,
    EmailAddress = 0x2A87,
    FatBurnHeartRateLowerLimit = 0x2A88,
    FatBurnHeartRateUpperLimit = 0x2A89,
    FirstName = 0x2A8A,
    FiveZoneHeartRateLimits = 0x2A8B,
    Gender = 0x2A8C,
    HeartRateMax = 0x2A8D,
    Height = 0x2A8E,
    HipCircumference = 0x2A8F,
    LastName = 0x2A90,
    MaximumRecommendedHeartRate = 0x2A91,
    RestingHeartRate = 0x2A92,
    SportTypeForAerobicAndAnaerobicThresholds = 0x2A93,
    ThreeZoneHeartRateLimits = 0x2A94,
    TwoZoneHeartRateLimits = 0x2A95,
    Vo2Max = 0x2A96,
    WaistCircumference = 0x2A97,
    Weight = 0x2A98,
    DatabaseChangeIncrement = 0x2A99,
    UserIndex = 0x2A9A,
    BodyCompositionFeature = 0x2A9B,
    BodyCompositionMeasurement = 0x2A9C,
    WeightMeasurement = 0x2A9D,
    WeightScaleFeature = 0x2A9E,
    UserControlPoint = 0x2A9F,
    MagneticFluxDensity2D = 0x2AA0,
    MagneticFluxDensity3D = 0x2AA1,
    Language = 0x2AA2,
    BarometricPressureTrend = 0x2AA3,
    BondManagementControlPoint = 0x2AA4,
    BondManagementFeature = 0x2AA5,
    CentralAddressResolution = 0x2AA6,
    CgmMeasurement = 0x2AA7,
    CgmFeature = 0x2AA8,
    CgmStatus = 0x2AA9,
    CgmSessionStartTime = 0x2AAA,
    CgmSessionRunTime = 0x2AAB,
    CgmSpecificOpsControlPoint = 0x2AAC,
    IndoorPositioningConfiguration = 0x2AAD,
    Latitude = 0x2AAE,
    Longitude = 0x2AAF,
    LocalNorthCoordinate = 0x2AB0,
    LocalEastCoordinate = 0x2AB1,
    FloorNumber = 0x2AB2,
    Altitude = 0x2AB3,
    Uncertainty = 0x2AB4,
    LocationName = 0x2AB5,
    Uri = 0x2AB6,
    HttpHeaders = 0x2AB7,
    HttpStatusCode = 0x2AB8,
    HttpEntityBody = 0x2AB9,
    HttpControlPoint = 0x2ABA,
    HttpsSecurity = 0x2ABB,
    TdsControlPoint = 0x2ABC,
    OtsFeature = 0x2ABD,
    ObjectName = 0x2ABE,
    ObjectType = 0x2ABF,
    ObjectSize = 0x2AC0,
    ObjectFirstCreated = 0x2AC1,
    ObjectLastModified = 0x2AC2,
    ObjectId = 0x2AC3,
    ObjectProperties = 0x2AC4,
    ObjectActionControlPoint = 0x2AC5,
    ObjectListControlPoint = 0x2AC6,
    ObjectListFilter = 0x2AC7,
    ObjectChanged = 0x2AC8,
    ResolvablePrivateAddressOnly = 0x2AC9,
    FitnessMachineFeature = 0x2ACC,
    TreadmillData = 0x2ACD,
    CrossTrainerData = 0x2ACE,
    StepClimberData = 0x2ACF,
    StairClimberData = 0x2AD0,
    RowerData = 0x2AD1,
    IndoorBikeData = 0x2AD2,
    TrainingStatus = 0x2AD3,
    SupportedSpeedRange = 0x2AD4,
    SupportedInclinationRange = 0x2AD5,
    SupportedResistanceLevelRange = 0x2AD6,
    SupportedHeartRateRange = 0x2AD7,
    SupportedPowerRange = 0x2AD8,
    FitnessMachineControlPoint = 0x2AD9,
    FitnessMachineStatus = 0x2ADA,
    MeshProvisioningDataIn = 0x2ADB,
    MeshProvisioningDataOut = 0x2ADC,
    MeshProxyDataIn = 0x2ADD,
    MeshProxyDataOut = 0x2ADE,
    AverageCurrent = 0x2AE0,
    AverageVoltage = 0x2AE1,
    Boolean = 0x2AE2,
    ChromaticDistanceFromPlanckian = 0x2AE3,
    ChromaticityCoordinates = 0x2AE4,
    ChromaticityInCctAndDuvValues = 0x2AE5,
    ChromaticityTolerance = 0x2AE6,
    CieColorRenderingIndex = 0x2AE7,
    Coefficient = 0x2AE8,
    CorrelatedColorTemperature = 0x2AE9,
    Count16 = 0x2AEA,
    Count24 = 0x2AEB,
    CountryCode = 0x2AEC,
    DateUtc = 0x2AED,
    ElectricCurrent = 0x2AEE,
    ElectricCurrentRange = 0x2AEF,
    ElectricCurrentSpecification = 0x2AF0,
    ElectricCurrentStatistics = 0x2AF1,
    Energy = 0x2AF2,
    EnergyInAPeriodOfDay = 0x2AF3,
    EventStatistics = 0x2AF4,
    FixedString16 = 0x2AF5,
    FixedString24 = 0x2AF6,
    FixedString36 = 0x2AF7,
    FixedString8 = 0x2AF8,
    GenericLevel = 0x2AF9,
    GlobalTradeItemNumber = 0x2AFA,
    Illuminance = 0x2AFB,
    LuminousEfficacy = 0x2AFC,
    LuminousEnergy = 0x2AFD,
    LuminousExposure = 0x2AFE,
    LuminousFlux = 0x2AFF,
    LuminousFluxRange = 0x2B00,
    LuminousIntensity = 0x2B01,
    MassFlow = 0x2B02,
    PerceivedLightness = 0x2B03,
    Percentage8 = 0x2B04,
    Power = 0x2B05,
    PowerSpecification = 0x2B06,
    RelativeRuntimeInACurrentRange = 0x2B07,
    RelativeRuntimeInAGenericLevelRange = 0x2B08,
    RelativeValueInAVoltageRange = 0x2B09,
    RelativeValueInAnIlluminanceRange = 0x2B0A,
    RelativeValueInAPeriodOfDay = 0x2B0B,
    RelativeValueInATemperatureRange = 0x2B0C,
    Temperature8 = 0x2B0D,
    Temperature8InAPeriodOfDay = 0x2B0E,
    Temperature8Statistics = 0x2B0F,
    TemperatureRange = 0x2B10,
    TemperatureStatistics = 0x2B11,
    TimeDecihour8 = 0x2B12,
    TimeExponential8 = 0x2B13,
    TimeHour24 = 0x2B14,
    TimeMillisecond24 = 0x2B15,
    TimeSecond16 = 0x2B16,
    TimeSecond8 = 0x2B17,
    Voltage = 0x2B18,
    VoltageSpecification = 0x2B19,
    VoltageStatistics = 0x2B1A,
    VolumeFlow = 0x2B1B,
    ChromaticityCoordinate = 0x2B1C,
    RcFeature = 0x2B1D,
    RcSettings = 0x2B1E,
    ReconnectionConfigurationControlPoint = 0x2B1F,
    IddStatusChanged = 0x2B20,
    IddStatus = 0x2B21,
    IddAnnunciationStatus = 0x2B22,
    IddFeatures = 0x2B23,
    IddStatusReaderControlPoint = 0x2B24,
    IddCommandControlPoint = 0x2B25,
    IddCommandData = 0x2B26,
    IddRecordAccessControlPoint = 0x2B27,
    IddHistoryData = 0x2B28,
    ClientSupportedFeatures = 0x2B29,
    DatabaseHash = 0x2B2A,
    BssControlPoint = 0x2B2B,
    BssResponse = 0x2B2C,
    EmergencyId = 0x2B2D,
    EmergencyText = 0x2B2E,
    AcsStatus = 0x2B2F,
    AcsDataIn = 0x2B30,
    AcsDataOutNotify = 0x2B31,
    AcsDataOutIndicate = 0x2B32,
    AcsControlPoint = 0x2B33,
    EnhancedBloodPressureMeasurement = 0x2B34,
    EnhancedIntermediateCuffPressure = 0x2B35,
    BloodPressureRecord = 0x2B36,
    RegisteredUser = 0x2B37,
    BredrHandoverData = 0x2B38,
    BluetoothSigData = 0x2B39,
    ServerSupportedFeatures = 0x2B3A,
    PhysicalActivityMonitorFeatures = 0x2B3B,
    GeneralActivityInstantaneousData = 0x2B3C,
    GeneralActivitySummaryData = 0x2B3D,
    CardioRespiratoryActivityInstantaneousData = 0x2B3E,
    CardioRespiratoryActivitySummaryData = 0x2B3F,
    StepCounterActivitySummaryData = 0x2B40,
    SleepActivityInstantaneousData = 0x2B41,
    SleepActivitySummaryData = 0x2B42,
    PhysicalActivityMonitorControlPoint = 0x2B43,
    ActivityCurrentSession = 0x2B44,
    PhysicalActivitySessionDescriptor = 0x2B45,
    PreferredUnits = 0x2B46,
    HighResolutionHeight = 0x2B47,
    MiddleName = 0x2B48,
    StrideLength = 0x2B49,
    Handedness = 0x2B4A,
    DeviceWearingPosition = 0x2B4B,
    FourZoneHeartRateLimits = 0x2B4C,
    HighIntensityExerciseThreshold = 0x2B4D,
    ActivityGoal = 0x2B4E,
    SedentaryIntervalNotification = 0x2B4F,
    CaloricIntake = 0x2B50,
    TmapRole = 0x2B51,
    AudioInputState = 0x2B77,
    GainSettingsAttribute = 0x2B78,
    AudioInputType = 0x2B79,
    AudioInputStatus = 0x2B7A,
    AudioInputControlPoint = 0x2B7B,
    AudioInputDescription = 0x2B7C,
    VolumeState = 0x2B7D,
    VolumeControlPoint = 0x2B7E,
    VolumeFlags = 0x2B7F,
    VolumeOffsetState = 0x2B80,
    AudioLocation = 0x2B81,
    VolumeOffsetControlPoint = 0x2B82,
    AudioOutputDescription = 0x2B83,
    SetIdentityResolvingKey = 0x2B84,
    CoordinatedSetSize = 0x2B85,
    SetMemberLock = 0x2B86,
    SetMemberRank = 0x2B87,
    ApparentEnergy32 = 0x2B89,
    ApparentPower = 0x2B8A,
    Co2Concentration = 0x2B8C,
    CosineOfTheAngle = 0x2B8D,
    DeviceTimeFeature = 0x2B8E,
    DeviceTimeParameters = 0x2B8F,
    DeviceTime = 0x2B90,
    DeviceTimeControlPoint = 0x2B91,
    TimeChangeLogData = 0x2B92,
    MediaPlayerName = 0x2B93,
    MediaPlayerIconObjectId = 0x2B94,
    MediaPlayerIconUrl = 0x2B95,
    TrackChanged = 0x2B96,
    TrackTitle = 0x2B97,
    TrackDuration = 0x2B98,
    TrackPosition = 0x2B99,
    PlaybackSpeed = 0x2B9A,
    SeekingSpeed = 0x2B9B,
    CurrentTrackSegmentsObjectId = 0x2B9C,
    CurrentTrackObjectId = 0x2B9D,
    NextTrackObjectId = 0x2B9E,
    ParentGroupObjectId = 0x2B9F,
    CurrentGroupObjectId = 0x2BA0,
    PlayingOrder = 0x2BA1,
    PlayingOrdersSupported = 0x2BA2,
    MediaState = 0x2BA3,
    MediaControlPoint = 0x2BA4,
    MediaControlPointOpcodesSupported = 0x2BA5,
    SearchResultsObjectId = 0x2BA6,
    SearchControlPoint = 0x2BA7,
    Energy32 = 0x2BA8,
    MediaPlayerIconObjectType = 0x2BA9,
    TrackSegmentsObjectType = 0x2BAA,
    TrackObjectType = 0x2BAB,
    GroupObjectType = 0x2BAC,
    ConstantToneExtensionEnable = 0x2BAD,
    AdvertisingConstantToneExtensionMinimumLength = 0x2BAE,
    AdvertisingConstantToneExtensionMinimumTransmitCount = 0x2BAF,
    AdvertisingConstantToneExtensionTransmitDuration = 0x2BB0,
    AdvertisingConstantToneExtensionInterval = 0x2BB1,
    AdvertisingConstantToneExtensionPhy = 0x2BB2,
    BearerProviderName = 0x2BB3,
    BearerUci = 0x2BB4,
    BearerTechnology = 0x2BB5,
    BearerUriSchemesSupportedList = 0x2BB6,
    BearerSignalStrength = 0x2BB7,
    BearerSignalStrengthReportingInterval = 0x2BB8,
    BearerListCurrentCalls = 0x2BB9,
    ContentControlId = 0x2BBA,
    StatusFlags = 0x2BBB,
    IncomingCallTargetBearerUri = 0x2BBC,
    CallState = 0x2BBD,
    CallControlPoint = 0x2BBE,
    CallControlPointOptionalOpcodes = 0x2BBF,
    TerminationReason = 0x2BC0,
    IncomingCall = 0x2BC1,
    CallFriendlyName = 0x2BC2,
    Mute = 0x2BC3,
    SinkAse = 0x2BC4,
    SourceAse = 0x2BC5,
    AseControlPoint = 0x2BC6,
    BroadcastAudioScanControlPoint = 0x2BC7,
    BroadcastReceiveState = 0x2BC8,
    SinkPac = 0x2BC9,
    SinkAudioLocations = 0x2BCA,
    SourcePac = 0x2BCB,
    SourceAudioLocations = 0x2BCC,
    AvailableAudioContexts = 0x2BCD,
    SupportedAudioContexts = 0x2BCE,
    AmmoniaConcentration = 0x2BCF,
    CarbonMonoxideConcentration = 0x2BD0,
    MethaneConcentration = 0x2BD1,
    NitrogenDioxideConcentration = 0x2BD2,
    NonMethaneVolatileOrganicCompoundsConcentration = 0x2BD3,
    OzoneConcentration = 0x2BD4,
    ParticulateMatterPm1Concentration = 0x2BD5,
    ParticulateMatterPm25Concentration = 0x2BD6,
    ParticulateMatterPm10Concentration = 0x2BD7,
    SulfurDioxideConcentration = 0x2BD8,
    SulfurHexafluorideConcentration = 0x2BD9,
    HearingAidFeatures = 0x2BDA,
    HearingAidPresetControlPoint = 0x2BDB,
    ActivePresetIndex = 0x2BDC,
    FixedString64 = 0x2BDE,
    HighTemperature = 0x2BDF,
    HighVoltage = 0x2BE0,
    LightDistribution = 0x2BE1,
    LightOutput = 0x2BE2,
    LightSourceType = 0x2BE3,
    Noise = 0x2BE4,
    RelativeRuntimeInACorrelatedColorTemperatureRange = 0x2BE5,
    TimeSecond32 = 0x2BE6,
    VocConcentration = 0x2BE7,
    VoltageFrequency = 0x2BE8,
}
