#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `BUFOVF` reader - Buffer Overflow"]
pub type BufovfR = crate::BitReader;
#[doc = "Field `BUFOVF` writer - Buffer Overflow"]
pub type BufovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUR` reader - Frame Transmit Underrun"]
pub type TurR = crate::BitReader;
#[doc = "Field `TUR` writer - Frame Transmit Underrun"]
pub type TurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LENERR` reader - Transaction Length Error"]
pub type LenerrR = crate::BitReader;
#[doc = "Field `LENERR` writer - Transaction Length Error"]
pub type LenerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Buffer Overflow"]
    #[inline(always)]
    pub fn bufovf(&self) -> BufovfR {
        BufovfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame Transmit Underrun"]
    #[inline(always)]
    pub fn tur(&self) -> TurR {
        TurR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Transaction Length Error"]
    #[inline(always)]
    pub fn lenerr(&self) -> LenerrR {
        LenerrR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Buffer Overflow"]
    #[inline(always)]
    pub fn bufovf(&mut self) -> BufovfW<'_, StatusSpec> {
        BufovfW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame Transmit Underrun"]
    #[inline(always)]
    pub fn tur(&mut self) -> TurW<'_, StatusSpec> {
        TurW::new(self, 3)
    }
    #[doc = "Bit 11 - Transaction Length Error"]
    #[inline(always)]
    pub fn lenerr(&mut self) -> LenerrW<'_, StatusSpec> {
        LenerrW::new(self, 11)
    }
}
#[doc = "SPIS Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
