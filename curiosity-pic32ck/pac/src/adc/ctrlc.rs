#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Field `CNT` reader - Clock Divider for Synchronous Trigger"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Clock Divider for Synchronous Trigger"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `COREINTERLEAVED` reader - Number of Core to Interleave Triggers"]
pub type CoreinterleavedR = crate::FieldReader;
#[doc = "Field `COREINTERLEAVED` writer - Number of Core to Interleave Triggers"]
pub type CoreinterleavedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - Clock Divider for Synchronous Trigger"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:30 - Number of Core to Interleave Triggers"]
    #[inline(always)]
    pub fn coreinterleaved(&self) -> CoreinterleavedR {
        CoreinterleavedR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divider for Synchronous Trigger"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, CtrlcSpec> {
        CntW::new(self, 0)
    }
    #[doc = "Bits 28:30 - Number of Core to Interleave Triggers"]
    #[inline(always)]
    pub fn coreinterleaved(&mut self) -> CoreinterleavedW<'_, CtrlcSpec> {
        CoreinterleavedW::new(self, 28)
    }
}
#[doc = "Control C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {}
