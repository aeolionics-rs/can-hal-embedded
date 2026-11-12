#[doc = "Register `PHY28` reader"]
pub type R = crate::R<Phy28Spec>;
#[doc = "Register `PHY28` writer"]
pub type W = crate::W<Phy28Spec>;
#[doc = "Field `HSDRIVST2` reader - MSB of HS transmit driver strength"]
pub type Hsdrivst2R = crate::BitReader;
#[doc = "Field `HSDRIVST2` writer - MSB of HS transmit driver strength"]
pub type Hsdrivst2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCONDET` reader - HOST disconnects detection trigger point"]
pub type DiscondetR = crate::FieldReader;
#[doc = "Field `DISCONDET` writer - HOST disconnects detection trigger point"]
pub type DiscondetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HSDRVCOMP` reader - HS drive current compensation voltage reference"]
pub type HsdrvcompR = crate::FieldReader;
#[doc = "Field `HSDRVCOMP` writer - HS drive current compensation voltage reference"]
pub type HsdrvcompW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - MSB of HS transmit driver strength"]
    #[inline(always)]
    pub fn hsdrivst2(&self) -> Hsdrivst2R {
        Hsdrivst2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - HOST disconnects detection trigger point"]
    #[inline(always)]
    pub fn discondet(&self) -> DiscondetR {
        DiscondetR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - HS drive current compensation voltage reference"]
    #[inline(always)]
    pub fn hsdrvcomp(&self) -> HsdrvcompR {
        HsdrvcompR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MSB of HS transmit driver strength"]
    #[inline(always)]
    pub fn hsdrivst2(&mut self) -> Hsdrivst2W<'_, Phy28Spec> {
        Hsdrivst2W::new(self, 0)
    }
    #[doc = "Bits 1:4 - HOST disconnects detection trigger point"]
    #[inline(always)]
    pub fn discondet(&mut self) -> DiscondetW<'_, Phy28Spec> {
        DiscondetW::new(self, 1)
    }
    #[doc = "Bits 5:7 - HS drive current compensation voltage reference"]
    #[inline(always)]
    pub fn hsdrvcomp(&mut self) -> HsdrvcompW<'_, Phy28Spec> {
        HsdrvcompW::new(self, 5)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy28Spec;
impl crate::RegisterSpec for Phy28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy28::R`](R) reader structure"]
impl crate::Readable for Phy28Spec {}
#[doc = "`write(|w| ..)` method takes [`phy28::W`](W) writer structure"]
impl crate::Writable for Phy28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY28 to value 0x1b"]
impl crate::Resettable for Phy28Spec {
    const RESET_VALUE: u32 = 0x1b;
}
