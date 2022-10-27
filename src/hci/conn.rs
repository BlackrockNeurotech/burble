use std::u16;

use crate::dev::{Addr, RawAddr};

use super::*;

/// Connection handle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct ConnHandle(u16);

impl ConnHandle {
    /// Returns a new connection handle.
    #[inline]
    #[must_use]
    pub(super) const fn new(h: u16) -> Self {
        Self(h)
    }

    /// Returns an invalid connection handle.
    #[inline]
    #[must_use]
    pub const fn invalid() -> Self {
        Self(0xFFFF)
    }

    /// Returns whether the connection handle is valid.
    #[inline]
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 >> 12 == 0 // [Vol 4] Part E, Section 5.4.2
    }
}

impl Default for ConnHandle {
    #[inline]
    fn default() -> Self {
        Self::invalid()
    }
}

impl From<ConnHandle> for u16 {
    #[inline]
    fn from(h: ConnHandle) -> Self {
        h.0
    }
}

/// `HCI_LE_Connection_Complete` and `HCI_LE_Enhanced_Connection_Complete` event
/// parameters.
#[derive(Clone, Debug)]
pub struct LeConnectionComplete {
    pub status: Status,
    pub handle: ConnHandle,
    pub role: Role,
    pub peer_addr: Addr,
    pub local_rpa: RawAddr,
    pub peer_rpa: RawAddr,
    pub conn_interval: Duration,
    pub peripheral_latency: u16,
    pub supervision_timeout: Duration,
    pub central_clock_accuracy: u16,
}

#[allow(clippy::fallible_impl_from)]
impl From<&mut Event<'_>> for LeConnectionComplete {
    fn from(e: &mut Event<'_>) -> Self {
        let role = Role::try_from(e.u8()).expect("invalid role");
        let peer_addr = Addr::peer(e.u8(), e.addr());
        let (local_rpa, peer_rpa) = match e.typ() {
            EventType::Le(SubeventCode::ConnectionComplete) => Default::default(),
            EventType::Le(SubeventCode::EnhancedConnectionComplete) => (e.addr(), e.addr()),
            t => panic!("Invalid event type: {t}"),
        };
        Self {
            status: e.status(),
            handle: e.conn_handle(),
            role,
            peer_addr,
            local_rpa,
            peer_rpa,
            conn_interval: duration_1250us(e.u16()),
            peripheral_latency: e.u16(),
            supervision_timeout: duration_10ms(e.u16()),
            central_clock_accuracy: match e.u8() {
                0x00 => 500,
                0x01 => 250,
                0x02 => 150,
                0x03 => 100,
                0x04 => 75,
                0x05 => 50,
                0x06 => 30,
                0x07 => 20,
                _ => 0,
            },
        }
    }
}
