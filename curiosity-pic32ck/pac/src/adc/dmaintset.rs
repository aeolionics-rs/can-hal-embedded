#[doc = "Register `DMAINTSET` reader"]
pub type R = crate::R<DmaintsetSpec>;
#[doc = "Register `DMAINTSET` writer"]
pub type W = crate::W<DmaintsetSpec>;
#[doc = "Field `RAF` reader - Ram Buffer A Full"]
pub type RafR = crate::FieldReader;
#[doc = "Field `RAF` writer - Ram Buffer A Full"]
pub type RafW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RBF` reader - Ram Buffer B Full"]
pub type RbfR = crate::FieldReader;
#[doc = "Field `RBF` writer - Ram Buffer B Full"]
pub type RbfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SOVFL` reader - Synchonizer Overflow"]
pub type SovflR = crate::BitReader;
#[doc = "Field `SOVFL` writer - Synchonizer Overflow"]
pub type SovflW<'a, REG> = crate::BitWriter1S<'a, REG>;
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
    #[doc = "Bit 16 - Synchonizer Overflow"]
    #[inline(always)]
    pub fn sovfl(&self) -> SovflR {
        SovflR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Ram Buffer A Full"]
    #[inline(always)]
    pub fn raf(&mut self) -> RafW<'_, DmaintsetSpec> {
        RafW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Ram Buffer B Full"]
    #[inline(always)]
    pub fn rbf(&mut self) -> RbfW<'_, DmaintsetSpec> {
        RbfW::new(self, 4)
    }
    #[doc = "Bit 16 - Synchonizer Overflow"]
    #[inline(always)]
    pub fn sovfl(&mut self) -> SovflW<'_, DmaintsetSpec> {
        SovflW::new(self, 16)
    }
}
#[doc = "DMA Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaintset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaintset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaintsetSpec;
impl crate::RegisterSpec for DmaintsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaintset::R`](R) reader structure"]
impl crate::Readable for DmaintsetSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaintset::W`](W) writer structure"]
impl crate::Writable for DmaintsetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_00ff;
}
#[doc = "`reset()` method sets DMAINTSET to value 0"]
impl crate::Resettable for DmaintsetSpec {}
