#[repr(C)]
#[doc = "Endpont 0 is selected"]
#[doc(alias = "ENDPOINT0")]
pub struct Endpoint0 {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    ctrlc: Ctrlc,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    syncbusy: Syncbusy,
    _reserved8: [u8; 0x0fe0],
    faddr: Faddr,
    powr: Powr,
    intrtx: Intrtx,
    intrrx: Intrrx,
    intrtxe: Intrtxe,
    intrrxe: Intrrxe,
    intrusb: Intrusb,
    intrusbe: Intrusbe,
    frame: Frame,
    index: Index,
    testmode: Testmode,
    _reserved19: [u8; 0x02],
    _reserved_19_csr0l: [u8; 0x01],
    _reserved_20_csr0h: [u8; 0x01],
    _reserved21: [u8; 0x04],
    count0: Count0,
    _reserved22: [u8; 0x01],
    type0: Type0,
    naklimit0: Naklimit0,
    _reserved24: [u8; 0x03],
    configdata: Configdata,
    fifox: [Fifox; 8],
    _reserved26: [u8; 0x20],
    devctl: Devctl,
    misc: Misc,
    txfifosz: Txfifosz,
    rxfifosz: Rxfifosz,
    txfifoadd: Txfifoadd,
    rxfifoadd: Rxfifoadd,
    _reserved32: [u8; 0x10],
    epinfo: Epinfo,
    raminfo: Raminfo,
    linkinfo: Linkinfo,
    vplen: Vplen,
    hseof1: Hseof1,
    fseof1: Fseof1,
    lseof1: Lseof1,
    softrst: Softrst,
    txfunaddr: [Txfunaddr; 8],
    _reserved41: [u8; 0x0140],
    dmaintr: Dmaintr,
    dmacnt: (),
    _reserved43: [u8; 0x0100],
    rqpktcount: [Rqpktcount; 7],
    _reserved44: [u8; 0x20],
    rxdpktbufdis: Rxdpktbufdis,
    txdpktbufdis: Txdpktbufdis,
    ctuch: Ctuch,
    cthhsrtn: Cthhsrtn,
    cthsbt: Cthsbt,
    _reserved49: [u8; 0x16],
    lpmattr: Lpmattr,
    _reserved_50_lpmcntrl: [u8; 0x01],
    lpmintren: Lpmintren,
    _reserved_52_lpmintr: [u8; 0x01],
    lpmfaddr: Lpmfaddr,
    _reserved54: [u8; 0x019a],
    phy00: Phy00,
    phy04: Phy04,
    phy08: Phy08,
    phy0c: Phy0c,
    phy10: Phy10,
    phy14: Phy14,
    phy18: Phy18,
    phy1c: Phy1c,
    phy20: Phy20,
    phy24: Phy24,
    phy28: Phy28,
    _reserved65: [u8; 0x18],
    phy44: Phy44,
    phy48: Phy48,
    phy4c: Phy4c,
    phy50: Phy50,
}
impl Endpoint0 {
    #[doc = "0x00 - USBHS Control Register A. (All bits except ENABLE and SWRST are Enable protected)"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - USBHS Control Register B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x08 - USBHS Control Register C"]
    #[inline(always)]
    pub const fn ctrlc(&self) -> &Ctrlc {
        &self.ctrlc
    }
    #[doc = "0x0c - USBHS Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x10 - USBHS Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x14 - USBHS Interrupt Flag Register"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x18 - USBHS Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x1c - USBHS Syncbusy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x1000 - Function Address Register"]
    #[inline(always)]
    pub const fn faddr(&self) -> &Faddr {
        &self.faddr
    }
    #[doc = "0x1001 - Used For Controlling Suspend And Resume Signaling And Some Basic Operation Of USBCORE"]
    #[inline(always)]
    pub const fn powr(&self) -> &Powr {
        &self.powr
    }
    #[doc = "0x1002 - Read Only Register Which Indicates Which Interrupts Are Currently Active For EP0 And TX EP1-15"]
    #[inline(always)]
    pub const fn intrtx(&self) -> &Intrtx {
        &self.intrtx
    }
    #[doc = "0x1004 - Read Only Register Which Indicates Which Interrupts Are Currently Active For RX EP1-15"]
    #[inline(always)]
    pub const fn intrrx(&self) -> &Intrrx {
        &self.intrrx
    }
    #[doc = "0x1006 - Interrupt Enable Bits for INTRTX"]
    #[inline(always)]
    pub const fn intrtxe(&self) -> &Intrtxe {
        &self.intrtxe
    }
    #[doc = "0x1008 - Interrupt Enable Bits for INTRRX"]
    #[inline(always)]
    pub const fn intrrxe(&self) -> &Intrrxe {
        &self.intrrxe
    }
    #[doc = "0x100a - Read Only Register Which Indicates Which USB Interrupts Are Currently Active."]
    #[inline(always)]
    pub const fn intrusb(&self) -> &Intrusb {
        &self.intrusb
    }
    #[doc = "0x100b - Interrupt Enable Bits for USB Interrupts"]
    #[inline(always)]
    pub const fn intrusbe(&self) -> &Intrusbe {
        &self.intrusbe
    }
    #[doc = "0x100c - Holds The Last Received Frame Number"]
    #[inline(always)]
    pub const fn frame(&self) -> &Frame {
        &self.frame
    }
    #[doc = "0x100e - Index Is A 4-Bit Register That Determines Which Endpoint Control/Status Registers Are Accessed"]
    #[inline(always)]
    pub const fn index(&self) -> &Index {
        &self.index
    }
    #[doc = "0x100f - Not Used In Normal Operation. Configuration To Put the USBCORE Into One Of The Four Test Modes For HighSpeed Operation"]
    #[inline(always)]
    pub const fn testmode(&self) -> &Testmode {
        &self.testmode
    }
    #[doc = "0x1012 - ENDPOINT0 Control And Status Bits for TX Endpoint 0"]
    #[inline(always)]
    pub const fn csr0l_peripheral_ep0_mode(&self) -> &Csr0lPeripheralEp0Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4114).cast() }
    }
    #[doc = "0x1012 - ENDPOINT0 Control And Status Bits for TX Endpoint 0"]
    #[inline(always)]
    pub const fn csr0l_host_ep0_mode(&self) -> &Csr0lHostEp0Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4114).cast() }
    }
    #[doc = "0x1013 - ENDPOINT0 Additional Control And Status Bits For TX Endpoint 0"]
    #[inline(always)]
    pub const fn csr0h_peripheral_ep0_mode(&self) -> &Csr0hPeripheralEp0Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4115).cast() }
    }
    #[doc = "0x1013 - ENDPOINT0 Additional Control And Status Bits For TX Endpoint 0"]
    #[inline(always)]
    pub const fn csr0h_host_ep0_mode(&self) -> &Csr0hHostEp0Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4115).cast() }
    }
    #[doc = "0x1018 - ENDPOINT0 Number Of Received Bytes In Endpoint 0 FIFO"]
    #[inline(always)]
    pub const fn count0(&self) -> &Count0 {
        &self.count0
    }
    #[doc = "0x101a - ENDPOINT0 Host Mode Only: Operating Speed Of The Targeted Device"]
    #[inline(always)]
    pub const fn type0(&self) -> &Type0 {
        &self.type0
    }
    #[doc = "0x101b - ENDPOINT0 Host Mode Only: Sets The NAK Response Timeout On Endpoint 0."]
    #[inline(always)]
    pub const fn naklimit0(&self) -> &Naklimit0 {
        &self.naklimit0
    }
    #[doc = "0x101f - ENDPOINT0 Returns Information About The Selected Core Configuration. Only Applicable when INDEX Regsiter Is 0. That Is Endpoint0 Is Selected."]
    #[inline(always)]
    pub const fn configdata(&self) -> &Configdata {
        &self.configdata
    }
    #[doc = "0x1020..0x1040 - Address Range Provides 16 Addresses For CPU Access To The FIFO's For Each Endpoint"]
    #[inline(always)]
    pub const fn fifox(&self, n: usize) -> &Fifox {
        &self.fifox[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x1040 - Address Range Provides 16 Addresses For CPU Access To The FIFO's For Each Endpoint"]
    #[inline(always)]
    pub fn fifox_iter(&self) -> impl Iterator<Item = &Fifox> {
        self.fifox.iter()
    }
    #[doc = "0x1060 - Selects Whether The Core Is Operating In Peripheral Or Host Mode"]
    #[inline(always)]
    pub const fn devctl(&self) -> &Devctl {
        &self.devctl
    }
    #[doc = "0x1061 - RX/TX Early DMA Enable Bits"]
    #[inline(always)]
    pub const fn misc(&self) -> &Misc {
        &self.misc
    }
    #[doc = "0x1062 - Controls The Size Of The Selected TX Endpoint FIFO"]
    #[inline(always)]
    pub const fn txfifosz(&self) -> &Txfifosz {
        &self.txfifosz
    }
    #[doc = "0x1063 - Controls The Size Of The Selected RX Endpoint FIFO"]
    #[inline(always)]
    pub const fn rxfifosz(&self) -> &Rxfifosz {
        &self.rxfifosz
    }
    #[doc = "0x1064 - Controls The Start Address Of the Selected TX Endpoint FIFO"]
    #[inline(always)]
    pub const fn txfifoadd(&self) -> &Txfifoadd {
        &self.txfifoadd
    }
    #[doc = "0x1066 - Controls The Start Address Of the Selected RX Endpoint FIFO"]
    #[inline(always)]
    pub const fn rxfifoadd(&self) -> &Rxfifoadd {
        &self.rxfifoadd
    }
    #[doc = "0x1078 - Allows Read-Back Of The Number Of TX And RX Endpoints"]
    #[inline(always)]
    pub const fn epinfo(&self) -> &Epinfo {
        &self.epinfo
    }
    #[doc = "0x1079 - Details About Width Of RAM and DMA Channels"]
    #[inline(always)]
    pub const fn raminfo(&self) -> &Raminfo {
        &self.raminfo
    }
    #[doc = "0x107a - Allows Some Delays To Be Specified"]
    #[inline(always)]
    pub const fn linkinfo(&self) -> &Linkinfo {
        &self.linkinfo
    }
    #[doc = "0x107b - Sets The Duration Of The VBus Pulsing Charge"]
    #[inline(always)]
    pub const fn vplen(&self) -> &Vplen {
        &self.vplen
    }
    #[doc = "0x107c - Sets The Minimum Time Gap That Is To Be Allowed Between The Start Of The Last Transaction And The EOF For HS Transactions"]
    #[inline(always)]
    pub const fn hseof1(&self) -> &Hseof1 {
        &self.hseof1
    }
    #[doc = "0x107d - Sets The Minimum Time Gap That Is To Be Allowed Between The Start Of The Last Transaction And The EOF For FS Transactions"]
    #[inline(always)]
    pub const fn fseof1(&self) -> &Fseof1 {
        &self.fseof1
    }
    #[doc = "0x107e - Sets The Minimum Time Gap That Is To Be Allowed Between The Start Of The Last Transaction And The EOF For LS Transactions"]
    #[inline(always)]
    pub const fn lseof1(&self) -> &Lseof1 {
        &self.lseof1
    }
    #[doc = "0x107f - Assert Low The Output Reset Signals NRSTO and NRSTXO"]
    #[inline(always)]
    pub const fn softrst(&self) -> &Softrst {
        &self.softrst
    }
    #[doc = "0x1080..0x10c0 - TXFUNADDR\\[%s\\]"]
    #[inline(always)]
    pub const fn txfunaddr(&self, n: usize) -> &Txfunaddr {
        &self.txfunaddr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10c0 - TXFUNADDR\\[%s\\]"]
    #[inline(always)]
    pub fn txfunaddr_iter(&self) -> impl Iterator<Item = &Txfunaddr> {
        self.txfunaddr.iter()
    }
    #[doc = "0x1200 - DMA Interrupt Status for Each Channel"]
    #[inline(always)]
    pub const fn dmaintr(&self) -> &Dmaintr {
        &self.dmaintr
    }
    #[doc = "0x1204..0x1264 - DMACNT\\[%s\\]"]
    #[inline(always)]
    pub const fn dmacnt(&self, n: usize) -> &Dmacnt {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4612).add(16 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1204..0x1264 - DMACNT\\[%s\\]"]
    #[inline(always)]
    pub fn dmacnt_iter(&self) -> impl Iterator<Item = &Dmacnt> {
        (0..8).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4612).add(16 * n).cast() })
    }
    #[doc = "0x1304..0x1320 - Host Mode Only: Used To Specify Number Of Packets That Are To Be Transaferred In A Block Transfer"]
    #[inline(always)]
    pub const fn rqpktcount(&self, n: usize) -> &Rqpktcount {
        &self.rqpktcount[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1304..0x1320 - Host Mode Only: Used To Specify Number Of Packets That Are To Be Transaferred In A Block Transfer"]
    #[inline(always)]
    pub fn rqpktcount_iter(&self) -> impl Iterator<Item = &Rqpktcount> {
        self.rqpktcount.iter()
    }
    #[doc = "0x1340 - Indicates Which Of The RX Endpoints Have Disabled Double Packet Buffer Functionality"]
    #[inline(always)]
    pub const fn rxdpktbufdis(&self) -> &Rxdpktbufdis {
        &self.rxdpktbufdis
    }
    #[doc = "0x1342 - Indicates Which Of The TX Endpoints Have Disabled Double Packet Buffer Functionality"]
    #[inline(always)]
    pub const fn txdpktbufdis(&self) -> &Txdpktbufdis {
        &self.txdpktbufdis
    }
    #[doc = "0x1344 - Chirp Timeout. Number Of XCLK Cycles Before The Timeout"]
    #[inline(always)]
    pub const fn ctuch(&self) -> &Ctuch {
        &self.ctuch
    }
    #[doc = "0x1346 - Sets The Delay From The End Of High-Speed Resume Signaling (Acting As A Host) To Enable The UTM Normal Operating Mode"]
    #[inline(always)]
    pub const fn cthhsrtn(&self) -> &Cthhsrtn {
        &self.cthhsrtn
    }
    #[doc = "0x1348 - This Register Represents The Value To Be Added To The Minimum High Speed Timeout Period Of 736 Bit Times."]
    #[inline(always)]
    pub const fn cthsbt(&self) -> &Cthsbt {
        &self.cthsbt
    }
    #[doc = "0x1360 - Defines The Attributes Of An LPM Transaction And Sleep Cycle."]
    #[inline(always)]
    pub const fn lpmattr(&self) -> &Lpmattr {
        &self.lpmattr
    }
    #[doc = "0x1362 - LPM Control Register"]
    #[inline(always)]
    pub const fn lpmcntrl_peripheral_mode(&self) -> &LpmcntrlPeripheralMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4962).cast() }
    }
    #[doc = "0x1362 - LPM Control Register"]
    #[inline(always)]
    pub const fn lpmcntrl_host_mode(&self) -> &LpmcntrlHostMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4962).cast() }
    }
    #[doc = "0x1363 - LPM Interrupts Enable Register"]
    #[inline(always)]
    pub const fn lpmintren(&self) -> &Lpmintren {
        &self.lpmintren
    }
    #[doc = "0x1364 - Status Of The LPM Power State"]
    #[inline(always)]
    pub const fn lpmintr_peripheral_mode(&self) -> &LpmintrPeripheralMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4964).cast() }
    }
    #[doc = "0x1364 - Status Of The LPM Power State"]
    #[inline(always)]
    pub const fn lpmintr_host_mode(&self) -> &LpmintrHostMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4964).cast() }
    }
    #[doc = "0x1365 - Function Address In LPM Payload"]
    #[inline(always)]
    pub const fn lpmfaddr(&self) -> &Lpmfaddr {
        &self.lpmfaddr
    }
    #[doc = "0x1500 - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy00(&self) -> &Phy00 {
        &self.phy00
    }
    #[doc = "0x1504 - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy04(&self) -> &Phy04 {
        &self.phy04
    }
    #[doc = "0x1508 - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy08(&self) -> &Phy08 {
        &self.phy08
    }
    #[doc = "0x150c - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy0c(&self) -> &Phy0c {
        &self.phy0c
    }
    #[doc = "0x1510 - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy10(&self) -> &Phy10 {
        &self.phy10
    }
    #[doc = "0x1514 - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy14(&self) -> &Phy14 {
        &self.phy14
    }
    #[doc = "0x1518 - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy18(&self) -> &Phy18 {
        &self.phy18
    }
    #[doc = "0x151c - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy1c(&self) -> &Phy1c {
        &self.phy1c
    }
    #[doc = "0x1520 - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy20(&self) -> &Phy20 {
        &self.phy20
    }
    #[doc = "0x1524 - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy24(&self) -> &Phy24 {
        &self.phy24
    }
    #[doc = "0x1528 - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy28(&self) -> &Phy28 {
        &self.phy28
    }
    #[doc = "0x1544 - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy44(&self) -> &Phy44 {
        &self.phy44
    }
    #[doc = "0x1548 - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy48(&self) -> &Phy48 {
        &self.phy48
    }
    #[doc = "0x154c - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy4c(&self) -> &Phy4c {
        &self.phy4c
    }
    #[doc = "0x1550 - USBHPHY Control Register."]
    #[inline(always)]
    pub const fn phy50(&self) -> &Phy50 {
        &self.phy50
    }
}
#[doc = "CTRLA (rw) register accessor: USBHS Control Register A. (All bits except ENABLE and SWRST are Enable protected)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "USBHS Control Register A. (All bits except ENABLE and SWRST are Enable protected)"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: USBHS Control Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "USBHS Control Register B"]
pub mod ctrlb;
#[doc = "CTRLC (rw) register accessor: USBHS Control Register C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlc`] module"]
#[doc(alias = "CTRLC")]
pub type Ctrlc = crate::Reg<ctrlc::CtrlcSpec>;
#[doc = "USBHS Control Register C"]
pub mod ctrlc;
#[doc = "INTENCLR (rw) register accessor: USBHS Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "USBHS Interrupt Enable Clear Register"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: USBHS Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "USBHS Interrupt Enable Clear Register"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: USBHS Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "USBHS Interrupt Flag Register"]
pub mod intflag;
#[doc = "STATUS (r) register accessor: USBHS Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "USBHS Status Register"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: USBHS Syncbusy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "USBHS Syncbusy Register"]
pub mod syncbusy;
#[doc = "FADDR (rw) register accessor: Function Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`faddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faddr`] module"]
#[doc(alias = "FADDR")]
pub type Faddr = crate::Reg<faddr::FaddrSpec>;
#[doc = "Function Address Register"]
pub mod faddr;
#[doc = "POWR (rw) register accessor: Used For Controlling Suspend And Resume Signaling And Some Basic Operation Of USBCORE\n\nYou can [`read`](crate::Reg::read) this register and get [`powr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`powr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@powr`] module"]
#[doc(alias = "POWR")]
pub type Powr = crate::Reg<powr::PowrSpec>;
#[doc = "Used For Controlling Suspend And Resume Signaling And Some Basic Operation Of USBCORE"]
pub mod powr;
#[doc = "INTRTX (r) register accessor: Read Only Register Which Indicates Which Interrupts Are Currently Active For EP0 And TX EP1-15\n\nYou can [`read`](crate::Reg::read) this register and get [`intrtx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>\n\nFor information about available fields see [`mod@intrtx`] module"]
#[doc(alias = "INTRTX")]
pub type Intrtx = crate::Reg<intrtx::IntrtxSpec>;
#[doc = "Read Only Register Which Indicates Which Interrupts Are Currently Active For EP0 And TX EP1-15"]
pub mod intrtx;
#[doc = "INTRRX (r) register accessor: Read Only Register Which Indicates Which Interrupts Are Currently Active For RX EP1-15\n\nYou can [`read`](crate::Reg::read) this register and get [`intrrx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>\n\nFor information about available fields see [`mod@intrrx`] module"]
#[doc(alias = "INTRRX")]
pub type Intrrx = crate::Reg<intrrx::IntrrxSpec>;
#[doc = "Read Only Register Which Indicates Which Interrupts Are Currently Active For RX EP1-15"]
pub mod intrrx;
#[doc = "INTRTXE (rw) register accessor: Interrupt Enable Bits for INTRTX\n\nYou can [`read`](crate::Reg::read) this register and get [`intrtxe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrtxe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrtxe`] module"]
#[doc(alias = "INTRTXE")]
pub type Intrtxe = crate::Reg<intrtxe::IntrtxeSpec>;
#[doc = "Interrupt Enable Bits for INTRTX"]
pub mod intrtxe;
#[doc = "INTRRXE (rw) register accessor: Interrupt Enable Bits for INTRRX\n\nYou can [`read`](crate::Reg::read) this register and get [`intrrxe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrrxe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrrxe`] module"]
#[doc(alias = "INTRRXE")]
pub type Intrrxe = crate::Reg<intrrxe::IntrrxeSpec>;
#[doc = "Interrupt Enable Bits for INTRRX"]
pub mod intrrxe;
#[doc = "INTRUSB (r) register accessor: Read Only Register Which Indicates Which USB Interrupts Are Currently Active.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrusb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrusb`] module"]
#[doc(alias = "INTRUSB")]
pub type Intrusb = crate::Reg<intrusb::IntrusbSpec>;
#[doc = "Read Only Register Which Indicates Which USB Interrupts Are Currently Active."]
pub mod intrusb;
#[doc = "INTRUSBE (rw) register accessor: Interrupt Enable Bits for USB Interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intrusbe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrusbe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrusbe`] module"]
#[doc(alias = "INTRUSBE")]
pub type Intrusbe = crate::Reg<intrusbe::IntrusbeSpec>;
#[doc = "Interrupt Enable Bits for USB Interrupts"]
pub mod intrusbe;
#[doc = "FRAME (r) register accessor: Holds The Last Received Frame Number\n\nYou can [`read`](crate::Reg::read) this register and get [`frame::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame`] module"]
#[doc(alias = "FRAME")]
pub type Frame = crate::Reg<frame::FrameSpec>;
#[doc = "Holds The Last Received Frame Number"]
pub mod frame;
#[doc = "INDEX (rw) register accessor: Index Is A 4-Bit Register That Determines Which Endpoint Control/Status Registers Are Accessed\n\nYou can [`read`](crate::Reg::read) this register and get [`index::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`index::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@index`] module"]
#[doc(alias = "INDEX")]
pub type Index = crate::Reg<index::IndexSpec>;
#[doc = "Index Is A 4-Bit Register That Determines Which Endpoint Control/Status Registers Are Accessed"]
pub mod index;
#[doc = "TESTMODE (rw) register accessor: Not Used In Normal Operation. Configuration To Put the USBCORE Into One Of The Four Test Modes For HighSpeed Operation\n\nYou can [`read`](crate::Reg::read) this register and get [`testmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testmode`] module"]
#[doc(alias = "TESTMODE")]
pub type Testmode = crate::Reg<testmode::TestmodeSpec>;
#[doc = "Not Used In Normal Operation. Configuration To Put the USBCORE Into One Of The Four Test Modes For HighSpeed Operation"]
pub mod testmode;
#[doc = "CSR0L_HOST_EP0_MODE (rw) register accessor: ENDPOINT0 Control And Status Bits for TX Endpoint 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0l_host_ep0_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0l_host_ep0_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr0l_host_ep0_mode`] module"]
#[doc(alias = "CSR0L_HOST_EP0_MODE")]
pub type Csr0lHostEp0Mode = crate::Reg<csr0l_host_ep0_mode::Csr0lHostEp0ModeSpec>;
#[doc = "ENDPOINT0 Control And Status Bits for TX Endpoint 0"]
pub mod csr0l_host_ep0_mode;
#[doc = "CSR0L_PERIPHERAL_EP0_MODE (rw) register accessor: ENDPOINT0 Control And Status Bits for TX Endpoint 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0l_peripheral_ep0_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0l_peripheral_ep0_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr0l_peripheral_ep0_mode`] module"]
#[doc(alias = "CSR0L_PERIPHERAL_EP0_MODE")]
pub type Csr0lPeripheralEp0Mode = crate::Reg<csr0l_peripheral_ep0_mode::Csr0lPeripheralEp0ModeSpec>;
#[doc = "ENDPOINT0 Control And Status Bits for TX Endpoint 0"]
pub mod csr0l_peripheral_ep0_mode;
#[doc = "CSR0H_HOST_EP0_MODE (rw) register accessor: ENDPOINT0 Additional Control And Status Bits For TX Endpoint 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0h_host_ep0_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0h_host_ep0_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr0h_host_ep0_mode`] module"]
#[doc(alias = "CSR0H_HOST_EP0_MODE")]
pub type Csr0hHostEp0Mode = crate::Reg<csr0h_host_ep0_mode::Csr0hHostEp0ModeSpec>;
#[doc = "ENDPOINT0 Additional Control And Status Bits For TX Endpoint 0"]
pub mod csr0h_host_ep0_mode;
#[doc = "CSR0H_PERIPHERAL_EP0_MODE (rw) register accessor: ENDPOINT0 Additional Control And Status Bits For TX Endpoint 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0h_peripheral_ep0_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0h_peripheral_ep0_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr0h_peripheral_ep0_mode`] module"]
#[doc(alias = "CSR0H_PERIPHERAL_EP0_MODE")]
pub type Csr0hPeripheralEp0Mode = crate::Reg<csr0h_peripheral_ep0_mode::Csr0hPeripheralEp0ModeSpec>;
#[doc = "ENDPOINT0 Additional Control And Status Bits For TX Endpoint 0"]
pub mod csr0h_peripheral_ep0_mode;
#[doc = "COUNT0 (r) register accessor: ENDPOINT0 Number Of Received Bytes In Endpoint 0 FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`count0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count0`] module"]
#[doc(alias = "COUNT0")]
pub type Count0 = crate::Reg<count0::Count0Spec>;
#[doc = "ENDPOINT0 Number Of Received Bytes In Endpoint 0 FIFO"]
pub mod count0;
#[doc = "TYPE0 (rw) register accessor: ENDPOINT0 Host Mode Only: Operating Speed Of The Targeted Device\n\nYou can [`read`](crate::Reg::read) this register and get [`type0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`type0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@type0`] module"]
#[doc(alias = "TYPE0")]
pub type Type0 = crate::Reg<type0::Type0Spec>;
#[doc = "ENDPOINT0 Host Mode Only: Operating Speed Of The Targeted Device"]
pub mod type0;
#[doc = "NAKLIMIT0 (rw) register accessor: ENDPOINT0 Host Mode Only: Sets The NAK Response Timeout On Endpoint 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`naklimit0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`naklimit0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@naklimit0`] module"]
#[doc(alias = "NAKLIMIT0")]
pub type Naklimit0 = crate::Reg<naklimit0::Naklimit0Spec>;
#[doc = "ENDPOINT0 Host Mode Only: Sets The NAK Response Timeout On Endpoint 0."]
pub mod naklimit0;
#[doc = "CONFIGDATA (r) register accessor: ENDPOINT0 Returns Information About The Selected Core Configuration. Only Applicable when INDEX Regsiter Is 0. That Is Endpoint0 Is Selected.\n\nYou can [`read`](crate::Reg::read) this register and get [`configdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@configdata`] module"]
#[doc(alias = "CONFIGDATA")]
pub type Configdata = crate::Reg<configdata::ConfigdataSpec>;
#[doc = "ENDPOINT0 Returns Information About The Selected Core Configuration. Only Applicable when INDEX Regsiter Is 0. That Is Endpoint0 Is Selected."]
pub mod configdata;
#[doc = "FIFOX (rw) register accessor: Address Range Provides 16 Addresses For CPU Access To The FIFO's For Each Endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`fifox::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifox::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifox`] module"]
#[doc(alias = "FIFOX")]
pub type Fifox = crate::Reg<fifox::FifoxSpec>;
#[doc = "Address Range Provides 16 Addresses For CPU Access To The FIFO's For Each Endpoint"]
pub mod fifox;
#[doc = "DEVCTL (rw) register accessor: Selects Whether The Core Is Operating In Peripheral Or Host Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`devctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devctl`] module"]
#[doc(alias = "DEVCTL")]
pub type Devctl = crate::Reg<devctl::DevctlSpec>;
#[doc = "Selects Whether The Core Is Operating In Peripheral Or Host Mode"]
pub mod devctl;
#[doc = "MISC (rw) register accessor: RX/TX Early DMA Enable Bits\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`] module"]
#[doc(alias = "MISC")]
pub type Misc = crate::Reg<misc::MiscSpec>;
#[doc = "RX/TX Early DMA Enable Bits"]
pub mod misc;
#[doc = "TXFIFOSZ (rw) register accessor: Controls The Size Of The Selected TX Endpoint FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifosz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifosz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifosz`] module"]
#[doc(alias = "TXFIFOSZ")]
pub type Txfifosz = crate::Reg<txfifosz::TxfifoszSpec>;
#[doc = "Controls The Size Of The Selected TX Endpoint FIFO"]
pub mod txfifosz;
#[doc = "RXFIFOSZ (rw) register accessor: Controls The Size Of The Selected RX Endpoint FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifosz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfifosz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifosz`] module"]
#[doc(alias = "RXFIFOSZ")]
pub type Rxfifosz = crate::Reg<rxfifosz::RxfifoszSpec>;
#[doc = "Controls The Size Of The Selected RX Endpoint FIFO"]
pub mod rxfifosz;
#[doc = "TXFIFOADD (rw) register accessor: Controls The Start Address Of the Selected TX Endpoint FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifoadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifoadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifoadd`] module"]
#[doc(alias = "TXFIFOADD")]
pub type Txfifoadd = crate::Reg<txfifoadd::TxfifoaddSpec>;
#[doc = "Controls The Start Address Of the Selected TX Endpoint FIFO"]
pub mod txfifoadd;
#[doc = "RXFIFOADD (rw) register accessor: Controls The Start Address Of the Selected RX Endpoint FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifoadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfifoadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifoadd`] module"]
#[doc(alias = "RXFIFOADD")]
pub type Rxfifoadd = crate::Reg<rxfifoadd::RxfifoaddSpec>;
#[doc = "Controls The Start Address Of the Selected RX Endpoint FIFO"]
pub mod rxfifoadd;
#[doc = "EPINFO (r) register accessor: Allows Read-Back Of The Number Of TX And RX Endpoints\n\nYou can [`read`](crate::Reg::read) this register and get [`epinfo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epinfo`] module"]
#[doc(alias = "EPINFO")]
pub type Epinfo = crate::Reg<epinfo::EpinfoSpec>;
#[doc = "Allows Read-Back Of The Number Of TX And RX Endpoints"]
pub mod epinfo;
#[doc = "RAMINFO (r) register accessor: Details About Width Of RAM and DMA Channels\n\nYou can [`read`](crate::Reg::read) this register and get [`raminfo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raminfo`] module"]
#[doc(alias = "RAMINFO")]
pub type Raminfo = crate::Reg<raminfo::RaminfoSpec>;
#[doc = "Details About Width Of RAM and DMA Channels"]
pub mod raminfo;
#[doc = "LINKINFO (rw) register accessor: Allows Some Delays To Be Specified\n\nYou can [`read`](crate::Reg::read) this register and get [`linkinfo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linkinfo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linkinfo`] module"]
#[doc(alias = "LINKINFO")]
pub type Linkinfo = crate::Reg<linkinfo::LinkinfoSpec>;
#[doc = "Allows Some Delays To Be Specified"]
pub mod linkinfo;
#[doc = "VPLEN (rw) register accessor: Sets The Duration Of The VBus Pulsing Charge\n\nYou can [`read`](crate::Reg::read) this register and get [`vplen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vplen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vplen`] module"]
#[doc(alias = "VPLEN")]
pub type Vplen = crate::Reg<vplen::VplenSpec>;
#[doc = "Sets The Duration Of The VBus Pulsing Charge"]
pub mod vplen;
#[doc = "HSEOF1 (rw) register accessor: Sets The Minimum Time Gap That Is To Be Allowed Between The Start Of The Last Transaction And The EOF For HS Transactions\n\nYou can [`read`](crate::Reg::read) this register and get [`hseof1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hseof1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hseof1`] module"]
#[doc(alias = "HSEOF1")]
pub type Hseof1 = crate::Reg<hseof1::Hseof1Spec>;
#[doc = "Sets The Minimum Time Gap That Is To Be Allowed Between The Start Of The Last Transaction And The EOF For HS Transactions"]
pub mod hseof1;
#[doc = "FSEOF1 (rw) register accessor: Sets The Minimum Time Gap That Is To Be Allowed Between The Start Of The Last Transaction And The EOF For FS Transactions\n\nYou can [`read`](crate::Reg::read) this register and get [`fseof1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fseof1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fseof1`] module"]
#[doc(alias = "FSEOF1")]
pub type Fseof1 = crate::Reg<fseof1::Fseof1Spec>;
#[doc = "Sets The Minimum Time Gap That Is To Be Allowed Between The Start Of The Last Transaction And The EOF For FS Transactions"]
pub mod fseof1;
#[doc = "LSEOF1 (rw) register accessor: Sets The Minimum Time Gap That Is To Be Allowed Between The Start Of The Last Transaction And The EOF For LS Transactions\n\nYou can [`read`](crate::Reg::read) this register and get [`lseof1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lseof1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lseof1`] module"]
#[doc(alias = "LSEOF1")]
pub type Lseof1 = crate::Reg<lseof1::Lseof1Spec>;
#[doc = "Sets The Minimum Time Gap That Is To Be Allowed Between The Start Of The Last Transaction And The EOF For LS Transactions"]
pub mod lseof1;
#[doc = "SOFTRST (rw) register accessor: Assert Low The Output Reset Signals NRSTO and NRSTXO\n\nYou can [`read`](crate::Reg::read) this register and get [`softrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst`] module"]
#[doc(alias = "SOFTRST")]
pub type Softrst = crate::Reg<softrst::SoftrstSpec>;
#[doc = "Assert Low The Output Reset Signals NRSTO and NRSTXO"]
pub mod softrst;
#[doc = "TXFUNADDR\\[%s\\]"]
pub use self::txfunaddr::Txfunaddr;
#[doc = r"Cluster"]
#[doc = "TXFUNADDR\\[%s\\]"]
pub mod txfunaddr;
#[doc = "DMAINTR (rw) register accessor: DMA Interrupt Status for Each Channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaintr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaintr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaintr`] module"]
#[doc(alias = "DMAINTR")]
pub type Dmaintr = crate::Reg<dmaintr::DmaintrSpec>;
#[doc = "DMA Interrupt Status for Each Channel"]
pub mod dmaintr;
#[doc = "DMACNT\\[%s\\]"]
pub use self::dmacnt::Dmacnt;
#[doc = r"Cluster"]
#[doc = "DMACNT\\[%s\\]"]
pub mod dmacnt;
#[doc = "RQPKTCOUNT (rw) register accessor: Host Mode Only: Used To Specify Number Of Packets That Are To Be Transaferred In A Block Transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`rqpktcount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqpktcount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rqpktcount`] module"]
#[doc(alias = "RQPKTCOUNT")]
pub type Rqpktcount = crate::Reg<rqpktcount::RqpktcountSpec>;
#[doc = "Host Mode Only: Used To Specify Number Of Packets That Are To Be Transaferred In A Block Transfer"]
pub mod rqpktcount;
#[doc = "RXDPKTBUFDIS (rw) register accessor: Indicates Which Of The RX Endpoints Have Disabled Double Packet Buffer Functionality\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdpktbufdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdpktbufdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdpktbufdis`] module"]
#[doc(alias = "RXDPKTBUFDIS")]
pub type Rxdpktbufdis = crate::Reg<rxdpktbufdis::RxdpktbufdisSpec>;
#[doc = "Indicates Which Of The RX Endpoints Have Disabled Double Packet Buffer Functionality"]
pub mod rxdpktbufdis;
#[doc = "TXDPKTBUFDIS (rw) register accessor: Indicates Which Of The TX Endpoints Have Disabled Double Packet Buffer Functionality\n\nYou can [`read`](crate::Reg::read) this register and get [`txdpktbufdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdpktbufdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdpktbufdis`] module"]
#[doc(alias = "TXDPKTBUFDIS")]
pub type Txdpktbufdis = crate::Reg<txdpktbufdis::TxdpktbufdisSpec>;
#[doc = "Indicates Which Of The TX Endpoints Have Disabled Double Packet Buffer Functionality"]
pub mod txdpktbufdis;
#[doc = "CTUCH (rw) register accessor: Chirp Timeout. Number Of XCLK Cycles Before The Timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`ctuch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctuch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctuch`] module"]
#[doc(alias = "CTUCH")]
pub type Ctuch = crate::Reg<ctuch::CtuchSpec>;
#[doc = "Chirp Timeout. Number Of XCLK Cycles Before The Timeout"]
pub mod ctuch;
#[doc = "CTHHSRTN (rw) register accessor: Sets The Delay From The End Of High-Speed Resume Signaling (Acting As A Host) To Enable The UTM Normal Operating Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`cthhsrtn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cthhsrtn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cthhsrtn`] module"]
#[doc(alias = "CTHHSRTN")]
pub type Cthhsrtn = crate::Reg<cthhsrtn::CthhsrtnSpec>;
#[doc = "Sets The Delay From The End Of High-Speed Resume Signaling (Acting As A Host) To Enable The UTM Normal Operating Mode"]
pub mod cthhsrtn;
#[doc = "CTHSBT (rw) register accessor: This Register Represents The Value To Be Added To The Minimum High Speed Timeout Period Of 736 Bit Times.\n\nYou can [`read`](crate::Reg::read) this register and get [`cthsbt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cthsbt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cthsbt`] module"]
#[doc(alias = "CTHSBT")]
pub type Cthsbt = crate::Reg<cthsbt::CthsbtSpec>;
#[doc = "This Register Represents The Value To Be Added To The Minimum High Speed Timeout Period Of 736 Bit Times."]
pub mod cthsbt;
#[doc = "LPMATTR (rw) register accessor: Defines The Attributes Of An LPM Transaction And Sleep Cycle.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmattr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmattr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmattr`] module"]
#[doc(alias = "LPMATTR")]
pub type Lpmattr = crate::Reg<lpmattr::LpmattrSpec>;
#[doc = "Defines The Attributes Of An LPM Transaction And Sleep Cycle."]
pub mod lpmattr;
#[doc = "LPMCNTRL_HOST_MODE (rw) register accessor: LPM Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmcntrl_host_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcntrl_host_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmcntrl_host_mode`] module"]
#[doc(alias = "LPMCNTRL_HOST_MODE")]
pub type LpmcntrlHostMode = crate::Reg<lpmcntrl_host_mode::LpmcntrlHostModeSpec>;
#[doc = "LPM Control Register"]
pub mod lpmcntrl_host_mode;
#[doc = "LPMCNTRL_PERIPHERAL_MODE (rw) register accessor: LPM Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmcntrl_peripheral_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcntrl_peripheral_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmcntrl_peripheral_mode`] module"]
#[doc(alias = "LPMCNTRL_PERIPHERAL_MODE")]
pub type LpmcntrlPeripheralMode = crate::Reg<lpmcntrl_peripheral_mode::LpmcntrlPeripheralModeSpec>;
#[doc = "LPM Control Register"]
pub mod lpmcntrl_peripheral_mode;
#[doc = "LPMINTREN (rw) register accessor: LPM Interrupts Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmintren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmintren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmintren`] module"]
#[doc(alias = "LPMINTREN")]
pub type Lpmintren = crate::Reg<lpmintren::LpmintrenSpec>;
#[doc = "LPM Interrupts Enable Register"]
pub mod lpmintren;
#[doc = "LPMINTR_HOST_MODE (rw) register accessor: Status Of The LPM Power State\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmintr_host_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmintr_host_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmintr_host_mode`] module"]
#[doc(alias = "LPMINTR_HOST_MODE")]
pub type LpmintrHostMode = crate::Reg<lpmintr_host_mode::LpmintrHostModeSpec>;
#[doc = "Status Of The LPM Power State"]
pub mod lpmintr_host_mode;
#[doc = "LPMINTR_PERIPHERAL_MODE (rw) register accessor: Status Of The LPM Power State\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmintr_peripheral_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmintr_peripheral_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmintr_peripheral_mode`] module"]
#[doc(alias = "LPMINTR_PERIPHERAL_MODE")]
pub type LpmintrPeripheralMode = crate::Reg<lpmintr_peripheral_mode::LpmintrPeripheralModeSpec>;
#[doc = "Status Of The LPM Power State"]
pub mod lpmintr_peripheral_mode;
#[doc = "LPMFADDR (rw) register accessor: Function Address In LPM Payload\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmfaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmfaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmfaddr`] module"]
#[doc(alias = "LPMFADDR")]
pub type Lpmfaddr = crate::Reg<lpmfaddr::LpmfaddrSpec>;
#[doc = "Function Address In LPM Payload"]
pub mod lpmfaddr;
#[doc = "PHY00 (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy00`] module"]
#[doc(alias = "PHY00")]
pub type Phy00 = crate::Reg<phy00::Phy00Spec>;
#[doc = "USBHPHY Control Register."]
pub mod phy00;
#[doc = "PHY04 (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy04`] module"]
#[doc(alias = "PHY04")]
pub type Phy04 = crate::Reg<phy04::Phy04Spec>;
#[doc = "USBHPHY Control Register."]
pub mod phy04;
#[doc = "PHY08 (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy08`] module"]
#[doc(alias = "PHY08")]
pub type Phy08 = crate::Reg<phy08::Phy08Spec>;
#[doc = "USBHPHY Control Register."]
pub mod phy08;
#[doc = "PHY0C (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy0c`] module"]
#[doc(alias = "PHY0C")]
pub type Phy0c = crate::Reg<phy0c::Phy0cSpec>;
#[doc = "USBHPHY Control Register."]
pub mod phy0c;
#[doc = "PHY10 (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy10`] module"]
#[doc(alias = "PHY10")]
pub type Phy10 = crate::Reg<phy10::Phy10Spec>;
#[doc = "USBHPHY Control Register."]
pub mod phy10;
#[doc = "PHY14 (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy14`] module"]
#[doc(alias = "PHY14")]
pub type Phy14 = crate::Reg<phy14::Phy14Spec>;
#[doc = "USBHPHY Control Register."]
pub mod phy14;
#[doc = "PHY18 (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy18`] module"]
#[doc(alias = "PHY18")]
pub type Phy18 = crate::Reg<phy18::Phy18Spec>;
#[doc = "USBHPHY Control Register."]
pub mod phy18;
#[doc = "PHY1C (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy1c`] module"]
#[doc(alias = "PHY1C")]
pub type Phy1c = crate::Reg<phy1c::Phy1cSpec>;
#[doc = "USBHPHY Control Register."]
pub mod phy1c;
#[doc = "PHY20 (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy20`] module"]
#[doc(alias = "PHY20")]
pub type Phy20 = crate::Reg<phy20::Phy20Spec>;
#[doc = "USBHPHY Control Register."]
pub mod phy20;
#[doc = "PHY24 (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy24`] module"]
#[doc(alias = "PHY24")]
pub type Phy24 = crate::Reg<phy24::Phy24Spec>;
#[doc = "USBHPHY Control Register."]
pub mod phy24;
#[doc = "PHY28 (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy28`] module"]
#[doc(alias = "PHY28")]
pub type Phy28 = crate::Reg<phy28::Phy28Spec>;
#[doc = "USBHPHY Control Register."]
pub mod phy28;
#[doc = "PHY44 (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy44`] module"]
#[doc(alias = "PHY44")]
pub type Phy44 = crate::Reg<phy44::Phy44Spec>;
#[doc = "USBHPHY Control Register."]
pub mod phy44;
#[doc = "PHY48 (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy48`] module"]
#[doc(alias = "PHY48")]
pub type Phy48 = crate::Reg<phy48::Phy48Spec>;
#[doc = "USBHPHY Control Register."]
pub mod phy48;
#[doc = "PHY4C (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy4c`] module"]
#[doc(alias = "PHY4C")]
pub type Phy4c = crate::Reg<phy4c::Phy4cSpec>;
#[doc = "USBHPHY Control Register."]
pub mod phy4c;
#[doc = "PHY50 (rw) register accessor: USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy50`] module"]
#[doc(alias = "PHY50")]
pub type Phy50 = crate::Reg<phy50::Phy50Spec>;
#[doc = "USBHPHY Control Register."]
pub mod phy50;
