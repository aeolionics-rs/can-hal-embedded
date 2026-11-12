#[doc = "Register `PHY20` reader"]
pub type R = crate::R<Phy20Spec>;
#[doc = "Register `PHY20` writer"]
pub type W = crate::W<Phy20Spec>;
#[doc = "Field `RSVD` reader - "]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - "]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HSSLEW10` reader - Lower 2 bits for HS slew adjust rate"]
pub type Hsslew10R = crate::FieldReader;
#[doc = "Field `HSSLEW10` writer - Lower 2 bits for HS slew adjust rate"]
pub type Hsslew10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Lower 2 bits for HS slew adjust rate"]
    #[inline(always)]
    pub fn hsslew10(&self) -> Hsslew10R {
        Hsslew10R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, Phy20Spec> {
        RsvdW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Lower 2 bits for HS slew adjust rate"]
    #[inline(always)]
    pub fn hsslew10(&mut self) -> Hsslew10W<'_, Phy20Spec> {
        Hsslew10W::new(self, 6)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy20Spec;
impl crate::RegisterSpec for Phy20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy20::R`](R) reader structure"]
impl crate::Readable for Phy20Spec {}
#[doc = "`write(|w| ..)` method takes [`phy20::W`](W) writer structure"]
impl crate::Writable for Phy20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY20 to value 0x80"]
impl crate::Resettable for Phy20Spec {
    const RESET_VALUE: u32 = 0x80;
}
