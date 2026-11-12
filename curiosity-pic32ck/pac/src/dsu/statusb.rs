#[doc = "Register `STATUSB` reader"]
pub type R = crate::R<StatusbSpec>;
#[doc = "Field `BCCD0` reader - Boot ROM Communication Channel 0 Dirty"]
pub type Bccd0R = crate::BitReader;
#[doc = "Field `BCCD1` reader - Boot ROM Communication Channel 1 Dirty"]
pub type Bccd1R = crate::BitReader;
#[doc = "Field `DCCD0` reader - Debug Communication Channel 0 Dirty"]
pub type Dccd0R = crate::BitReader;
#[doc = "Field `DCCD1` reader - Debug Communication Channel 1 Dirty"]
pub type Dccd1R = crate::BitReader;
#[doc = "Field `DBGPRES` reader - Debugger Present"]
pub type DbgpresR = crate::BitReader;
#[doc = "Field `HPE` reader - Hot-Plugging Enable"]
pub type HpeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Boot ROM Communication Channel 0 Dirty"]
    #[inline(always)]
    pub fn bccd0(&self) -> Bccd0R {
        Bccd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Boot ROM Communication Channel 1 Dirty"]
    #[inline(always)]
    pub fn bccd1(&self) -> Bccd1R {
        Bccd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Communication Channel 0 Dirty"]
    #[inline(always)]
    pub fn dccd0(&self) -> Dccd0R {
        Dccd0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Communication Channel 1 Dirty"]
    #[inline(always)]
    pub fn dccd1(&self) -> Dccd1R {
        Dccd1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Debugger Present"]
    #[inline(always)]
    pub fn dbgpres(&self) -> DbgpresR {
        DbgpresR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Hot-Plugging Enable"]
    #[inline(always)]
    pub fn hpe(&self) -> HpeR {
        HpeR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Status B\n\nYou can [`read`](crate::Reg::read) this register and get [`statusb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusbSpec;
impl crate::RegisterSpec for StatusbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusb::R`](R) reader structure"]
impl crate::Readable for StatusbSpec {}
#[doc = "`reset()` method sets STATUSB to value 0"]
impl crate::Resettable for StatusbSpec {}
