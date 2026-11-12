#[doc = "Register `PHY14` reader"]
pub type R = crate::R<Phy14Spec>;
#[doc = "Register `PHY14` writer"]
pub type W = crate::W<Phy14Spec>;
#[doc = "Field `DRVTUNE43` reader - Upper 2 bits for HS/FS/LS driver strength tuning"]
pub type Drvtune43R = crate::FieldReader;
#[doc = "Field `DRVTUNE43` writer - Upper 2 bits for HS/FS/LS driver strength tuning"]
pub type Drvtune43W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMPBYPSS` reader - Auto compensation bypass"]
pub type CompbypssR = crate::FieldReader;
#[doc = "Field `COMPBYPSS` writer - Auto compensation bypass"]
pub type CompbypssW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BYPSSSQUELCH` reader - Bypass squelch trigger point configure in chirp mode"]
pub type BypsssquelchR = crate::BitReader;
#[doc = "Field `BYPSSSQUELCH` writer - Bypass squelch trigger point configure in chirp mode"]
pub type BypsssquelchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT0` reader - On die termination compensation voltage reference"]
pub type Odt0R = crate::BitReader;
#[doc = "Field `ODT0` writer - On die termination compensation voltage reference"]
pub type Odt0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Upper 2 bits for HS/FS/LS driver strength tuning"]
    #[inline(always)]
    pub fn drvtune43(&self) -> Drvtune43R {
        Drvtune43R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Auto compensation bypass"]
    #[inline(always)]
    pub fn compbypss(&self) -> CompbypssR {
        CompbypssR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Bypass squelch trigger point configure in chirp mode"]
    #[inline(always)]
    pub fn bypsssquelch(&self) -> BypsssquelchR {
        BypsssquelchR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - On die termination compensation voltage reference"]
    #[inline(always)]
    pub fn odt0(&self) -> Odt0R {
        Odt0R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Upper 2 bits for HS/FS/LS driver strength tuning"]
    #[inline(always)]
    pub fn drvtune43(&mut self) -> Drvtune43W<'_, Phy14Spec> {
        Drvtune43W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Auto compensation bypass"]
    #[inline(always)]
    pub fn compbypss(&mut self) -> CompbypssW<'_, Phy14Spec> {
        CompbypssW::new(self, 2)
    }
    #[doc = "Bit 4 - Bypass squelch trigger point configure in chirp mode"]
    #[inline(always)]
    pub fn bypsssquelch(&mut self) -> BypsssquelchW<'_, Phy14Spec> {
        BypsssquelchW::new(self, 4)
    }
    #[doc = "Bit 7 - On die termination compensation voltage reference"]
    #[inline(always)]
    pub fn odt0(&mut self) -> Odt0W<'_, Phy14Spec> {
        Odt0W::new(self, 7)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy14Spec;
impl crate::RegisterSpec for Phy14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy14::R`](R) reader structure"]
impl crate::Readable for Phy14Spec {}
#[doc = "`write(|w| ..)` method takes [`phy14::W`](W) writer structure"]
impl crate::Writable for Phy14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY14 to value 0x12"]
impl crate::Resettable for Phy14Spec {
    const RESET_VALUE: u32 = 0x12;
}
