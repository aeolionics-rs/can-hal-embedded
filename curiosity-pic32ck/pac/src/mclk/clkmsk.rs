#[doc = "Register `CLKMSK[%s]` reader"]
pub type R = crate::R<ClkmskSpec>;
#[doc = "Register `CLKMSK[%s]` writer"]
pub type W = crate::W<ClkmskSpec>;
#[doc = "Field `MASK` reader - Peripheral Clock Enable Mask"]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - Peripheral Clock Enable Mask"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral Clock Enable Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<'_, ClkmskSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "Peripheral Clock Enable Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`clkmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkmskSpec;
impl crate::RegisterSpec for ClkmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkmsk::R`](R) reader structure"]
impl crate::Readable for ClkmskSpec {}
#[doc = "`write(|w| ..)` method takes [`clkmsk::W`](W) writer structure"]
impl crate::Writable for ClkmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKMSK[%s] to value 0"]
impl crate::Resettable for ClkmskSpec {}
