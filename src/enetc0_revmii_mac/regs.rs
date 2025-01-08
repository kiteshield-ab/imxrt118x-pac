#[doc = "AN Advertisement"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnAdv(pub u16);
impl AnAdv {
    #[doc = "Selector Field"]
    #[must_use]
    #[inline(always)]
    pub const fn selector(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Selector Field"]
    #[inline(always)]
    pub const fn set_selector(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
    }
    #[doc = "Technology Ability Field"]
    #[must_use]
    #[inline(always)]
    pub const fn tech_abil(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x7f;
        val as u8
    }
    #[doc = "Technology Ability Field"]
    #[inline(always)]
    pub const fn set_tech_abil(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 5usize)) | (((val as u16) & 0x7f) << 5usize);
    }
    #[doc = "Extended Next Page"]
    #[must_use]
    #[inline(always)]
    pub const fn xtnd_nxtp(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Next Page"]
    #[inline(always)]
    pub const fn set_xtnd_nxtp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Remote Fault"]
    #[must_use]
    #[inline(always)]
    pub const fn rem_fault(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Fault"]
    #[inline(always)]
    pub const fn set_rem_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Next page"]
    #[must_use]
    #[inline(always)]
    pub const fn next_pg(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Next page"]
    #[inline(always)]
    pub const fn set_next_pg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for AnAdv {
    #[inline(always)]
    fn default() -> AnAdv {
        AnAdv(3585u64 as u16)
    }
}
impl core::fmt::Debug for AnAdv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnAdv")
            .field("selector", &self.selector())
            .field("tech_abil", &self.tech_abil())
            .field("xtnd_nxtp", &self.xtnd_nxtp())
            .field("rem_fault", &self.rem_fault())
            .field("next_pg", &self.next_pg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnAdv {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AnAdv {
            selector: u8,
            tech_abil: u8,
            xtnd_nxtp: bool,
            rem_fault: bool,
            next_pg: bool,
        }
        let proxy = AnAdv {
            selector: self.selector(),
            tech_abil: self.tech_abil(),
            xtnd_nxtp: self.xtnd_nxtp(),
            rem_fault: self.rem_fault(),
            next_pg: self.next_pg(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AN Expansion"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnExp(pub u16);
impl AnExp {
    #[doc = "Link Partner Auto-Negotiation Able"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_an_able(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Link Partner Auto-Negotiation Able"]
    #[inline(always)]
    pub const fn set_lp_an_able(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Page Received"]
    #[must_use]
    #[inline(always)]
    pub const fn page_rcvd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Page Received"]
    #[inline(always)]
    pub const fn set_page_rcvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Next Page Able"]
    #[must_use]
    #[inline(always)]
    pub const fn next_pg_abl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Next Page Able"]
    #[inline(always)]
    pub const fn set_next_pg_abl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Link Partner Next Page able"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_next_pg_abl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Link Partner Next Page able"]
    #[inline(always)]
    pub const fn set_lp_next_pg_abl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Parallel Detection Fault"]
    #[must_use]
    #[inline(always)]
    pub const fn det_fault(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Parallel Detection Fault"]
    #[inline(always)]
    pub const fn set_det_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Received Next Page Storage Location"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_next_pg_loc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Received Next Page Storage Location"]
    #[inline(always)]
    pub const fn set_rx_next_pg_loc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Receive Next Page Location Able"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_next_pg_abl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Next Page Location Able"]
    #[inline(always)]
    pub const fn set_rx_next_pg_abl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
}
impl Default for AnExp {
    #[inline(always)]
    fn default() -> AnExp {
        AnExp(111u64 as u16)
    }
}
impl core::fmt::Debug for AnExp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnExp")
            .field("lp_an_able", &self.lp_an_able())
            .field("page_rcvd", &self.page_rcvd())
            .field("next_pg_abl", &self.next_pg_abl())
            .field("lp_next_pg_abl", &self.lp_next_pg_abl())
            .field("det_fault", &self.det_fault())
            .field("rx_next_pg_loc", &self.rx_next_pg_loc())
            .field("rx_next_pg_abl", &self.rx_next_pg_abl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnExp {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AnExp {
            lp_an_able: bool,
            page_rcvd: bool,
            next_pg_abl: bool,
            lp_next_pg_abl: bool,
            det_fault: bool,
            rx_next_pg_loc: bool,
            rx_next_pg_abl: bool,
        }
        let proxy = AnExp {
            lp_an_able: self.lp_an_able(),
            page_rcvd: self.page_rcvd(),
            next_pg_abl: self.next_pg_abl(),
            lp_next_pg_abl: self.lp_next_pg_abl(),
            det_fault: self.det_fault(),
            rx_next_pg_loc: self.rx_next_pg_loc(),
            rx_next_pg_abl: self.rx_next_pg_abl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AN link partner ability"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnLpAbil(pub u16);
impl AnLpAbil {
    #[doc = "Selector Field"]
    #[must_use]
    #[inline(always)]
    pub const fn selector(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Selector Field"]
    #[inline(always)]
    pub const fn set_selector(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
    }
    #[doc = "Technology Ability Field"]
    #[must_use]
    #[inline(always)]
    pub const fn tech_abil(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x7f;
        val as u8
    }
    #[doc = "Technology Ability Field"]
    #[inline(always)]
    pub const fn set_tech_abil(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 5usize)) | (((val as u16) & 0x7f) << 5usize);
    }
    #[doc = "Extended Next Page"]
    #[must_use]
    #[inline(always)]
    pub const fn xtnd_nxtp(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Next Page"]
    #[inline(always)]
    pub const fn set_xtnd_nxtp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Remote Fault"]
    #[must_use]
    #[inline(always)]
    pub const fn rem_fault(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Fault"]
    #[inline(always)]
    pub const fn set_rem_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn ack(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Acknowledge"]
    #[inline(always)]
    pub const fn set_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Next page"]
    #[must_use]
    #[inline(always)]
    pub const fn next_pg(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Next page"]
    #[inline(always)]
    pub const fn set_next_pg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for AnLpAbil {
    #[inline(always)]
    fn default() -> AnLpAbil {
        AnLpAbil(19969u64 as u16)
    }
}
impl core::fmt::Debug for AnLpAbil {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnLpAbil")
            .field("selector", &self.selector())
            .field("tech_abil", &self.tech_abil())
            .field("xtnd_nxtp", &self.xtnd_nxtp())
            .field("rem_fault", &self.rem_fault())
            .field("ack", &self.ack())
            .field("next_pg", &self.next_pg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnLpAbil {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AnLpAbil {
            selector: u8,
            tech_abil: u8,
            xtnd_nxtp: bool,
            rem_fault: bool,
            ack: bool,
            next_pg: bool,
        }
        let proxy = AnLpAbil {
            selector: self.selector(),
            tech_abil: self.tech_abil(),
            xtnd_nxtp: self.xtnd_nxtp(),
            rem_fault: self.rem_fault(),
            ack: self.ack(),
            next_pg: self.next_pg(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AN Link Partner Received Next Page"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnLpRxNextPg(pub u16);
impl AnLpRxNextPg {
    #[doc = "Message/Unformatted Code"]
    #[must_use]
    #[inline(always)]
    pub const fn code(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Message/Unformatted Code"]
    #[inline(always)]
    pub const fn set_code(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
    #[doc = "Toggle"]
    #[must_use]
    #[inline(always)]
    pub const fn togl(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub const fn set_togl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Acknowledge 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ack2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Acknowledge 2"]
    #[inline(always)]
    pub const fn set_ack2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Message Page"]
    #[must_use]
    #[inline(always)]
    pub const fn msg_page(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Message Page"]
    #[inline(always)]
    pub const fn set_msg_page(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn ack(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Acknowledge"]
    #[inline(always)]
    pub const fn set_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Next page"]
    #[must_use]
    #[inline(always)]
    pub const fn next_pg(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Next page"]
    #[inline(always)]
    pub const fn set_next_pg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for AnLpRxNextPg {
    #[inline(always)]
    fn default() -> AnLpRxNextPg {
        AnLpRxNextPg(0u64 as u16)
    }
}
impl core::fmt::Debug for AnLpRxNextPg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnLpRxNextPg")
            .field("code", &self.code())
            .field("togl", &self.togl())
            .field("ack2", &self.ack2())
            .field("msg_page", &self.msg_page())
            .field("ack", &self.ack())
            .field("next_pg", &self.next_pg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnLpRxNextPg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AnLpRxNextPg {
            code: u16,
            togl: bool,
            ack2: bool,
            msg_page: bool,
            ack: bool,
            next_pg: bool,
        }
        let proxy = AnLpRxNextPg {
            code: self.code(),
            togl: self.togl(),
            ack2: self.ack2(),
            msg_page: self.msg_page(),
            ack: self.ack(),
            next_pg: self.next_pg(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "AN Next Page transmit register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnNextTx(pub u16);
impl AnNextTx {
    #[doc = "Message/Unformatted Code"]
    #[must_use]
    #[inline(always)]
    pub const fn code(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Message/Unformatted Code"]
    #[inline(always)]
    pub const fn set_code(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
    #[doc = "Toggle"]
    #[must_use]
    #[inline(always)]
    pub const fn togl(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub const fn set_togl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Acknowledge 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ack2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Acknowledge 2"]
    #[inline(always)]
    pub const fn set_ack2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Message Page"]
    #[must_use]
    #[inline(always)]
    pub const fn msg_page(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Message Page"]
    #[inline(always)]
    pub const fn set_msg_page(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Next page"]
    #[must_use]
    #[inline(always)]
    pub const fn next_pg(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Next page"]
    #[inline(always)]
    pub const fn set_next_pg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for AnNextTx {
    #[inline(always)]
    fn default() -> AnNextTx {
        AnNextTx(0u64 as u16)
    }
}
impl core::fmt::Debug for AnNextTx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnNextTx")
            .field("code", &self.code())
            .field("togl", &self.togl())
            .field("ack2", &self.ack2())
            .field("msg_page", &self.msg_page())
            .field("next_pg", &self.next_pg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnNextTx {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AnNextTx {
            code: u16,
            togl: bool,
            ack2: bool,
            msg_page: bool,
            next_pg: bool,
        }
        let proxy = AnNextTx {
            code: self.code(),
            togl: self.togl(),
            ack2: self.ack2(),
            msg_page: self.msg_page(),
            next_pg: self.next_pg(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u16);
impl Control {
    #[doc = "Speed selection MSB"]
    #[must_use]
    #[inline(always)]
    pub const fn speedmsb(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Speed selection MSB"]
    #[inline(always)]
    pub const fn set_speedmsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Collision test"]
    #[must_use]
    #[inline(always)]
    pub const fn coll_test(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Collision test"]
    #[inline(always)]
    pub const fn set_coll_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Full Duplex"]
    #[must_use]
    #[inline(always)]
    pub const fn fd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Full Duplex"]
    #[inline(always)]
    pub const fn set_fd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Restart Auto-Negotiate"]
    #[must_use]
    #[inline(always)]
    pub const fn restart_an(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Restart Auto-Negotiate"]
    #[inline(always)]
    pub const fn set_restart_an(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Isolate"]
    #[must_use]
    #[inline(always)]
    pub const fn isolate(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Isolate"]
    #[inline(always)]
    pub const fn set_isolate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Power down"]
    #[must_use]
    #[inline(always)]
    pub const fn pd(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Power down"]
    #[inline(always)]
    pub const fn set_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Auto-Negotiate Enable Enables Auto-Negotiate process"]
    #[must_use]
    #[inline(always)]
    pub const fn auto(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-Negotiate Enable Enables Auto-Negotiate process"]
    #[inline(always)]
    pub const fn set_auto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Speed selection LSB"]
    #[must_use]
    #[inline(always)]
    pub const fn speedlsb(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Speed selection LSB"]
    #[inline(always)]
    pub const fn set_speedlsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Loopback"]
    #[must_use]
    #[inline(always)]
    pub const fn loopback(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Loopback"]
    #[inline(always)]
    pub const fn set_loopback(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Control {
    #[inline(always)]
    fn default() -> Control {
        Control(4416u64 as u16)
    }
}
impl core::fmt::Debug for Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Control")
            .field("speedmsb", &self.speedmsb())
            .field("coll_test", &self.coll_test())
            .field("fd", &self.fd())
            .field("restart_an", &self.restart_an())
            .field("isolate", &self.isolate())
            .field("pd", &self.pd())
            .field("auto", &self.auto())
            .field("speedlsb", &self.speedlsb())
            .field("loopback", &self.loopback())
            .field("rst", &self.rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Control {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Control {
            speedmsb: bool,
            coll_test: bool,
            fd: bool,
            restart_an: bool,
            isolate: bool,
            pd: bool,
            auto: bool,
            speedlsb: bool,
            loopback: bool,
            rst: bool,
        }
        let proxy = Control {
            speedmsb: self.speedmsb(),
            coll_test: self.coll_test(),
            fd: self.fd(),
            restart_an: self.restart_an(),
            isolate: self.isolate(),
            pd: self.pd(),
            auto: self.auto(),
            speedlsb: self.speedlsb(),
            loopback: self.loopback(),
            rst: self.rst(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MASTER-SLAVE Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MsCtl(pub u16);
impl MsCtl {
    #[doc = "1000Base-T Half Duplex"]
    #[must_use]
    #[inline(always)]
    pub const fn t_1g_hd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1000Base-T Half Duplex"]
    #[inline(always)]
    pub const fn set_t_1g_hd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "1000Base-T Full Duplex"]
    #[must_use]
    #[inline(always)]
    pub const fn t_1g_fd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "1000Base-T Full Duplex"]
    #[inline(always)]
    pub const fn set_t_1g_fd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Port type"]
    #[must_use]
    #[inline(always)]
    pub const fn port_type(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port type"]
    #[inline(always)]
    pub const fn set_port_type(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "MASTER-SLAVE Config Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ms_cfg_value(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "MASTER-SLAVE Config Value"]
    #[inline(always)]
    pub const fn set_ms_cfg_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "MASTER-SLAVE Manual Config Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ms_cfg_enable(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "MASTER-SLAVE Manual Config Enable"]
    #[inline(always)]
    pub const fn set_ms_cfg_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Test Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn test_mode(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Test Mode"]
    #[inline(always)]
    pub const fn set_test_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for MsCtl {
    #[inline(always)]
    fn default() -> MsCtl {
        MsCtl(4608u64 as u16)
    }
}
impl core::fmt::Debug for MsCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MsCtl")
            .field("t_1g_hd", &self.t_1g_hd())
            .field("t_1g_fd", &self.t_1g_fd())
            .field("port_type", &self.port_type())
            .field("ms_cfg_value", &self.ms_cfg_value())
            .field("ms_cfg_enable", &self.ms_cfg_enable())
            .field("test_mode", &self.test_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MsCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MsCtl {
            t_1g_hd: bool,
            t_1g_fd: bool,
            port_type: bool,
            ms_cfg_value: bool,
            ms_cfg_enable: bool,
            test_mode: u8,
        }
        let proxy = MsCtl {
            t_1g_hd: self.t_1g_hd(),
            t_1g_fd: self.t_1g_fd(),
            port_type: self.port_type(),
            ms_cfg_value: self.ms_cfg_value(),
            ms_cfg_enable: self.ms_cfg_enable(),
            test_mode: self.test_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MASTER-SLAVE Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MsSta(pub u16);
impl MsSta {
    #[doc = "Idle Error Count"]
    #[must_use]
    #[inline(always)]
    pub const fn idle_err_cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Idle Error Count"]
    #[inline(always)]
    pub const fn set_idle_err_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "LP 1000T HD"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_1gt_hd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "LP 1000T HD"]
    #[inline(always)]
    pub const fn set_lp_1gt_hd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "LP 1000T FD"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_1gt_fd(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "LP 1000T FD"]
    #[inline(always)]
    pub const fn set_lp_1gt_fd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Remote receiver status"]
    #[must_use]
    #[inline(always)]
    pub const fn rem_rx_stat(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Remote receiver status"]
    #[inline(always)]
    pub const fn set_rem_rx_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Local receiver status"]
    #[must_use]
    #[inline(always)]
    pub const fn loc_rx_stat(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Local receiver status"]
    #[inline(always)]
    pub const fn set_loc_rx_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "MASTER-SLAVE configuration resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn ms_cfg_res(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "MASTER-SLAVE configuration resolution"]
    #[inline(always)]
    pub const fn set_ms_cfg_res(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "MASTER-SLAVE Manual Configuration Fault"]
    #[must_use]
    #[inline(always)]
    pub const fn ms_cfg_fault(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "MASTER-SLAVE Manual Configuration Fault"]
    #[inline(always)]
    pub const fn set_ms_cfg_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for MsSta {
    #[inline(always)]
    fn default() -> MsSta {
        MsSta(14336u64 as u16)
    }
}
impl core::fmt::Debug for MsSta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MsSta")
            .field("idle_err_cnt", &self.idle_err_cnt())
            .field("lp_1gt_hd", &self.lp_1gt_hd())
            .field("lp_1gt_fd", &self.lp_1gt_fd())
            .field("rem_rx_stat", &self.rem_rx_stat())
            .field("loc_rx_stat", &self.loc_rx_stat())
            .field("ms_cfg_res", &self.ms_cfg_res())
            .field("ms_cfg_fault", &self.ms_cfg_fault())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MsSta {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MsSta {
            idle_err_cnt: u8,
            lp_1gt_hd: bool,
            lp_1gt_fd: bool,
            rem_rx_stat: bool,
            loc_rx_stat: bool,
            ms_cfg_res: bool,
            ms_cfg_fault: bool,
        }
        let proxy = MsSta {
            idle_err_cnt: self.idle_err_cnt(),
            lp_1gt_hd: self.lp_1gt_hd(),
            lp_1gt_fd: self.lp_1gt_fd(),
            rem_rx_stat: self.rem_rx_stat(),
            loc_rx_stat: self.loc_rx_stat(),
            ms_cfg_res: self.ms_cfg_res(),
            ms_cfg_fault: self.ms_cfg_fault(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u16);
impl Status {
    #[doc = "Extended Capability"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_cap(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Capability"]
    #[inline(always)]
    pub const fn set_ext_cap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Jabber detect"]
    #[must_use]
    #[inline(always)]
    pub const fn jabber(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Jabber detect"]
    #[inline(always)]
    pub const fn set_jabber(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Link status"]
    #[must_use]
    #[inline(always)]
    pub const fn linkst(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Link status"]
    #[inline(always)]
    pub const fn set_linkst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "AN Ability"]
    #[must_use]
    #[inline(always)]
    pub const fn an_abil(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "AN Ability"]
    #[inline(always)]
    pub const fn set_an_abil(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Remote fault"]
    #[must_use]
    #[inline(always)]
    pub const fn rem_fault(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Remote fault"]
    #[inline(always)]
    pub const fn set_rem_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Auto-Negotiation Complete"]
    #[must_use]
    #[inline(always)]
    pub const fn an_comp(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-Negotiation Complete"]
    #[inline(always)]
    pub const fn set_an_comp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Preamble status"]
    #[must_use]
    #[inline(always)]
    pub const fn preamble(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Preamble status"]
    #[inline(always)]
    pub const fn set_preamble(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Unidirectional ability"]
    #[must_use]
    #[inline(always)]
    pub const fn uni(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Unidirectional ability"]
    #[inline(always)]
    pub const fn set_uni(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Extended status"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_stat(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Extended status"]
    #[inline(always)]
    pub const fn set_ext_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Current speed/duplex"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "Current speed/duplex"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u16) & 0x7f) << 9usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(37225u64 as u16)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("ext_cap", &self.ext_cap())
            .field("jabber", &self.jabber())
            .field("linkst", &self.linkst())
            .field("an_abil", &self.an_abil())
            .field("rem_fault", &self.rem_fault())
            .field("an_comp", &self.an_comp())
            .field("preamble", &self.preamble())
            .field("uni", &self.uni())
            .field("ext_stat", &self.ext_stat())
            .field("mode", &self.mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Status {
            ext_cap: bool,
            jabber: bool,
            linkst: bool,
            an_abil: bool,
            rem_fault: bool,
            an_comp: bool,
            preamble: bool,
            uni: bool,
            ext_stat: bool,
            mode: u8,
        }
        let proxy = Status {
            ext_cap: self.ext_cap(),
            jabber: self.jabber(),
            linkst: self.linkst(),
            an_abil: self.an_abil(),
            rem_fault: self.rem_fault(),
            an_comp: self.an_comp(),
            preamble: self.preamble(),
            uni: self.uni(),
            ext_stat: self.ext_stat(),
            mode: self.mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
