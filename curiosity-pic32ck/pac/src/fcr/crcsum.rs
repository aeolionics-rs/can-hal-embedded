#[doc = "Register `CRCSUM` reader"]
pub type R = crate::R<CrcsumSpec>;
#[doc = "Register `CRCSUM` writer"]
pub type W = crate::W<CrcsumSpec>;
#[doc = "Field `CRCSUM` reader - CRC Checksum"]
pub type CrcsumR = crate::FieldReader<u32>;
#[doc = "Field `CRCSUM` writer - CRC Checksum"]
pub type CrcsumW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Checksum"]
    #[inline(always)]
    pub fn crcsum(&self) -> CrcsumR {
        CrcsumR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Checksum"]
    #[inline(always)]
    pub fn crcsum(&mut self) -> CrcsumW<'_, CrcsumSpec> {
        CrcsumW::new(self, 0)
    }
}
#[doc = "CRC Checksum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcsum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcsumSpec;
impl crate::RegisterSpec for CrcsumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcsum::R`](R) reader structure"]
impl crate::Readable for CrcsumSpec {}
#[doc = "`write(|w| ..)` method takes [`crcsum::W`](W) writer structure"]
impl crate::Writable for CrcsumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCSUM to value 0"]
impl crate::Resettable for CrcsumSpec {}
