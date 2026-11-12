#[doc = "Register `UR` reader"]
pub type R = crate::R<UrSpec>;
#[doc = "Register `UR` writer"]
pub type W = crate::W<UrSpec>;
#[doc = "Field `UO` reader - User Programmable Outputs"]
pub type UoR = crate::FieldReader<u16>;
#[doc = "Field `UO` writer - User Programmable Outputs"]
pub type UoW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UI` reader - User Programmable Inputs"]
pub type UiR = crate::FieldReader<u16>;
#[doc = "Field `UI` writer - User Programmable Inputs"]
pub type UiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - User Programmable Outputs"]
    #[inline(always)]
    pub fn uo(&self) -> UoR {
        UoR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - User Programmable Inputs"]
    #[inline(always)]
    pub fn ui(&self) -> UiR {
        UiR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - User Programmable Outputs"]
    #[inline(always)]
    pub fn uo(&mut self) -> UoW<'_, UrSpec> {
        UoW::new(self, 0)
    }
    #[doc = "Bits 16:31 - User Programmable Inputs"]
    #[inline(always)]
    pub fn ui(&mut self) -> UiW<'_, UrSpec> {
        UiW::new(self, 16)
    }
}
#[doc = "User Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrSpec;
impl crate::RegisterSpec for UrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur::R`](R) reader structure"]
impl crate::Readable for UrSpec {}
#[doc = "`write(|w| ..)` method takes [`ur::W`](W) writer structure"]
impl crate::Writable for UrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UR to value 0"]
impl crate::Resettable for UrSpec {}
