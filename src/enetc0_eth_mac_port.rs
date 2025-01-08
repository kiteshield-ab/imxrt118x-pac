#[doc = "Ethernet MAC port"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enetc0EthMacPort {
    ptr: *mut u8,
}
unsafe impl Send for Enetc0EthMacPort {}
unsafe impl Sync for Enetc0EthMacPort {}
impl Enetc0EthMacPort {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port MAC 0 Command and Configuration Register"]
    #[inline(always)]
    pub const fn pm0_command_config(
        self,
    ) -> crate::common::Reg<regs::Pm0CommandConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Port MAC 0 MAC Address Register 0"]
    #[inline(always)]
    pub const fn pm0_mac_addr_0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Port MAC 0 MAC Address Register 1"]
    #[inline(always)]
    pub const fn pm0_mac_addr_1(self) -> crate::common::Reg<regs::Pm0MacAddr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Port MAC 0 Maximum Frame Length Register"]
    #[inline(always)]
    pub const fn pm0_maxfrm(self) -> crate::common::Reg<regs::Pm0Maxfrm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Port MAC 0 Minimum Frame Length Register"]
    #[inline(always)]
    pub const fn pm0_minfrm(self) -> crate::common::Reg<regs::Pm0Minfrm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Port MAC 0 Internal MDIO Configuration Register"]
    #[inline(always)]
    pub const fn pm0_mdio_cfg(self) -> crate::common::Reg<regs::Pm0MdioCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Port MAC 0 Internal MDIO Interface Control Register"]
    #[inline(always)]
    pub const fn pm0_mdio_ctl(self) -> crate::common::Reg<regs::Pm0MdioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Port MAC 0 Internal MDIO Interface Data Register"]
    #[inline(always)]
    pub const fn pm0_mdio_data(self) -> crate::common::Reg<regs::Pm0MdioData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Port MAC 0 Interrupt Event Register"]
    #[inline(always)]
    pub const fn pm0_ievent(self) -> crate::common::Reg<regs::Pm0Ievent, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Inter-Packet Gap Length and Flexible Preamble length Register"]
    #[inline(always)]
    pub const fn pm0_tx_ipg_preamble(
        self,
    ) -> crate::common::Reg<regs::Pm0TxIpgPreamble, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Port MAC 0 Interrupt Mask Register(INT_MASK)"]
    #[inline(always)]
    pub const fn pm0_imask(self) -> crate::common::Reg<regs::Pm0Imask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Port MAC 0 Pause Quanta Register"]
    #[inline(always)]
    pub const fn pm0_pause_quanta(
        self,
    ) -> crate::common::Reg<regs::Pm0PauseQuanta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Port MAC 0 Pause Quanta Threshold Register"]
    #[inline(always)]
    pub const fn pm0_pause_thresh(
        self,
    ) -> crate::common::Reg<regs::Pm0PauseThresh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Pause Status Register"]
    #[inline(always)]
    pub const fn pm0_rx_pause_status(
        self,
    ) -> crate::common::Reg<regs::Pm0RxPauseStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "Port MAC 0 EEE Low Power Wakeup Timer Register"]
    #[inline(always)]
    pub const fn pm0_lpwake_timer(
        self,
    ) -> crate::common::Reg<regs::Pm0LpwakeTimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit EEE Low Power Timer Register"]
    #[inline(always)]
    pub const fn pm0_sleep_timer(
        self,
    ) -> crate::common::Reg<regs::Pm0SleepTimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "Port MAC 0 IEEE1588 Single-Step Control Register"]
    #[inline(always)]
    pub const fn pm0_single_step(
        self,
    ) -> crate::common::Reg<regs::Pm0SingleStep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "Port MAC 0 half-duplex backoff entropy register"]
    #[inline(always)]
    pub const fn pm0_hd_backoff_entropy(
        self,
    ) -> crate::common::Reg<regs::Pm0HdBackoffEntropy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "Port MAC 0 Half-Duplex Flow Control Register"]
    #[inline(always)]
    pub const fn pm0_hd_flow_ctrl(
        self,
    ) -> crate::common::Reg<regs::Pm0HdFlowCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "Port MAC 0 Statistics Configuration Register"]
    #[inline(always)]
    pub const fn pm0_statn_config(
        self,
    ) -> crate::common::Reg<regs::Pm0StatnConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Ethernet Octets Counter(etherStatsOctetsn)"]
    #[inline(always)]
    pub const fn pm0_reoctn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Octets Counter(iflnOctetsn)"]
    #[inline(always)]
    pub const fn pm0_roctn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
    #[inline(always)]
    pub const fn pm0_rxpfn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Frame Counter Register(aFramesReceivedOKn)"]
    #[inline(always)]
    pub const fn pm0_rfrmn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Frame Check Sequence Error Counter Register()"]
    #[inline(always)]
    pub const fn pm0_rfcsn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "Port MAC 0 Receive VLAN Frame Counter Register(VLANReceivedOKn)"]
    #[inline(always)]
    pub const fn pm0_rvlann(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Frame Error Counter Register(ifInErrorsn)"]
    #[inline(always)]
    pub const fn pm0_rerrn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Unicast Frame Counter Register(ifInUcastPktsn)"]
    #[inline(always)]
    pub const fn pm0_rucan(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Multicast Frame Counter Register(ifInMulticastPktsn)"]
    #[inline(always)]
    pub const fn pm0_rmcan(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Broadcast Frame Counter Register(ifInBroadcastPktsn)"]
    #[inline(always)]
    pub const fn pm0_rbcan(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Dropped Packets Counter Register(etherStatsDropEventsn)"]
    #[inline(always)]
    pub const fn pm0_rdrpn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Packets Counter Register(etherStatsPktsn)"]
    #[inline(always)]
    pub const fn pm0_rpktn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
    #[inline(always)]
    pub const fn pm0_rundn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "Port MAC 0 Receive 64-Octet Packet Counter Register(etherStatsPkts64OctetsN)"]
    #[inline(always)]
    pub const fn pm0_r64n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[doc = "Port MAC 0 Receive 65 to 127-Octet Packet Counter Register(etherStatsPkts65to127OctetsN)"]
    #[inline(always)]
    pub const fn pm0_r127n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[doc = "Port MAC 0 Receive 128 to 255-Octet Packet Counter Register(etherStatsPkts128to255OctetsN)"]
    #[inline(always)]
    pub const fn pm0_r255n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Port MAC 0 Receive 256 to 511-Octet Packet Counter Register(etherStatsPkts256to511OctetsN)"]
    #[inline(always)]
    pub const fn pm0_r511n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "Port MAC 0 Receive 512 to 1023-Octet Packet Counter Register(etherStatsPkts512to1023OctetsN)"]
    #[inline(always)]
    pub const fn pm0_r1023n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "Port MAC 0 Receive 1024 to 1522-Octet Packet Counter Register(etherStatsPkts1024to1522OctetsN)"]
    #[inline(always)]
    pub const fn pm0_r1522n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
    }
    #[doc = "Port MAC 0 Receive 1523 to Max-Octet Packet Counter Register(etherStatsPkts1523toMaxOctetsN)"]
    #[inline(always)]
    pub const fn pm0_r1523xn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Oversized Packet Counter Register(etherStatsOversizePktsn)"]
    #[inline(always)]
    pub const fn pm0_rovrn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Jabber Packet Counter Register(etherStatsJabbersn)"]
    #[inline(always)]
    pub const fn pm0_rjbrn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b0usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Fragment Packet Counter Register(etherStatsFragmentsn"]
    #[inline(always)]
    pub const fn pm0_rfrgn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b8usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Control Packet Counter Register"]
    #[inline(always)]
    pub const fn pm0_rcnpn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Dropped Not Truncated Packets Counter Register(etherStatsDropEventsn)"]
    #[inline(always)]
    pub const fn pm0_rdrntpn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c8usize) as _) }
    }
    #[doc = "Port MAC 0 Receive Valid Small Packet Counter Register"]
    #[inline(always)]
    pub const fn pm0_rmin63n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Ethernet Octets Counter(etherStatsOctetsn)"]
    #[inline(always)]
    pub const fn pm0_teoctn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Octets Counter Register(ifOutOctetsn)"]
    #[inline(always)]
    pub const fn pm0_toctn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
    #[inline(always)]
    pub const fn pm0_txpfn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Frame Counter Register(aFramesTransmittedOKn)"]
    #[inline(always)]
    pub const fn pm0_tfrmn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Frame Check Sequence Error Counter Register()"]
    #[inline(always)]
    pub const fn pm0_tfcsn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0228usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit VLAN Frame Counter Register(VLANTransmittedOKn)"]
    #[inline(always)]
    pub const fn pm0_tvlann(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Frame Error Counter Register(ifOutErrorsn)"]
    #[inline(always)]
    pub const fn pm0_terrn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0238usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Unicast Frame Counter Register(ifOutUcastPktsn)"]
    #[inline(always)]
    pub const fn pm0_tucan(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Multicast Frame Counter Register(ifOutMulticastPktsn)"]
    #[inline(always)]
    pub const fn pm0_tmcan(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0248usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Broadcast Frame Counter Register(ifOutBroadcastPktsn)"]
    #[inline(always)]
    pub const fn pm0_tbcan(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0250usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Packets Counter Register(etherStatsPktsn)"]
    #[inline(always)]
    pub const fn pm0_tpktn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0260usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
    #[inline(always)]
    pub const fn pm0_tundn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0268usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit 64-Octet Packet Counter Register (etherStatsPkts64OctetsN)"]
    #[inline(always)]
    pub const fn pm0_t64n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0270usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit 65 to 127-Octet Packet Counter Register (etherStatsPkts65to127OctetsN)"]
    #[inline(always)]
    pub const fn pm0_t127n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0278usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit 128 to 255-Octet Packet Counter Register (etherStatsPkts128to255OctetsN)"]
    #[inline(always)]
    pub const fn pm0_t255n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit 256 to 511-Octet Packet Counter Register (etherStatsPkts256to511OctetsN)"]
    #[inline(always)]
    pub const fn pm0_t511n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0288usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit 512 to 1023-Octet Packet Counter Register (etherStatsPkts512to1023OctetsN)"]
    #[inline(always)]
    pub const fn pm0_t1023n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0290usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit 1024 to 1522-Octet Packet Counter Register (etherStatsPkts1024to1522OctetsN)"]
    #[inline(always)]
    pub const fn pm0_t1522n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0298usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit 1523 to TX_MTU-Octet Packet Counter Register (etherStatsPkts1523toMaxOctetsN)"]
    #[inline(always)]
    pub const fn pm0_t1523xn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a0usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Control Packet Counter Register"]
    #[inline(always)]
    pub const fn pm0_tcnpn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c0usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Deferred Packet Counter Register(aFramesWithDeferredXmissions)"]
    #[inline(always)]
    pub const fn pm0_tdfrn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02d0usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Multiple Collisions Counter Register(aMultipleCollisionFrames)"]
    #[inline(always)]
    pub const fn pm0_tmcoln(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02d8usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Single Collision Counter(aSingleCollisionFrames) Register"]
    #[inline(always)]
    pub const fn pm0_tscoln(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02e0usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Late Collision Counter(aLateCollisions) Register"]
    #[inline(always)]
    pub const fn pm0_tlcoln(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02e8usize) as _) }
    }
    #[doc = "Port MAC 0 Transmit Excessive Collisions Counter Register"]
    #[inline(always)]
    pub const fn pm0_tecoln(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02f0usize) as _) }
    }
    #[doc = "Port MAC 0 Interface Mode Control Register"]
    #[inline(always)]
    pub const fn pm0_if_mode(self) -> crate::common::Reg<regs::Pm0IfMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "Port MAC 1 Command and Configuration Register"]
    #[inline(always)]
    pub const fn pm1_command_config(
        self,
    ) -> crate::common::Reg<regs::Pm1CommandConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[doc = "Port MAC 1 MAC Address Register 0"]
    #[inline(always)]
    pub const fn pm1_mac_addr_0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[doc = "Port MAC 1 MAC Address Register 1"]
    #[inline(always)]
    pub const fn pm1_mac_addr_1(self) -> crate::common::Reg<regs::Pm1MacAddr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "Port MAC 1 Maximum Frame Length Register"]
    #[inline(always)]
    pub const fn pm1_maxfrm(self) -> crate::common::Reg<regs::Pm1Maxfrm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[doc = "Port MAC 1 Minimum Frame Length Register"]
    #[inline(always)]
    pub const fn pm1_minfrm(self) -> crate::common::Reg<regs::Pm1Minfrm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
    }
    #[doc = "Port MAC 1 Interrupt Event Register"]
    #[inline(always)]
    pub const fn pm1_ievent(self) -> crate::common::Reg<regs::Pm1Ievent, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Inter-Packet Gap Length and Flexible Preamble length Register"]
    #[inline(always)]
    pub const fn pm1_tx_ipg_preamble(
        self,
    ) -> crate::common::Reg<regs::Pm1TxIpgPreamble, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0444usize) as _) }
    }
    #[doc = "Port MAC 1 Interrupt Mask Register(INT_MASK)"]
    #[inline(always)]
    pub const fn pm1_imask(self) -> crate::common::Reg<regs::Pm1Imask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x044cusize) as _) }
    }
    #[doc = "Port MAC 1 Pause Quanta Register"]
    #[inline(always)]
    pub const fn pm1_pause_quanta(
        self,
    ) -> crate::common::Reg<regs::Pm1PauseQuanta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0454usize) as _) }
    }
    #[doc = "Port MAC 1 Pause Quanta Threshold Register"]
    #[inline(always)]
    pub const fn pm1_pause_thresh(
        self,
    ) -> crate::common::Reg<regs::Pm1PauseThresh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0464usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Pause Status Register"]
    #[inline(always)]
    pub const fn pm1_rx_pause_status(
        self,
    ) -> crate::common::Reg<regs::Pm1RxPauseStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0474usize) as _) }
    }
    #[doc = "Port MAC 1 EEE Low Power Wakeup Timer Register"]
    #[inline(always)]
    pub const fn pm1_lpwake_timer(
        self,
    ) -> crate::common::Reg<regs::Pm1LpwakeTimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04b8usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit EEE Low Power Timer Register"]
    #[inline(always)]
    pub const fn pm1_sleep_timer(
        self,
    ) -> crate::common::Reg<regs::Pm1SleepTimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04bcusize) as _) }
    }
    #[doc = "Port MAC 1 IEEE1588 Single-Step Control Register"]
    #[inline(always)]
    pub const fn pm1_single_step(
        self,
    ) -> crate::common::Reg<regs::Pm1SingleStep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c0usize) as _) }
    }
    #[doc = "Port MAC 1 half-duplex backoff entropy register"]
    #[inline(always)]
    pub const fn pm1_hd_backoff_entropy(
        self,
    ) -> crate::common::Reg<regs::Pm1HdBackoffEntropy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d0usize) as _) }
    }
    #[doc = "Port MAC 1 Half-Duplex Flow Control Register"]
    #[inline(always)]
    pub const fn pm1_hd_flow_ctrl(
        self,
    ) -> crate::common::Reg<regs::Pm1HdFlowCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d4usize) as _) }
    }
    #[doc = "Port MAC 1 Statistics Configuration Register"]
    #[inline(always)]
    pub const fn pm1_statn_config(
        self,
    ) -> crate::common::Reg<regs::Pm1StatnConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04e0usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Ethernet Octets Counter(etherStatsOctetsn)"]
    #[inline(always)]
    pub const fn pm1_reoctn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Octets Counter(iflnOctetsn)"]
    #[inline(always)]
    pub const fn pm1_roctn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
    #[inline(always)]
    pub const fn pm1_rxpfn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Frame Counter Register(aFramesReceivedOKn)"]
    #[inline(always)]
    pub const fn pm1_rfrmn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Frame Check Sequence Error Counter Register()"]
    #[inline(always)]
    pub const fn pm1_rfcsn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
    }
    #[doc = "Port MAC 1 Receive VLAN Frame Counter Register(VLANReceivedOKn)"]
    #[inline(always)]
    pub const fn pm1_rvlann(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Frame Error Counter Register(ifInErrorsn)"]
    #[inline(always)]
    pub const fn pm1_rerrn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Unicast Frame Counter Register(ifInUcastPktsn)"]
    #[inline(always)]
    pub const fn pm1_rucan(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Multicast Frame Counter Register(ifInMulticastPktsn)"]
    #[inline(always)]
    pub const fn pm1_rmcan(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Broadcast Frame Counter Register(ifInBroadcastPktsn)"]
    #[inline(always)]
    pub const fn pm1_rbcan(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Dropped Packets Counter Register(etherStatsDropEventsn)"]
    #[inline(always)]
    pub const fn pm1_rdrpn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0558usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Packets Counter Register(etherStatsPktsn)"]
    #[inline(always)]
    pub const fn pm1_rpktn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
    #[inline(always)]
    pub const fn pm1_rundn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0568usize) as _) }
    }
    #[doc = "Port MAC 1 Receive 64-Octet Packet Counter Register(etherStatsPkts64OctetsN)"]
    #[inline(always)]
    pub const fn pm1_r64n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0570usize) as _) }
    }
    #[doc = "Port MAC 1 Receive 65 to 127-Octet Packet Counter Register(etherStatsPkts65to127OctetsN)"]
    #[inline(always)]
    pub const fn pm1_r127n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0578usize) as _) }
    }
    #[doc = "Port MAC 1 Receive 128 to 255-Octet Packet Counter Register(etherStatsPkts128to255OctetsN)"]
    #[inline(always)]
    pub const fn pm1_r255n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0580usize) as _) }
    }
    #[doc = "Port MAC 1 Receive 256 to 511-Octet Packet Counter Register(etherStatsPkts256to511OctetsN)"]
    #[inline(always)]
    pub const fn pm1_r511n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
    }
    #[doc = "Port MAC 1 Receive 512 to 1023-Octet Packet Counter Register(etherStatsPkts512to1023OctetsN)"]
    #[inline(always)]
    pub const fn pm1_r1023n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0590usize) as _) }
    }
    #[doc = "Port MAC 1 Receive 1024 to 1522-Octet Packet Counter Register(etherStatsPkts1024to1522OctetsN)"]
    #[inline(always)]
    pub const fn pm1_r1522n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0598usize) as _) }
    }
    #[doc = "Port MAC 1 Receive 1523 to Max-Octet Packet Counter Register(etherStatsPkts1523toMaxOctetsN)"]
    #[inline(always)]
    pub const fn pm1_r1523xn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a0usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Oversized Packet Counter Register(etherStatsOversizePktsn)"]
    #[inline(always)]
    pub const fn pm1_rovrn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a8usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Jabber Packet Counter Register(etherStatsJabbersn)"]
    #[inline(always)]
    pub const fn pm1_rjbrn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b0usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Fragment Packet Counter Register(etherStatsFragmentsn"]
    #[inline(always)]
    pub const fn pm1_rfrgn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b8usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Control Packet Counter Register"]
    #[inline(always)]
    pub const fn pm1_rcnpn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Dropped Not Truncated Packets Counter Register(etherStatsDropEventsn)"]
    #[inline(always)]
    pub const fn pm1_rdrntpn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c8usize) as _) }
    }
    #[doc = "Port MAC 1 Receive Valid Small Packet Counter Register"]
    #[inline(always)]
    pub const fn pm1_rmin63n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d0usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Ethernet Octets Counter(etherStatsOctetsn)"]
    #[inline(always)]
    pub const fn pm1_teoctn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Octets Counter Register(ifOutOctetsn)"]
    #[inline(always)]
    pub const fn pm1_toctn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0608usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
    #[inline(always)]
    pub const fn pm1_txpfn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0618usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Frame Counter Register(aFramesTransmittedOKn)"]
    #[inline(always)]
    pub const fn pm1_tfrmn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Frame Check Sequence Error Counter Register()"]
    #[inline(always)]
    pub const fn pm1_tfcsn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0628usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit VLAN Frame Counter Register(VLANTransmittedOKn)"]
    #[inline(always)]
    pub const fn pm1_tvlann(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0630usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Frame Error Counter Register(ifOutErrorsn)"]
    #[inline(always)]
    pub const fn pm1_terrn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0638usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Unicast Frame Counter Register(ifOutUcastPktsn)"]
    #[inline(always)]
    pub const fn pm1_tucan(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0640usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Multicast Frame Counter Register(ifOutMulticastPktsn)"]
    #[inline(always)]
    pub const fn pm1_tmcan(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0648usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Broadcast Frame Counter Register(ifOutBroadcastPktsn)"]
    #[inline(always)]
    pub const fn pm1_tbcan(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0650usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Packets Counter Register(etherStatsPktsn)"]
    #[inline(always)]
    pub const fn pm1_tpktn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0660usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
    #[inline(always)]
    pub const fn pm1_tundn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0668usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit 64-Octet Packet Counter Register (etherStatsPkts64OctetsN)"]
    #[inline(always)]
    pub const fn pm1_t64n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0670usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit 65 to 127-Octet Packet Counter Register (etherStatsPkts65to127OctetsN)"]
    #[inline(always)]
    pub const fn pm1_t127n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0678usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit 128 to 255-Octet Packet Counter Register (etherStatsPkts128to255OctetsN)"]
    #[inline(always)]
    pub const fn pm1_t255n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0680usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit 256 to 511-Octet Packet Counter Register (etherStatsPkts256to511OctetsN)"]
    #[inline(always)]
    pub const fn pm1_t511n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0688usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit 512 to 1023-Octet Packet Counter Register (etherStatsPkts512to1023OctetsN)"]
    #[inline(always)]
    pub const fn pm1_t1023n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0690usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit 1024 to 1522-Octet Packet Counter Register (etherStatsPkts1024to1522OctetsN)"]
    #[inline(always)]
    pub const fn pm1_t1522n(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0698usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit 1523 to TX_MTU-Octet Packet Counter Register (etherStatsPkts1523toMaxOctetsN)"]
    #[inline(always)]
    pub const fn pm1_t1523xn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a0usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Control Packet Counter Register"]
    #[inline(always)]
    pub const fn pm1_tcnpn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c0usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Deferred Packet Counter Register(aFramesWithDeferredXmissions)"]
    #[inline(always)]
    pub const fn pm1_tdfrn(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06d0usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Multiple Collisions Counter Register(aMultipleCollisionFrames)"]
    #[inline(always)]
    pub const fn pm1_tmcoln(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06d8usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Single Collision Counter(aSingleCollisionFrames) Register"]
    #[inline(always)]
    pub const fn pm1_tscoln(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06e0usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Late Collision Counter(aLateCollisions) Register"]
    #[inline(always)]
    pub const fn pm1_tlcoln(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06e8usize) as _) }
    }
    #[doc = "Port MAC 1 Transmit Excessive Collisions Counter Register"]
    #[inline(always)]
    pub const fn pm1_tecoln(self) -> crate::common::Reg<u64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06f0usize) as _) }
    }
    #[doc = "Port MAC 1 Interface Mode Control Register"]
    #[inline(always)]
    pub const fn pm1_if_mode(self) -> crate::common::Reg<regs::Pm1IfMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize) as _) }
    }
    #[doc = "Port MAC Merge Control and Status Register"]
    #[inline(always)]
    pub const fn mac_merge_mmcsr(
        self,
    ) -> crate::common::Reg<regs::MacMergeMmcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[doc = "Port MAC Merge Frame Assembly Error Count Register"]
    #[inline(always)]
    pub const fn mac_merge_mmfaecr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0808usize) as _) }
    }
    #[doc = "Port MAC Merge Frame SMD Error Count Register"]
    #[inline(always)]
    pub const fn mac_merge_mmfsecr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x080cusize) as _) }
    }
    #[doc = "Port MAC Merge Frame Assembly OK Count Register"]
    #[inline(always)]
    pub const fn mac_merge_mmfaocr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0810usize) as _) }
    }
    #[doc = "Port MAC Merge Fragment Count RX Register"]
    #[inline(always)]
    pub const fn mac_merge_mmfcrxr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0814usize) as _) }
    }
    #[doc = "Port MAC Merge Fragment Count TX Register"]
    #[inline(always)]
    pub const fn mac_merge_mmfctxr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0818usize) as _) }
    }
    #[doc = "Port MAC Merge Hold Count Register"]
    #[inline(always)]
    pub const fn mac_merge_mmhcr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x081cusize) as _) }
    }
    #[doc = "Port external MDIO configuration register"]
    #[inline(always)]
    pub const fn pemdiocr(self) -> crate::common::Reg<regs::Pemdiocr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c00usize) as _) }
    }
    #[doc = "Port external MDIO interface control register"]
    #[inline(always)]
    pub const fn pemdioicr(self) -> crate::common::Reg<regs::Pemdioicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c04usize) as _) }
    }
    #[doc = "Port external MDIO interface data register"]
    #[inline(always)]
    pub const fn pemdioidr(self) -> crate::common::Reg<regs::Pemdioidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c08usize) as _) }
    }
    #[doc = "Port external MDIO register address register"]
    #[inline(always)]
    pub const fn pemdiorar(self) -> crate::common::Reg<regs::Pemdiorar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c0cusize) as _) }
    }
    #[doc = "Port external MDIO status register"]
    #[inline(always)]
    pub const fn pemdiosr(self) -> crate::common::Reg<regs::Pemdiosr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c10usize) as _) }
    }
    #[doc = "PHY status configuration register"]
    #[inline(always)]
    pub const fn ppscr(self) -> crate::common::Reg<regs::Ppscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c20usize) as _) }
    }
    #[doc = "Port PHY status control register"]
    #[inline(always)]
    pub const fn ppsctrlr(self) -> crate::common::Reg<regs::Ppsctrlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c24usize) as _) }
    }
    #[doc = "Port PHY status data register"]
    #[inline(always)]
    pub const fn ppsdr(self) -> crate::common::Reg<regs::Ppsdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c28usize) as _) }
    }
    #[doc = "Port PHY status register address register"]
    #[inline(always)]
    pub const fn ppsrar(self) -> crate::common::Reg<regs::Ppsrar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c2cusize) as _) }
    }
    #[doc = "Port PHY status event register"]
    #[inline(always)]
    pub const fn ppser(self) -> crate::common::Reg<regs::Ppser, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c30usize) as _) }
    }
    #[doc = "Port PHY status mask register"]
    #[inline(always)]
    pub const fn ppsmr(self) -> crate::common::Reg<regs::Ppsmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c34usize) as _) }
    }
}
pub mod regs;
pub mod vals;
