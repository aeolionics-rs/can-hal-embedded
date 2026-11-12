#[doc = "Register `PHY48` reader"]
pub type R = crate::R<Phy48Spec>;
#[doc = "Register `PHY48` writer"]
pub type W = crate::W<Phy48Spec>;
#[doc = "Field `FRCASESSVAL` reader - force A_sessionvalid"]
pub type FrcasessvalR = crate::BitReader;
#[doc = "Field `FRCASESSVAL` writer - force A_sessionvalid"]
pub type FrcasessvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCBSESSVAL` reader - force B_sessionvalid"]
pub type FrcbsessvalR = crate::BitReader;
#[doc = "Field `FRCBSESSVAL` writer - force B_sessionvalid"]
pub type FrcbsessvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSCHRGE` reader - VBus charging/discharging bypass"]
pub type VbuschrgeR = crate::BitReader;
#[doc = "Field `VBUSCHRGE` writer - VBus charging/discharging bypass"]
pub type VbuschrgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SESSENDTUNE` reader - Session end reference tuning"]
pub type SessendtuneR = crate::FieldReader;
#[doc = "Field `SESSENDTUNE` writer - Session end reference tuning"]
pub type SessendtuneW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - force A_sessionvalid"]
    #[inline(always)]
    pub fn frcasessval(&self) -> FrcasessvalR {
        FrcasessvalR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - force B_sessionvalid"]
    #[inline(always)]
    pub fn frcbsessval(&self) -> FrcbsessvalR {
        FrcbsessvalR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBus charging/discharging bypass"]
    #[inline(always)]
    pub fn vbuschrge(&self) -> VbuschrgeR {
        VbuschrgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Session end reference tuning"]
    #[inline(always)]
    pub fn sessendtune(&self) -> SessendtuneR {
        SessendtuneR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - force A_sessionvalid"]
    #[inline(always)]
    pub fn frcasessval(&mut self) -> FrcasessvalW<'_, Phy48Spec> {
        FrcasessvalW::new(self, 0)
    }
    #[doc = "Bit 1 - force B_sessionvalid"]
    #[inline(always)]
    pub fn frcbsessval(&mut self) -> FrcbsessvalW<'_, Phy48Spec> {
        FrcbsessvalW::new(self, 1)
    }
    #[doc = "Bit 2 - VBus charging/discharging bypass"]
    #[inline(always)]
    pub fn vbuschrge(&mut self) -> VbuschrgeW<'_, Phy48Spec> {
        VbuschrgeW::new(self, 2)
    }
    #[doc = "Bits 5:7 - Session end reference tuning"]
    #[inline(always)]
    pub fn sessendtune(&mut self) -> SessendtuneW<'_, Phy48Spec> {
        SessendtuneW::new(self, 5)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy48Spec;
impl crate::RegisterSpec for Phy48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy48::R`](R) reader structure"]
impl crate::Readable for Phy48Spec {}
#[doc = "`write(|w| ..)` method takes [`phy48::W`](W) writer structure"]
impl crate::Writable for Phy48Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY48 to value 0x04"]
impl crate::Resettable for Phy48Spec {
    const RESET_VALUE: u32 = 0x04;
}
