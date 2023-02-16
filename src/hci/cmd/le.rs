use burble_crypto::LTK;

use crate::hci::*;
use crate::le::{Addr, RawAddr, TxPower};

// LE Controller commands ([Vol 4] Part E, Section 7.8).
impl Host {
    /// Configures which LE events can be generated by the controller
    /// ([Vol 4] Part E, Section 7.8.1).
    pub async fn le_set_event_mask(&self, enable: LeEventMask) -> Result<()> {
        let r = self.exec_params(Opcode::LeSetEventMask, |cmd| {
            cmd.u64(enable.0);
        });
        r.await?.into()
    }

    /// Returns the controller's packet size and count limits. ISO limits will
    /// be missing if the controller does not support v2 of this command
    /// ([Vol 4] Part E, Section 7.8.2).
    pub async fn le_read_buffer_size(&self) -> Result<LeBufferSize> {
        // TODO: Use supported features to determine which version to use?
        {
            let r = self.exec(Opcode::LeReadBufferSizeV2).await?;
            if r.status() != Status::UnknownCommand {
                return r.into();
            }
        }
        self.exec(Opcode::LeReadBufferSize).await?.into()
    }

    /// Replies to an `HCI_LE_Long_Term_Key_Request` event from the controller,
    /// specifying the Long Term Key for the connection, if one is available
    /// ([Vol 4] Part E, Section 7.8.25 and 7.8.26).
    pub async fn le_long_term_key_request_reply(
        &self,
        cn: ConnHandle,
        k: Option<&LTK>,
    ) -> Result<()> {
        let r = if let Some(k) = k {
            let r = self.exec_params(Opcode::LeLongTermKeyRequestReply, |cmd| {
                cmd.u16(cn).u128(k);
            });
            r.await?
        } else {
            let r = self.exec_params(Opcode::LeLongTermKeyRequestNegativeReply, |cmd| {
                cmd.u16(cn);
            });
            r.await?
        };
        assert_eq!(ConnHandle::new(r.get().u16()), Some(cn));
        Ok(())
    }

    /// Sets the random device address for an advertising set
    /// ([Vol 4] Part E, Section 7.8.4).
    pub async fn le_set_advertising_set_random_address(
        &self,
        h: AdvHandle,
        a: RawAddr,
    ) -> Result<()> {
        let r = self.exec_params(Opcode::LeSetAdvertisingSetRandomAddress, |cmd| {
            cmd.u8(h).put(a);
        });
        r.await?.into()
    }

    /// Sets advertising parameters ([Vol 4] Part E, Section 7.8.53).
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
                .put(p.peer_addr.raw())
                .u8(p.filter_policy)
                .i8(p.tx_power.map_or(TxPower::NONE, i8::from))
                .u8(p.pri_phy)
                .u8(p.sec_max_skip)
                .u8(p.sec_phy)
                .u8(p.sid)
                .bool(p.scan_request_notify);
        });
        r.await?.into()
    }

    /// Sets the data used in advertising PDUs that have a data field
    /// ([Vol 4] Part E, Section 7.8.54).
    pub async fn le_set_extended_advertising_data(
        &self,
        h: AdvHandle,
        op: AdvDataOp,
        dont_frag: bool,
        data: &[u8],
    ) -> Result<()> {
        let r = self.exec_params(Opcode::LeSetExtendedAdvertisingData, |cmd| {
            cmd.u8(h).u8(op).bool(dont_frag);
            cmd.u8(u8::try_from(data.len()).unwrap()).put(data);
        });
        r.await?.into()
    }

    /// Sets the data used in scan response PDUs
    /// ([Vol 4] Part E, Section 7.8.55).
    pub async fn le_set_extended_scan_response_data(
        &self,
        h: AdvHandle,
        op: AdvDataOp,
        dont_frag: bool,
        data: &[u8],
    ) -> Result<()> {
        let r = self.exec_params(Opcode::LeSetExtendedScanResponseData, |cmd| {
            cmd.u8(h).u8(op).bool(dont_frag);
            cmd.u8(u8::try_from(data.len()).unwrap()).put(data);
        });
        r.await?.into()
    }

    /// Enables or disables one or more advertising sets
    /// ([Vol 4] Part E, Section 7.8.56).
    pub async fn le_set_extended_advertising_enable(
        &self,
        enable: bool,
        cfg: &[AdvEnableParams],
    ) -> Result<()> {
        let r = self.exec_params(Opcode::LeSetExtendedAdvertisingEnable, |cmd| {
            cmd.bool(enable);
            cmd.u8(u8::try_from(cfg.len()).unwrap());
            for c in cfg {
                cmd.u8(c.handle);
                cmd.u16(ticks_10ms(c.duration).expect("invalid advertising duration"));
                cmd.u8(c.max_events);
            }
        });
        r.await?.into()
    }

    /// Returns the maximum length of advertisement or scan response data
    /// supported by the controller ([Vol 4] Part E, Section 7.8.57).
    pub async fn le_read_maximum_advertising_data_length(&self) -> Result<usize> {
        let r = self.exec(Opcode::LeReadMaximumAdvertisingDataLength);
        Ok(usize::from(r.await?.ok()?.u16()))
    }

    /// Returns the maximum number of advertising sets supported by the
    /// controller at this time ([Vol 4] Part E, Section 7.8.58). This value is
    /// dynamic.
    pub async fn le_read_number_of_supported_advertising_sets(&self) -> Result<u8> {
        let r = self.exec(Opcode::LeReadNumberOfSupportedAdvertisingSets);
        Ok(r.await?.ok()?.u8())
    }

    /// Removes an advertising set from the controller
    /// ([Vol 4] Part E, Section 7.8.59).
    pub async fn le_remove_advertising_set(&self, h: AdvHandle) -> Result<()> {
        let r = self.exec_params(Opcode::LeRemoveAdvertisingSet, |cmd| {
            cmd.u8(h);
        });
        r.await?.into()
    }

    /// Removes all advertising sets from the controller
    /// ([Vol 4] Part E, Section 7.8.60).
    pub async fn le_clear_advertising_sets(&self) -> Result<()> {
        self.exec(Opcode::LeClearAdvertisingSets).await?.into()
    }

    /// Sets the parameters for periodic advertising
    /// ([Vol 4] Part E, Section 7.8.61).
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

    /// Sets the data used in periodic advertising PDUs
    /// ([Vol 4] Part E, Section 7.8.62).
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
                .put(data);
        });
        r.await?.into()
    }

    /// Enables or disables periodic advertising
    /// ([Vol 4] Part E, Section 7.8.63).
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

/// `HCI_LE_Set_Event_Mask` command parameters ([Vol 4] Part E, Section 7.8.1).
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct LeEventMask(u64);

impl LeEventMask {
    /// Creates an event mask from an iterator of events to enable. Unsupported
    /// events are ignored.
    #[inline]
    #[must_use]
    pub fn enable(events: impl Iterator<Item = SubeventCode>) -> Self {
        let mut mask = 0;
        for v in events {
            mask |= v.mask();
        }
        Self(mask)
    }
}

/// `HCI_LE_Read_Buffer_Size` return parameters ([Vol 4] Part E, Section 7.8.2).
#[derive(Clone, Copy, Debug, Default)]
pub struct LeBufferSize {
    pub acl_data_len: u16,
    pub acl_num_pkts: u8,
    pub iso_data_len: u16,
    pub iso_num_pkts: u8,
}

impl From<&mut Event<'_>> for LeBufferSize {
    fn from(e: &mut Event) -> Self {
        let v2 = e.opcode() == Opcode::LeReadBufferSizeV2;
        Self {
            acl_data_len: e.u16(),
            acl_num_pkts: e.u8(),
            iso_data_len: v2.then(|| e.u16()).unwrap_or_default(),
            iso_num_pkts: v2.then(|| e.u8()).unwrap_or_default(),
        }
    }
}

/// `HCI_LE_Set_Extended_Advertising_Parameters` command parameters
/// ([Vol 4] Part E, Section 7.8.53).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AdvParams {
    pub props: AdvProp,
    pub pri_interval: (Duration, Duration),
    pub pri_chan_map: AdvChanMap,
    pub addr_type: AdvAddrType,
    pub peer_addr: Addr,
    pub filter_policy: AdvFilterPolicy,
    pub tx_power: Option<TxPower>,
    pub pri_phy: AdvPhy,
    pub sec_max_skip: u8,
    pub sec_phy: AdvPhy,
    pub sid: u8,
    pub scan_request_notify: bool,
}

/// `HCI_LE_Set_Extended_Advertising_Enable` command parameters
/// ([Vol 4] Part E, Section 7.8.56).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdvEnableParams {
    pub handle: AdvHandle,
    pub duration: Duration,
    pub max_events: u8,
}

impl From<AdvHandle> for AdvEnableParams {
    #[inline]
    fn from(h: AdvHandle) -> Self {
        Self {
            handle: h,
            duration: Duration::default(),
            max_events: 0,
        }
    }
}
