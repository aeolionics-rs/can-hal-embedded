#[doc = "Register `CRP` reader"]
pub type R = crate::R<CrpSpec>;
#[doc = "Register `CRP` writer"]
pub type W = crate::W<CrpSpec>;
#[doc = "Field `BC1RP` reader - Boot Configuration Page 1 Read Protect Bit"]
pub type Bc1rpR = crate::BitReader;
#[doc = "Field `BC1RP` writer - Boot Configuration Page 1 Read Protect Bit"]
pub type Bc1rpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BC2RP` reader - Boot Configuration Page 2 Read Protect Bit"]
pub type Bc2rpR = crate::BitReader;
#[doc = "Field `BC2RP` writer - Boot Configuration Page 2 Read Protect Bit"]
pub type Bc2rpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BC1RPLOCK` reader - Boot Configuration Page 1 RP Lock Bit"]
pub type Bc1rplockR = crate::BitReader;
#[doc = "Field `BC1RPLOCK` writer - Boot Configuration Page 1 RP Lock Bit"]
pub type Bc1rplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BC2RPLOCK` reader - Boot Configuration Page 2 RP Lock Bit"]
pub type Bc2rplockR = crate::BitReader;
#[doc = "Field `BC2RPLOCK` writer - Boot Configuration Page 2 RP Lock Bit"]
pub type Bc2rplockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Boot Configuration Page 1 Read Protect Bit"]
    #[inline(always)]
    pub fn bc1rp(&self) -> Bc1rpR {
        Bc1rpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - Boot Configuration Page 2 Read Protect Bit"]
    #[inline(always)]
    pub fn bc2rp(&self) -> Bc2rpR {
        Bc2rpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 18 - Boot Configuration Page 1 RP Lock Bit"]
    #[inline(always)]
    pub fn bc1rplock(&self) -> Bc1rplockR {
        Bc1rplockR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 26 - Boot Configuration Page 2 RP Lock Bit"]
    #[inline(always)]
    pub fn bc2rplock(&self) -> Bc2rplockR {
        Bc2rplockR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Boot Configuration Page 1 Read Protect Bit"]
    #[inline(always)]
    pub fn bc1rp(&mut self) -> Bc1rpW<'_, CrpSpec> {
        Bc1rpW::new(self, 2)
    }
    #[doc = "Bit 10 - Boot Configuration Page 2 Read Protect Bit"]
    #[inline(always)]
    pub fn bc2rp(&mut self) -> Bc2rpW<'_, CrpSpec> {
        Bc2rpW::new(self, 10)
    }
    #[doc = "Bit 18 - Boot Configuration Page 1 RP Lock Bit"]
    #[inline(always)]
    pub fn bc1rplock(&mut self) -> Bc1rplockW<'_, CrpSpec> {
        Bc1rplockW::new(self, 18)
    }
    #[doc = "Bit 26 - Boot Configuration Page 2 RP Lock Bit"]
    #[inline(always)]
    pub fn bc2rplock(&mut self) -> Bc2rplockW<'_, CrpSpec> {
        Bc2rplockW::new(self, 26)
    }
}
#[doc = "CFM Page Read Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrpSpec;
impl crate::RegisterSpec for CrpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crp::R`](R) reader structure"]
impl crate::Readable for CrpSpec {}
#[doc = "`write(|w| ..)` method takes [`crp::W`](W) writer structure"]
impl crate::Writable for CrpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRP to value 0"]
impl crate::Resettable for CrpSpec {}
