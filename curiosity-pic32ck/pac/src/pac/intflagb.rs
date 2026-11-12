#[doc = "Register `INTFLAGB` reader"]
pub type R = crate::R<IntflagbSpec>;
#[doc = "Register `INTFLAGB` writer"]
pub type W = crate::W<IntflagbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral interrupt flag status - Bridge B\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagbSpec;
impl crate::RegisterSpec for IntflagbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagb::R`](R) reader structure"]
impl crate::Readable for IntflagbSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagb::W`](W) writer structure"]
impl crate::Writable for IntflagbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFLAGB to value 0"]
impl crate::Resettable for IntflagbSpec {}
