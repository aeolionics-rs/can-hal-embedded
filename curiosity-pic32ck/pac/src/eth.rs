#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    _reserved2: [u8; 0x04],
    evctrl: Evctrl,
    _reserved3: [u8; 0x10],
    syncb: Syncb,
    _reserved4: [u8; 0x0fdc],
    ncr: Ncr,
    ncfgr: Ncfgr,
    nsr: Nsr,
    ur: Ur,
    dcfgr: Dcfgr,
    tsr: Tsr,
    rbqb: Rbqb,
    tbqb: Tbqb,
    rsr: Rsr,
    isr: Isr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    man: Man,
    rpq: Rpq,
    tpq: Tpq,
    tpsf: Tpsf,
    rpsf: Rpsf,
    rjfml: Rjfml,
    _reserved23: [u8; 0x34],
    hrb: Hrb,
    hrt: Hrt,
    sa: [Sa; 4],
    tidm: [Tidm; 4],
    wol: Wol,
    ipgs: Ipgs,
    svlan: Svlan,
    tpfcp: Tpfcp,
    samb1: Samb1,
    samt1: Samt1,
    _reserved33: [u8; 0x0c],
    nsc: Nsc,
    scl: Scl,
    sch: Sch,
    eftsh: Eftsh,
    efrsh: Efrsh,
    peftsh: Peftsh,
    pefrsh: Pefrsh,
    _reserved40: [u8; 0x08],
    otlo: Otlo,
    othi: Othi,
    ft: Ft,
    bcft: Bcft,
    mft: Mft,
    pft: Pft,
    bft64: Bft64,
    tbft127: Tbft127,
    tbft255: Tbft255,
    tbft511: Tbft511,
    tbft1023: Tbft1023,
    tbft1518: Tbft1518,
    gtbft1518: Gtbft1518,
    tur: Tur,
    scf: Scf,
    mcf: Mcf,
    ec: Ec,
    lc: Lc,
    dtf: Dtf,
    cse: Cse,
    orlo: Orlo,
    orhi: Orhi,
    fr: Fr,
    bcfr: Bcfr,
    mfr: Mfr,
    pfr: Pfr,
    bfr64: Bfr64,
    tbfr127: Tbfr127,
    tbfr255: Tbfr255,
    tbfr511: Tbfr511,
    tbfr1023: Tbfr1023,
    tbfr1518: Tbfr1518,
    tmxbfr: Tmxbfr,
    ufr: Ufr,
    ofr: Ofr,
    jr: Jr,
    fcse: Fcse,
    lffe: Lffe,
    rse: Rse,
    ae: Ae,
    rre: Rre,
    roe: Roe,
    ihce: Ihce,
    tce: Tce,
    uce: Uce,
    _reserved85: [u8; 0x08],
    tisubn: Tisubn,
    tsh: Tsh,
    _reserved87: [u8; 0x04],
    tsssl: Tsssl,
    tssn: Tssn,
    tsl: Tsl,
    tn: Tn,
    ta: Ta,
    ti: Ti,
    eftsl: Eftsl,
    eftn: Eftn,
    efrsl: Efrsl,
    efrn: Efrn,
    peftsl: Peftsl,
    peftn: Peftn,
    pefrsl: Pefrsl,
    pefrn: Pefrn,
    _reserved101: [u8; 0x70],
    rlpitr: Rlpitr,
    rlpiti: Rlpiti,
    tlpitr: Tlpitr,
    tlpiti: Tlpiti,
}
impl RegisterBlock {
    #[doc = "0x00 - CTRLA Register"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - CTRLB Register"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x0c - Event Control Register"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x20 - Sync Busy Register"]
    #[inline(always)]
    pub const fn syncb(&self) -> &Syncb {
        &self.syncb
    }
    #[doc = "0x1000 - Network Control Register"]
    #[inline(always)]
    pub const fn ncr(&self) -> &Ncr {
        &self.ncr
    }
    #[doc = "0x1004 - Network Configuration Register"]
    #[inline(always)]
    pub const fn ncfgr(&self) -> &Ncfgr {
        &self.ncfgr
    }
    #[doc = "0x1008 - Network Status Register"]
    #[inline(always)]
    pub const fn nsr(&self) -> &Nsr {
        &self.nsr
    }
    #[doc = "0x100c - User Register"]
    #[inline(always)]
    pub const fn ur(&self) -> &Ur {
        &self.ur
    }
    #[doc = "0x1010 - DMA Configuration Register"]
    #[inline(always)]
    pub const fn dcfgr(&self) -> &Dcfgr {
        &self.dcfgr
    }
    #[doc = "0x1014 - Transmit Status Register"]
    #[inline(always)]
    pub const fn tsr(&self) -> &Tsr {
        &self.tsr
    }
    #[doc = "0x1018 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    pub const fn rbqb(&self) -> &Rbqb {
        &self.rbqb
    }
    #[doc = "0x101c - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    pub const fn tbqb(&self) -> &Tbqb {
        &self.tbqb
    }
    #[doc = "0x1020 - Receive Status Register"]
    #[inline(always)]
    pub const fn rsr(&self) -> &Rsr {
        &self.rsr
    }
    #[doc = "0x1024 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x1028 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x102c - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x1030 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x1034 - PHY Maintenance Register"]
    #[inline(always)]
    pub const fn man(&self) -> &Man {
        &self.man
    }
    #[doc = "0x1038 - Received Pause Quantum Register"]
    #[inline(always)]
    pub const fn rpq(&self) -> &Rpq {
        &self.rpq
    }
    #[doc = "0x103c - Transmit Pause Quantum Register"]
    #[inline(always)]
    pub const fn tpq(&self) -> &Tpq {
        &self.tpq
    }
    #[doc = "0x1040 - TX partial store and forward Register"]
    #[inline(always)]
    pub const fn tpsf(&self) -> &Tpsf {
        &self.tpsf
    }
    #[doc = "0x1044 - RX partial store and forward Register"]
    #[inline(always)]
    pub const fn rpsf(&self) -> &Rpsf {
        &self.rpsf
    }
    #[doc = "0x1048 - RX Jumbo Frame Max Length Register"]
    #[inline(always)]
    pub const fn rjfml(&self) -> &Rjfml {
        &self.rjfml
    }
    #[doc = "0x1080 - Hash Register Bottom \\[31:0\\]"]
    #[inline(always)]
    pub const fn hrb(&self) -> &Hrb {
        &self.hrb
    }
    #[doc = "0x1084 - Hash Register Top \\[63:32\\]"]
    #[inline(always)]
    pub const fn hrt(&self) -> &Hrt {
        &self.hrt
    }
    #[doc = "0x1088..0x10a8 - SA\\[%s\\]"]
    #[inline(always)]
    pub const fn sa(&self, n: usize) -> &Sa {
        &self.sa[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1088..0x10a8 - SA\\[%s\\]"]
    #[inline(always)]
    pub fn sa_iter(&self) -> impl Iterator<Item = &Sa> {
        self.sa.iter()
    }
    #[doc = "0x10a8..0x10b8 - Type ID Match Register"]
    #[inline(always)]
    pub const fn tidm(&self, n: usize) -> &Tidm {
        &self.tidm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10a8..0x10b8 - Type ID Match Register"]
    #[inline(always)]
    pub fn tidm_iter(&self) -> impl Iterator<Item = &Tidm> {
        self.tidm.iter()
    }
    #[doc = "0x10b8 - Wake on LAN"]
    #[inline(always)]
    pub const fn wol(&self) -> &Wol {
        &self.wol
    }
    #[doc = "0x10bc - IPG Stretch Register"]
    #[inline(always)]
    pub const fn ipgs(&self) -> &Ipgs {
        &self.ipgs
    }
    #[doc = "0x10c0 - Stacked VLAN Register"]
    #[inline(always)]
    pub const fn svlan(&self) -> &Svlan {
        &self.svlan
    }
    #[doc = "0x10c4 - Transmit PFC Pause Register"]
    #[inline(always)]
    pub const fn tpfcp(&self) -> &Tpfcp {
        &self.tpfcp
    }
    #[doc = "0x10c8 - Specific Address 1 Mask Bottom \\[31:0\\] Register"]
    #[inline(always)]
    pub const fn samb1(&self) -> &Samb1 {
        &self.samb1
    }
    #[doc = "0x10cc - Specific Address 1 Mask Top \\[47:32\\] Register"]
    #[inline(always)]
    pub const fn samt1(&self) -> &Samt1 {
        &self.samt1
    }
    #[doc = "0x10dc - Tsu timer comparison nanoseconds Register"]
    #[inline(always)]
    pub const fn nsc(&self) -> &Nsc {
        &self.nsc
    }
    #[doc = "0x10e0 - Tsu timer second comparison Register"]
    #[inline(always)]
    pub const fn scl(&self) -> &Scl {
        &self.scl
    }
    #[doc = "0x10e4 - Tsu timer second comparison Register"]
    #[inline(always)]
    pub const fn sch(&self) -> &Sch {
        &self.sch
    }
    #[doc = "0x10e8 - PTP Event Frame Transmitted Seconds High Register"]
    #[inline(always)]
    pub const fn eftsh(&self) -> &Eftsh {
        &self.eftsh
    }
    #[doc = "0x10ec - PTP Event Frame Received Seconds High Register"]
    #[inline(always)]
    pub const fn efrsh(&self) -> &Efrsh {
        &self.efrsh
    }
    #[doc = "0x10f0 - PTP Peer Event Frame Transmitted Seconds High Register"]
    #[inline(always)]
    pub const fn peftsh(&self) -> &Peftsh {
        &self.peftsh
    }
    #[doc = "0x10f4 - PTP Peer Event Frame Received Seconds High Register"]
    #[inline(always)]
    pub const fn pefrsh(&self) -> &Pefrsh {
        &self.pefrsh
    }
    #[doc = "0x1100 - Octets Transmitted \\[31:0\\] Register"]
    #[inline(always)]
    pub const fn otlo(&self) -> &Otlo {
        &self.otlo
    }
    #[doc = "0x1104 - Octets Transmitted \\[47:32\\] Register"]
    #[inline(always)]
    pub const fn othi(&self) -> &Othi {
        &self.othi
    }
    #[doc = "0x1108 - Frames Transmitted Register"]
    #[inline(always)]
    pub const fn ft(&self) -> &Ft {
        &self.ft
    }
    #[doc = "0x110c - Broadcast Frames Transmitted Register"]
    #[inline(always)]
    pub const fn bcft(&self) -> &Bcft {
        &self.bcft
    }
    #[doc = "0x1110 - Multicast Frames Transmitted Register"]
    #[inline(always)]
    pub const fn mft(&self) -> &Mft {
        &self.mft
    }
    #[doc = "0x1114 - Pause Frames Transmitted Register"]
    #[inline(always)]
    pub const fn pft(&self) -> &Pft {
        &self.pft
    }
    #[doc = "0x1118 - 64 Byte Frames Transmitted Register"]
    #[inline(always)]
    pub const fn bft64(&self) -> &Bft64 {
        &self.bft64
    }
    #[doc = "0x111c - 65 to 127 Byte Frames Transmitted Register"]
    #[inline(always)]
    pub const fn tbft127(&self) -> &Tbft127 {
        &self.tbft127
    }
    #[doc = "0x1120 - 128 to 255 Byte Frames Transmitted Register"]
    #[inline(always)]
    pub const fn tbft255(&self) -> &Tbft255 {
        &self.tbft255
    }
    #[doc = "0x1124 - 256 to 511 Byte Frames Transmitted Register"]
    #[inline(always)]
    pub const fn tbft511(&self) -> &Tbft511 {
        &self.tbft511
    }
    #[doc = "0x1128 - 512 to 1023 Byte Frames Transmitted Register"]
    #[inline(always)]
    pub const fn tbft1023(&self) -> &Tbft1023 {
        &self.tbft1023
    }
    #[doc = "0x112c - 1024 to 1518 Byte Frames Transmitted Register"]
    #[inline(always)]
    pub const fn tbft1518(&self) -> &Tbft1518 {
        &self.tbft1518
    }
    #[doc = "0x1130 - Greater Than 1518 Byte Frames Transmitted Register"]
    #[inline(always)]
    pub const fn gtbft1518(&self) -> &Gtbft1518 {
        &self.gtbft1518
    }
    #[doc = "0x1134 - Transmit Underruns Register"]
    #[inline(always)]
    pub const fn tur(&self) -> &Tur {
        &self.tur
    }
    #[doc = "0x1138 - Single Collision Frames Register"]
    #[inline(always)]
    pub const fn scf(&self) -> &Scf {
        &self.scf
    }
    #[doc = "0x113c - Multiple Collision Frames Register"]
    #[inline(always)]
    pub const fn mcf(&self) -> &Mcf {
        &self.mcf
    }
    #[doc = "0x1140 - Excessive Collisions Register"]
    #[inline(always)]
    pub const fn ec(&self) -> &Ec {
        &self.ec
    }
    #[doc = "0x1144 - Late Collisions Register"]
    #[inline(always)]
    pub const fn lc(&self) -> &Lc {
        &self.lc
    }
    #[doc = "0x1148 - Deferred Transmission Frames Register"]
    #[inline(always)]
    pub const fn dtf(&self) -> &Dtf {
        &self.dtf
    }
    #[doc = "0x114c - Carrier Sense Errors Register"]
    #[inline(always)]
    pub const fn cse(&self) -> &Cse {
        &self.cse
    }
    #[doc = "0x1150 - Octets Received \\[31:0\\] Received"]
    #[inline(always)]
    pub const fn orlo(&self) -> &Orlo {
        &self.orlo
    }
    #[doc = "0x1154 - Octets Received \\[47:32\\] Received"]
    #[inline(always)]
    pub const fn orhi(&self) -> &Orhi {
        &self.orhi
    }
    #[doc = "0x1158 - Frames Received Register"]
    #[inline(always)]
    pub const fn fr(&self) -> &Fr {
        &self.fr
    }
    #[doc = "0x115c - Broadcast Frames Received Register"]
    #[inline(always)]
    pub const fn bcfr(&self) -> &Bcfr {
        &self.bcfr
    }
    #[doc = "0x1160 - Multicast Frames Received Register"]
    #[inline(always)]
    pub const fn mfr(&self) -> &Mfr {
        &self.mfr
    }
    #[doc = "0x1164 - Pause Frames Received Register"]
    #[inline(always)]
    pub const fn pfr(&self) -> &Pfr {
        &self.pfr
    }
    #[doc = "0x1168 - 64 Byte Frames Received Register"]
    #[inline(always)]
    pub const fn bfr64(&self) -> &Bfr64 {
        &self.bfr64
    }
    #[doc = "0x116c - 65 to 127 Byte Frames Received Register"]
    #[inline(always)]
    pub const fn tbfr127(&self) -> &Tbfr127 {
        &self.tbfr127
    }
    #[doc = "0x1170 - 128 to 255 Byte Frames Received Register"]
    #[inline(always)]
    pub const fn tbfr255(&self) -> &Tbfr255 {
        &self.tbfr255
    }
    #[doc = "0x1174 - 256 to 511Byte Frames Received Register"]
    #[inline(always)]
    pub const fn tbfr511(&self) -> &Tbfr511 {
        &self.tbfr511
    }
    #[doc = "0x1178 - 512 to 1023 Byte Frames Received Register"]
    #[inline(always)]
    pub const fn tbfr1023(&self) -> &Tbfr1023 {
        &self.tbfr1023
    }
    #[doc = "0x117c - 1024 to 1518 Byte Frames Received Register"]
    #[inline(always)]
    pub const fn tbfr1518(&self) -> &Tbfr1518 {
        &self.tbfr1518
    }
    #[doc = "0x1180 - 1519 to Maximum Byte Frames Received Register"]
    #[inline(always)]
    pub const fn tmxbfr(&self) -> &Tmxbfr {
        &self.tmxbfr
    }
    #[doc = "0x1184 - Undersize Frames Received Register"]
    #[inline(always)]
    pub const fn ufr(&self) -> &Ufr {
        &self.ufr
    }
    #[doc = "0x1188 - Oversize Frames Received Register"]
    #[inline(always)]
    pub const fn ofr(&self) -> &Ofr {
        &self.ofr
    }
    #[doc = "0x118c - Jabbers Received Register"]
    #[inline(always)]
    pub const fn jr(&self) -> &Jr {
        &self.jr
    }
    #[doc = "0x1190 - Frame Check Sequence Errors Register"]
    #[inline(always)]
    pub const fn fcse(&self) -> &Fcse {
        &self.fcse
    }
    #[doc = "0x1194 - Length Field Frame Errors Register"]
    #[inline(always)]
    pub const fn lffe(&self) -> &Lffe {
        &self.lffe
    }
    #[doc = "0x1198 - Receive Symbol Errors Register"]
    #[inline(always)]
    pub const fn rse(&self) -> &Rse {
        &self.rse
    }
    #[doc = "0x119c - Alignment Errors Register"]
    #[inline(always)]
    pub const fn ae(&self) -> &Ae {
        &self.ae
    }
    #[doc = "0x11a0 - Receive Resource Errors Register"]
    #[inline(always)]
    pub const fn rre(&self) -> &Rre {
        &self.rre
    }
    #[doc = "0x11a4 - Receive Overrun Register"]
    #[inline(always)]
    pub const fn roe(&self) -> &Roe {
        &self.roe
    }
    #[doc = "0x11a8 - IP Header Checksum Errors Register"]
    #[inline(always)]
    pub const fn ihce(&self) -> &Ihce {
        &self.ihce
    }
    #[doc = "0x11ac - TCP Checksum Errors Register"]
    #[inline(always)]
    pub const fn tce(&self) -> &Tce {
        &self.tce
    }
    #[doc = "0x11b0 - UDP Checksum Errors Register"]
    #[inline(always)]
    pub const fn uce(&self) -> &Uce {
        &self.uce
    }
    #[doc = "0x11bc - 1588 Timer Increment \\[15:0\\] Sub-Nanoseconds Register"]
    #[inline(always)]
    pub const fn tisubn(&self) -> &Tisubn {
        &self.tisubn
    }
    #[doc = "0x11c0 - 1588 Timer Seconds High \\[15:0\\] Register"]
    #[inline(always)]
    pub const fn tsh(&self) -> &Tsh {
        &self.tsh
    }
    #[doc = "0x11c8 - 1588 Timer Sync Strobe Seconds \\[31:0\\] Register"]
    #[inline(always)]
    pub const fn tsssl(&self) -> &Tsssl {
        &self.tsssl
    }
    #[doc = "0x11cc - 1588 Timer Sync Strobe Nanoseconds Register"]
    #[inline(always)]
    pub const fn tssn(&self) -> &Tssn {
        &self.tssn
    }
    #[doc = "0x11d0 - 1588 Timer Seconds \\[31:0\\] Register"]
    #[inline(always)]
    pub const fn tsl(&self) -> &Tsl {
        &self.tsl
    }
    #[doc = "0x11d4 - 1588 Timer Nanoseconds Register"]
    #[inline(always)]
    pub const fn tn(&self) -> &Tn {
        &self.tn
    }
    #[doc = "0x11d8 - 1588 Timer Adjust Register"]
    #[inline(always)]
    pub const fn ta(&self) -> &Ta {
        &self.ta
    }
    #[doc = "0x11dc - 1588 Timer Increment Register"]
    #[inline(always)]
    pub const fn ti(&self) -> &Ti {
        &self.ti
    }
    #[doc = "0x11e0 - PTP Event Frame Transmitted Seconds Low Register"]
    #[inline(always)]
    pub const fn eftsl(&self) -> &Eftsl {
        &self.eftsl
    }
    #[doc = "0x11e4 - PTP Event Frame Transmitted Nanoseconds"]
    #[inline(always)]
    pub const fn eftn(&self) -> &Eftn {
        &self.eftn
    }
    #[doc = "0x11e8 - PTP Event Frame Received Seconds Low Register"]
    #[inline(always)]
    pub const fn efrsl(&self) -> &Efrsl {
        &self.efrsl
    }
    #[doc = "0x11ec - PTP Event Frame Received Nanoseconds"]
    #[inline(always)]
    pub const fn efrn(&self) -> &Efrn {
        &self.efrn
    }
    #[doc = "0x11f0 - PTP Peer Event Frame Transmitted Seconds Low Register"]
    #[inline(always)]
    pub const fn peftsl(&self) -> &Peftsl {
        &self.peftsl
    }
    #[doc = "0x11f4 - PTP Peer Event Frame Transmitted Nanoseconds"]
    #[inline(always)]
    pub const fn peftn(&self) -> &Peftn {
        &self.peftn
    }
    #[doc = "0x11f8 - PTP Peer Event Frame Received Seconds Low Register"]
    #[inline(always)]
    pub const fn pefrsl(&self) -> &Pefrsl {
        &self.pefrsl
    }
    #[doc = "0x11fc - PTP Peer Event Frame Received Nanoseconds"]
    #[inline(always)]
    pub const fn pefrn(&self) -> &Pefrn {
        &self.pefrn
    }
    #[doc = "0x1270 - Receive LPI transition Register"]
    #[inline(always)]
    pub const fn rlpitr(&self) -> &Rlpitr {
        &self.rlpitr
    }
    #[doc = "0x1274 - Receive LPI Time Register"]
    #[inline(always)]
    pub const fn rlpiti(&self) -> &Rlpiti {
        &self.rlpiti
    }
    #[doc = "0x1278 - Receive LPI transition Register"]
    #[inline(always)]
    pub const fn tlpitr(&self) -> &Tlpitr {
        &self.tlpitr
    }
    #[doc = "0x127c - Receive LPI Time Register"]
    #[inline(always)]
    pub const fn tlpiti(&self) -> &Tlpiti {
        &self.tlpiti
    }
}
#[doc = "CTRLA (rw) register accessor: CTRLA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "CTRLA Register"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: CTRLB Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "CTRLB Register"]
pub mod ctrlb;
#[doc = "EVCTRL (rw) register accessor: Event Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`] module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "Event Control Register"]
pub mod evctrl;
#[doc = "SYNCB (r) register accessor: Sync Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncb`] module"]
#[doc(alias = "SYNCB")]
pub type Syncb = crate::Reg<syncb::SyncbSpec>;
#[doc = "Sync Busy Register"]
pub mod syncb;
#[doc = "NCR (rw) register accessor: Network Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ncr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncr`] module"]
#[doc(alias = "NCR")]
pub type Ncr = crate::Reg<ncr::NcrSpec>;
#[doc = "Network Control Register"]
pub mod ncr;
#[doc = "NCFGR (rw) register accessor: Network Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ncfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncfgr`] module"]
#[doc(alias = "NCFGR")]
pub type Ncfgr = crate::Reg<ncfgr::NcfgrSpec>;
#[doc = "Network Configuration Register"]
pub mod ncfgr;
#[doc = "NSR (r) register accessor: Network Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsr`] module"]
#[doc(alias = "NSR")]
pub type Nsr = crate::Reg<nsr::NsrSpec>;
#[doc = "Network Status Register"]
pub mod nsr;
#[doc = "UR (rw) register accessor: User Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur`] module"]
#[doc(alias = "UR")]
pub type Ur = crate::Reg<ur::UrSpec>;
#[doc = "User Register"]
pub mod ur;
#[doc = "DCFGR (rw) register accessor: DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfgr`] module"]
#[doc(alias = "DCFGR")]
pub type Dcfgr = crate::Reg<dcfgr::DcfgrSpec>;
#[doc = "DMA Configuration Register"]
pub mod dcfgr;
#[doc = "TSR (rw) register accessor: Transmit Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsr`] module"]
#[doc(alias = "TSR")]
pub type Tsr = crate::Reg<tsr::TsrSpec>;
#[doc = "Transmit Status Register"]
pub mod tsr;
#[doc = "RBQB (rw) register accessor: Receive Buffer Queue Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`rbqb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbqb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbqb`] module"]
#[doc(alias = "RBQB")]
pub type Rbqb = crate::Reg<rbqb::RbqbSpec>;
#[doc = "Receive Buffer Queue Base Address"]
pub mod rbqb;
#[doc = "TBQB (rw) register accessor: Transmit Buffer Queue Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tbqb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbqb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbqb`] module"]
#[doc(alias = "TBQB")]
pub type Tbqb = crate::Reg<tbqb::TbqbSpec>;
#[doc = "Transmit Buffer Queue Base Address"]
pub mod tbqb;
#[doc = "RSR (rw) register accessor: Receive Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`] module"]
#[doc(alias = "RSR")]
pub type Rsr = crate::Reg<rsr::RsrSpec>;
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "ISR (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "MAN (rw) register accessor: PHY Maintenance Register\n\nYou can [`read`](crate::Reg::read) this register and get [`man::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`man::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@man`] module"]
#[doc(alias = "MAN")]
pub type Man = crate::Reg<man::ManSpec>;
#[doc = "PHY Maintenance Register"]
pub mod man;
#[doc = "RPQ (r) register accessor: Received Pause Quantum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpq::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpq`] module"]
#[doc(alias = "RPQ")]
pub type Rpq = crate::Reg<rpq::RpqSpec>;
#[doc = "Received Pause Quantum Register"]
pub mod rpq;
#[doc = "TPQ (rw) register accessor: Transmit Pause Quantum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpq`] module"]
#[doc(alias = "TPQ")]
pub type Tpq = crate::Reg<tpq::TpqSpec>;
#[doc = "Transmit Pause Quantum Register"]
pub mod tpq;
#[doc = "TPSF (rw) register accessor: TX partial store and forward Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpsf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpsf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpsf`] module"]
#[doc(alias = "TPSF")]
pub type Tpsf = crate::Reg<tpsf::TpsfSpec>;
#[doc = "TX partial store and forward Register"]
pub mod tpsf;
#[doc = "RPSF (rw) register accessor: RX partial store and forward Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpsf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpsf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpsf`] module"]
#[doc(alias = "RPSF")]
pub type Rpsf = crate::Reg<rpsf::RpsfSpec>;
#[doc = "RX partial store and forward Register"]
pub mod rpsf;
#[doc = "RJFML (rw) register accessor: RX Jumbo Frame Max Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rjfml::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rjfml::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rjfml`] module"]
#[doc(alias = "RJFML")]
pub type Rjfml = crate::Reg<rjfml::RjfmlSpec>;
#[doc = "RX Jumbo Frame Max Length Register"]
pub mod rjfml;
#[doc = "HRB (rw) register accessor: Hash Register Bottom \\[31:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`hrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrb`] module"]
#[doc(alias = "HRB")]
pub type Hrb = crate::Reg<hrb::HrbSpec>;
#[doc = "Hash Register Bottom \\[31:0\\]"]
pub mod hrb;
#[doc = "HRT (rw) register accessor: Hash Register Top \\[63:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`hrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrt`] module"]
#[doc(alias = "HRT")]
pub type Hrt = crate::Reg<hrt::HrtSpec>;
#[doc = "Hash Register Top \\[63:32\\]"]
pub mod hrt;
#[doc = "SA\\[%s\\]"]
pub use self::sa::Sa;
#[doc = r"Cluster"]
#[doc = "SA\\[%s\\]"]
pub mod sa;
#[doc = "TIDM (rw) register accessor: Type ID Match Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tidm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tidm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tidm`] module"]
#[doc(alias = "TIDM")]
pub type Tidm = crate::Reg<tidm::TidmSpec>;
#[doc = "Type ID Match Register"]
pub mod tidm;
#[doc = "WOL (rw) register accessor: Wake on LAN\n\nYou can [`read`](crate::Reg::read) this register and get [`wol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wol`] module"]
#[doc(alias = "WOL")]
pub type Wol = crate::Reg<wol::WolSpec>;
#[doc = "Wake on LAN"]
pub mod wol;
#[doc = "IPGS (rw) register accessor: IPG Stretch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipgs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipgs`] module"]
#[doc(alias = "IPGS")]
pub type Ipgs = crate::Reg<ipgs::IpgsSpec>;
#[doc = "IPG Stretch Register"]
pub mod ipgs;
#[doc = "SVLAN (rw) register accessor: Stacked VLAN Register\n\nYou can [`read`](crate::Reg::read) this register and get [`svlan::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svlan::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@svlan`] module"]
#[doc(alias = "SVLAN")]
pub type Svlan = crate::Reg<svlan::SvlanSpec>;
#[doc = "Stacked VLAN Register"]
pub mod svlan;
#[doc = "TPFCP (rw) register accessor: Transmit PFC Pause Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpfcp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpfcp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpfcp`] module"]
#[doc(alias = "TPFCP")]
pub type Tpfcp = crate::Reg<tpfcp::TpfcpSpec>;
#[doc = "Transmit PFC Pause Register"]
pub mod tpfcp;
#[doc = "SAMB1 (rw) register accessor: Specific Address 1 Mask Bottom \\[31:0\\] Register\n\nYou can [`read`](crate::Reg::read) this register and get [`samb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@samb1`] module"]
#[doc(alias = "SAMB1")]
pub type Samb1 = crate::Reg<samb1::Samb1Spec>;
#[doc = "Specific Address 1 Mask Bottom \\[31:0\\] Register"]
pub mod samb1;
#[doc = "SAMT1 (rw) register accessor: Specific Address 1 Mask Top \\[47:32\\] Register\n\nYou can [`read`](crate::Reg::read) this register and get [`samt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@samt1`] module"]
#[doc(alias = "SAMT1")]
pub type Samt1 = crate::Reg<samt1::Samt1Spec>;
#[doc = "Specific Address 1 Mask Top \\[47:32\\] Register"]
pub mod samt1;
#[doc = "NSC (rw) register accessor: Tsu timer comparison nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsc`] module"]
#[doc(alias = "NSC")]
pub type Nsc = crate::Reg<nsc::NscSpec>;
#[doc = "Tsu timer comparison nanoseconds Register"]
pub mod nsc;
#[doc = "SCL (rw) register accessor: Tsu timer second comparison Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl`] module"]
#[doc(alias = "SCL")]
pub type Scl = crate::Reg<scl::SclSpec>;
#[doc = "Tsu timer second comparison Register"]
pub mod scl;
#[doc = "SCH (rw) register accessor: Tsu timer second comparison Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sch`] module"]
#[doc(alias = "SCH")]
pub type Sch = crate::Reg<sch::SchSpec>;
#[doc = "Tsu timer second comparison Register"]
pub mod sch;
#[doc = "EFTSH (r) register accessor: PTP Event Frame Transmitted Seconds High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eftsh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eftsh`] module"]
#[doc(alias = "EFTSH")]
pub type Eftsh = crate::Reg<eftsh::EftshSpec>;
#[doc = "PTP Event Frame Transmitted Seconds High Register"]
pub mod eftsh;
#[doc = "EFRSH (r) register accessor: PTP Event Frame Received Seconds High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`efrsh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efrsh`] module"]
#[doc(alias = "EFRSH")]
pub type Efrsh = crate::Reg<efrsh::EfrshSpec>;
#[doc = "PTP Event Frame Received Seconds High Register"]
pub mod efrsh;
#[doc = "PEFTSH (r) register accessor: PTP Peer Event Frame Transmitted Seconds High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`peftsh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peftsh`] module"]
#[doc(alias = "PEFTSH")]
pub type Peftsh = crate::Reg<peftsh::PeftshSpec>;
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register"]
pub mod peftsh;
#[doc = "PEFRSH (r) register accessor: PTP Peer Event Frame Received Seconds High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pefrsh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pefrsh`] module"]
#[doc(alias = "PEFRSH")]
pub type Pefrsh = crate::Reg<pefrsh::PefrshSpec>;
#[doc = "PTP Peer Event Frame Received Seconds High Register"]
pub mod pefrsh;
#[doc = "OTLO (r) register accessor: Octets Transmitted \\[31:0\\] Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otlo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otlo`] module"]
#[doc(alias = "OTLO")]
pub type Otlo = crate::Reg<otlo::OtloSpec>;
#[doc = "Octets Transmitted \\[31:0\\] Register"]
pub mod otlo;
#[doc = "OTHI (r) register accessor: Octets Transmitted \\[47:32\\] Register\n\nYou can [`read`](crate::Reg::read) this register and get [`othi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@othi`] module"]
#[doc(alias = "OTHI")]
pub type Othi = crate::Reg<othi::OthiSpec>;
#[doc = "Octets Transmitted \\[47:32\\] Register"]
pub mod othi;
#[doc = "FT (r) register accessor: Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ft::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ft`] module"]
#[doc(alias = "FT")]
pub type Ft = crate::Reg<ft::FtSpec>;
#[doc = "Frames Transmitted Register"]
pub mod ft;
#[doc = "BCFT (r) register accessor: Broadcast Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcft::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcft`] module"]
#[doc(alias = "BCFT")]
pub type Bcft = crate::Reg<bcft::BcftSpec>;
#[doc = "Broadcast Frames Transmitted Register"]
pub mod bcft;
#[doc = "MFT (r) register accessor: Multicast Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mft::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mft`] module"]
#[doc(alias = "MFT")]
pub type Mft = crate::Reg<mft::MftSpec>;
#[doc = "Multicast Frames Transmitted Register"]
pub mod mft;
#[doc = "PFT (r) register accessor: Pause Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pft::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pft`] module"]
#[doc(alias = "PFT")]
pub type Pft = crate::Reg<pft::PftSpec>;
#[doc = "Pause Frames Transmitted Register"]
pub mod pft;
#[doc = "BFT64 (r) register accessor: 64 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bft64::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bft64`] module"]
#[doc(alias = "BFT64")]
pub type Bft64 = crate::Reg<bft64::Bft64Spec>;
#[doc = "64 Byte Frames Transmitted Register"]
pub mod bft64;
#[doc = "TBFT127 (r) register accessor: 65 to 127 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft127::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft127`] module"]
#[doc(alias = "TBFT127")]
pub type Tbft127 = crate::Reg<tbft127::Tbft127Spec>;
#[doc = "65 to 127 Byte Frames Transmitted Register"]
pub mod tbft127;
#[doc = "TBFT255 (r) register accessor: 128 to 255 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft255::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft255`] module"]
#[doc(alias = "TBFT255")]
pub type Tbft255 = crate::Reg<tbft255::Tbft255Spec>;
#[doc = "128 to 255 Byte Frames Transmitted Register"]
pub mod tbft255;
#[doc = "TBFT511 (r) register accessor: 256 to 511 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft511::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft511`] module"]
#[doc(alias = "TBFT511")]
pub type Tbft511 = crate::Reg<tbft511::Tbft511Spec>;
#[doc = "256 to 511 Byte Frames Transmitted Register"]
pub mod tbft511;
#[doc = "TBFT1023 (r) register accessor: 512 to 1023 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft1023::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft1023`] module"]
#[doc(alias = "TBFT1023")]
pub type Tbft1023 = crate::Reg<tbft1023::Tbft1023Spec>;
#[doc = "512 to 1023 Byte Frames Transmitted Register"]
pub mod tbft1023;
#[doc = "TBFT1518 (r) register accessor: 1024 to 1518 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft1518::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft1518`] module"]
#[doc(alias = "TBFT1518")]
pub type Tbft1518 = crate::Reg<tbft1518::Tbft1518Spec>;
#[doc = "1024 to 1518 Byte Frames Transmitted Register"]
pub mod tbft1518;
#[doc = "GTBFT1518 (r) register accessor: Greater Than 1518 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtbft1518::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtbft1518`] module"]
#[doc(alias = "GTBFT1518")]
pub type Gtbft1518 = crate::Reg<gtbft1518::Gtbft1518Spec>;
#[doc = "Greater Than 1518 Byte Frames Transmitted Register"]
pub mod gtbft1518;
#[doc = "TUR (r) register accessor: Transmit Underruns Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tur`] module"]
#[doc(alias = "TUR")]
pub type Tur = crate::Reg<tur::TurSpec>;
#[doc = "Transmit Underruns Register"]
pub mod tur;
#[doc = "SCF (r) register accessor: Single Collision Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scf`] module"]
#[doc(alias = "SCF")]
pub type Scf = crate::Reg<scf::ScfSpec>;
#[doc = "Single Collision Frames Register"]
pub mod scf;
#[doc = "MCF (r) register accessor: Multiple Collision Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcf`] module"]
#[doc(alias = "MCF")]
pub type Mcf = crate::Reg<mcf::McfSpec>;
#[doc = "Multiple Collision Frames Register"]
pub mod mcf;
#[doc = "EC (r) register accessor: Excessive Collisions Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ec`] module"]
#[doc(alias = "EC")]
pub type Ec = crate::Reg<ec::EcSpec>;
#[doc = "Excessive Collisions Register"]
pub mod ec;
#[doc = "LC (r) register accessor: Late Collisions Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc`] module"]
#[doc(alias = "LC")]
pub type Lc = crate::Reg<lc::LcSpec>;
#[doc = "Late Collisions Register"]
pub mod lc;
#[doc = "DTF (r) register accessor: Deferred Transmission Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtf`] module"]
#[doc(alias = "DTF")]
pub type Dtf = crate::Reg<dtf::DtfSpec>;
#[doc = "Deferred Transmission Frames Register"]
pub mod dtf;
#[doc = "CSE (r) register accessor: Carrier Sense Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cse::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cse`] module"]
#[doc(alias = "CSE")]
pub type Cse = crate::Reg<cse::CseSpec>;
#[doc = "Carrier Sense Errors Register"]
pub mod cse;
#[doc = "ORLO (r) register accessor: Octets Received \\[31:0\\] Received\n\nYou can [`read`](crate::Reg::read) this register and get [`orlo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@orlo`] module"]
#[doc(alias = "ORLO")]
pub type Orlo = crate::Reg<orlo::OrloSpec>;
#[doc = "Octets Received \\[31:0\\] Received"]
pub mod orlo;
#[doc = "ORHI (r) register accessor: Octets Received \\[47:32\\] Received\n\nYou can [`read`](crate::Reg::read) this register and get [`orhi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@orhi`] module"]
#[doc(alias = "ORHI")]
pub type Orhi = crate::Reg<orhi::OrhiSpec>;
#[doc = "Octets Received \\[47:32\\] Received"]
pub mod orhi;
#[doc = "FR (r) register accessor: Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fr`] module"]
#[doc(alias = "FR")]
pub type Fr = crate::Reg<fr::FrSpec>;
#[doc = "Frames Received Register"]
pub mod fr;
#[doc = "BCFR (r) register accessor: Broadcast Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcfr`] module"]
#[doc(alias = "BCFR")]
pub type Bcfr = crate::Reg<bcfr::BcfrSpec>;
#[doc = "Broadcast Frames Received Register"]
pub mod bcfr;
#[doc = "MFR (r) register accessor: Multicast Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfr`] module"]
#[doc(alias = "MFR")]
pub type Mfr = crate::Reg<mfr::MfrSpec>;
#[doc = "Multicast Frames Received Register"]
pub mod mfr;
#[doc = "PFR (r) register accessor: Pause Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr`] module"]
#[doc(alias = "PFR")]
pub type Pfr = crate::Reg<pfr::PfrSpec>;
#[doc = "Pause Frames Received Register"]
pub mod pfr;
#[doc = "BFR64 (r) register accessor: 64 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bfr64::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bfr64`] module"]
#[doc(alias = "BFR64")]
pub type Bfr64 = crate::Reg<bfr64::Bfr64Spec>;
#[doc = "64 Byte Frames Received Register"]
pub mod bfr64;
#[doc = "TBFR127 (r) register accessor: 65 to 127 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr127::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr127`] module"]
#[doc(alias = "TBFR127")]
pub type Tbfr127 = crate::Reg<tbfr127::Tbfr127Spec>;
#[doc = "65 to 127 Byte Frames Received Register"]
pub mod tbfr127;
#[doc = "TBFR255 (r) register accessor: 128 to 255 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr255::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr255`] module"]
#[doc(alias = "TBFR255")]
pub type Tbfr255 = crate::Reg<tbfr255::Tbfr255Spec>;
#[doc = "128 to 255 Byte Frames Received Register"]
pub mod tbfr255;
#[doc = "TBFR511 (r) register accessor: 256 to 511Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr511::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr511`] module"]
#[doc(alias = "TBFR511")]
pub type Tbfr511 = crate::Reg<tbfr511::Tbfr511Spec>;
#[doc = "256 to 511Byte Frames Received Register"]
pub mod tbfr511;
#[doc = "TBFR1023 (r) register accessor: 512 to 1023 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr1023::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr1023`] module"]
#[doc(alias = "TBFR1023")]
pub type Tbfr1023 = crate::Reg<tbfr1023::Tbfr1023Spec>;
#[doc = "512 to 1023 Byte Frames Received Register"]
pub mod tbfr1023;
#[doc = "TBFR1518 (r) register accessor: 1024 to 1518 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr1518::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr1518`] module"]
#[doc(alias = "TBFR1518")]
pub type Tbfr1518 = crate::Reg<tbfr1518::Tbfr1518Spec>;
#[doc = "1024 to 1518 Byte Frames Received Register"]
pub mod tbfr1518;
#[doc = "TMXBFR (r) register accessor: 1519 to Maximum Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmxbfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmxbfr`] module"]
#[doc(alias = "TMXBFR")]
pub type Tmxbfr = crate::Reg<tmxbfr::TmxbfrSpec>;
#[doc = "1519 to Maximum Byte Frames Received Register"]
pub mod tmxbfr;
#[doc = "UFR (r) register accessor: Undersize Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ufr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ufr`] module"]
#[doc(alias = "UFR")]
pub type Ufr = crate::Reg<ufr::UfrSpec>;
#[doc = "Undersize Frames Received Register"]
pub mod ufr;
#[doc = "OFR (r) register accessor: Oversize Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr`] module"]
#[doc(alias = "OFR")]
pub type Ofr = crate::Reg<ofr::OfrSpec>;
#[doc = "Oversize Frames Received Register"]
pub mod ofr;
#[doc = "JR (r) register accessor: Jabbers Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`jr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jr`] module"]
#[doc(alias = "JR")]
pub type Jr = crate::Reg<jr::JrSpec>;
#[doc = "Jabbers Received Register"]
pub mod jr;
#[doc = "FCSE (r) register accessor: Frame Check Sequence Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcse::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcse`] module"]
#[doc(alias = "FCSE")]
pub type Fcse = crate::Reg<fcse::FcseSpec>;
#[doc = "Frame Check Sequence Errors Register"]
pub mod fcse;
#[doc = "LFFE (r) register accessor: Length Field Frame Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lffe::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lffe`] module"]
#[doc(alias = "LFFE")]
pub type Lffe = crate::Reg<lffe::LffeSpec>;
#[doc = "Length Field Frame Errors Register"]
pub mod lffe;
#[doc = "RSE (r) register accessor: Receive Symbol Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rse::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rse`] module"]
#[doc(alias = "RSE")]
pub type Rse = crate::Reg<rse::RseSpec>;
#[doc = "Receive Symbol Errors Register"]
pub mod rse;
#[doc = "AE (r) register accessor: Alignment Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ae::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae`] module"]
#[doc(alias = "AE")]
pub type Ae = crate::Reg<ae::AeSpec>;
#[doc = "Alignment Errors Register"]
pub mod ae;
#[doc = "RRE (r) register accessor: Receive Resource Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rre::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rre`] module"]
#[doc(alias = "RRE")]
pub type Rre = crate::Reg<rre::RreSpec>;
#[doc = "Receive Resource Errors Register"]
pub mod rre;
#[doc = "ROE (r) register accessor: Receive Overrun Register\n\nYou can [`read`](crate::Reg::read) this register and get [`roe::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@roe`] module"]
#[doc(alias = "ROE")]
pub type Roe = crate::Reg<roe::RoeSpec>;
#[doc = "Receive Overrun Register"]
pub mod roe;
#[doc = "IHCE (r) register accessor: IP Header Checksum Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ihce::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ihce`] module"]
#[doc(alias = "IHCE")]
pub type Ihce = crate::Reg<ihce::IhceSpec>;
#[doc = "IP Header Checksum Errors Register"]
pub mod ihce;
#[doc = "TCE (r) register accessor: TCP Checksum Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tce::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tce`] module"]
#[doc(alias = "TCE")]
pub type Tce = crate::Reg<tce::TceSpec>;
#[doc = "TCP Checksum Errors Register"]
pub mod tce;
#[doc = "UCE (r) register accessor: UDP Checksum Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uce::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uce`] module"]
#[doc(alias = "UCE")]
pub type Uce = crate::Reg<uce::UceSpec>;
#[doc = "UDP Checksum Errors Register"]
pub mod uce;
#[doc = "TISUBN (rw) register accessor: 1588 Timer Increment \\[15:0\\] Sub-Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tisubn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisubn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tisubn`] module"]
#[doc(alias = "TISUBN")]
pub type Tisubn = crate::Reg<tisubn::TisubnSpec>;
#[doc = "1588 Timer Increment \\[15:0\\] Sub-Nanoseconds Register"]
pub mod tisubn;
#[doc = "TSH (rw) register accessor: 1588 Timer Seconds High \\[15:0\\] Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsh`] module"]
#[doc(alias = "TSH")]
pub type Tsh = crate::Reg<tsh::TshSpec>;
#[doc = "1588 Timer Seconds High \\[15:0\\] Register"]
pub mod tsh;
#[doc = "TSSSL (rw) register accessor: 1588 Timer Sync Strobe Seconds \\[31:0\\] Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsssl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsssl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsssl`] module"]
#[doc(alias = "TSSSL")]
pub type Tsssl = crate::Reg<tsssl::TssslSpec>;
#[doc = "1588 Timer Sync Strobe Seconds \\[31:0\\] Register"]
pub mod tsssl;
#[doc = "TSSN (rw) register accessor: 1588 Timer Sync Strobe Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tssn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tssn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tssn`] module"]
#[doc(alias = "TSSN")]
pub type Tssn = crate::Reg<tssn::TssnSpec>;
#[doc = "1588 Timer Sync Strobe Nanoseconds Register"]
pub mod tssn;
#[doc = "TSL (rw) register accessor: 1588 Timer Seconds \\[31:0\\] Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsl`] module"]
#[doc(alias = "TSL")]
pub type Tsl = crate::Reg<tsl::TslSpec>;
#[doc = "1588 Timer Seconds \\[31:0\\] Register"]
pub mod tsl;
#[doc = "TN (rw) register accessor: 1588 Timer Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn`] module"]
#[doc(alias = "TN")]
pub type Tn = crate::Reg<tn::TnSpec>;
#[doc = "1588 Timer Nanoseconds Register"]
pub mod tn;
#[doc = "TA (w) register accessor: 1588 Timer Adjust Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta`] module"]
#[doc(alias = "TA")]
pub type Ta = crate::Reg<ta::TaSpec>;
#[doc = "1588 Timer Adjust Register"]
pub mod ta;
#[doc = "TI (rw) register accessor: 1588 Timer Increment Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ti`] module"]
#[doc(alias = "TI")]
pub type Ti = crate::Reg<ti::TiSpec>;
#[doc = "1588 Timer Increment Register"]
pub mod ti;
#[doc = "EFTSL (r) register accessor: PTP Event Frame Transmitted Seconds Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eftsl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eftsl`] module"]
#[doc(alias = "EFTSL")]
pub type Eftsl = crate::Reg<eftsl::EftslSpec>;
#[doc = "PTP Event Frame Transmitted Seconds Low Register"]
pub mod eftsl;
#[doc = "EFTN (r) register accessor: PTP Event Frame Transmitted Nanoseconds\n\nYou can [`read`](crate::Reg::read) this register and get [`eftn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eftn`] module"]
#[doc(alias = "EFTN")]
pub type Eftn = crate::Reg<eftn::EftnSpec>;
#[doc = "PTP Event Frame Transmitted Nanoseconds"]
pub mod eftn;
#[doc = "EFRSL (r) register accessor: PTP Event Frame Received Seconds Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`efrsl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efrsl`] module"]
#[doc(alias = "EFRSL")]
pub type Efrsl = crate::Reg<efrsl::EfrslSpec>;
#[doc = "PTP Event Frame Received Seconds Low Register"]
pub mod efrsl;
#[doc = "EFRN (r) register accessor: PTP Event Frame Received Nanoseconds\n\nYou can [`read`](crate::Reg::read) this register and get [`efrn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efrn`] module"]
#[doc(alias = "EFRN")]
pub type Efrn = crate::Reg<efrn::EfrnSpec>;
#[doc = "PTP Event Frame Received Nanoseconds"]
pub mod efrn;
#[doc = "PEFTSL (r) register accessor: PTP Peer Event Frame Transmitted Seconds Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`peftsl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peftsl`] module"]
#[doc(alias = "PEFTSL")]
pub type Peftsl = crate::Reg<peftsl::PeftslSpec>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register"]
pub mod peftsl;
#[doc = "PEFTN (r) register accessor: PTP Peer Event Frame Transmitted Nanoseconds\n\nYou can [`read`](crate::Reg::read) this register and get [`peftn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peftn`] module"]
#[doc(alias = "PEFTN")]
pub type Peftn = crate::Reg<peftn::PeftnSpec>;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds"]
pub mod peftn;
#[doc = "PEFRSL (r) register accessor: PTP Peer Event Frame Received Seconds Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pefrsl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pefrsl`] module"]
#[doc(alias = "PEFRSL")]
pub type Pefrsl = crate::Reg<pefrsl::PefrslSpec>;
#[doc = "PTP Peer Event Frame Received Seconds Low Register"]
pub mod pefrsl;
#[doc = "PEFRN (r) register accessor: PTP Peer Event Frame Received Nanoseconds\n\nYou can [`read`](crate::Reg::read) this register and get [`pefrn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pefrn`] module"]
#[doc(alias = "PEFRN")]
pub type Pefrn = crate::Reg<pefrn::PefrnSpec>;
#[doc = "PTP Peer Event Frame Received Nanoseconds"]
pub mod pefrn;
#[doc = "RLPITR (r) register accessor: Receive LPI transition Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rlpitr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlpitr`] module"]
#[doc(alias = "RLPITR")]
pub type Rlpitr = crate::Reg<rlpitr::RlpitrSpec>;
#[doc = "Receive LPI transition Register"]
pub mod rlpitr;
#[doc = "RLPITI (r) register accessor: Receive LPI Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rlpiti::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlpiti`] module"]
#[doc(alias = "RLPITI")]
pub type Rlpiti = crate::Reg<rlpiti::RlpitiSpec>;
#[doc = "Receive LPI Time Register"]
pub mod rlpiti;
#[doc = "TLPITR (r) register accessor: Receive LPI transition Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tlpitr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlpitr`] module"]
#[doc(alias = "TLPITR")]
pub type Tlpitr = crate::Reg<tlpitr::TlpitrSpec>;
#[doc = "Receive LPI transition Register"]
pub mod tlpitr;
#[doc = "TLPITI (r) register accessor: Receive LPI Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tlpiti::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlpiti`] module"]
#[doc(alias = "TLPITI")]
pub type Tlpiti = crate::Reg<tlpiti::TlpitiSpec>;
#[doc = "Receive LPI Time Register"]
pub mod tlpiti;
