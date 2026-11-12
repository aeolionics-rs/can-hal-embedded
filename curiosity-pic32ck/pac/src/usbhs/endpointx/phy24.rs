#[doc = "Register `PHY24` reader"]
pub type R = crate::R<Phy24Spec>;
#[doc = "Register `PHY24` writer"]
pub type W = crate::W<Phy24Spec>;
#[doc = "Field `HSSLEW2` reader - MSB of HS slew rate adjust"]
pub type Hsslew2R = crate::BitReader;
#[doc = "Field `HSSLEW2` writer - MSB of HS slew rate adjust"]
pub type Hsslew2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSDETEN` reader - VBUS Detection Switch 0 - Closed, 1 - Open"]
pub type VbusdetenR = crate::BitReader;
#[doc = "Field `VBUSDETEN` writer - VBUS Detection Switch 0 - Closed, 1 - Open"]
pub type VbusdetenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENHLFBITPRE` reader - Enable half-bit pre-emphasis for HS transmit"]
pub type EnhlfbitpreR = crate::BitReader;
#[doc = "Field `ENHLFBITPRE` writer - Enable half-bit pre-emphasis for HS transmit"]
pub type EnhlfbitpreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSPREEMPST` reader - HS transmit pre emphasis strength"]
pub type HspreempstR = crate::FieldReader;
#[doc = "Field `HSPREEMPST` writer - HS transmit pre emphasis strength"]
pub type HspreempstW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HSDRIVST10` reader - HS transmit drive strength"]
pub type Hsdrivst10R = crate::FieldReader;
#[doc = "Field `HSDRIVST10` writer - HS transmit drive strength"]
pub type Hsdrivst10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - MSB of HS slew rate adjust"]
    #[inline(always)]
    pub fn hsslew2(&self) -> Hsslew2R {
        Hsslew2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBUS Detection Switch 0 - Closed, 1 - Open"]
    #[inline(always)]
    pub fn vbusdeten(&self) -> VbusdetenR {
        VbusdetenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable half-bit pre-emphasis for HS transmit"]
    #[inline(always)]
    pub fn enhlfbitpre(&self) -> EnhlfbitpreR {
        EnhlfbitpreR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - HS transmit pre emphasis strength"]
    #[inline(always)]
    pub fn hspreempst(&self) -> HspreempstR {
        HspreempstR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - HS transmit drive strength"]
    #[inline(always)]
    pub fn hsdrivst10(&self) -> Hsdrivst10R {
        Hsdrivst10R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MSB of HS slew rate adjust"]
    #[inline(always)]
    pub fn hsslew2(&mut self) -> Hsslew2W<'_, Phy24Spec> {
        Hsslew2W::new(self, 0)
    }
    #[doc = "Bit 1 - VBUS Detection Switch 0 - Closed, 1 - Open"]
    #[inline(always)]
    pub fn vbusdeten(&mut self) -> VbusdetenW<'_, Phy24Spec> {
        VbusdetenW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable half-bit pre-emphasis for HS transmit"]
    #[inline(always)]
    pub fn enhlfbitpre(&mut self) -> EnhlfbitpreW<'_, Phy24Spec> {
        EnhlfbitpreW::new(self, 2)
    }
    #[doc = "Bits 3:5 - HS transmit pre emphasis strength"]
    #[inline(always)]
    pub fn hspreempst(&mut self) -> HspreempstW<'_, Phy24Spec> {
        HspreempstW::new(self, 3)
    }
    #[doc = "Bits 6:7 - HS transmit drive strength"]
    #[inline(always)]
    pub fn hsdrivst10(&mut self) -> Hsdrivst10W<'_, Phy24Spec> {
        Hsdrivst10W::new(self, 6)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy24Spec;
impl crate::RegisterSpec for Phy24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy24::R`](R) reader structure"]
impl crate::Readable for Phy24Spec {}
#[doc = "`write(|w| ..)` method takes [`phy24::W`](W) writer structure"]
impl crate::Writable for Phy24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY24 to value 0x0c"]
impl crate::Resettable for Phy24Spec {
    const RESET_VALUE: u32 = 0x0c;
}
