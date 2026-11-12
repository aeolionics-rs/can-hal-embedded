#[doc = "Register `RXHUBPORT` reader"]
pub type R = crate::R<RxhubportSpec>;
#[doc = "Register `RXHUBPORT` writer"]
pub type W = crate::W<RxhubportSpec>;
#[doc = "Field `HUBPORT` reader - Hub Port"]
pub type HubportR = crate::FieldReader;
#[doc = "Field `HUBPORT` writer - Hub Port"]
pub type HubportW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    pub fn hubport(&self) -> HubportR {
        HubportR::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    pub fn hubport(&mut self) -> HubportW<'_, RxhubportSpec> {
        HubportW::new(self, 0)
    }
}
#[doc = "Relevant In Host Mode Only. HUB Address\n\nYou can [`read`](crate::Reg::read) this register and get [`rxhubport::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxhubport::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxhubportSpec;
impl crate::RegisterSpec for RxhubportSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxhubport::R`](R) reader structure"]
impl crate::Readable for RxhubportSpec {}
#[doc = "`write(|w| ..)` method takes [`rxhubport::W`](W) writer structure"]
impl crate::Writable for RxhubportSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXHUBPORT to value 0"]
impl crate::Resettable for RxhubportSpec {}
