#[doc = "Register `BRG` reader"]
pub type R = crate::R<BrgSpec>;
#[doc = "Register `BRG` writer"]
pub type W = crate::W<BrgSpec>;
#[doc = "Field `BRG` reader - Baud Rate Divisor bits"]
pub type BrgR = crate::FieldReader<u16>;
#[doc = "Field `BRG` writer - Baud Rate Divisor bits"]
pub type BrgW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Baud Rate Divisor bits"]
    #[inline(always)]
    pub fn brg(&self) -> BrgR {
        BrgR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Baud Rate Divisor bits"]
    #[inline(always)]
    pub fn brg(&mut self) -> BrgW<'_, BrgSpec> {
        BrgW::new(self, 0)
    }
}
#[doc = "SPI Baud Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`brg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrgSpec;
impl crate::RegisterSpec for BrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brg::R`](R) reader structure"]
impl crate::Readable for BrgSpec {}
#[doc = "`write(|w| ..)` method takes [`brg::W`](W) writer structure"]
impl crate::Writable for BrgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRG to value 0"]
impl crate::Resettable for BrgSpec {}
