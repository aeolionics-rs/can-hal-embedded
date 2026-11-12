#[doc = "Register `HSEOF1` reader"]
pub type R = crate::R<Hseof1Spec>;
#[doc = "Register `HSEOF1` writer"]
pub type W = crate::W<Hseof1Spec>;
#[doc = "Field `HSEOF1` reader - sets for high-speed transactions the time before eof to stop beginning new transactions, in units of 133.3ns. (the default setting corresponds to 17.07us.)"]
pub type Hseof1R = crate::FieldReader;
#[doc = "Field `HSEOF1` writer - sets for high-speed transactions the time before eof to stop beginning new transactions, in units of 133.3ns. (the default setting corresponds to 17.07us.)"]
pub type Hseof1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - sets for high-speed transactions the time before eof to stop beginning new transactions, in units of 133.3ns. (the default setting corresponds to 17.07us.)"]
    #[inline(always)]
    pub fn hseof1(&self) -> Hseof1R {
        Hseof1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - sets for high-speed transactions the time before eof to stop beginning new transactions, in units of 133.3ns. (the default setting corresponds to 17.07us.)"]
    #[inline(always)]
    pub fn hseof1(&mut self) -> Hseof1W<'_, Hseof1Spec> {
        Hseof1W::new(self, 0)
    }
}
#[doc = "Sets The Minimum Time Gap That Is To Be Allowed Between The Start Of The Last Transaction And The EOF For HS Transactions\n\nYou can [`read`](crate::Reg::read) this register and get [`hseof1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hseof1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hseof1Spec;
impl crate::RegisterSpec for Hseof1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hseof1::R`](R) reader structure"]
impl crate::Readable for Hseof1Spec {}
#[doc = "`write(|w| ..)` method takes [`hseof1::W`](W) writer structure"]
impl crate::Writable for Hseof1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSEOF1 to value 0x80"]
impl crate::Resettable for Hseof1Spec {
    const RESET_VALUE: u8 = 0x80;
}
