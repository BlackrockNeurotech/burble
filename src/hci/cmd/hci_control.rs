use crate::hci::*;

/// HCI Control and Baseband commands ([Vol 4] Part E, Section 7.3).
impl Host {
    /// Configures which events can be generated by the controller.
    pub async fn set_event_mask(&self, enable: &EventMask) -> Result<()> {
        let r = self.exec_params(Opcode::SetEventMask, |cmd| {
            cmd.u64(enable.p1);
        });
        r.await?.ok()
    }

    /// Resets the controller's link manager, baseband, and link layer.
    pub async fn reset(&self) -> Result<()> {
        self.exec(Opcode::Reset).await?.ok()
    }

    /// Turns flow control on or off for data sent from the controller to the
    /// host.
    pub async fn set_controller_to_host_flow_control(&self, enable: bool) -> Result<()> {
        let r = self.exec_params(Opcode::SetControllerToHostFlowControl, |cmd| {
            cmd.u8(u8::from(enable));
        });
        r.await?.ok()
    }

    /// Sets the maximum size of the data portion of ACL and SCO data packets
    /// sent from the controller to the host.
    pub async fn host_buffer_size(&self, bs: BufferSize) -> Result<()> {
        let r = self.exec_params(Opcode::HostBufferSize, |cmd| {
            cmd.u16(bs.acl_data_len);
            cmd.u8(0);
            cmd.u16(bs.acl_num_pkts);
            cmd.u16(0_u16);
        });
        r.await?.ok()
    }

    /// Configures which events can be generated by the controller.
    pub async fn set_event_mask_page_2(&self, enable: &EventMask) -> Result<()> {
        let r = self.exec_params(Opcode::SetEventMaskPage2, |cmd| {
            cmd.u64(enable.p2);
        });
        r.await?.ok()
    }

    /// Sets the LE Supported (Host) Link Manager Protocol feature bit.
    pub async fn write_le_host_support(&self, enable: bool) -> Result<()> {
        let r = self.exec_params(Opcode::WriteLeHostSupport, |cmd| {
            cmd.bool(enable).u8(0);
        });
        r.await?.ok()
    }
}

/// `HCI_Set_Event_Mask`, `HCI_Set_Event_Mask_Page_2`, and
/// `HCI_LE_Set_Event_Mask` command parameters
/// ([Vol 4] Part E, Section 7.3.1, 7.3.69, 7.8.1).
#[derive(Clone, Copy, Debug, Default)]
pub struct EventMask {
    pub(in crate::hci) p1: u64,
    pub(in crate::hci) p2: u64,
    pub(in crate::hci) le: u64,
}

impl FromIterator<EventCode> for EventMask {
    /// Creates an event mask from an iterator of events to enable.
    #[must_use]
    fn from_iter<T: IntoIterator<Item = EventCode>>(it: T) -> Self {
        let mut m = Self::default();
        for c in it {
            c.set(&mut m, true);
        }
        m
    }
}
