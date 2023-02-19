#![allow(clippy::use_self)]

use std::fmt::Debug;

use bitflags::bitflags;

pub use burble_const::CompanyId;
use OpcodeGroup::*;

/// HCI command header and buffer sizes ([Vol 4] Part E, Section 5.4.1).
pub(super) const CMD_HDR: usize = 3;
pub(crate) const CMD_BUF: usize = CMD_HDR + u8::MAX as usize;

/// HCI ACL data header and buffer sizes ([Vol 4] Part E, Section 5.4.2).
pub(crate) const ACL_HDR: usize = 4;
pub(crate) const ACL_LE_MIN_DATA_LEN: u16 = 27;

/// HCI event header and buffer sizes ([Vol 4] Part E, Section 5.4.4).
pub(super) const EVT_HDR: usize = 2;
pub(crate) const EVT_BUF: usize = EVT_HDR + u8::MAX as usize;

/// HCI command opcodes ([Vol 4] Part E, Section 7).
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
    strum::Display,
)]
#[non_exhaustive]
#[repr(u16)]
pub enum Opcode {
    /// Opcode 0x0000 is used to update `Num_HCI_Command_Packets`
    /// ([Vol 4] Part E, Section 7.7.14).
    #[default]
    None = 0x0000,

    // HCI Control and Baseband commands ([Vol 4] Part E, Section 7.3)
    SetEventMask = HciControl.ocf(0x0001),
    Reset = HciControl.ocf(0x0003),
    SetControllerToHostFlowControl = HciControl.ocf(0x0031),
    HostBufferSize = HciControl.ocf(0x0033),
    SetEventMaskPage2 = HciControl.ocf(0x0063),
    WriteLeHostSupport = HciControl.ocf(0x006D),

    // Informational parameters commands ([Vol 4] Part E, Section 7.4)
    ReadLocalVersionInformation = InfoParams.ocf(0x0001),
    ReadLocalSupportedCommands = InfoParams.ocf(0x0002),
    ReadBufferSize = InfoParams.ocf(0x0005),
    ReadBdAddr = InfoParams.ocf(0x0009),

    // LE Controller commands ([Vol 4] Part E, Section 7.8)
    LeSetEventMask = Le.ocf(0x0001),
    LeReadBufferSize = Le.ocf(0x0002),
    LeReadBufferSizeV2 = Le.ocf(0x0060),
    LeLongTermKeyRequestReply = Le.ocf(0x001A),
    LeLongTermKeyRequestNegativeReply = Le.ocf(0x001B),
    LeSetAdvertisingSetRandomAddress = Le.ocf(0x0035),
    LeSetExtendedAdvertisingParameters = Le.ocf(0x0036),
    LeSetExtendedAdvertisingData = Le.ocf(0x0037),
    LeSetExtendedScanResponseData = Le.ocf(0x0038),
    LeSetExtendedAdvertisingEnable = Le.ocf(0x0039),
    LeReadMaximumAdvertisingDataLength = Le.ocf(0x003A),
    LeReadNumberOfSupportedAdvertisingSets = Le.ocf(0x003B),
    LeRemoveAdvertisingSet = Le.ocf(0x003C),
    LeClearAdvertisingSets = Le.ocf(0x003D),
    LeSetPeriodicAdvertisingParameters = Le.ocf(0x003E),
    LeSetPeriodicAdvertisingData = Le.ocf(0x003F),
    LeSetPeriodicAdvertisingEnable = Le.ocf(0x0040),
}

impl Opcode {
    /// Returns whether the opcode is `None`.
    #[inline(always)]
    #[must_use]
    pub const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns whether the opcode is other than `None`.
    #[inline(always)]
    #[must_use]
    pub const fn is_some(self) -> bool {
        !self.is_none()
    }
}

// Opcode group field definitions.
#[derive(Clone, Copy)]
#[repr(u16)]
enum OpcodeGroup {
    _LinkControl = 0x01,
    _LinkPolicy = 0x02,
    HciControl = 0x03,
    InfoParams = 0x04,
    _StatusParams = 0x05,
    _Testing = 0x06,
    Le = 0x08,
    _Vendor = 0x3F, // [Vol 4] Part E, Section 5.4.1
}

impl OpcodeGroup {
    /// Combines OGF with OCF to create a full opcode.
    #[inline]
    const fn ocf(self, ocf: u16) -> u16 {
        (self as u16) << 10 | ocf
    }
}

/// HCI event codes ([Vol 4] Part E, Section 7.7).
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, num_enum::TryFromPrimitive, strum::Display, strum::EnumIter,
)]
#[non_exhaustive]
#[repr(u16)]
pub enum EventCode {
    InquiryComplete = 0x01,
    InquiryResult = 0x02,
    ConnectionComplete = 0x03,
    ConnectionRequest = 0x04,
    DisconnectionComplete = 0x05,
    AuthenticationComplete = 0x06,
    RemoteNameRequestComplete = 0x07,
    EncryptionChangeV1 = 0x08,
    EncryptionChangeV2 = 0x59,
    ChangeConnectionLinkKeyComplete = 0x09,
    LinkKeyTypeChanged = 0x0A,
    ReadRemoteSupportedFeaturesComplete = 0x0B,
    ReadRemoteVersionInformationComplete = 0x0C,
    QosSetupComplete = 0x0D,
    CommandComplete = 0x0E,
    CommandStatus = 0x0F,
    HardwareError = 0x10,
    FlushOccurred = 0x11,
    RoleChange = 0x12,
    NumberOfCompletedPackets = 0x13,
    ModeChange = 0x14,
    ReturnLinkKeys = 0x15,
    PinCodeRequest = 0x16,
    LinkKeyRequest = 0x17,
    LinkKeyNotification = 0x18,
    LoopbackCommand = 0x19,
    DataBufferOverflow = 0x1A,
    MaxSlotsChange = 0x1B,
    ReadClockOffsetComplete = 0x1C,
    ConnectionPacketTypeChanged = 0x1D,
    QosViolation = 0x1E,
    PageScanRepetitionModeChange = 0x20,
    FlowSpecificationComplete = 0x21,
    InquiryResultWithRssi = 0x22,
    ReadRemoteExtendedFeaturesComplete = 0x23,
    SynchronousConnectionComplete = 0x2C,
    SynchronousConnectionChanged = 0x2D,
    SniffSubrating = 0x2E,
    ExtendedInquiryResult = 0x2F,
    EncryptionKeyRefreshComplete = 0x30,
    IoCapabilityRequest = 0x31,
    IoCapabilityResponse = 0x32,
    UserConfirmationRequest = 0x33,
    UserPasskeyRequest = 0x34,
    RemoteOobDataRequest = 0x35,
    SimplePairingComplete = 0x36,
    LinkSupervisionTimeoutChanged = 0x38,
    EnhancedFlushComplete = 0x39,
    UserPasskeyNotification = 0x3B,
    KeypressNotification = 0x3C,
    RemoteHostSupportedFeaturesNotification = 0x3D,
    NumberOfCompletedDataBlocks = 0x48,

    // HCI LE subevent codes ([Vol 4] Part E, Section 7.7.65).
    LeMetaEvent = 0x3E,
    LeConnectionComplete = le(0x01),
    LeAdvertisingReport = le(0x02),
    LeConnectionUpdateComplete = le(0x03),
    LeReadRemoteFeaturesComplete = le(0x04),
    LeLongTermKeyRequest = le(0x05),
    LeRemoteConnectionParameterRequest = le(0x06),
    LeDataLengthChange = le(0x07),
    LeReadLocalP256PublicKeyComplete = le(0x08),
    LeGenerateDhKeyComplete = le(0x09),
    LeEnhancedConnectionComplete = le(0x0A),
    LeDirectedAdvertisingReport = le(0x0B),
    LePhyUpdateComplete = le(0x0C),
    LeExtendedAdvertisingReport = le(0x0D),
    LePeriodicAdvertisingSyncEstablished = le(0x0E),
    LePeriodicAdvertisingReport = le(0x0F),
    LePeriodicAdvertisingSyncLost = le(0x10),
    LeScanTimeout = le(0x11),
    LeAdvertisingSetTerminated = le(0x12),
    LeScanRequestReceived = le(0x13),
    LeChannelSelectionAlgorithm = le(0x14),
    LeConnectionlessIqReport = le(0x15),
    LeConnectionIqReport = le(0x16),
    LeCteRequestFailed = le(0x17),
    LePeriodicAdvertisingSyncTransferReceived = le(0x18),
    LeCisEstablished = le(0x19),
    LeCisRequest = le(0x1A),
    LeCreateBigComplete = le(0x1B),
    LeTerminateBigComplete = le(0x1C),
    LeBigSyncEstablished = le(0x1D),
    LeBigSyncLost = le(0x1E),
    LeRequestPeerScaComplete = le(0x1F),
    LePathLossThreshold = le(0x20),
    LeTransmitPowerReporting = le(0x21),
    LeBigInfoAdvertisingReport = le(0x22),
    LeSubrateChange = le(0x23),

    TriggeredClockCapture = 0x4E,
    SynchronizationTrainComplete = 0x4F,
    SynchronizationTrainReceived = 0x50,
    ConnectionlessPeripheralBroadcastReceive = 0x51,
    ConnectionlessPeripheralBroadcastTimeout = 0x52,
    TruncatedPageComplete = 0x53,
    PeripheralPageResponseTimeout = 0x54,
    ConnectionlessPeripheralBroadcastChannelMapChange = 0x55,
    InquiryResponseNotification = 0x56,
    AuthenticatedPayloadTimeoutExpired = 0x57,
    SamStatusChange = 0x58,
    Vendor = 0xFF, // [Vol 4] Part E, Section 5.4.4
}

/// Returns an [`EventCode`] from LE subevent code.
const fn le(subevent: u8) -> u16 {
    (subevent as u16) << 8 | EventCode::LeMetaEvent as u16
}

impl EventCode {
    /// Returns whether the event code is either `CommandComplete` or
    /// `CommandStatus`.
    #[inline(always)]
    #[must_use]
    pub const fn is_cmd(self) -> bool {
        const CMD: u16 = EventCode::CommandComplete as _;
        (self as u16) & CMD == CMD
    }

    /// Returns the format of the associated event parameters.
    #[allow(clippy::too_many_lines)]
    pub(super) const fn param_fmt(self) -> EventFmt {
        use EventCode::*;
        const OTHER: EventFmt = EventFmt::empty();
        const STATUS: EventFmt = EventFmt::STATUS;
        const CONN_HANDLE: EventFmt = EventFmt::CONN_HANDLE;
        const SYNC_HANDLE: EventFmt = EventFmt::SYNC_HANDLE;
        const ADV_HANDLE: EventFmt = EventFmt::ADV_HANDLE;
        const BIG_HANDLE: EventFmt = EventFmt::BIG_HANDLE;
        #[allow(clippy::match_same_arms)]
        match self {
            InquiryComplete => STATUS,
            InquiryResult => OTHER,
            ConnectionComplete => STATUS.union(CONN_HANDLE),
            ConnectionRequest => OTHER,
            DisconnectionComplete => STATUS.union(CONN_HANDLE),
            AuthenticationComplete => STATUS.union(CONN_HANDLE),
            RemoteNameRequestComplete => STATUS,
            EncryptionChangeV1 => STATUS.union(CONN_HANDLE),
            EncryptionChangeV2 => STATUS.union(CONN_HANDLE),
            ChangeConnectionLinkKeyComplete => STATUS.union(CONN_HANDLE),
            LinkKeyTypeChanged => STATUS.union(CONN_HANDLE),
            ReadRemoteSupportedFeaturesComplete => STATUS.union(CONN_HANDLE),
            ReadRemoteVersionInformationComplete => STATUS.union(CONN_HANDLE),
            QosSetupComplete => STATUS.union(CONN_HANDLE),
            CommandComplete => STATUS, // Other format, but want has_status() == true
            CommandStatus => STATUS,
            HardwareError => OTHER,
            FlushOccurred => CONN_HANDLE,
            RoleChange => STATUS,
            NumberOfCompletedPackets => OTHER,
            ModeChange => STATUS.union(CONN_HANDLE),
            ReturnLinkKeys => OTHER,
            PinCodeRequest => OTHER,
            LinkKeyRequest => OTHER,
            LinkKeyNotification => OTHER,
            LoopbackCommand => OTHER,
            DataBufferOverflow => OTHER,
            MaxSlotsChange => CONN_HANDLE,
            ReadClockOffsetComplete => STATUS.union(CONN_HANDLE),
            ConnectionPacketTypeChanged => STATUS.union(CONN_HANDLE),
            QosViolation => CONN_HANDLE,
            PageScanRepetitionModeChange => OTHER,
            FlowSpecificationComplete => STATUS.union(CONN_HANDLE),
            InquiryResultWithRssi => OTHER,
            ReadRemoteExtendedFeaturesComplete => STATUS.union(CONN_HANDLE),
            SynchronousConnectionComplete => STATUS.union(CONN_HANDLE),
            SynchronousConnectionChanged => STATUS.union(CONN_HANDLE),
            SniffSubrating => STATUS.union(CONN_HANDLE),
            ExtendedInquiryResult => OTHER,
            EncryptionKeyRefreshComplete => STATUS.union(CONN_HANDLE),
            IoCapabilityRequest => OTHER,
            IoCapabilityResponse => OTHER,
            UserConfirmationRequest => OTHER,
            UserPasskeyRequest => OTHER,
            RemoteOobDataRequest => OTHER,
            SimplePairingComplete => STATUS,
            LinkSupervisionTimeoutChanged => CONN_HANDLE,
            EnhancedFlushComplete => CONN_HANDLE,
            UserPasskeyNotification => OTHER,
            KeypressNotification => OTHER,
            RemoteHostSupportedFeaturesNotification => OTHER,
            NumberOfCompletedDataBlocks => OTHER,

            LeMetaEvent => OTHER,
            LeConnectionComplete => STATUS.union(CONN_HANDLE),
            LeAdvertisingReport => OTHER,
            LeConnectionUpdateComplete => STATUS.union(CONN_HANDLE),
            LeReadRemoteFeaturesComplete => STATUS.union(CONN_HANDLE),
            LeLongTermKeyRequest => CONN_HANDLE,
            LeRemoteConnectionParameterRequest => CONN_HANDLE,
            LeDataLengthChange => CONN_HANDLE,
            LeReadLocalP256PublicKeyComplete => STATUS,
            LeGenerateDhKeyComplete => STATUS,
            LeEnhancedConnectionComplete => STATUS.union(CONN_HANDLE),
            LeDirectedAdvertisingReport => OTHER,
            LePhyUpdateComplete => STATUS.union(CONN_HANDLE),
            LeExtendedAdvertisingReport => OTHER,
            LePeriodicAdvertisingSyncEstablished => STATUS.union(SYNC_HANDLE),
            LePeriodicAdvertisingReport => SYNC_HANDLE,
            LePeriodicAdvertisingSyncLost => SYNC_HANDLE,
            LeScanTimeout => OTHER,
            LeAdvertisingSetTerminated => STATUS.union(ADV_HANDLE),
            LeScanRequestReceived => ADV_HANDLE,
            LeChannelSelectionAlgorithm => CONN_HANDLE,
            LeConnectionlessIqReport => SYNC_HANDLE,
            LeConnectionIqReport => CONN_HANDLE,
            LeCteRequestFailed => STATUS.union(CONN_HANDLE),
            LePeriodicAdvertisingSyncTransferReceived => STATUS.union(CONN_HANDLE),
            LeCisEstablished => STATUS.union(CONN_HANDLE),
            LeCisRequest => CONN_HANDLE,
            LeCreateBigComplete => STATUS.union(BIG_HANDLE),
            LeTerminateBigComplete => BIG_HANDLE,
            LeBigSyncEstablished => STATUS.union(BIG_HANDLE),
            LeBigSyncLost => BIG_HANDLE,
            LeRequestPeerScaComplete => STATUS.union(CONN_HANDLE),
            LePathLossThreshold => CONN_HANDLE,
            LeTransmitPowerReporting => STATUS.union(CONN_HANDLE),
            LeBigInfoAdvertisingReport => SYNC_HANDLE,
            LeSubrateChange => STATUS.union(CONN_HANDLE),

            TriggeredClockCapture => CONN_HANDLE,
            SynchronizationTrainComplete => STATUS,
            SynchronizationTrainReceived => STATUS,
            ConnectionlessPeripheralBroadcastReceive => OTHER,
            ConnectionlessPeripheralBroadcastTimeout => OTHER,
            TruncatedPageComplete => STATUS,
            PeripheralPageResponseTimeout => OTHER,
            ConnectionlessPeripheralBroadcastChannelMapChange => OTHER,
            InquiryResponseNotification => OTHER,
            AuthenticatedPayloadTimeoutExpired => CONN_HANDLE,
            SamStatusChange => CONN_HANDLE,
            Vendor => OTHER,
        }
    }

    /// Sets or clears the associated event mask bit.
    #[allow(clippy::too_many_lines)]
    pub(super) fn set(self, m: &mut super::EventMask, enable: bool) {
        use EventCode::*;
        let (p1, p2, le) = (&mut m.p1, &mut m.p2, &mut m.le);
        let (pg, bit): (&mut u64, u8) = match self {
            // Page 1 ([Vol 4] Part E, Section 7.3.1)
            InquiryComplete => (p1, 0),
            InquiryResult => (p1, 1),
            ConnectionComplete => (p1, 2),
            ConnectionRequest => (p1, 3),
            DisconnectionComplete => (p1, 4),
            AuthenticationComplete => (p1, 5),
            RemoteNameRequestComplete => (p1, 6),
            EncryptionChangeV1 => (p1, 7),
            ChangeConnectionLinkKeyComplete => (p1, 8),
            LinkKeyTypeChanged => (p1, 9),
            ReadRemoteSupportedFeaturesComplete => (p1, 10),
            ReadRemoteVersionInformationComplete => (p1, 11),
            QosSetupComplete => (p1, 12),
            HardwareError => (p1, 15),
            FlushOccurred => (p1, 16),
            RoleChange => (p1, 17),
            ModeChange => (p1, 19),
            ReturnLinkKeys => (p1, 20),
            PinCodeRequest => (p1, 21),
            LinkKeyRequest => (p1, 22),
            LinkKeyNotification => (p1, 23),
            LoopbackCommand => (p1, 24),
            DataBufferOverflow => (p1, 25),
            MaxSlotsChange => (p1, 26),
            ReadClockOffsetComplete => (p1, 27),
            ConnectionPacketTypeChanged => (p1, 28),
            QosViolation => (p1, 29),
            PageScanRepetitionModeChange => (p1, 31),
            FlowSpecificationComplete => (p1, 32),
            InquiryResultWithRssi => (p1, 33),
            ReadRemoteExtendedFeaturesComplete => (p1, 34),
            SynchronousConnectionComplete => (p1, 43),
            SynchronousConnectionChanged => (p1, 44),
            SniffSubrating => (p1, 45),
            ExtendedInquiryResult => (p1, 46),
            EncryptionKeyRefreshComplete => (p1, 47),
            IoCapabilityRequest => (p1, 48),
            IoCapabilityResponse => (p1, 49),
            UserConfirmationRequest => (p1, 50),
            UserPasskeyRequest => (p1, 51),
            RemoteOobDataRequest => (p1, 52),
            SimplePairingComplete => (p1, 53),
            LinkSupervisionTimeoutChanged => (p1, 55),
            EnhancedFlushComplete => (p1, 56),
            UserPasskeyNotification => (p1, 58),
            KeypressNotification => (p1, 59),
            RemoteHostSupportedFeaturesNotification => (p1, 60),
            LeMetaEvent => (p1, 61),

            // LE ([Vol 4] Part E, Section 7.8.1)
            LeConnectionComplete => (le, 0),
            LeAdvertisingReport => (le, 1),
            LeConnectionUpdateComplete => (le, 2),
            LeReadRemoteFeaturesComplete => (le, 3),
            LeLongTermKeyRequest => (le, 4),
            LeRemoteConnectionParameterRequest => (le, 5),
            LeDataLengthChange => (le, 6),
            LeReadLocalP256PublicKeyComplete => (le, 7),
            LeGenerateDhKeyComplete => (le, 8),
            LeEnhancedConnectionComplete => (le, 9),
            LeDirectedAdvertisingReport => (le, 10),
            LePhyUpdateComplete => (le, 11),
            LeExtendedAdvertisingReport => (le, 12),
            LePeriodicAdvertisingSyncEstablished => (le, 13),
            LePeriodicAdvertisingReport => (le, 14),
            LePeriodicAdvertisingSyncLost => (le, 15),
            LeScanTimeout => (le, 16),
            LeAdvertisingSetTerminated => (le, 17),
            LeScanRequestReceived => (le, 18),
            LeChannelSelectionAlgorithm => (le, 19),
            LeConnectionlessIqReport => (le, 20),
            LeConnectionIqReport => (le, 21),
            LeCteRequestFailed => (le, 22),
            LePeriodicAdvertisingSyncTransferReceived => (le, 23),
            LeCisEstablished => (le, 24),
            LeCisRequest => (le, 25),
            LeCreateBigComplete => (le, 26),
            LeTerminateBigComplete => (le, 27),
            LeBigSyncEstablished => (le, 28),
            LeBigSyncLost => (le, 29),
            LeRequestPeerScaComplete => (le, 30),
            LePathLossThreshold => (le, 31),
            LeTransmitPowerReporting => (le, 32),
            LeBigInfoAdvertisingReport => (le, 33),
            LeSubrateChange => (le, 34),

            // Page 2 ([Vol 4] Part E, Section 7.3.69)
            NumberOfCompletedDataBlocks => (p2, 8),
            TriggeredClockCapture => (p2, 14),
            SynchronizationTrainComplete => (p2, 15),
            SynchronizationTrainReceived => (p2, 16),
            ConnectionlessPeripheralBroadcastReceive => (p2, 17),
            ConnectionlessPeripheralBroadcastTimeout => (p2, 18),
            TruncatedPageComplete => (p2, 19),
            PeripheralPageResponseTimeout => (p2, 20),
            ConnectionlessPeripheralBroadcastChannelMapChange => (p2, 21),
            InquiryResponseNotification => (p2, 22),
            AuthenticatedPayloadTimeoutExpired => (p2, 23),
            SamStatusChange => (p2, 24),
            EncryptionChangeV2 => (p2, 25),

            // Unmaskable events
            CommandComplete | CommandStatus | NumberOfCompletedPackets | Vendor => return,
        };
        *pg = *pg & !(1 << bit) | u64::from(enable) << bit;
    }
}

bitflags! {
    /// Event parameter format.
    #[repr(transparent)]
    #[must_use]
    pub(super) struct EventFmt: u8 {
        /// Event contains a status parameter.
        const STATUS = 1 << 0;
        /// Event contains a connection handle.
        const CONN_HANDLE = 1 << 1;
        /// Event contains a periodic advertising handle.
        const SYNC_HANDLE = 1 << 2;
        /// Event contains an advertising handle.
        const ADV_HANDLE = 1 << 3;
        /// Event contains a BIG handle.
        const BIG_HANDLE = 1 << 4;
        /// Handle type mask ([Vol 4] Part E, Section 5.3.1)
        const HANDLE = Self::CONN_HANDLE.bits | Self::SYNC_HANDLE.bits |
                       Self::ADV_HANDLE.bits | Self::BIG_HANDLE.bits;
    }
}

/// HCI status codes ([Vol 1] Part F, Section 1.3).
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, num_enum::FromPrimitive, strum::Display, thiserror::Error,
)]
#[non_exhaustive]
#[repr(u8)]
pub enum Status {
    Success = 0x00,
    UnknownCommand = 0x01,
    UnknownConnectionIdentifier = 0x02,
    HardwareFailure = 0x03,
    PageTimeout = 0x04,
    AuthenticationFailure = 0x05,
    PinOrKeyMissing = 0x06,
    MemoryCapacityExceeded = 0x07,
    ConnectionTimeout = 0x08,
    ConnectionLimitExceeded = 0x09,
    SynchronousConnectionLimitToADeviceExceeded = 0x0A,
    ConnectionAlreadyExists = 0x0B,
    CommandDisallowed = 0x0C,
    ConnectionRejectedDueToLimitedResources = 0x0D,
    ConnectionRejectedDueToSecurityReasons = 0x0E,
    ConnectionRejectedDueToUnacceptableBdAddr = 0x0F,
    ConnectionAcceptTimeoutExceeded = 0x10,
    UnsupportedFeatureOrParameterValue = 0x11,
    InvalidCommandParameters = 0x12,
    RemoteUserTerminatedConnection = 0x13,
    RemoteDeviceTerminatedConnectionDueToLowResources = 0x14,
    RemoteDeviceTerminatedConnectionDueToPowerOff = 0x15,
    ConnectionTerminatedByLocalHost = 0x16,
    RepeatedAttempts = 0x17,
    PairingNotAllowed = 0x18,
    UnknownLmpPdu = 0x19,
    UnsupportedRemoteFeature = 0x1A,
    ScoOffsetRejected = 0x1B,
    ScoIntervalRejected = 0x1C,
    ScoAirModeRejected = 0x1D,
    InvalidLmpLlParameters = 0x1E,
    #[num_enum(default)] // [Vol 4] Part E, Section 1.2
    UnspecifiedError = 0x1F,
    UnsupportedLmpLlParameterValue = 0x20,
    RoleChangeNotAllowed = 0x21,
    LmpLlResponseTimeout = 0x22,
    LmpLlErrorTransactionCollision = 0x23,
    LmpPduNotAllowed = 0x24,
    EncryptionModeNotAcceptable = 0x25,
    LinkKeyCannotBeChanged = 0x26,
    RequestedQosNotSupported = 0x27,
    InstantPassed = 0x28,
    PairingWithUnitKeyNotSupported = 0x29,
    DifferentTransactionCollision = 0x2A,
    QosUnacceptableParameter = 0x2C,
    QosRejected = 0x2D,
    ChannelClassificationNotSupported = 0x2E,
    InsufficientSecurity = 0x2F,
    ParameterOutOfMandatoryRange = 0x30,
    RoleSwitchPending = 0x32,
    ReservedSlotViolation = 0x34,
    RoleSwitchFailed = 0x35,
    ExtendedInquiryResponseTooLarge = 0x36,
    SecureSimplePairingNotSupportedByHost = 0x37,
    HostBusyPairing = 0x38,
    ConnectionRejectedDueToNoSuitableChannelFound = 0x39,
    ControllerBusy = 0x3A,
    UnacceptableConnectionParameters = 0x3B,
    AdvertisingTimeout = 0x3C,
    ConnectionTerminatedDueToMicFailure = 0x3D,
    ConnectionFailedToBeEstablished = 0x3E,
    CoarseClockAdjustmentRejected = 0x40,
    Type0SubmapNotDefined = 0x41,
    UnknownAdvertisingIdentifier = 0x42,
    LimitReached = 0x43,
    OperationCancelledByHost = 0x44,
    PacketTooLong = 0x45,
}

impl Status {
    /// Returns whether status is `Success`.
    #[inline(always)]
    #[must_use]
    pub const fn is_ok(self) -> bool {
        matches!(self, Status::Success)
    }
}

/// Device connection role ([Vol 4] Part E, Sections 7.7.65.1 and 7.7.65.10).
#[allow(clippy::exhaustive_enums)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum Role {
    /// Device is acting as Central.
    Central = 0x00,
    /// Device is acting as Peripheral.
    Peripheral = 0x01,
}

bitflags! {
    /// Basic properties of an advertising event
    /// ([Vol 4] Part E, Section 7.8.53).
    #[derive(Default)]
    #[repr(transparent)]
    pub struct AdvProp: u16 {
        const CONNECTABLE = 1 << 0;
        const SCANNABLE = 1 << 1;
        const DIRECTED = 1 << 2;
        const HIGH_DUTY_CYCLE = 1 << 3;
        const LEGACY = 1 << 4;
        const ANONYMOUS = 1 << 5;
        const INCLUDE_TX_POWER = 1 << 6;
    }
}

bitflags! {
    /// Channels used for transmitting advertising packets
    /// ([Vol 4] Part E, Section 7.8.53).
    #[repr(transparent)]
    pub struct AdvChanMap: u8 {
        const CH37 = 1 << 0;
        const CH38 = 1 << 1;
        const CH39 = 1 << 2;
    }
}

impl Default for AdvChanMap {
    #[inline]
    fn default() -> Self {
        Self::all()
    }
}

/// Type of address being used in an advertising packet
/// ([Vol 4] Part E, Section 7.8.53).
#[allow(clippy::exhaustive_enums)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, num_enum::IntoPrimitive)]
#[repr(u8)]
pub enum AdvAddrType {
    /// Public Device Address.
    #[default]
    Public = 0x00,
    /// Random Device Address
    Random = 0x01,
    /// Controller generates the Resolvable Private Address based on the local
    /// IRK from the resolving list. If the resolving list contains no matching
    /// entry, use the public address.
    PrivateOrPublic = 0x02,
    /// Controller generates the Resolvable Private Address based on the local
    /// IRK from the resolving list. If the resolving list contains no matching
    /// entry, use the random address from
    /// `le_set_advertising_set_random_address`.
    PrivateOrRandom = 0x03,
}

/// Type of filtering to perform for scan and connection requests
/// ([Vol 4] Part E, Section 7.8.53).
#[allow(clippy::exhaustive_enums)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, num_enum::IntoPrimitive)]
#[repr(u8)]
pub enum AdvFilterPolicy {
    /// Process scan and connection requests from all devices (i.e., the Filter
    /// Accept List is not in use).
    #[default]
    None = 0x00,
    /// Process connection requests from all devices and scan requests only from
    /// devices that are in the Filter Accept List.
    FilterScan = 0x01,
    /// Process scan requests from all devices and connection requests only from
    /// devices that are in the Filter Accept List.
    FilterConnect = 0x02,
    /// Process scan and connection requests only from devices in the Filter
    /// Accept List.
    FilterAll = 0x03,
}

/// Physical layer for advertising. LE Coded assumes S=8
/// ([Vol 4] Part E, Section 7.8.53).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, num_enum::IntoPrimitive)]
#[non_exhaustive]
#[repr(u8)]
pub enum AdvPhy {
    #[default]
    Le1M = 0x01,
    Le2M = 0x02,
    LeCoded = 0x03,
}

/// Defines the interpretation of advertising data
/// ([Vol 4] Part E, Section 7.8.54).
#[allow(clippy::exhaustive_enums)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, num_enum::IntoPrimitive)]
#[repr(u8)]
pub enum AdvDataOp {
    /// Intermediate fragment of fragmented extended advertising data.
    Cont = 0x00,
    /// First fragment of fragmented extended advertising data.
    First = 0x01,
    /// Last fragment of fragmented extended advertising data.
    Last = 0x02,
    /// Complete extended advertising data.
    Complete = 0x03,
    /// Unchanged data (just update the Advertising DID).
    Unchanged = 0x04,
}

/// Bluetooth Core Specification versions ([Assigned Numbers] Section 2.1).
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
    strum::Display,
)]
#[non_exhaustive]
#[repr(u8)]
pub enum CoreVersion {
    V1_0b = 0x00,
    V1_1 = 0x01,
    V1_2 = 0x02,
    V2_0 = 0x03,
    V2_1 = 0x04,
    V3_0 = 0x05,
    V4_0 = 0x06,
    V4_1 = 0x07,
    V4_2 = 0x08,
    V5_0 = 0x09,
    V5_1 = 0x0A,
    V5_2 = 0x0B,
    V5_3 = 0x0C,
    #[default]
    Unknown = 0xFF,
}
