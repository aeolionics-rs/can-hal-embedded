#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `SDALCPU0` reader - Set DAL for CPU n"]
pub type Sdalcpu0R = crate::FieldReader;
#[doc = "Field `SDALCPU0` writer - Set DAL for CPU n"]
pub type Sdalcpu0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDALCPU1` reader - Set DAL for CPU n"]
pub type Sdalcpu1R = crate::FieldReader;
#[doc = "Field `SDALCPU1` writer - Set DAL for CPU n"]
pub type Sdalcpu1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Set DAL for CPU n"]
    #[inline(always)]
    pub fn sdalcpu0(&self) -> Sdalcpu0R {
        Sdalcpu0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Set DAL for CPU n"]
    #[inline(always)]
    pub fn sdalcpu1(&self) -> Sdalcpu1R {
        Sdalcpu1R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set DAL for CPU n"]
    #[inline(always)]
    pub fn sdalcpu0(&mut self) -> Sdalcpu0W<'_, CtrlbSpec> {
        Sdalcpu0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Set DAL for CPU n"]
    #[inline(always)]
    pub fn sdalcpu1(&mut self) -> Sdalcpu1W<'_, CtrlbSpec> {
        Sdalcpu1W::new(self, 2)
    }
}
#[doc = "NVM Control B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRLB to value 0xff"]
impl crate::Resettable for CtrlbSpec {
    const RESET_VALUE: u32 = 0xff;
}
