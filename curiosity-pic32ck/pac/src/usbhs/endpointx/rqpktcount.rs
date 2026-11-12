#[doc = "Register `RQPKTCOUNT[%s]` reader"]
pub type R = crate::R<RqpktcountSpec>;
#[doc = "Register `RQPKTCOUNT[%s]` writer"]
pub type W = crate::W<RqpktcountSpec>;
#[doc = "Field `RQPKTCOUNT` reader - sets the number of packets of size maxp that are to be transferred in a block transfer. only used in host mode when autoreq is set. has no effect in peripheral mode or when autoreq is not set."]
pub type RqpktcountR = crate::FieldReader<u16>;
#[doc = "Field `RQPKTCOUNT` writer - sets the number of packets of size maxp that are to be transferred in a block transfer. only used in host mode when autoreq is set. has no effect in peripheral mode or when autoreq is not set."]
pub type RqpktcountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sets the number of packets of size maxp that are to be transferred in a block transfer. only used in host mode when autoreq is set. has no effect in peripheral mode or when autoreq is not set."]
    #[inline(always)]
    pub fn rqpktcount(&self) -> RqpktcountR {
        RqpktcountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sets the number of packets of size maxp that are to be transferred in a block transfer. only used in host mode when autoreq is set. has no effect in peripheral mode or when autoreq is not set."]
    #[inline(always)]
    pub fn rqpktcount(&mut self) -> RqpktcountW<'_, RqpktcountSpec> {
        RqpktcountW::new(self, 0)
    }
}
#[doc = "Host Mode Only: Used To Specify Number Of Packets That Are To Be Transaferred In A Block Transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`rqpktcount::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqpktcount::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RqpktcountSpec;
impl crate::RegisterSpec for RqpktcountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rqpktcount::R`](R) reader structure"]
impl crate::Readable for RqpktcountSpec {}
#[doc = "`write(|w| ..)` method takes [`rqpktcount::W`](W) writer structure"]
impl crate::Writable for RqpktcountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RQPKTCOUNT[%s] to value 0"]
impl crate::Resettable for RqpktcountSpec {}
