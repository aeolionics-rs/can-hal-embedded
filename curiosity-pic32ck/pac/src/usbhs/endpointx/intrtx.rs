#[doc = "Register `INTRTX` reader"]
pub type R = crate::R<IntrtxSpec>;
#[doc = "Field `EP0TX` reader - Interrupts Active For TX0 Endpoint"]
pub type Ep0txR = crate::BitReader;
#[doc = "Field `EP1TX` reader - Interrupts Active For TX1 Endpoint"]
pub type Ep1txR = crate::BitReader;
#[doc = "Field `EP2TX` reader - Interrupts Active For TX2 Endpoint"]
pub type Ep2txR = crate::BitReader;
#[doc = "Field `EP3TX` reader - Interrupts Active For TX3 Endpoint"]
pub type Ep3txR = crate::BitReader;
#[doc = "Field `EP4TX` reader - Interrupts Active For TX4 Endpoint"]
pub type Ep4txR = crate::BitReader;
#[doc = "Field `EP5TX` reader - Interrupts Active For TX5 Endpoint"]
pub type Ep5txR = crate::BitReader;
#[doc = "Field `EP6TX` reader - Interrupts Active For TX6 Endpoint"]
pub type Ep6txR = crate::BitReader;
#[doc = "Field `EP7TX` reader - Interrupts Active For TX7 Endpoint"]
pub type Ep7txR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupts Active For TX0 Endpoint"]
    #[inline(always)]
    pub fn ep0tx(&self) -> Ep0txR {
        Ep0txR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupts Active For TX1 Endpoint"]
    #[inline(always)]
    pub fn ep1tx(&self) -> Ep1txR {
        Ep1txR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupts Active For TX2 Endpoint"]
    #[inline(always)]
    pub fn ep2tx(&self) -> Ep2txR {
        Ep2txR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupts Active For TX3 Endpoint"]
    #[inline(always)]
    pub fn ep3tx(&self) -> Ep3txR {
        Ep3txR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupts Active For TX4 Endpoint"]
    #[inline(always)]
    pub fn ep4tx(&self) -> Ep4txR {
        Ep4txR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupts Active For TX5 Endpoint"]
    #[inline(always)]
    pub fn ep5tx(&self) -> Ep5txR {
        Ep5txR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupts Active For TX6 Endpoint"]
    #[inline(always)]
    pub fn ep6tx(&self) -> Ep6txR {
        Ep6txR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupts Active For TX7 Endpoint"]
    #[inline(always)]
    pub fn ep7tx(&self) -> Ep7txR {
        Ep7txR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Read Only Register Which Indicates Which Interrupts Are Currently Active For EP0 And TX EP1-15\n\nYou can [`read`](crate::Reg::read) this register and get [`intrtx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct IntrtxSpec;
impl crate::RegisterSpec for IntrtxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intrtx::R`](R) reader structure"]
impl crate::Readable for IntrtxSpec {}
#[doc = "`reset()` method sets INTRTX to value 0"]
impl crate::Resettable for IntrtxSpec {}
