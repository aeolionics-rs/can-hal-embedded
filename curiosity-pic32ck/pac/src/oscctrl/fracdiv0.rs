#[doc = "Register `FRACDIV0` reader"]
pub type R = crate::R<Fracdiv0Spec>;
#[doc = "Register `FRACDIV0` writer"]
pub type W = crate::W<Fracdiv0Spec>;
#[doc = "Field `REMDIV` reader - Frequency division factor reminder part"]
pub type RemdivR = crate::FieldReader<u16>;
#[doc = "Field `REMDIV` writer - Frequency division factor reminder part"]
pub type RemdivW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `INTDIV` reader - Frequency division factor integer part"]
pub type IntdivR = crate::FieldReader<u16>;
#[doc = "Field `INTDIV` writer - Frequency division factor integer part"]
pub type IntdivW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 7:15 - Frequency division factor reminder part"]
    #[inline(always)]
    pub fn remdiv(&self) -> RemdivR {
        RemdivR::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:30 - Frequency division factor integer part"]
    #[inline(always)]
    pub fn intdiv(&self) -> IntdivR {
        IntdivR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 7:15 - Frequency division factor reminder part"]
    #[inline(always)]
    pub fn remdiv(&mut self) -> RemdivW<'_, Fracdiv0Spec> {
        RemdivW::new(self, 7)
    }
    #[doc = "Bits 16:30 - Frequency division factor integer part"]
    #[inline(always)]
    pub fn intdiv(&mut self) -> IntdivW<'_, Fracdiv0Spec> {
        IntdivW::new(self, 16)
    }
}
#[doc = "Fractional Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fracdiv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fracdiv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fracdiv0Spec;
impl crate::RegisterSpec for Fracdiv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fracdiv0::R`](R) reader structure"]
impl crate::Readable for Fracdiv0Spec {}
#[doc = "`write(|w| ..)` method takes [`fracdiv0::W`](W) writer structure"]
impl crate::Writable for Fracdiv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRACDIV0 to value 0x0020_0000"]
impl crate::Resettable for Fracdiv0Spec {
    const RESET_VALUE: u32 = 0x0020_0000;
}
