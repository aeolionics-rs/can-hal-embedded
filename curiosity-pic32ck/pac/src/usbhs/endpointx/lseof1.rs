#[doc = "Register `LSEOF1` reader"]
pub type R = crate::R<Lseof1Spec>;
#[doc = "Register `LSEOF1` writer"]
pub type W = crate::W<Lseof1Spec>;
#[doc = "Field `LSEOF1` reader - sets for low-speed transactions the time before eof to stop beginning new transactions, in units of 1.067us. (the default setting corresponds to 121.6us.)"]
pub type Lseof1R = crate::FieldReader;
#[doc = "Field `LSEOF1` writer - sets for low-speed transactions the time before eof to stop beginning new transactions, in units of 1.067us. (the default setting corresponds to 121.6us.)"]
pub type Lseof1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - sets for low-speed transactions the time before eof to stop beginning new transactions, in units of 1.067us. (the default setting corresponds to 121.6us.)"]
    #[inline(always)]
    pub fn lseof1(&self) -> Lseof1R {
        Lseof1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - sets for low-speed transactions the time before eof to stop beginning new transactions, in units of 1.067us. (the default setting corresponds to 121.6us.)"]
    #[inline(always)]
    pub fn lseof1(&mut self) -> Lseof1W<'_, Lseof1Spec> {
        Lseof1W::new(self, 0)
    }
}
#[doc = "Sets The Minimum Time Gap That Is To Be Allowed Between The Start Of The Last Transaction And The EOF For LS Transactions\n\nYou can [`read`](crate::Reg::read) this register and get [`lseof1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lseof1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lseof1Spec;
impl crate::RegisterSpec for Lseof1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lseof1::R`](R) reader structure"]
impl crate::Readable for Lseof1Spec {}
#[doc = "`write(|w| ..)` method takes [`lseof1::W`](W) writer structure"]
impl crate::Writable for Lseof1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LSEOF1 to value 0x72"]
impl crate::Resettable for Lseof1Spec {
    const RESET_VALUE: u8 = 0x72;
}
