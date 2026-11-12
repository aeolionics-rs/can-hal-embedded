#[doc = "Register `BDPOLLCON` reader"]
pub type R = crate::R<BdpollconSpec>;
#[doc = "Register `BDPOLLCON` writer"]
pub type W = crate::W<BdpollconSpec>;
#[doc = "Field `POLLCON` reader - Number of cycles that BDP block would wait before re-fetching the Descriptor control word if the previous descriptor fetched was disabled"]
pub type PollconR = crate::FieldReader<u16>;
#[doc = "Field `POLLCON` writer - Number of cycles that BDP block would wait before re-fetching the Descriptor control word if the previous descriptor fetched was disabled"]
pub type PollconW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of cycles that BDP block would wait before re-fetching the Descriptor control word if the previous descriptor fetched was disabled"]
    #[inline(always)]
    pub fn pollcon(&self) -> PollconR {
        PollconR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of cycles that BDP block would wait before re-fetching the Descriptor control word if the previous descriptor fetched was disabled"]
    #[inline(always)]
    pub fn pollcon(&mut self) -> PollconW<'_, BdpollconSpec> {
        PollconW::new(self, 0)
    }
}
#[doc = "BD_POLL_CTRL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdpollcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdpollcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdpollconSpec;
impl crate::RegisterSpec for BdpollconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdpollcon::R`](R) reader structure"]
impl crate::Readable for BdpollconSpec {}
#[doc = "`write(|w| ..)` method takes [`bdpollcon::W`](W) writer structure"]
impl crate::Writable for BdpollconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BDPOLLCON to value 0"]
impl crate::Resettable for BdpollconSpec {}
