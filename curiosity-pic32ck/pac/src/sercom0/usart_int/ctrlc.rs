#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Field `GTIME` reader - Guard Time"]
pub type GtimeR = crate::FieldReader;
#[doc = "Field `GTIME` writer - Guard Time"]
pub type GtimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "LIN Master Break Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Brklenselect {
    #[doc = "0: Break field transmission is 13 bit times"]
    _13bits = 0,
    #[doc = "1: Break field transmission is 17 bit times"]
    _17bits = 1,
    #[doc = "2: Break field transmission is 21 bit times"]
    _21bits = 2,
    #[doc = "3: Break field transmission is 26 bit times"]
    _26bits = 3,
}
impl From<Brklenselect> for u8 {
    #[inline(always)]
    fn from(variant: Brklenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Brklenselect {
    type Ux = u8;
}
impl crate::IsEnum for Brklenselect {}
#[doc = "Field `BRKLEN` reader - LIN Master Break Length"]
pub type BrklenR = crate::FieldReader<Brklenselect>;
impl BrklenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brklenselect {
        match self.bits {
            0 => Brklenselect::_13bits,
            1 => Brklenselect::_17bits,
            2 => Brklenselect::_21bits,
            3 => Brklenselect::_26bits,
            _ => unreachable!(),
        }
    }
    #[doc = "Break field transmission is 13 bit times"]
    #[inline(always)]
    pub fn is_13bits(&self) -> bool {
        *self == Brklenselect::_13bits
    }
    #[doc = "Break field transmission is 17 bit times"]
    #[inline(always)]
    pub fn is_17bits(&self) -> bool {
        *self == Brklenselect::_17bits
    }
    #[doc = "Break field transmission is 21 bit times"]
    #[inline(always)]
    pub fn is_21bits(&self) -> bool {
        *self == Brklenselect::_21bits
    }
    #[doc = "Break field transmission is 26 bit times"]
    #[inline(always)]
    pub fn is_26bits(&self) -> bool {
        *self == Brklenselect::_26bits
    }
}
#[doc = "Field `BRKLEN` writer - LIN Master Break Length"]
pub type BrklenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Brklenselect, crate::Safe>;
impl<'a, REG> BrklenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Break field transmission is 13 bit times"]
    #[inline(always)]
    pub fn _13bits(self) -> &'a mut crate::W<REG> {
        self.variant(Brklenselect::_13bits)
    }
    #[doc = "Break field transmission is 17 bit times"]
    #[inline(always)]
    pub fn _17bits(self) -> &'a mut crate::W<REG> {
        self.variant(Brklenselect::_17bits)
    }
    #[doc = "Break field transmission is 21 bit times"]
    #[inline(always)]
    pub fn _21bits(self) -> &'a mut crate::W<REG> {
        self.variant(Brklenselect::_21bits)
    }
    #[doc = "Break field transmission is 26 bit times"]
    #[inline(always)]
    pub fn _26bits(self) -> &'a mut crate::W<REG> {
        self.variant(Brklenselect::_26bits)
    }
}
#[doc = "LIN Master Header Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hdrdlyselect {
    #[doc = "0: Delay between break and sync transmission is 1-bit time"]
    _1bit = 0,
    #[doc = "1: Delay between break and sync transmission is 4-bit time"]
    _4bits = 1,
    #[doc = "2: Delay between break and sync transmission is 8-bit time"]
    _8bits = 2,
    #[doc = "3: Delay between break and sync transmission is 14-bit time"]
    _14bits = 3,
}
impl From<Hdrdlyselect> for u8 {
    #[inline(always)]
    fn from(variant: Hdrdlyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hdrdlyselect {
    type Ux = u8;
}
impl crate::IsEnum for Hdrdlyselect {}
#[doc = "Field `HDRDLY` reader - LIN Master Header Delay"]
pub type HdrdlyR = crate::FieldReader<Hdrdlyselect>;
impl HdrdlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdrdlyselect {
        match self.bits {
            0 => Hdrdlyselect::_1bit,
            1 => Hdrdlyselect::_4bits,
            2 => Hdrdlyselect::_8bits,
            3 => Hdrdlyselect::_14bits,
            _ => unreachable!(),
        }
    }
    #[doc = "Delay between break and sync transmission is 1-bit time"]
    #[inline(always)]
    pub fn is_1bit(&self) -> bool {
        *self == Hdrdlyselect::_1bit
    }
    #[doc = "Delay between break and sync transmission is 4-bit time"]
    #[inline(always)]
    pub fn is_4bits(&self) -> bool {
        *self == Hdrdlyselect::_4bits
    }
    #[doc = "Delay between break and sync transmission is 8-bit time"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == Hdrdlyselect::_8bits
    }
    #[doc = "Delay between break and sync transmission is 14-bit time"]
    #[inline(always)]
    pub fn is_14bits(&self) -> bool {
        *self == Hdrdlyselect::_14bits
    }
}
#[doc = "Field `HDRDLY` writer - LIN Master Header Delay"]
pub type HdrdlyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hdrdlyselect, crate::Safe>;
impl<'a, REG> HdrdlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Delay between break and sync transmission is 1-bit time"]
    #[inline(always)]
    pub fn _1bit(self) -> &'a mut crate::W<REG> {
        self.variant(Hdrdlyselect::_1bit)
    }
    #[doc = "Delay between break and sync transmission is 4-bit time"]
    #[inline(always)]
    pub fn _4bits(self) -> &'a mut crate::W<REG> {
        self.variant(Hdrdlyselect::_4bits)
    }
    #[doc = "Delay between break and sync transmission is 8-bit time"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut crate::W<REG> {
        self.variant(Hdrdlyselect::_8bits)
    }
    #[doc = "Delay between break and sync transmission is 14-bit time"]
    #[inline(always)]
    pub fn _14bits(self) -> &'a mut crate::W<REG> {
        self.variant(Hdrdlyselect::_14bits)
    }
}
#[doc = "Field `INACK` reader - Inhibit Not Acknowledge"]
pub type InackR = crate::BitReader;
#[doc = "Field `INACK` writer - Inhibit Not Acknowledge"]
pub type InackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSNACK` reader - Disable Successive NACK"]
pub type DsnackR = crate::BitReader;
#[doc = "Field `DSNACK` writer - Disable Successive NACK"]
pub type DsnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAXITER` reader - Maximum Iterations"]
pub type MaxiterR = crate::FieldReader;
#[doc = "Field `MAXITER` writer - Maximum Iterations"]
pub type MaxiterW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DATA32B` reader - Data 32 Bit"]
pub type Data32bR = crate::FieldReader;
#[doc = "Field `DATA32B` writer - Data 32 Bit"]
pub type Data32bW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "2: Interrupt and DMA triggers are generated when the FIFO is empty"]
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
    #[doc = "Interrupt and DMA triggers are generated when the FIFO is empty"]
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
    #[doc = "Interrupt and DMA triggers are generated when the FIFO is empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Txtrholdselect::Empty)
    }
}
impl R {
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline(always)]
    pub fn gtime(&self) -> GtimeR {
        GtimeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline(always)]
    pub fn brklen(&self) -> BrklenR {
        BrklenR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline(always)]
    pub fn hdrdly(&self) -> HdrdlyR {
        HdrdlyR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - Inhibit Not Acknowledge"]
    #[inline(always)]
    pub fn inack(&self) -> InackR {
        InackR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&self) -> DsnackR {
        DsnackR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Maximum Iterations"]
    #[inline(always)]
    pub fn maxiter(&self) -> MaxiterR {
        MaxiterR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Data 32 Bit"]
    #[inline(always)]
    pub fn data32b(&self) -> Data32bR {
        Data32bR::new(((self.bits >> 24) & 3) as u8)
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
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline(always)]
    pub fn gtime(&mut self) -> GtimeW<'_, CtrlcSpec> {
        GtimeW::new(self, 0)
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline(always)]
    pub fn brklen(&mut self) -> BrklenW<'_, CtrlcSpec> {
        BrklenW::new(self, 8)
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline(always)]
    pub fn hdrdly(&mut self) -> HdrdlyW<'_, CtrlcSpec> {
        HdrdlyW::new(self, 10)
    }
    #[doc = "Bit 16 - Inhibit Not Acknowledge"]
    #[inline(always)]
    pub fn inack(&mut self) -> InackW<'_, CtrlcSpec> {
        InackW::new(self, 16)
    }
    #[doc = "Bit 17 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&mut self) -> DsnackW<'_, CtrlcSpec> {
        DsnackW::new(self, 17)
    }
    #[doc = "Bits 20:22 - Maximum Iterations"]
    #[inline(always)]
    pub fn maxiter(&mut self) -> MaxiterW<'_, CtrlcSpec> {
        MaxiterW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Data 32 Bit"]
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
#[doc = "USART Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
