#[doc = "Register `ERROR` reader"]
pub type R = crate::R<ErrorSpec>;
#[doc = "Register `ERROR` writer"]
pub type W = crate::W<ErrorSpec>;
#[doc = "Field `BERR` reader - AHB Bus Error Detection"]
pub type BerrR = crate::BitReader;
#[doc = "Field `BERR` writer - AHB Bus Error Detection"]
pub type BerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AHB Bus Error Detection"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB Bus Error Detection"]
    #[inline(always)]
    pub fn berr(&mut self) -> BerrW<'_, ErrorSpec> {
        BerrW::new(self, 0)
    }
}
#[doc = "Error Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`error::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorSpec;
impl crate::RegisterSpec for ErrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error::R`](R) reader structure"]
impl crate::Readable for ErrorSpec {}
#[doc = "`write(|w| ..)` method takes [`error::W`](W) writer structure"]
impl crate::Writable for ErrorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERROR to value 0"]
impl crate::Resettable for ErrorSpec {}
