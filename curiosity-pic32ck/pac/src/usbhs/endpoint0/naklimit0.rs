#[doc = "Register `NAKLIMIT0` reader"]
pub type R = crate::R<Naklimit0Spec>;
#[doc = "Register `NAKLIMIT0` writer"]
pub type W = crate::W<Naklimit0Spec>;
#[doc = "Field `EP0NAKLIMIT` reader - Number Of Frames/Microframes After Which Endpoint 0 Should Timeout"]
pub type Ep0naklimitR = crate::FieldReader;
#[doc = "Field `EP0NAKLIMIT` writer - Number Of Frames/Microframes After Which Endpoint 0 Should Timeout"]
pub type Ep0naklimitW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Number Of Frames/Microframes After Which Endpoint 0 Should Timeout"]
    #[inline(always)]
    pub fn ep0naklimit(&self) -> Ep0naklimitR {
        Ep0naklimitR::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number Of Frames/Microframes After Which Endpoint 0 Should Timeout"]
    #[inline(always)]
    pub fn ep0naklimit(&mut self) -> Ep0naklimitW<'_, Naklimit0Spec> {
        Ep0naklimitW::new(self, 0)
    }
}
#[doc = "ENDPOINT0 Host Mode Only: Sets The NAK Response Timeout On Endpoint 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`naklimit0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`naklimit0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Naklimit0Spec;
impl crate::RegisterSpec for Naklimit0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`naklimit0::R`](R) reader structure"]
impl crate::Readable for Naklimit0Spec {}
#[doc = "`write(|w| ..)` method takes [`naklimit0::W`](W) writer structure"]
impl crate::Writable for Naklimit0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NAKLIMIT0 to value 0"]
impl crate::Resettable for Naklimit0Spec {}
