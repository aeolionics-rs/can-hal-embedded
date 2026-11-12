#[doc = "Register `PHY10` reader"]
pub type R = crate::R<Phy10Spec>;
#[doc = "Register `PHY10` writer"]
pub type W = crate::W<Phy10Spec>;
#[doc = "Field `TUNE76543` reader - Upper 5 bits for HS amplitude tuning"]
pub type Tune76543R = crate::FieldReader;
#[doc = "Field `TUNE76543` writer - Upper 5 bits for HS amplitude tuning"]
pub type Tune76543W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DRVTUNE210` reader - Lower 3 bits for HS/FS/LS driver strength tuning"]
pub type Drvtune210R = crate::FieldReader;
#[doc = "Field `DRVTUNE210` writer - Lower 3 bits for HS/FS/LS driver strength tuning"]
pub type Drvtune210W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - Upper 5 bits for HS amplitude tuning"]
    #[inline(always)]
    pub fn tune76543(&self) -> Tune76543R {
        Tune76543R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Lower 3 bits for HS/FS/LS driver strength tuning"]
    #[inline(always)]
    pub fn drvtune210(&self) -> Drvtune210R {
        Drvtune210R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Upper 5 bits for HS amplitude tuning"]
    #[inline(always)]
    pub fn tune76543(&mut self) -> Tune76543W<'_, Phy10Spec> {
        Tune76543W::new(self, 0)
    }
    #[doc = "Bits 5:7 - Lower 3 bits for HS/FS/LS driver strength tuning"]
    #[inline(always)]
    pub fn drvtune210(&mut self) -> Drvtune210W<'_, Phy10Spec> {
        Drvtune210W::new(self, 5)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy10Spec;
impl crate::RegisterSpec for Phy10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy10::R`](R) reader structure"]
impl crate::Readable for Phy10Spec {}
#[doc = "`write(|w| ..)` method takes [`phy10::W`](W) writer structure"]
impl crate::Writable for Phy10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY10 to value 0xaa"]
impl crate::Resettable for Phy10Spec {
    const RESET_VALUE: u32 = 0xaa;
}
