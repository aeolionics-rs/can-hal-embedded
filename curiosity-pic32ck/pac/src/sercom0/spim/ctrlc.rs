#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Field `ICSPACE` reader - Inter-Character Spacing"]
pub type IcspaceR = crate::FieldReader;
#[doc = "Field `ICSPACE` writer - Inter-Character Spacing"]
pub type IcspaceW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FSES` reader - Frame Synch Edge Select"]
pub type FsesR = crate::BitReader;
#[doc = "Field `FSES` writer - Frame Synch Edge Select"]
pub type FsesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Frame Synch Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fslenselect {
    #[doc = "0: One SCK pulse"]
    Strobe = 0,
    #[doc = "1: One frame duration"]
    Level = 1,
}
impl From<Fslenselect> for bool {
    #[inline(always)]
    fn from(variant: Fslenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSLEN` reader - Frame Synch Length"]
pub type FslenR = crate::BitReader<Fslenselect>;
impl FslenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fslenselect {
        match self.bits {
            false => Fslenselect::Strobe,
            true => Fslenselect::Level,
        }
    }
    #[doc = "One SCK pulse"]
    #[inline(always)]
    pub fn is_strobe(&self) -> bool {
        *self == Fslenselect::Strobe
    }
    #[doc = "One frame duration"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Fslenselect::Level
    }
}
#[doc = "Field `FSLEN` writer - Frame Synch Length"]
pub type FslenW<'a, REG> = crate::BitWriter<'a, REG, Fslenselect>;
impl<'a, REG> FslenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One SCK pulse"]
    #[inline(always)]
    pub fn strobe(self) -> &'a mut crate::W<REG> {
        self.variant(Fslenselect::Strobe)
    }
    #[doc = "One frame duration"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Fslenselect::Level)
    }
}
#[doc = "Frame Synch Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fspolselect {
    #[doc = "0: VCC-level valid polarity"]
    High = 0,
    #[doc = "1: GND-level valid polarity"]
    Low = 1,
}
impl From<Fspolselect> for bool {
    #[inline(always)]
    fn from(variant: Fspolselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSPOL` reader - Frame Synch Polarity"]
pub type FspolR = crate::BitReader<Fspolselect>;
impl FspolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fspolselect {
        match self.bits {
            false => Fspolselect::High,
            true => Fspolselect::Low,
        }
    }
    #[doc = "VCC-level valid polarity"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Fspolselect::High
    }
    #[doc = "GND-level valid polarity"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Fspolselect::Low
    }
}
#[doc = "Field `FSPOL` writer - Frame Synch Polarity"]
pub type FspolW<'a, REG> = crate::BitWriter<'a, REG, Fspolselect>;
impl<'a, REG> FspolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VCC-level valid polarity"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Fspolselect::High)
    }
    #[doc = "GND-level valid polarity"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Fspolselect::Low)
    }
}
#[doc = "Field `IGNTUR` reader - Ignore Transmit Underrun"]
pub type IgnturR = crate::BitReader;
#[doc = "Field `IGNTUR` writer - Ignore Transmit Underrun"]
pub type IgnturW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMEN` reader - Frame mode enable"]
pub type FrmenR = crate::BitReader;
#[doc = "Field `FRMEN` writer - Frame mode enable"]
pub type FrmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Frame mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmodeselect {
    #[doc = "0: Frame Master"]
    Master = 0,
    #[doc = "1: Frame Slave"]
    Slave = 1,
}
impl From<Fmodeselect> for bool {
    #[inline(always)]
    fn from(variant: Fmodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMODE` reader - Frame mode"]
pub type FmodeR = crate::BitReader<Fmodeselect>;
impl FmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmodeselect {
        match self.bits {
            false => Fmodeselect::Master,
            true => Fmodeselect::Slave,
        }
    }
    #[doc = "Frame Master"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Fmodeselect::Master
    }
    #[doc = "Frame Slave"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Fmodeselect::Slave
    }
}
#[doc = "Field `FMODE` writer - Frame mode"]
pub type FmodeW<'a, REG> = crate::BitWriter<'a, REG, Fmodeselect>;
impl<'a, REG> FmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frame Master"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Fmodeselect::Master)
    }
    #[doc = "Frame Slave"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Fmodeselect::Slave)
    }
}
#[doc = "Field `DATA32B` reader - Data 32 Bit"]
pub type Data32bR = crate::BitReader;
#[doc = "Field `DATA32B` writer - Data 32 Bit"]
pub type Data32bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEN` reader - FIFO Enable"]
pub type FifoenR = crate::BitReader;
#[doc = "Field `FIFOEN` writer - FIFO Enable"]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receive FIFO Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxtrholdselect {
    #[doc = "0: Interrupt and DMA triggers are generated when DATA is present in the FIFO"]
    Default = 0,
    #[doc = "1: Interrupt and DMA triggers are generated when FIFO is half-full"]
    Half = 1,
    #[doc = "2: Interrupt and DMA triggers are generated when FIFO is full"]
    Full = 2,
}
impl From<Rxtrholdselect> for u8 {
    #[inline(always)]
    fn from(variant: Rxtrholdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxtrholdselect {
    type Ux = u8;
}
impl crate::IsEnum for Rxtrholdselect {}
#[doc = "Field `RXTRHOLD` reader - Receive FIFO Threshold"]
pub type RxtrholdR = crate::FieldReader<Rxtrholdselect>;
impl RxtrholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxtrholdselect> {
        match self.bits {
            0 => Some(Rxtrholdselect::Default),
            1 => Some(Rxtrholdselect::Half),
            2 => Some(Rxtrholdselect::Full),
            _ => None,
        }
    }
    #[doc = "Interrupt and DMA triggers are generated when DATA is present in the FIFO"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Rxtrholdselect::Default
    }
    #[doc = "Interrupt and DMA triggers are generated when FIFO is half-full"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Rxtrholdselect::Half
    }
    #[doc = "Interrupt and DMA triggers are generated when FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Rxtrholdselect::Full
    }
}
#[doc = "Field `RXTRHOLD` writer - Receive FIFO Threshold"]
pub type RxtrholdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxtrholdselect>;
impl<'a, REG> RxtrholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt and DMA triggers are generated when DATA is present in the FIFO"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtrholdselect::Default)
    }
    #[doc = "Interrupt and DMA triggers are generated when FIFO is half-full"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtrholdselect::Half)
    }
    #[doc = "Interrupt and DMA triggers are generated when FIFO is full"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtrholdselect::Full)
    }
}
#[doc = "Transmit FIFO Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txtrholdselect {
    #[doc = "0: Interrupt and DMA triggers are generated as long as the FIFO is not full"]
    Default = 0,
    #[doc = "1: Interrupt and DMA triggers are generated when half FIFO space is free"]
    Half = 1,
    #[doc = "2: Interrupt and DMA triggers are generated as long as the FIFO is empty"]
    Empty = 2,
}
impl From<Txtrholdselect> for u8 {
    #[inline(always)]
    fn from(variant: Txtrholdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txtrholdselect {
    type Ux = u8;
}
impl crate::IsEnum for Txtrholdselect {}
#[doc = "Field `TXTRHOLD` reader - Transmit FIFO Threshold"]
pub type TxtrholdR = crate::FieldReader<Txtrholdselect>;
impl TxtrholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txtrholdselect> {
        match self.bits {
            0 => Some(Txtrholdselect::Default),
            1 => Some(Txtrholdselect::Half),
            2 => Some(Txtrholdselect::Empty),
            _ => None,
        }
    }
    #[doc = "Interrupt and DMA triggers are generated as long as the FIFO is not full"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Txtrholdselect::Default
    }
    #[doc = "Interrupt and DMA triggers are generated when half FIFO space is free"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Txtrholdselect::Half
    }
    #[doc = "Interrupt and DMA triggers are generated as long as the FIFO is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Txtrholdselect::Empty
    }
}
#[doc = "Field `TXTRHOLD` writer - Transmit FIFO Threshold"]
pub type TxtrholdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txtrholdselect>;
impl<'a, REG> TxtrholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt and DMA triggers are generated as long as the FIFO is not full"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrholdselect::Default)
    }
    #[doc = "Interrupt and DMA triggers are generated when half FIFO space is free"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrholdselect::Half)
    }
    #[doc = "Interrupt and DMA triggers are generated as long as the FIFO is empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrholdselect::Empty)
    }
}
impl R {
    #[doc = "Bits 0:5 - Inter-Character Spacing"]
    #[inline(always)]
    pub fn icspace(&self) -> IcspaceR {
        IcspaceR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Frame Synch Edge Select"]
    #[inline(always)]
    pub fn fses(&self) -> FsesR {
        FsesR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame Synch Length"]
    #[inline(always)]
    pub fn fslen(&self) -> FslenR {
        FslenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Frame Synch Polarity"]
    #[inline(always)]
    pub fn fspol(&self) -> FspolR {
        FspolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Ignore Transmit Underrun"]
    #[inline(always)]
    pub fn igntur(&self) -> IgnturR {
        IgnturR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Frame mode enable"]
    #[inline(always)]
    pub fn frmen(&self) -> FrmenR {
        FrmenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Frame mode"]
    #[inline(always)]
    pub fn fmode(&self) -> FmodeR {
        FmodeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&self) -> Data32bR {
        Data32bR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - FIFO Enable"]
    #[inline(always)]
    pub fn fifoen(&self) -> FifoenR {
        FifoenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Receive FIFO Threshold"]
    #[inline(always)]
    pub fn rxtrhold(&self) -> RxtrholdR {
        RxtrholdR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Transmit FIFO Threshold"]
    #[inline(always)]
    pub fn txtrhold(&self) -> TxtrholdR {
        TxtrholdR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Inter-Character Spacing"]
    #[inline(always)]
    pub fn icspace(&mut self) -> IcspaceW<'_, CtrlcSpec> {
        IcspaceW::new(self, 0)
    }
    #[doc = "Bit 8 - Frame Synch Edge Select"]
    #[inline(always)]
    pub fn fses(&mut self) -> FsesW<'_, CtrlcSpec> {
        FsesW::new(self, 8)
    }
    #[doc = "Bit 9 - Frame Synch Length"]
    #[inline(always)]
    pub fn fslen(&mut self) -> FslenW<'_, CtrlcSpec> {
        FslenW::new(self, 9)
    }
    #[doc = "Bit 10 - Frame Synch Polarity"]
    #[inline(always)]
    pub fn fspol(&mut self) -> FspolW<'_, CtrlcSpec> {
        FspolW::new(self, 10)
    }
    #[doc = "Bit 11 - Ignore Transmit Underrun"]
    #[inline(always)]
    pub fn igntur(&mut self) -> IgnturW<'_, CtrlcSpec> {
        IgnturW::new(self, 11)
    }
    #[doc = "Bit 16 - Frame mode enable"]
    #[inline(always)]
    pub fn frmen(&mut self) -> FrmenW<'_, CtrlcSpec> {
        FrmenW::new(self, 16)
    }
    #[doc = "Bit 17 - Frame mode"]
    #[inline(always)]
    pub fn fmode(&mut self) -> FmodeW<'_, CtrlcSpec> {
        FmodeW::new(self, 17)
    }
    #[doc = "Bit 24 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&mut self) -> Data32bW<'_, CtrlcSpec> {
        Data32bW::new(self, 24)
    }
    #[doc = "Bit 27 - FIFO Enable"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FifoenW<'_, CtrlcSpec> {
        FifoenW::new(self, 27)
    }
    #[doc = "Bits 28:29 - Receive FIFO Threshold"]
    #[inline(always)]
    pub fn rxtrhold(&mut self) -> RxtrholdW<'_, CtrlcSpec> {
        RxtrholdW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Transmit FIFO Threshold"]
    #[inline(always)]
    pub fn txtrhold(&mut self) -> TxtrholdW<'_, CtrlcSpec> {
        TxtrholdW::new(self, 30)
    }
}
#[doc = "SPIM Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {}
