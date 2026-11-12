#[doc = "Register `MRCFG` reader"]
pub type R = crate::R<MrcfgSpec>;
#[doc = "Register `MRCFG` writer"]
pub type W = crate::W<MrcfgSpec>;
#[doc = "Field `OFFSET` reader - Base address offset"]
pub type OffsetR = crate::FieldReader;
#[doc = "Field `OFFSET` writer - Base address offset"]
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23 - Base address offset"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Base address offset"]
    #[inline(always)]
    pub fn offset(&mut self) -> OffsetW<'_, MrcfgSpec> {
        OffsetW::new(self, 16)
    }
}
#[doc = "Message RAM Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mrcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrcfgSpec;
impl crate::RegisterSpec for MrcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrcfg::R`](R) reader structure"]
impl crate::Readable for MrcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mrcfg::W`](W) writer structure"]
impl crate::Writable for MrcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MRCFG to value 0"]
impl crate::Resettable for MrcfgSpec {}
