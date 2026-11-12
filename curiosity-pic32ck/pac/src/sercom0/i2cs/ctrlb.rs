#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `SMEN` reader - Smart Mode Enable"]
pub type SmenR = crate::BitReader;
#[doc = "Field `SMEN` writer - Smart Mode Enable"]
pub type SmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCMD` reader - PMBus Group Command"]
pub type GcmdR = crate::BitReader;
#[doc = "Field `GCMD` writer - PMBus Group Command"]
pub type GcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AACKEN` reader - Automatic Address Acknowledge"]
pub type AackenR = crate::BitReader;
#[doc = "Field `AACKEN` writer - Automatic Address Acknowledge"]
pub type AackenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Address Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Amodeselect {
    #[doc = "0: The slave responds to the address written in ADDR.ADDR masked by the value in ADDR.ADDRMASK"]
    Mask = 0,
    #[doc = "1: The slave responds to the two unique addresses in ADDR and ADDRMASK"]
    _2addrs = 1,
    #[doc = "2: The slave responds to the range of addresses between and including ADDR and ADDRMASK. ADDR is the upper limit"]
    Range = 2,
}
impl From<Amodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Amodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Amodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Amodeselect {}
#[doc = "Field `AMODE` reader - Address Mode"]
pub type AmodeR = crate::FieldReader<Amodeselect>;
impl AmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Amodeselect> {
        match self.bits {
            0 => Some(Amodeselect::Mask),
            1 => Some(Amodeselect::_2addrs),
            2 => Some(Amodeselect::Range),
            _ => None,
        }
    }
    #[doc = "The slave responds to the address written in ADDR.ADDR masked by the value in ADDR.ADDRMASK"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == Amodeselect::Mask
    }
    #[doc = "The slave responds to the two unique addresses in ADDR and ADDRMASK"]
    #[inline(always)]
    pub fn is_2addrs(&self) -> bool {
        *self == Amodeselect::_2addrs
    }
    #[doc = "The slave responds to the range of addresses between and including ADDR and ADDRMASK. ADDR is the upper limit"]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == Amodeselect::Range
    }
}
#[doc = "Field `AMODE` writer - Address Mode"]
pub type AmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Amodeselect>;
impl<'a, REG> AmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The slave responds to the address written in ADDR.ADDR masked by the value in ADDR.ADDRMASK"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(Amodeselect::Mask)
    }
    #[doc = "The slave responds to the two unique addresses in ADDR and ADDRMASK"]
    #[inline(always)]
    pub fn _2addrs(self) -> &'a mut crate::W<REG> {
        self.variant(Amodeselect::_2addrs)
    }
    #[doc = "The slave responds to the range of addresses between and including ADDR and ADDRMASK. ADDR is the upper limit"]
    #[inline(always)]
    pub fn range(self) -> &'a mut crate::W<REG> {
        self.variant(Amodeselect::Range)
    }
}
#[doc = "Field `CMD` reader - Command"]
pub type CmdR = crate::FieldReader;
#[doc = "Field `CMD` writer - Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACKACT` reader - Acknowledge Action"]
pub type AckactR = crate::BitReader;
#[doc = "Field `ACKACT` writer - Acknowledge Action"]
pub type AckactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "FIFO Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fifoclrselect {
    #[doc = "0: No action"]
    None = 0,
    #[doc = "1: Clear TX FIFO"]
    Txfifo = 1,
    #[doc = "2: Clear RX FIFO"]
    Rxfifo = 2,
    #[doc = "3: Clear both TX and RF FIFOs"]
    Both = 3,
}
impl From<Fifoclrselect> for u8 {
    #[inline(always)]
    fn from(variant: Fifoclrselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fifoclrselect {
    type Ux = u8;
}
impl crate::IsEnum for Fifoclrselect {}
#[doc = "Field `FIFOCLR` reader - FIFO Clear"]
pub type FifoclrR = crate::FieldReader<Fifoclrselect>;
impl FifoclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifoclrselect {
        match self.bits {
            0 => Fifoclrselect::None,
            1 => Fifoclrselect::Txfifo,
            2 => Fifoclrselect::Rxfifo,
            3 => Fifoclrselect::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Fifoclrselect::None
    }
    #[doc = "Clear TX FIFO"]
    #[inline(always)]
    pub fn is_txfifo(&self) -> bool {
        *self == Fifoclrselect::Txfifo
    }
    #[doc = "Clear RX FIFO"]
    #[inline(always)]
    pub fn is_rxfifo(&self) -> bool {
        *self == Fifoclrselect::Rxfifo
    }
    #[doc = "Clear both TX and RF FIFOs"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Fifoclrselect::Both
    }
}
#[doc = "Field `FIFOCLR` writer - FIFO Clear"]
pub type FifoclrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fifoclrselect, crate::Safe>;
impl<'a, REG> FifoclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoclrselect::None)
    }
    #[doc = "Clear TX FIFO"]
    #[inline(always)]
    pub fn txfifo(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoclrselect::Txfifo)
    }
    #[doc = "Clear RX FIFO"]
    #[inline(always)]
    pub fn rxfifo(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoclrselect::Rxfifo)
    }
    #[doc = "Clear both TX and RF FIFOs"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoclrselect::Both)
    }
}
impl R {
    #[doc = "Bit 8 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SmenR {
        SmenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PMBus Group Command"]
    #[inline(always)]
    pub fn gcmd(&self) -> GcmdR {
        GcmdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Automatic Address Acknowledge"]
    #[inline(always)]
    pub fn aacken(&self) -> AackenR {
        AackenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&self) -> AmodeR {
        AmodeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Acknowledge Action"]
    #[inline(always)]
    pub fn ackact(&self) -> AckactR {
        AckactR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 22:23 - FIFO Clear"]
    #[inline(always)]
    pub fn fifoclr(&self) -> FifoclrR {
        FifoclrR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&mut self) -> SmenW<'_, CtrlbSpec> {
        SmenW::new(self, 8)
    }
    #[doc = "Bit 9 - PMBus Group Command"]
    #[inline(always)]
    pub fn gcmd(&mut self) -> GcmdW<'_, CtrlbSpec> {
        GcmdW::new(self, 9)
    }
    #[doc = "Bit 10 - Automatic Address Acknowledge"]
    #[inline(always)]
    pub fn aacken(&mut self) -> AackenW<'_, CtrlbSpec> {
        AackenW::new(self, 10)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&mut self) -> AmodeW<'_, CtrlbSpec> {
        AmodeW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, CtrlbSpec> {
        CmdW::new(self, 16)
    }
    #[doc = "Bit 18 - Acknowledge Action"]
    #[inline(always)]
    pub fn ackact(&mut self) -> AckactW<'_, CtrlbSpec> {
        AckactW::new(self, 18)
    }
    #[doc = "Bits 22:23 - FIFO Clear"]
    #[inline(always)]
    pub fn fifoclr(&mut self) -> FifoclrW<'_, CtrlbSpec> {
        FifoclrW::new(self, 22)
    }
}
#[doc = "I2CS Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}
