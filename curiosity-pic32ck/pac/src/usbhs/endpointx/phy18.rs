#[doc = "Register `PHY18` reader"]
pub type R = crate::R<Phy18Spec>;
#[doc = "Register `PHY18` writer"]
pub type W = crate::W<Phy18Spec>;
#[doc = "Field `ODT21` reader - Upper 3 bits for on die termination compensation voltage reference"]
pub type Odt21R = crate::FieldReader;
#[doc = "Field `ODT21` writer - Upper 3 bits for on die termination compensation voltage reference"]
pub type Odt21W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD` reader - "]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - "]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Upper 3 bits for on die termination compensation voltage reference"]
    #[inline(always)]
    pub fn odt21(&self) -> Odt21R {
        Odt21R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Upper 3 bits for on die termination compensation voltage reference"]
    #[inline(always)]
    pub fn odt21(&mut self) -> Odt21W<'_, Phy18Spec> {
        Odt21W::new(self, 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, Phy18Spec> {
        RsvdW::new(self, 2)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy18Spec;
impl crate::RegisterSpec for Phy18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy18::R`](R) reader structure"]
impl crate::Readable for Phy18Spec {}
#[doc = "`write(|w| ..)` method takes [`phy18::W`](W) writer structure"]
impl crate::Writable for Phy18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY18 to value 0x08"]
impl crate::Resettable for Phy18Spec {
    const RESET_VALUE: u32 = 0x08;
}
