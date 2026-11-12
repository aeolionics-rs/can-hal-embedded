#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "DMA Trigger 0 Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dccdmalevel0select {
    #[doc = "0: Trigger x rises when DCC is read and falls when it is written"]
    Empty = 0,
    #[doc = "1: Trigger x rises when DCC is written and falls when it is read"]
    Full = 1,
}
impl From<Dccdmalevel0select> for bool {
    #[inline(always)]
    fn from(variant: Dccdmalevel0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCCDMALEVEL0` reader - DMA Trigger 0 Level"]
pub type Dccdmalevel0R = crate::BitReader<Dccdmalevel0select>;
impl Dccdmalevel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dccdmalevel0select {
        match self.bits {
            false => Dccdmalevel0select::Empty,
            true => Dccdmalevel0select::Full,
        }
    }
    #[doc = "Trigger x rises when DCC is read and falls when it is written"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Dccdmalevel0select::Empty
    }
    #[doc = "Trigger x rises when DCC is written and falls when it is read"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Dccdmalevel0select::Full
    }
}
#[doc = "Field `DCCDMALEVEL0` writer - DMA Trigger 0 Level"]
pub type Dccdmalevel0W<'a, REG> = crate::BitWriter<'a, REG, Dccdmalevel0select>;
impl<'a, REG> Dccdmalevel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger x rises when DCC is read and falls when it is written"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Dccdmalevel0select::Empty)
    }
    #[doc = "Trigger x rises when DCC is written and falls when it is read"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Dccdmalevel0select::Full)
    }
}
#[doc = "DMA Trigger 1 Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dccdmalevel1select {
    #[doc = "0: Trigger x rises when DCC is read and falls when it is written"]
    Empty = 0,
    #[doc = "1: Trigger x rises when DCC is written and falls when it is read"]
    Full = 1,
}
impl From<Dccdmalevel1select> for bool {
    #[inline(always)]
    fn from(variant: Dccdmalevel1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCCDMALEVEL1` reader - DMA Trigger 1 Level"]
pub type Dccdmalevel1R = crate::BitReader<Dccdmalevel1select>;
impl Dccdmalevel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dccdmalevel1select {
        match self.bits {
            false => Dccdmalevel1select::Empty,
            true => Dccdmalevel1select::Full,
        }
    }
    #[doc = "Trigger x rises when DCC is read and falls when it is written"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Dccdmalevel1select::Empty
    }
    #[doc = "Trigger x rises when DCC is written and falls when it is read"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Dccdmalevel1select::Full
    }
}
#[doc = "Field `DCCDMALEVEL1` writer - DMA Trigger 1 Level"]
pub type Dccdmalevel1W<'a, REG> = crate::BitWriter<'a, REG, Dccdmalevel1select>;
impl<'a, REG> Dccdmalevel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger x rises when DCC is read and falls when it is written"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Dccdmalevel1select::Empty)
    }
    #[doc = "Trigger x rises when DCC is written and falls when it is read"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Dccdmalevel1select::Full)
    }
}
#[doc = "Field `MBFI` reader - Enables the Memory Bist Fault Injection"]
pub type MbfiR = crate::BitReader;
#[doc = "Field `MBFI` writer - Enables the Memory Bist Fault Injection"]
pub type MbfiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISC` reader - Device Specific Configuration Register"]
pub type MiscR = crate::FieldReader<u32>;
#[doc = "Field `MISC` writer - Device Specific Configuration Register"]
pub type MiscW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - DMA Trigger 0 Level"]
    #[inline(always)]
    pub fn dccdmalevel0(&self) -> Dccdmalevel0R {
        Dccdmalevel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Trigger 1 Level"]
    #[inline(always)]
    pub fn dccdmalevel1(&self) -> Dccdmalevel1R {
        Dccdmalevel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the Memory Bist Fault Injection"]
    #[inline(always)]
    pub fn mbfi(&self) -> MbfiR {
        MbfiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Device Specific Configuration Register"]
    #[inline(always)]
    pub fn misc(&self) -> MiscR {
        MiscR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Trigger 0 Level"]
    #[inline(always)]
    pub fn dccdmalevel0(&mut self) -> Dccdmalevel0W<'_, CfgSpec> {
        Dccdmalevel0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Trigger 1 Level"]
    #[inline(always)]
    pub fn dccdmalevel1(&mut self) -> Dccdmalevel1W<'_, CfgSpec> {
        Dccdmalevel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables the Memory Bist Fault Injection"]
    #[inline(always)]
    pub fn mbfi(&mut self) -> MbfiW<'_, CfgSpec> {
        MbfiW::new(self, 2)
    }
    #[doc = "Bits 8:31 - Device Specific Configuration Register"]
    #[inline(always)]
    pub fn misc(&mut self) -> MiscW<'_, CfgSpec> {
        MiscW::new(self, 8)
    }
}
#[doc = "Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}
