#[doc = "Register `CRCMADR` reader"]
pub type R = crate::R<CrcmadrSpec>;
#[doc = "Register `CRCMADR` writer"]
pub type W = crate::W<CrcmadrSpec>;
#[doc = "Field `CRCMADR` reader - Message Start Address in Flash"]
pub type CrcmadrR = crate::FieldReader<u32>;
#[doc = "Field `CRCMADR` writer - Message Start Address in Flash"]
pub type CrcmadrW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Message Start Address in Flash"]
    #[inline(always)]
    pub fn crcmadr(&self) -> CrcmadrR {
        CrcmadrR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Message Start Address in Flash"]
    #[inline(always)]
    pub fn crcmadr(&mut self) -> CrcmadrW<'_, CrcmadrSpec> {
        CrcmadrW::new(self, 0)
    }
}
#[doc = "CRC Message Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcmadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcmadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcmadrSpec;
impl crate::RegisterSpec for CrcmadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcmadr::R`](R) reader structure"]
impl crate::Readable for CrcmadrSpec {}
#[doc = "`write(|w| ..)` method takes [`crcmadr::W`](W) writer structure"]
impl crate::Writable for CrcmadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCMADR to value 0"]
impl crate::Resettable for CrcmadrSpec {}
