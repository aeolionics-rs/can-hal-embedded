#[doc = "Register `PHY44` reader"]
pub type R = crate::R<Phy44Spec>;
#[doc = "Register `PHY44` writer"]
pub type W = crate::W<Phy44Spec>;
#[doc = "Field `PLLDAMP` reader - PLL damping factor"]
pub type PlldampR = crate::BitReader;
#[doc = "Field `PLLDAMP` writer - PLL damping factor"]
pub type PlldampW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIGDBG` reader - Digital debug interface"]
pub type DigdbgR = crate::BitReader;
#[doc = "Field `DIGDBG` writer - Digital debug interface"]
pub type DigdbgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCVBUSVAL` reader - Force output vbus_valid"]
pub type FrcvbusvalR = crate::BitReader;
#[doc = "Field `FRCVBUSVAL` writer - Force output vbus_valid"]
pub type FrcvbusvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - "]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - "]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FRCSESSEND` reader - force session end"]
pub type FrcsessendR = crate::BitReader;
#[doc = "Field `FRCSESSEND` writer - force session end"]
pub type FrcsessendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - PLL damping factor"]
    #[inline(always)]
    pub fn plldamp(&self) -> PlldampR {
        PlldampR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Digital debug interface"]
    #[inline(always)]
    pub fn digdbg(&self) -> DigdbgR {
        DigdbgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force output vbus_valid"]
    #[inline(always)]
    pub fn frcvbusval(&self) -> FrcvbusvalR {
        FrcvbusvalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - force session end"]
    #[inline(always)]
    pub fn frcsessend(&self) -> FrcsessendR {
        FrcsessendR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PLL damping factor"]
    #[inline(always)]
    pub fn plldamp(&mut self) -> PlldampW<'_, Phy44Spec> {
        PlldampW::new(self, 1)
    }
    #[doc = "Bit 2 - Digital debug interface"]
    #[inline(always)]
    pub fn digdbg(&mut self) -> DigdbgW<'_, Phy44Spec> {
        DigdbgW::new(self, 2)
    }
    #[doc = "Bit 3 - Force output vbus_valid"]
    #[inline(always)]
    pub fn frcvbusval(&mut self) -> FrcvbusvalW<'_, Phy44Spec> {
        FrcvbusvalW::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, Phy44Spec> {
        RsvdW::new(self, 4)
    }
    #[doc = "Bit 7 - force session end"]
    #[inline(always)]
    pub fn frcsessend(&mut self) -> FrcsessendW<'_, Phy44Spec> {
        FrcsessendW::new(self, 7)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy44Spec;
impl crate::RegisterSpec for Phy44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy44::R`](R) reader structure"]
impl crate::Readable for Phy44Spec {}
#[doc = "`write(|w| ..)` method takes [`phy44::W`](W) writer structure"]
impl crate::Writable for Phy44Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY44 to value 0x40"]
impl crate::Resettable for Phy44Spec {
    const RESET_VALUE: u32 = 0x40;
}
