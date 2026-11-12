#[doc = "Register `LPMCNTRL_HOST_MODE` reader"]
pub type R = crate::R<LpmcntrlHostModeSpec>;
#[doc = "Register `LPMCNTRL_HOST_MODE` writer"]
pub type W = crate::W<LpmcntrlHostModeSpec>;
#[doc = "Field `LPMXMT` reader - software should set this bit to transmit an lpm transaction."]
pub type LpmxmtR = crate::BitReader;
#[doc = "Field `LPMXMT` writer - software should set this bit to transmit an lpm transaction."]
pub type LpmxmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMRES` reader - this bit is used by software to initiate a resume from the l1 state"]
pub type LpmresR = crate::BitReader;
#[doc = "Field `LPMRES` writer - this bit is used by software to initiate a resume from the l1 state"]
pub type LpmresW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - software should set this bit to transmit an lpm transaction."]
    #[inline(always)]
    pub fn lpmxmt(&self) -> LpmxmtR {
        LpmxmtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this bit is used by software to initiate a resume from the l1 state"]
    #[inline(always)]
    pub fn lpmres(&self) -> LpmresR {
        LpmresR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - software should set this bit to transmit an lpm transaction."]
    #[inline(always)]
    pub fn lpmxmt(&mut self) -> LpmxmtW<'_, LpmcntrlHostModeSpec> {
        LpmxmtW::new(self, 0)
    }
    #[doc = "Bit 1 - this bit is used by software to initiate a resume from the l1 state"]
    #[inline(always)]
    pub fn lpmres(&mut self) -> LpmresW<'_, LpmcntrlHostModeSpec> {
        LpmresW::new(self, 1)
    }
}
#[doc = "LPM Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmcntrl_host_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcntrl_host_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmcntrlHostModeSpec;
impl crate::RegisterSpec for LpmcntrlHostModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lpmcntrl_host_mode::R`](R) reader structure"]
impl crate::Readable for LpmcntrlHostModeSpec {}
#[doc = "`write(|w| ..)` method takes [`lpmcntrl_host_mode::W`](W) writer structure"]
impl crate::Writable for LpmcntrlHostModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPMCNTRL_HOST_MODE to value 0"]
impl crate::Resettable for LpmcntrlHostModeSpec {}
