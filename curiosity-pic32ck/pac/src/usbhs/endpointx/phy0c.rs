#[doc = "Register `PHY0C` reader"]
pub type R = crate::R<Phy0cSpec>;
#[doc = "Register `PHY0C` writer"]
pub type W = crate::W<Phy0cSpec>;
#[doc = "Field `TUNE210` reader - Lower 3 bits for HS amplitude tuning"]
pub type Tune210R = crate::FieldReader;
#[doc = "Field `TUNE210` writer - Lower 3 bits for HS amplitude tuning"]
pub type Tune210W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 5:11 - Lower 3 bits for HS amplitude tuning"]
    #[inline(always)]
    pub fn tune210(&self) -> Tune210R {
        Tune210R::new(((self.bits >> 5) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 5:11 - Lower 3 bits for HS amplitude tuning"]
    #[inline(always)]
    pub fn tune210(&mut self) -> Tune210W<'_, Phy0cSpec> {
        Tune210W::new(self, 5)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy0cSpec;
impl crate::RegisterSpec for Phy0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy0c::R`](R) reader structure"]
impl crate::Readable for Phy0cSpec {}
#[doc = "`write(|w| ..)` method takes [`phy0c::W`](W) writer structure"]
impl crate::Writable for Phy0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY0C to value 0xe0"]
impl crate::Resettable for Phy0cSpec {
    const RESET_VALUE: u32 = 0xe0;
}
