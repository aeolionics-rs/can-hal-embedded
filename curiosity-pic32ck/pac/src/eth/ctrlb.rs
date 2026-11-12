#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `TSUCLKREQ` reader - TSU Clock Request"]
pub type TsuclkreqR = crate::BitReader;
#[doc = "Field `TSUCLKREQ` writer - TSU Clock Request"]
pub type TsuclkreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUMS` reader - Timer Adjust"]
pub type TsumsR = crate::BitReader;
#[doc = "Field `TSUMS` writer - Timer Adjust"]
pub type TsumsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUINC` reader - Timer Adjust Mode"]
pub type TsuincR = crate::FieldReader;
#[doc = "Field `TSUINC` writer - Timer Adjust Mode"]
pub type TsuincW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 2 - TSU Clock Request"]
    #[inline(always)]
    pub fn tsuclkreq(&self) -> TsuclkreqR {
        TsuclkreqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer Adjust"]
    #[inline(always)]
    pub fn tsums(&self) -> TsumsR {
        TsumsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Timer Adjust Mode"]
    #[inline(always)]
    pub fn tsuinc(&self) -> TsuincR {
        TsuincR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - TSU Clock Request"]
    #[inline(always)]
    pub fn tsuclkreq(&mut self) -> TsuclkreqW<'_, CtrlbSpec> {
        TsuclkreqW::new(self, 2)
    }
    #[doc = "Bit 5 - Timer Adjust"]
    #[inline(always)]
    pub fn tsums(&mut self) -> TsumsW<'_, CtrlbSpec> {
        TsumsW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Timer Adjust Mode"]
    #[inline(always)]
    pub fn tsuinc(&mut self) -> TsuincW<'_, CtrlbSpec> {
        TsuincW::new(self, 6)
    }
}
#[doc = "CTRLB Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0xc0"]
impl crate::Resettable for CtrlbSpec {
    const RESET_VALUE: u32 = 0xc0;
}
