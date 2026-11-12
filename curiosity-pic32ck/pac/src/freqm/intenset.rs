#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `DONE` reader - Measurement Done Interrupt Enable"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - Measurement Done Interrupt Enable"]
pub type DoneW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `WINMON` reader - Window Monitor Interrupt Enable"]
pub type WinmonR = crate::BitReader;
#[doc = "Field `WINMON` writer - Window Monitor Interrupt Enable"]
pub type WinmonW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - Measurement Done Interrupt Enable"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Window Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn winmon(&self) -> WinmonR {
        WinmonR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Measurement Done Interrupt Enable"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IntensetSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Window Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn winmon(&mut self) -> WinmonW<'_, IntensetSpec> {
        WinmonW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x03;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
