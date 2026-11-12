#[doc = "Register `INTFLAGA` reader"]
pub type R = crate::R<IntflagaSpec>;
#[doc = "Register `INTFLAGA` writer"]
pub type W = crate::W<IntflagaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral interrupt flag status - Bridge A\n\nYou can [`read`](crate::Reg::read) this register and get [`intflaga::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflaga::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagaSpec;
impl crate::RegisterSpec for IntflagaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflaga::R`](R) reader structure"]
impl crate::Readable for IntflagaSpec {}
#[doc = "`write(|w| ..)` method takes [`intflaga::W`](W) writer structure"]
impl crate::Writable for IntflagaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFLAGA to value 0"]
impl crate::Resettable for IntflagaSpec {}
