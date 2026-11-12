#[doc = "Register `CRCPOLYA` reader"]
pub type R = crate::R<CrcpolyaSpec>;
#[doc = "Register `CRCPOLYA` writer"]
pub type W = crate::W<CrcpolyaSpec>;
#[doc = "Field `POLYA` reader - CRC Polynomial Coefficients A Register"]
pub type PolyaR = crate::FieldReader<u32>;
#[doc = "Field `POLYA` writer - CRC Polynomial Coefficients A Register"]
pub type PolyaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Polynomial Coefficients A Register"]
    #[inline(always)]
    pub fn polya(&self) -> PolyaR {
        PolyaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Polynomial Coefficients A Register"]
    #[inline(always)]
    pub fn polya(&mut self) -> PolyaW<'_, CrcpolyaSpec> {
        PolyaW::new(self, 0)
    }
}
#[doc = "DMA CRC POLYNOMIAL A REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpolya::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpolya::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcpolyaSpec;
impl crate::RegisterSpec for CrcpolyaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcpolya::R`](R) reader structure"]
impl crate::Readable for CrcpolyaSpec {}
#[doc = "`write(|w| ..)` method takes [`crcpolya::W`](W) writer structure"]
impl crate::Writable for CrcpolyaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCPOLYA to value 0"]
impl crate::Resettable for CrcpolyaSpec {}
