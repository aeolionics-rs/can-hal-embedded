#[doc = "Register `SELCTRL` reader"]
pub type R = crate::R<SelctrlSpec>;
#[doc = "Register `SELCTRL` writer"]
pub type W = crate::W<SelctrlSpec>;
#[doc = "Serial Clock Input Delay for SDI sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkindlyselect {
    #[doc = "0: 0 Tap Delays"]
    Tdly0 = 0,
    #[doc = "1: 1 Tap Delay"]
    Tdly1 = 1,
    #[doc = "2: 2 Tap Delay"]
    Tdly2 = 2,
    #[doc = "3: 3 Tap Delay"]
    Tdly3 = 3,
}
impl From<Clkindlyselect> for u8 {
    #[inline(always)]
    fn from(variant: Clkindlyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkindlyselect {
    type Ux = u8;
}
impl crate::IsEnum for Clkindlyselect {}
#[doc = "Field `CLKINDLY` reader - Serial Clock Input Delay for SDI sampling"]
pub type ClkindlyR = crate::FieldReader<Clkindlyselect>;
impl ClkindlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkindlyselect {
        match self.bits {
            0 => Clkindlyselect::Tdly0,
            1 => Clkindlyselect::Tdly1,
            2 => Clkindlyselect::Tdly2,
            3 => Clkindlyselect::Tdly3,
            _ => unreachable!(),
        }
    }
    #[doc = "0 Tap Delays"]
    #[inline(always)]
    pub fn is_tdly0(&self) -> bool {
        *self == Clkindlyselect::Tdly0
    }
    #[doc = "1 Tap Delay"]
    #[inline(always)]
    pub fn is_tdly1(&self) -> bool {
        *self == Clkindlyselect::Tdly1
    }
    #[doc = "2 Tap Delay"]
    #[inline(always)]
    pub fn is_tdly2(&self) -> bool {
        *self == Clkindlyselect::Tdly2
    }
    #[doc = "3 Tap Delay"]
    #[inline(always)]
    pub fn is_tdly3(&self) -> bool {
        *self == Clkindlyselect::Tdly3
    }
}
#[doc = "Field `CLKINDLY` writer - Serial Clock Input Delay for SDI sampling"]
pub type ClkindlyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clkindlyselect, crate::Safe>;
impl<'a, REG> ClkindlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 Tap Delays"]
    #[inline(always)]
    pub fn tdly0(self) -> &'a mut crate::W<REG> {
        self.variant(Clkindlyselect::Tdly0)
    }
    #[doc = "1 Tap Delay"]
    #[inline(always)]
    pub fn tdly1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkindlyselect::Tdly1)
    }
    #[doc = "2 Tap Delay"]
    #[inline(always)]
    pub fn tdly2(self) -> &'a mut crate::W<REG> {
        self.variant(Clkindlyselect::Tdly2)
    }
    #[doc = "3 Tap Delay"]
    #[inline(always)]
    pub fn tdly3(self) -> &'a mut crate::W<REG> {
        self.variant(Clkindlyselect::Tdly3)
    }
}
#[doc = "SPI Receive Service Request Interrupt Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srxiselselect {
    #[doc = "0: RXB Empty"]
    Rxbempty = 0,
    #[doc = "1: RXB Not Empty"]
    Rxbnempty = 1,
    #[doc = "2: RXB Half Full"]
    Rxbhfull = 2,
    #[doc = "3: RXB Full"]
    Rxbfull = 3,
}
impl From<Srxiselselect> for u8 {
    #[inline(always)]
    fn from(variant: Srxiselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srxiselselect {
    type Ux = u8;
}
impl crate::IsEnum for Srxiselselect {}
#[doc = "Field `SRXISEL` reader - SPI Receive Service Request Interrupt Select"]
pub type SrxiselR = crate::FieldReader<Srxiselselect>;
impl SrxiselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srxiselselect {
        match self.bits {
            0 => Srxiselselect::Rxbempty,
            1 => Srxiselselect::Rxbnempty,
            2 => Srxiselselect::Rxbhfull,
            3 => Srxiselselect::Rxbfull,
            _ => unreachable!(),
        }
    }
    #[doc = "RXB Empty"]
    #[inline(always)]
    pub fn is_rxbempty(&self) -> bool {
        *self == Srxiselselect::Rxbempty
    }
    #[doc = "RXB Not Empty"]
    #[inline(always)]
    pub fn is_rxbnempty(&self) -> bool {
        *self == Srxiselselect::Rxbnempty
    }
    #[doc = "RXB Half Full"]
    #[inline(always)]
    pub fn is_rxbhfull(&self) -> bool {
        *self == Srxiselselect::Rxbhfull
    }
    #[doc = "RXB Full"]
    #[inline(always)]
    pub fn is_rxbfull(&self) -> bool {
        *self == Srxiselselect::Rxbfull
    }
}
#[doc = "Field `SRXISEL` writer - SPI Receive Service Request Interrupt Select"]
pub type SrxiselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Srxiselselect, crate::Safe>;
impl<'a, REG> SrxiselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RXB Empty"]
    #[inline(always)]
    pub fn rxbempty(self) -> &'a mut crate::W<REG> {
        self.variant(Srxiselselect::Rxbempty)
    }
    #[doc = "RXB Not Empty"]
    #[inline(always)]
    pub fn rxbnempty(self) -> &'a mut crate::W<REG> {
        self.variant(Srxiselselect::Rxbnempty)
    }
    #[doc = "RXB Half Full"]
    #[inline(always)]
    pub fn rxbhfull(self) -> &'a mut crate::W<REG> {
        self.variant(Srxiselselect::Rxbhfull)
    }
    #[doc = "RXB Full"]
    #[inline(always)]
    pub fn rxbfull(self) -> &'a mut crate::W<REG> {
        self.variant(Srxiselselect::Rxbfull)
    }
}
#[doc = "Field `IGNROV` reader - Ignore Receive Overflow (for Audio Data Transmissions)"]
pub type IgnrovR = crate::BitReader;
#[doc = "Field `IGNROV` writer - Ignore Receive Overflow (for Audio Data Transmissions)"]
pub type IgnrovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - SPI Clock Edge Select bit"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - SPI Clock Edge Select bit"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock Polarity Select bit"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock Polarity Select bit"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SPI Transmit Service Request Interrupt Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stxiselselect {
    #[doc = "0: TXB And SR Empty"]
    Txbsrempty = 0,
    #[doc = "1: TXB Empty"]
    Txbempty = 1,
    #[doc = "2: TXB Half Empty"]
    Txbhempty = 2,
    #[doc = "3: TXB Not Full"]
    Txbnfull = 3,
}
impl From<Stxiselselect> for u8 {
    #[inline(always)]
    fn from(variant: Stxiselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stxiselselect {
    type Ux = u8;
}
impl crate::IsEnum for Stxiselselect {}
#[doc = "Field `STXISEL` reader - SPI Transmit Service Request Interrupt Select"]
pub type StxiselR = crate::FieldReader<Stxiselselect>;
impl StxiselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stxiselselect {
        match self.bits {
            0 => Stxiselselect::Txbsrempty,
            1 => Stxiselselect::Txbempty,
            2 => Stxiselselect::Txbhempty,
            3 => Stxiselselect::Txbnfull,
            _ => unreachable!(),
        }
    }
    #[doc = "TXB And SR Empty"]
    #[inline(always)]
    pub fn is_txbsrempty(&self) -> bool {
        *self == Stxiselselect::Txbsrempty
    }
    #[doc = "TXB Empty"]
    #[inline(always)]
    pub fn is_txbempty(&self) -> bool {
        *self == Stxiselselect::Txbempty
    }
    #[doc = "TXB Half Empty"]
    #[inline(always)]
    pub fn is_txbhempty(&self) -> bool {
        *self == Stxiselselect::Txbhempty
    }
    #[doc = "TXB Not Full"]
    #[inline(always)]
    pub fn is_txbnfull(&self) -> bool {
        *self == Stxiselselect::Txbnfull
    }
}
#[doc = "Field `STXISEL` writer - SPI Transmit Service Request Interrupt Select"]
pub type StxiselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Stxiselselect, crate::Safe>;
impl<'a, REG> StxiselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TXB And SR Empty"]
    #[inline(always)]
    pub fn txbsrempty(self) -> &'a mut crate::W<REG> {
        self.variant(Stxiselselect::Txbsrempty)
    }
    #[doc = "TXB Empty"]
    #[inline(always)]
    pub fn txbempty(self) -> &'a mut crate::W<REG> {
        self.variant(Stxiselselect::Txbempty)
    }
    #[doc = "TXB Half Empty"]
    #[inline(always)]
    pub fn txbhempty(self) -> &'a mut crate::W<REG> {
        self.variant(Stxiselselect::Txbhempty)
    }
    #[doc = "TXB Not Full"]
    #[inline(always)]
    pub fn txbnfull(self) -> &'a mut crate::W<REG> {
        self.variant(Stxiselselect::Txbnfull)
    }
}
#[doc = "Field `IGNTUR` reader - Ignore Transmit Underrun (for Audio Data Transmissions)"]
pub type IgnturR = crate::BitReader;
#[doc = "Field `IGNTUR` writer - Ignore Transmit Underrun (for Audio Data Transmissions)"]
pub type IgnturW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATFMTLR` reader - Packed data format -- left or right justified"]
pub type DatfmtlrR = crate::BitReader;
#[doc = "Field `DATFMTLR` writer - Packed data format -- left or right justified"]
pub type DatfmtlrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TURSAMP` reader - Transmit Under-run last sample sent"]
pub type TursampR = crate::BitReader;
#[doc = "Field `TURSAMP` writer - Transmit Under-run last sample sent"]
pub type TursampW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATFILL` reader - DATFILL undefined bits 1 or 0"]
pub type DatfillR = crate::BitReader;
#[doc = "Field `DATFILL` writer - DATFILL undefined bits 1 or 0"]
pub type DatfillW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MACRO MODE ENABLED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modeenselect {
    #[doc = "0: Default Mode"]
    Dftlen = 0,
    #[doc = "1: Audio Mode"]
    Auden = 1,
    #[doc = "2: Framed Mode"]
    Tdmen = 2,
    #[doc = "3: Broadcast Mode"]
    Tpden = 3,
}
impl From<Modeenselect> for u8 {
    #[inline(always)]
    fn from(variant: Modeenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modeenselect {
    type Ux = u8;
}
impl crate::IsEnum for Modeenselect {}
#[doc = "Field `MODEEN` reader - MACRO MODE ENABLED"]
pub type ModeenR = crate::FieldReader<Modeenselect>;
impl ModeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modeenselect {
        match self.bits {
            0 => Modeenselect::Dftlen,
            1 => Modeenselect::Auden,
            2 => Modeenselect::Tdmen,
            3 => Modeenselect::Tpden,
            _ => unreachable!(),
        }
    }
    #[doc = "Default Mode"]
    #[inline(always)]
    pub fn is_dftlen(&self) -> bool {
        *self == Modeenselect::Dftlen
    }
    #[doc = "Audio Mode"]
    #[inline(always)]
    pub fn is_auden(&self) -> bool {
        *self == Modeenselect::Auden
    }
    #[doc = "Framed Mode"]
    #[inline(always)]
    pub fn is_tdmen(&self) -> bool {
        *self == Modeenselect::Tdmen
    }
    #[doc = "Broadcast Mode"]
    #[inline(always)]
    pub fn is_tpden(&self) -> bool {
        *self == Modeenselect::Tpden
    }
}
#[doc = "Field `MODEEN` writer - MACRO MODE ENABLED"]
pub type ModeenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Modeenselect, crate::Safe>;
impl<'a, REG> ModeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default Mode"]
    #[inline(always)]
    pub fn dftlen(self) -> &'a mut crate::W<REG> {
        self.variant(Modeenselect::Dftlen)
    }
    #[doc = "Audio Mode"]
    #[inline(always)]
    pub fn auden(self) -> &'a mut crate::W<REG> {
        self.variant(Modeenselect::Auden)
    }
    #[doc = "Framed Mode"]
    #[inline(always)]
    pub fn tdmen(self) -> &'a mut crate::W<REG> {
        self.variant(Modeenselect::Tdmen)
    }
    #[doc = "Broadcast Mode"]
    #[inline(always)]
    pub fn tpden(self) -> &'a mut crate::W<REG> {
        self.variant(Modeenselect::Tpden)
    }
}
impl R {
    #[doc = "Bits 0:1 - Serial Clock Input Delay for SDI sampling"]
    #[inline(always)]
    pub fn clkindly(&self) -> ClkindlyR {
        ClkindlyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - SPI Receive Service Request Interrupt Select"]
    #[inline(always)]
    pub fn srxisel(&self) -> SrxiselR {
        SrxiselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Ignore Receive Overflow (for Audio Data Transmissions)"]
    #[inline(always)]
    pub fn ignrov(&self) -> IgnrovR {
        IgnrovR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI Clock Edge Select bit"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock Polarity Select bit"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - SPI Transmit Service Request Interrupt Select"]
    #[inline(always)]
    pub fn stxisel(&self) -> StxiselR {
        StxiselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Ignore Transmit Underrun (for Audio Data Transmissions)"]
    #[inline(always)]
    pub fn igntur(&self) -> IgnturR {
        IgnturR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Packed data format -- left or right justified"]
    #[inline(always)]
    pub fn datfmtlr(&self) -> DatfmtlrR {
        DatfmtlrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmit Under-run last sample sent"]
    #[inline(always)]
    pub fn tursamp(&self) -> TursampR {
        TursampR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DATFILL undefined bits 1 or 0"]
    #[inline(always)]
    pub fn datfill(&self) -> DatfillR {
        DatfillR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 30:31 - MACRO MODE ENABLED"]
    #[inline(always)]
    pub fn modeen(&self) -> ModeenR {
        ModeenR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Serial Clock Input Delay for SDI sampling"]
    #[inline(always)]
    pub fn clkindly(&mut self) -> ClkindlyW<'_, SelctrlSpec> {
        ClkindlyW::new(self, 0)
    }
    #[doc = "Bits 4:5 - SPI Receive Service Request Interrupt Select"]
    #[inline(always)]
    pub fn srxisel(&mut self) -> SrxiselW<'_, SelctrlSpec> {
        SrxiselW::new(self, 4)
    }
    #[doc = "Bit 6 - Ignore Receive Overflow (for Audio Data Transmissions)"]
    #[inline(always)]
    pub fn ignrov(&mut self) -> IgnrovW<'_, SelctrlSpec> {
        IgnrovW::new(self, 6)
    }
    #[doc = "Bit 10 - SPI Clock Edge Select bit"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, SelctrlSpec> {
        CphaW::new(self, 10)
    }
    #[doc = "Bit 11 - Clock Polarity Select bit"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, SelctrlSpec> {
        CpolW::new(self, 11)
    }
    #[doc = "Bits 12:13 - SPI Transmit Service Request Interrupt Select"]
    #[inline(always)]
    pub fn stxisel(&mut self) -> StxiselW<'_, SelctrlSpec> {
        StxiselW::new(self, 12)
    }
    #[doc = "Bit 14 - Ignore Transmit Underrun (for Audio Data Transmissions)"]
    #[inline(always)]
    pub fn igntur(&mut self) -> IgnturW<'_, SelctrlSpec> {
        IgnturW::new(self, 14)
    }
    #[doc = "Bit 16 - Packed data format -- left or right justified"]
    #[inline(always)]
    pub fn datfmtlr(&mut self) -> DatfmtlrW<'_, SelctrlSpec> {
        DatfmtlrW::new(self, 16)
    }
    #[doc = "Bit 23 - Transmit Under-run last sample sent"]
    #[inline(always)]
    pub fn tursamp(&mut self) -> TursampW<'_, SelctrlSpec> {
        TursampW::new(self, 23)
    }
    #[doc = "Bit 24 - DATFILL undefined bits 1 or 0"]
    #[inline(always)]
    pub fn datfill(&mut self) -> DatfillW<'_, SelctrlSpec> {
        DatfillW::new(self, 24)
    }
    #[doc = "Bits 30:31 - MACRO MODE ENABLED"]
    #[inline(always)]
    pub fn modeen(&mut self) -> ModeenW<'_, SelctrlSpec> {
        ModeenW::new(self, 30)
    }
}
#[doc = "SPI Control Options Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`selctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SelctrlSpec;
impl crate::RegisterSpec for SelctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`selctrl::R`](R) reader structure"]
impl crate::Readable for SelctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`selctrl::W`](W) writer structure"]
impl crate::Writable for SelctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SELCTRL to value 0"]
impl crate::Resettable for SelctrlSpec {}
