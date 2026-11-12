#[doc = "Register `CRCFXOR` reader"]
pub type R = crate::R<CrcfxorSpec>;
#[doc = "Register `CRCFXOR` writer"]
pub type W = crate::W<CrcfxorSpec>;
#[doc = "Field `CRCFXOR` reader - CRC Final XOR"]
pub type CrcfxorR = crate::FieldReader<u32>;
#[doc = "Field `CRCFXOR` writer - CRC Final XOR"]
pub type CrcfxorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Final XOR"]
    #[inline(always)]
    pub fn crcfxor(&self) -> CrcfxorR {
        CrcfxorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Final XOR"]
    #[inline(always)]
    pub fn crcfxor(&mut self) -> CrcfxorW<'_, CrcfxorSpec> {
        CrcfxorW::new(self, 0)
    }
}
#[doc = "CRC Final XOR Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcfxor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcfxor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcfxorSpec;
impl crate::RegisterSpec for CrcfxorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcfxor::R`](R) reader structure"]
impl crate::Readable for CrcfxorSpec {}
#[doc = "`write(|w| ..)` method takes [`crcfxor::W`](W) writer structure"]
impl crate::Writable for CrcfxorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCFXOR to value 0"]
impl crate::Resettable for CrcfxorSpec {}
