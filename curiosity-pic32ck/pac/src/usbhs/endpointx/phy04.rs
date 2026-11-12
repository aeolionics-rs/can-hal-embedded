#[doc = "Register `PHY04` reader"]
pub type R = crate::R<Phy04Spec>;
#[doc = "Register `PHY04` writer"]
pub type W = crate::W<Phy04Spec>;
#[doc = "Field `TXPHSSEL` reader - set the Tx clock phase select"]
pub type TxphsselR = crate::FieldReader;
#[doc = "Field `TXPHSSEL` writer - set the Tx clock phase select"]
pub type TxphsselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD` reader - "]
pub type RsvdR = crate::BitReader;
#[doc = "Field `RSVD` writer - "]
pub type RsvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIZ` reader - Set D+/D- to a high impedence state"]
pub type HizR = crate::BitReader;
#[doc = "Field `HIZ` writer - Set D+/D- to a high impedence state"]
pub type HizW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SQUELCH210` reader - Lower 3 bits of RX squelch trigger point configuration"]
pub type Squelch210R = crate::FieldReader;
#[doc = "Field `SQUELCH210` writer - Lower 3 bits of RX squelch trigger point configuration"]
pub type Squelch210W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - set the Tx clock phase select"]
    #[inline(always)]
    pub fn txphssel(&self) -> TxphsselR {
        TxphsselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set D+/D- to a high impedence state"]
    #[inline(always)]
    pub fn hiz(&self) -> HizR {
        HizR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Lower 3 bits of RX squelch trigger point configuration"]
    #[inline(always)]
    pub fn squelch210(&self) -> Squelch210R {
        Squelch210R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - set the Tx clock phase select"]
    #[inline(always)]
    pub fn txphssel(&mut self) -> TxphsselW<'_, Phy04Spec> {
        TxphsselW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, Phy04Spec> {
        RsvdW::new(self, 3)
    }
    #[doc = "Bit 4 - Set D+/D- to a high impedence state"]
    #[inline(always)]
    pub fn hiz(&mut self) -> HizW<'_, Phy04Spec> {
        HizW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Lower 3 bits of RX squelch trigger point configuration"]
    #[inline(always)]
    pub fn squelch210(&mut self) -> Squelch210W<'_, Phy04Spec> {
        Squelch210W::new(self, 5)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy04Spec;
impl crate::RegisterSpec for Phy04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy04::R`](R) reader structure"]
impl crate::Readable for Phy04Spec {}
#[doc = "`write(|w| ..)` method takes [`phy04::W`](W) writer structure"]
impl crate::Writable for Phy04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY04 to value 0x8f"]
impl crate::Resettable for Phy04Spec {
    const RESET_VALUE: u32 = 0x8f;
}
