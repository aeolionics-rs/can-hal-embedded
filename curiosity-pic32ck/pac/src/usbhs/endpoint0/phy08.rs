#[doc = "Register `PHY08` reader"]
pub type R = crate::R<Phy08Spec>;
#[doc = "Register `PHY08` writer"]
pub type W = crate::W<Phy08Spec>;
#[doc = "Field `SQUELCH3` reader - MSB of Squelch configuration"]
pub type Squelch3R = crate::BitReader;
#[doc = "Field `SQUELCH3` writer - MSB of Squelch configuration"]
pub type Squelch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - "]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - "]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - MSB of Squelch configuration"]
    #[inline(always)]
    pub fn squelch3(&self) -> Squelch3R {
        Squelch3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MSB of Squelch configuration"]
    #[inline(always)]
    pub fn squelch3(&mut self) -> Squelch3W<'_, Phy08Spec> {
        Squelch3W::new(self, 0)
    }
    #[doc = "Bits 1:7"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, Phy08Spec> {
        RsvdW::new(self, 1)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy08Spec;
impl crate::RegisterSpec for Phy08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy08::R`](R) reader structure"]
impl crate::Readable for Phy08Spec {}
#[doc = "`write(|w| ..)` method takes [`phy08::W`](W) writer structure"]
impl crate::Writable for Phy08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY08 to value 0x07"]
impl crate::Resettable for Phy08Spec {
    const RESET_VALUE: u32 = 0x07;
}
