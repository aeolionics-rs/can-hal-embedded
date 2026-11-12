#[doc = "Register `DFLLTUNE` reader"]
pub type R = crate::R<DflltuneSpec>;
#[doc = "Register `DFLLTUNE` writer"]
pub type W = crate::W<DflltuneSpec>;
#[doc = "Field `TUNE` reader - Tune Value"]
pub type TuneR = crate::FieldReader;
#[doc = "Field `TUNE` writer - Tune Value"]
pub type TuneW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Tune Value"]
    #[inline(always)]
    pub fn tune(&self) -> TuneR {
        TuneR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Tune Value"]
    #[inline(always)]
    pub fn tune(&mut self) -> TuneW<'_, DflltuneSpec> {
        TuneW::new(self, 0)
    }
}
#[doc = "DFLL48M Tune\n\nYou can [`read`](crate::Reg::read) this register and get [`dflltune::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflltune::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DflltuneSpec;
impl crate::RegisterSpec for DflltuneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dflltune::R`](R) reader structure"]
impl crate::Readable for DflltuneSpec {}
#[doc = "`write(|w| ..)` method takes [`dflltune::W`](W) writer structure"]
impl crate::Writable for DflltuneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFLLTUNE to value 0"]
impl crate::Resettable for DflltuneSpec {}
