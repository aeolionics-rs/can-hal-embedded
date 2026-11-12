#[doc = "Register `FSEOF1` reader"]
pub type R = crate::R<Fseof1Spec>;
#[doc = "Register `FSEOF1` writer"]
pub type W = crate::W<Fseof1Spec>;
#[doc = "Field `FSEOF1` reader - sets for full-speed transactions the time before eof to stop beginning new transactions, in units of 533.3ns. (the default setting corresponds to 63.46us.)"]
pub type Fseof1R = crate::FieldReader;
#[doc = "Field `FSEOF1` writer - sets for full-speed transactions the time before eof to stop beginning new transactions, in units of 533.3ns. (the default setting corresponds to 63.46us.)"]
pub type Fseof1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - sets for full-speed transactions the time before eof to stop beginning new transactions, in units of 533.3ns. (the default setting corresponds to 63.46us.)"]
    #[inline(always)]
    pub fn fseof1(&self) -> Fseof1R {
        Fseof1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - sets for full-speed transactions the time before eof to stop beginning new transactions, in units of 533.3ns. (the default setting corresponds to 63.46us.)"]
    #[inline(always)]
    pub fn fseof1(&mut self) -> Fseof1W<'_, Fseof1Spec> {
        Fseof1W::new(self, 0)
    }
}
#[doc = "Sets The Minimum Time Gap That Is To Be Allowed Between The Start Of The Last Transaction And The EOF For FS Transactions\n\nYou can [`read`](crate::Reg::read) this register and get [`fseof1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fseof1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fseof1Spec;
impl crate::RegisterSpec for Fseof1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fseof1::R`](R) reader structure"]
impl crate::Readable for Fseof1Spec {}
#[doc = "`write(|w| ..)` method takes [`fseof1::W`](W) writer structure"]
impl crate::Writable for Fseof1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FSEOF1 to value 0x77"]
impl crate::Resettable for Fseof1Spec {
    const RESET_VALUE: u8 = 0x77;
}
