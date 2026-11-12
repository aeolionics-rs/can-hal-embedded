#[doc = "Register `DSCC` writer"]
pub type W = crate::W<DsccSpec>;
#[doc = "Field `DSCKEY` writer - Data Scramble Key"]
pub type DsckeyW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `DSCEN` writer - Data Scramble Enable"]
pub type DscenW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:29 - Data Scramble Key"]
    #[inline(always)]
    pub fn dsckey(&mut self) -> DsckeyW<'_, DsccSpec> {
        DsckeyW::new(self, 0)
    }
    #[doc = "Bit 31 - Data Scramble Enable"]
    #[inline(always)]
    pub fn dscen(&mut self) -> DscenW<'_, DsccSpec> {
        DscenW::new(self, 31)
    }
}
#[doc = "Data Scramble Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsccSpec;
impl crate::RegisterSpec for DsccSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dscc::W`](W) writer structure"]
impl crate::Writable for DsccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSCC to value 0"]
impl crate::Resettable for DsccSpec {}
