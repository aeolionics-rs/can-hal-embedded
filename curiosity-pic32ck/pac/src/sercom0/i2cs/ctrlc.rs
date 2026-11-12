#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Field `SDASETUP` reader - SDA Setup Time"]
pub type SdasetupR = crate::FieldReader;
#[doc = "Field `SDASETUP` writer - SDA Setup Time"]
pub type SdasetupW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "2: Interrupt and DMA triggers are generated as long as the FIFO is not full"]
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
    #[doc = "Interrupt and DMA triggers are generated as long as the FIFO is not full"]
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
    #[doc = "Interrupt and DMA triggers are generated as long as the FIFO is not full"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrholdselect::Empty)
    }
}
impl R {
    #[doc = "Bits 0:3 - SDA Setup Time"]
    #[inline(always)]
    pub fn sdasetup(&self) -> SdasetupR {
        SdasetupR::new((self.bits & 0x0f) as u8)
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
    #[doc = "Bits 0:3 - SDA Setup Time"]
    #[inline(always)]
    pub fn sdasetup(&mut self) -> SdasetupW<'_, CtrlcSpec> {
        SdasetupW::new(self, 0)
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
#[doc = "I2CS Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
