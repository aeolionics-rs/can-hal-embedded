#[doc = "Register `INTRRX` reader"]
pub type R = crate::R<IntrrxSpec>;
#[doc = "Field `EP1RX` reader - Interrupts Active For Which RX1 Endpoint"]
pub type Ep1rxR = crate::BitReader;
#[doc = "Field `EP2RX` reader - Interrupts Active For Which RX2 Endpoint"]
pub type Ep2rxR = crate::BitReader;
#[doc = "Field `EP3RX` reader - Interrupts Active For Which RX3 Endpoint"]
pub type Ep3rxR = crate::BitReader;
#[doc = "Field `EP4RX` reader - Interrupts Active For Which RX4 Endpoint"]
pub type Ep4rxR = crate::BitReader;
#[doc = "Field `EP5RX` reader - Interrupts Active For Which RX5 Endpoint"]
pub type Ep5rxR = crate::BitReader;
#[doc = "Field `EP6RX` reader - Interrupts Active For Which RX6 Endpoint"]
pub type Ep6rxR = crate::BitReader;
#[doc = "Field `EP7RX` reader - Interrupts Active For Which RX7 Endpoint"]
pub type Ep7rxR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Interrupts Active For Which RX1 Endpoint"]
    #[inline(always)]
    pub fn ep1rx(&self) -> Ep1rxR {
        Ep1rxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupts Active For Which RX2 Endpoint"]
    #[inline(always)]
    pub fn ep2rx(&self) -> Ep2rxR {
        Ep2rxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupts Active For Which RX3 Endpoint"]
    #[inline(always)]
    pub fn ep3rx(&self) -> Ep3rxR {
        Ep3rxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupts Active For Which RX4 Endpoint"]
    #[inline(always)]
    pub fn ep4rx(&self) -> Ep4rxR {
        Ep4rxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupts Active For Which RX5 Endpoint"]
    #[inline(always)]
    pub fn ep5rx(&self) -> Ep5rxR {
        Ep5rxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupts Active For Which RX6 Endpoint"]
    #[inline(always)]
    pub fn ep6rx(&self) -> Ep6rxR {
        Ep6rxR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupts Active For Which RX7 Endpoint"]
    #[inline(always)]
    pub fn ep7rx(&self) -> Ep7rxR {
        Ep7rxR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Read Only Register Which Indicates Which Interrupts Are Currently Active For RX EP1-15\n\nYou can [`read`](crate::Reg::read) this register and get [`intrrx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct IntrrxSpec;
impl crate::RegisterSpec for IntrrxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intrrx::R`](R) reader structure"]
impl crate::Readable for IntrrxSpec {}
#[doc = "`reset()` method sets INTRRX to value 0"]
impl crate::Resettable for IntrrxSpec {}
