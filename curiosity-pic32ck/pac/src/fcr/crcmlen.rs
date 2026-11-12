#[doc = "Register `CRCMLEN` reader"]
pub type R = crate::R<CrcmlenSpec>;
#[doc = "Register `CRCMLEN` writer"]
pub type W = crate::W<CrcmlenSpec>;
#[doc = "Field `CRCMLEN` reader - Message Length in Bytes"]
pub type CrcmlenR = crate::FieldReader<u32>;
#[doc = "Field `CRCMLEN` writer - Message Length in Bytes"]
pub type CrcmlenW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Message Length in Bytes"]
    #[inline(always)]
    pub fn crcmlen(&self) -> CrcmlenR {
        CrcmlenR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Message Length in Bytes"]
    #[inline(always)]
    pub fn crcmlen(&mut self) -> CrcmlenW<'_, CrcmlenSpec> {
        CrcmlenW::new(self, 0)
    }
}
#[doc = "CRC Message Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcmlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcmlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcmlenSpec;
impl crate::RegisterSpec for CrcmlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcmlen::R`](R) reader structure"]
impl crate::Readable for CrcmlenSpec {}
#[doc = "`write(|w| ..)` method takes [`crcmlen::W`](W) writer structure"]
impl crate::Writable for CrcmlenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCMLEN to value 0"]
impl crate::Resettable for CrcmlenSpec {}
