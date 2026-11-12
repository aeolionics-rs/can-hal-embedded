#[doc = "Register `DMAINTFLAG` reader"]
pub type R = crate::R<DmaintflagSpec>;
#[doc = "Register `DMAINTFLAG` writer"]
pub type W = crate::W<DmaintflagSpec>;
#[doc = "Field `RAF` reader - Ram Buffer A Full"]
pub type RafR = crate::FieldReader;
#[doc = "Field `RAF` writer - Ram Buffer A Full"]
pub type RafW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RBF` reader - Ram Buffer B Full"]
pub type RbfR = crate::FieldReader;
#[doc = "Field `RBF` writer - Ram Buffer B Full"]
pub type RbfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EAF` reader - Ram Buffer A Overflow Error"]
pub type EafR = crate::FieldReader;
#[doc = "Field `EAF` writer - Ram Buffer A Overflow Error"]
pub type EafW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EBF` reader - Ram Buffer B Overflow Error"]
pub type EbfR = crate::FieldReader;
#[doc = "Field `EBF` writer - Ram Buffer B Overflow Error"]
pub type EbfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SOVFL` reader - Synchronizer overflow"]
pub type SovflR = crate::BitReader;
#[doc = "Field `SOVFL` writer - Synchronizer overflow"]
pub type SovflW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DMAERR` reader - DMA Bus Error"]
pub type DmaerrR = crate::BitReader;
#[doc = "Field `DMAERR` writer - DMA Bus Error"]
pub type DmaerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Ram Buffer A Full"]
    #[inline(always)]
    pub fn raf(&self) -> RafR {
        RafR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Ram Buffer B Full"]
    #[inline(always)]
    pub fn rbf(&self) -> RbfR {
        RbfR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Ram Buffer A Overflow Error"]
    #[inline(always)]
    pub fn eaf(&self) -> EafR {
        EafR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Ram Buffer B Overflow Error"]
    #[inline(always)]
    pub fn ebf(&self) -> EbfR {
        EbfR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Synchronizer overflow"]
    #[inline(always)]
    pub fn sovfl(&self) -> SovflR {
        SovflR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA Bus Error"]
    #[inline(always)]
    pub fn dmaerr(&self) -> DmaerrR {
        DmaerrR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Ram Buffer A Full"]
    #[inline(always)]
    pub fn raf(&mut self) -> RafW<'_, DmaintflagSpec> {
        RafW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Ram Buffer B Full"]
    #[inline(always)]
    pub fn rbf(&mut self) -> RbfW<'_, DmaintflagSpec> {
        RbfW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Ram Buffer A Overflow Error"]
    #[inline(always)]
    pub fn eaf(&mut self) -> EafW<'_, DmaintflagSpec> {
        EafW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Ram Buffer B Overflow Error"]
    #[inline(always)]
    pub fn ebf(&mut self) -> EbfW<'_, DmaintflagSpec> {
        EbfW::new(self, 12)
    }
    #[doc = "Bit 16 - Synchronizer overflow"]
    #[inline(always)]
    pub fn sovfl(&mut self) -> SovflW<'_, DmaintflagSpec> {
        SovflW::new(self, 16)
    }
    #[doc = "Bit 17 - DMA Bus Error"]
    #[inline(always)]
    pub fn dmaerr(&mut self) -> DmaerrW<'_, DmaintflagSpec> {
        DmaerrW::new(self, 17)
    }
}
#[doc = "DMA Interrupt Flag and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaintflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaintflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaintflagSpec;
impl crate::RegisterSpec for DmaintflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaintflag::R`](R) reader structure"]
impl crate::Readable for DmaintflagSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaintflag::W`](W) writer structure"]
impl crate::Writable for DmaintflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0003_ffff;
}
#[doc = "`reset()` method sets DMAINTFLAG to value 0"]
impl crate::Resettable for DmaintflagSpec {}
