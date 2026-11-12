#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `DONE` reader - Measurement Done Interrupt Disable"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - Measurement Done Interrupt Disable"]
pub type DoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WINMON` reader - Window Monitor Interrupt Disable"]
pub type WinmonR = crate::BitReader;
#[doc = "Field `WINMON` writer - Window Monitor Interrupt Disable"]
pub type WinmonW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Measurement Done Interrupt Disable"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Window Monitor Interrupt Disable"]
    #[inline(always)]
    pub fn winmon(&self) -> WinmonR {
        WinmonR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Measurement Done Interrupt Disable"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IntenclrSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Window Monitor Interrupt Disable"]
    #[inline(always)]
    pub fn winmon(&mut self) -> WinmonW<'_, IntenclrSpec> {
        WinmonW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x03;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
