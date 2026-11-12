#[doc = "Register `SMC_SETUP` reader"]
pub type R = crate::R<SmcSetupSpec>;
#[doc = "Register `SMC_SETUP` writer"]
pub type W = crate::W<SmcSetupSpec>;
#[doc = "Field `NWE_SETUP` reader - NWE Setup Length"]
pub type NweSetupR = crate::FieldReader;
#[doc = "Field `NWE_SETUP` writer - NWE Setup Length"]
pub type NweSetupW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NCS_WR_SETUP` reader - NCS Setup Length in WRITE Access"]
pub type NcsWrSetupR = crate::FieldReader;
#[doc = "Field `NCS_WR_SETUP` writer - NCS Setup Length in WRITE Access"]
pub type NcsWrSetupW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NRD_SETUP` reader - NRD Setup Length"]
pub type NrdSetupR = crate::FieldReader;
#[doc = "Field `NRD_SETUP` writer - NRD Setup Length"]
pub type NrdSetupW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NCS_RD_SETUP` reader - NCS Setup Length in READ Access"]
pub type NcsRdSetupR = crate::FieldReader;
#[doc = "Field `NCS_RD_SETUP` writer - NCS Setup Length in READ Access"]
pub type NcsRdSetupW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - NWE Setup Length"]
    #[inline(always)]
    pub fn nwe_setup(&self) -> NweSetupR {
        NweSetupR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - NCS Setup Length in WRITE Access"]
    #[inline(always)]
    pub fn ncs_wr_setup(&self) -> NcsWrSetupR {
        NcsWrSetupR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - NRD Setup Length"]
    #[inline(always)]
    pub fn nrd_setup(&self) -> NrdSetupR {
        NrdSetupR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - NCS Setup Length in READ Access"]
    #[inline(always)]
    pub fn ncs_rd_setup(&self) -> NcsRdSetupR {
        NcsRdSetupR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - NWE Setup Length"]
    #[inline(always)]
    pub fn nwe_setup(&mut self) -> NweSetupW<'_, SmcSetupSpec> {
        NweSetupW::new(self, 0)
    }
    #[doc = "Bits 8:13 - NCS Setup Length in WRITE Access"]
    #[inline(always)]
    pub fn ncs_wr_setup(&mut self) -> NcsWrSetupW<'_, SmcSetupSpec> {
        NcsWrSetupW::new(self, 8)
    }
    #[doc = "Bits 16:21 - NRD Setup Length"]
    #[inline(always)]
    pub fn nrd_setup(&mut self) -> NrdSetupW<'_, SmcSetupSpec> {
        NrdSetupW::new(self, 16)
    }
    #[doc = "Bits 24:29 - NCS Setup Length in READ Access"]
    #[inline(always)]
    pub fn ncs_rd_setup(&mut self) -> NcsRdSetupW<'_, SmcSetupSpec> {
        NcsRdSetupW::new(self, 24)
    }
}
#[doc = "SMC Setup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smc_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcSetupSpec;
impl crate::RegisterSpec for SmcSetupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smc_setup::R`](R) reader structure"]
impl crate::Readable for SmcSetupSpec {}
#[doc = "`write(|w| ..)` method takes [`smc_setup::W`](W) writer structure"]
impl crate::Writable for SmcSetupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMC_SETUP to value 0x0101_0101"]
impl crate::Resettable for SmcSetupSpec {
    const RESET_VALUE: u32 = 0x0101_0101;
}
