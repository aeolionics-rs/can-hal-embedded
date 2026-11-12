#[doc = "Register `CRCIV` reader"]
pub type R = crate::R<CrcivSpec>;
#[doc = "Register `CRCIV` writer"]
pub type W = crate::W<CrcivSpec>;
#[doc = "Field `CRCIV` reader - CRC Initial Value"]
pub type CrcivR = crate::FieldReader<u32>;
#[doc = "Field `CRCIV` writer - CRC Initial Value"]
pub type CrcivW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Initial Value"]
    #[inline(always)]
    pub fn crciv(&self) -> CrcivR {
        CrcivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Initial Value"]
    #[inline(always)]
    pub fn crciv(&mut self) -> CrcivW<'_, CrcivSpec> {
        CrcivW::new(self, 0)
    }
}
#[doc = "CRC Initial Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crciv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crciv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcivSpec;
impl crate::RegisterSpec for CrcivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crciv::R`](R) reader structure"]
impl crate::Readable for CrcivSpec {}
#[doc = "`write(|w| ..)` method takes [`crciv::W`](W) writer structure"]
impl crate::Writable for CrcivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCIV to value 0"]
impl crate::Resettable for CrcivSpec {}
