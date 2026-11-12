#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chsizeselect {
    #[doc = "0: 8-bits character"]
    _8Bit = 0,
    #[doc = "1: 9-bits character"]
    _9Bit = 1,
}
impl From<Chsizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Chsizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chsizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Chsizeselect {}
#[doc = "Field `CHSIZE` reader - Character Size"]
pub type ChsizeR = crate::FieldReader<Chsizeselect>;
impl ChsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chsizeselect> {
        match self.bits {
            0 => Some(Chsizeselect::_8Bit),
            1 => Some(Chsizeselect::_9Bit),
            _ => None,
        }
    }
    #[doc = "8-bits character"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Chsizeselect::_8Bit
    }
    #[doc = "9-bits character"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == Chsizeselect::_9Bit
    }
}
#[doc = "Field `CHSIZE` writer - Character Size"]
pub type ChsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Chsizeselect>;
impl<'a, REG> ChsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bits character"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_8Bit)
    }
    #[doc = "9-bits character"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_9Bit)
    }
}
#[doc = "Field `PLOADEN` reader - Data Preload Enable"]
pub type PloadenR = crate::BitReader;
#[doc = "Field `PLOADEN` writer - Data Preload Enable"]
pub type PloadenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSDE` reader - Slave Select Low Detect Enable"]
pub type SsdeR = crate::BitReader;
#[doc = "Field `SSDE` writer - Slave Select Low Detect Enable"]
pub type SsdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSSEN` reader - Master Slave Select Enable"]
pub type MssenR = crate::BitReader;
#[doc = "Field `MSSEN` writer - Master Slave Select Enable"]
pub type MssenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Address Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Amodeselect {
    #[doc = "0: ADDRMASK is used as a mask to the AADR register"]
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
    #[doc = "ADDRMASK is used as a mask to the AADR register"]
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
    #[doc = "ADDRMASK is used as a mask to the AADR register"]
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
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&self) -> ChsizeR {
        ChsizeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - Data Preload Enable"]
    #[inline(always)]
    pub fn ploaden(&self) -> PloadenR {
        PloadenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave Select Low Detect Enable"]
    #[inline(always)]
    pub fn ssde(&self) -> SsdeR {
        SsdeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Master Slave Select Enable"]
    #[inline(always)]
    pub fn mssen(&self) -> MssenR {
        MssenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&self) -> AmodeR {
        AmodeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 22:23 - FIFO Clear"]
    #[inline(always)]
    pub fn fifoclr(&self) -> FifoclrR {
        FifoclrR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&mut self) -> ChsizeW<'_, CtrlbSpec> {
        ChsizeW::new(self, 0)
    }
    #[doc = "Bit 6 - Data Preload Enable"]
    #[inline(always)]
    pub fn ploaden(&mut self) -> PloadenW<'_, CtrlbSpec> {
        PloadenW::new(self, 6)
    }
    #[doc = "Bit 9 - Slave Select Low Detect Enable"]
    #[inline(always)]
    pub fn ssde(&mut self) -> SsdeW<'_, CtrlbSpec> {
        SsdeW::new(self, 9)
    }
    #[doc = "Bit 13 - Master Slave Select Enable"]
    #[inline(always)]
    pub fn mssen(&mut self) -> MssenW<'_, CtrlbSpec> {
        MssenW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&mut self) -> AmodeW<'_, CtrlbSpec> {
        AmodeW::new(self, 14)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<'_, CtrlbSpec> {
        RxenW::new(self, 17)
    }
    #[doc = "Bits 22:23 - FIFO Clear"]
    #[inline(always)]
    pub fn fifoclr(&mut self) -> FifoclrW<'_, CtrlbSpec> {
        FifoclrW::new(self, 22)
    }
}
#[doc = "SPIS Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
