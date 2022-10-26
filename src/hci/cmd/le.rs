use crate::dev::{Addr, RawAddr};
use crate::hci::*;

// LE Controller commands ([Vol 4] Part E, Section 7.8).
impl<T: host::Transport> Host<T> {
    /// Returns the controller's ACL and ISO packet size and count limits. ISO
    /// limits may be missing if the controller does not support v2 of this
    /// command.
    pub async fn le_read_buffer_size(&self) -> Result<LeBufferInfo> {
        // TODO: Use supported features to determine which version to use?
        {
            let r = self.exec(Opcode::LeReadBufferSizeV2).await?;
            if r.status() != Status::UnknownCommand {
                return r.into();
            }
        }
        self.exec(Opcode::LeReadBufferSize).await?.into()
    }

    /// Sets the random device address for an advertising set.
    pub async fn le_set_advertising_set_random_address(
        &self,
        h: AdvHandle,
        a: RawAddr,
    ) -> Result<()> {
        let r = self.exec_params(Opcode::LeSetAdvertisingSetRandomAddress, |cmd| {
            cmd.u8(h).slice(a);
        });
        r.await?.into()
    }

    /// Sets advertising parameters.
    pub async fn le_set_extended_advertising_parameters(
        &self,
        h: AdvHandle,
        p: AdvParams,
    ) -> Result<TxPower> {
        let r = self.exec_params(Opcode::LeSetExtendedAdvertisingParameters, |cmd| {
            cmd.u8(h)
                .u16(p.props.bits())
                .u24(ticks_625us(p.pri_interval.0).unwrap_or(0))
                .u24(ticks_625us(p.pri_interval.1).unwrap_or(0))
                .u8(p.pri_chan_map.bits())
                .u8(p.addr_type)
                .u8(match p.peer_addr {
                    Addr::Public(_) => 0x00,
                    Addr::Random(_) => 0x01,
                })
                .slice(p.peer_addr.raw())
                .u8(p.filter_policy)
                .i8(p.tx_power.map_or(0x7F, |p| p.0))
                .u8(p.pri_phy)
                .u8(p.sec_max_skip)
                .u8(p.sec_phy)
                .u8(p.sid)
                .bool(p.scan_request_notify);
        });
        r.await?.into()
    }

    /// Sets the data used in advertising PDUs that have a data field.
    pub async fn le_set_extended_advertising_data(
        &self,
        h: AdvHandle,
        op: AdvDataOp,
        dont_frag: bool,
        data: &[u8],
    ) -> Result<()> {
        let r = self.exec_params(Opcode::LeSetExtendedAdvertisingData, |cmd| {
            cmd.u8(h).u8(op).bool(dont_frag);
            cmd.u8(u8::try_from(data.len()).unwrap()).slice(data);
        });
        r.await?.into()
    }

    /// Sets the data used in scan response PDUs.
    pub async fn le_set_extended_scan_response_data(
        &self,
        h: AdvHandle,
        op: AdvDataOp,
        dont_frag: bool,
        data: &[u8],
    ) -> Result<()> {
        let r = self.exec_params(Opcode::LeSetExtendedScanResponseData, |cmd| {
            cmd.u8(h).u8(op).bool(dont_frag);
            cmd.u8(u8::try_from(data.len()).unwrap()).slice(data);
        });
        r.await?.into()
    }

    /// Enables or disables one or more advertising sets.
    pub async fn le_set_extended_advertising_enable(
        &self,
        enable: bool,
        cfg: &[AdvEnableParams],
    ) -> Result<()> {
        let r = self.exec_params(Opcode::LeSetExtendedAdvertisingEnable, |cmd| {
            cmd.bool(enable);
            cmd.u8(u8::try_from(cfg.len()).unwrap());
            for c in cfg.iter() {
                cmd.u8(c.handle);
            }
            for c in cfg.iter() {
                cmd.u16(ticks_10ms(c.duration).expect("invalid advertising duration"));
            }
            for c in cfg.iter() {
                cmd.u8(c.max_events);
            }
        });
        r.await?.into()
    }

    /// Returns the maximum length of advertisement or scan response data
    /// supported by the controller.
    pub async fn le_read_maximum_advertising_data_length(&self) -> Result<usize> {
        let r = self.exec(Opcode::LeReadMaximumAdvertisingDataLength);
        Ok(r.await?.ok()?.u16() as _)
    }

    /// Returns the maximum number of advertising sets supported by the
    /// controller at this time. This value is dynamic.
    pub async fn le_read_number_of_supported_advertising_sets(&self) -> Result<u8> {
        let r = self.exec(Opcode::LeReadNumberOfSupportedAdvertisingSets);
        Ok(r.await?.ok()?.u8())
    }

    /// Removes an advertising set from the controller.
    pub async fn le_remove_advertising_set(&self, h: AdvHandle) -> Result<()> {
        let r = self.exec_params(Opcode::LeRemoveAdvertisingSet, |cmd| {
            cmd.u8(h);
        });
        r.await?.into()
    }

    /// Removes all advertising sets from the controller.
    pub async fn le_clear_advertising_sets(&self) -> Result<()> {
        self.exec(Opcode::LeClearAdvertisingSets).await?.into()
    }

    /// Sets the parameters for periodic advertising.
    pub async fn le_set_periodic_advertising_parameters(
        &self,
        h: AdvHandle,
        min: Duration,
        max: Duration,
        p: AdvProp,
    ) -> Result<()> {
        let r = self.exec_params(Opcode::LeSetPeriodicAdvertisingParameters, |cmd| {
            cmd.u8(h)
                .u16(ticks_1250us(min).unwrap_or(0))
                .u16(ticks_1250us(max).unwrap_or(0))
                .u16(p.bits());
        });
        r.await?.into()
    }

    /// Sets the data used in periodic advertising PDUs.
    pub async fn le_set_periodic_advertising_data(
        &self,
        h: AdvHandle,
        op: AdvDataOp,
        data: &[u8],
    ) -> Result<()> {
        let r = self.exec_params(Opcode::LeSetPeriodicAdvertisingData, |cmd| {
            cmd.u8(h)
                .u8(op)
                .u8(u8::try_from(data.len()).unwrap())
                .slice(data);
        });
        r.await?.into()
    }

    /// Enables or disables periodic advertising.
    pub async fn le_set_periodic_advertising_enable(
        &self,
        enable: bool,
        include_adi: bool,
        h: AdvHandle,
    ) -> Result<()> {
        let r = self.exec_params(Opcode::LeSetPeriodicAdvertisingEnable, |cmd| {
            cmd.u8(u8::from(include_adi) << 1 | u8::from(enable)).u8(h);
        });
        r.await?.into()
    }
}

/// `HCI_LE_Read_Buffer_Size [v2]` return parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct LeBufferInfo {
    pub acl_max_len: usize,
    pub acl_max_pkts: usize,
    pub iso_max_len: usize,
    pub iso_max_pkts: usize,
}

impl From<&mut Event<'_>> for LeBufferInfo {
    fn from(e: &mut Event) -> Self {
        if e.opcode() == Opcode::LeReadBufferSize {
            Self {
                acl_max_len: e.u16() as _,
                acl_max_pkts: e.u8() as _,
                ..Self::default()
            }
        } else {
            Self {
                acl_max_len: e.u16() as _,
                acl_max_pkts: e.u8() as _,
                iso_max_len: e.u16() as _,
                iso_max_pkts: e.u8() as _,
            }
        }
    }
}

/// TX power level in dBm.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TxPower(i8);

impl TxPower {
    /// Returns a power level of `v` dBm.
    #[inline]
    #[must_use]
    pub const fn new(v: i8) -> Self {
        Self(v)
    }
}

impl From<&mut Event<'_>> for TxPower {
    #[inline]
    fn from(e: &mut Event<'_>) -> Self {
        #[allow(clippy::cast_possible_wrap)]
        Self(e.u8() as i8)
    }
}
