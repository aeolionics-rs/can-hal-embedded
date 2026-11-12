#[doc = "Register `CRCPOLYB` reader"]
pub type R = crate::R<CrcpolybSpec>;
#[doc = "Register `CRCPOLYB` writer"]
pub type W = crate::W<CrcpolybSpec>;
#[doc = "Field `POLYB` reader - CRC Polynomial Coefficients B Register"]
pub type PolybR = crate::FieldReader<u32>;
#[doc = "Field `POLYB` writer - CRC Polynomial Coefficients B Register"]
pub type PolybW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Polynomial Coefficients B Register"]
    #[inline(always)]
    pub fn polyb(&self) -> PolybR {
        PolybR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Polynomial Coefficients B Register"]
    #[inline(always)]
    pub fn polyb(&mut self) -> PolybW<'_, CrcpolybSpec> {
        PolybW::new(self, 0)
    }
}
#[doc = "DMA CRC POLYNOMIAL B REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpolyb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpolyb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcpolybSpec;
impl crate::RegisterSpec for CrcpolybSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcpolyb::R`](R) reader structure"]
impl crate::Readable for CrcpolybSpec {}
#[doc = "`write(|w| ..)` method takes [`crcpolyb::W`](W) writer structure"]
impl crate::Writable for CrcpolybSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCPOLYB to value 0"]
impl crate::Resettable for CrcpolybSpec {}
