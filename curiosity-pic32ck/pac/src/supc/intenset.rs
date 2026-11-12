#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `LVDET` reader - Low Voltage Detector Interrupt Enable"]
pub type LvdetR = crate::BitReader;
#[doc = "Field `LVDET` writer - Low Voltage Detector Interrupt Enable"]
pub type LvdetW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `LVDRDY` reader - Low Voltage Detector Ready Interrupt Enable"]
pub type LvdrdyR = crate::BitReader;
#[doc = "Field `LVDRDY` writer - Low Voltage Detector Ready Interrupt Enable"]
pub type LvdrdyW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `BORVDDUSB` reader - BORVDDUSB Interrupt Enable"]
pub type BorvddusbR = crate::BitReader;
#[doc = "Field `BORVDDUSB` writer - BORVDDUSB Interrupt Enable"]
pub type BorvddusbW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ADDVREGRDY0` reader - Additional Regulator ready 0 Interrupt Enable"]
pub type Addvregrdy0R = crate::BitReader;
#[doc = "Field `ADDVREGRDY0` writer - Additional Regulator ready 0 Interrupt Enable"]
pub type Addvregrdy0W<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low Voltage Detector Interrupt Enable"]
    #[inline(always)]
    pub fn lvdet(&self) -> LvdetR {
        LvdetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Voltage Detector Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lvdrdy(&self) -> LvdrdyR {
        LvdrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - BORVDDUSB Interrupt Enable"]
    #[inline(always)]
    pub fn borvddusb(&self) -> BorvddusbR {
        BorvddusbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Additional Regulator ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn addvregrdy0(&self) -> Addvregrdy0R {
        Addvregrdy0R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Voltage Detector Interrupt Enable"]
    #[inline(always)]
    pub fn lvdet(&mut self) -> LvdetW<'_, IntensetSpec> {
        LvdetW::new(self, 0)
    }
    #[doc = "Bit 1 - Low Voltage Detector Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lvdrdy(&mut self) -> LvdrdyW<'_, IntensetSpec> {
        LvdrdyW::new(self, 1)
    }
    #[doc = "Bit 5 - BORVDDUSB Interrupt Enable"]
    #[inline(always)]
    pub fn borvddusb(&mut self) -> BorvddusbW<'_, IntensetSpec> {
        BorvddusbW::new(self, 5)
    }
    #[doc = "Bit 8 - Additional Regulator ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn addvregrdy0(&mut self) -> Addvregrdy0W<'_, IntensetSpec> {
        Addvregrdy0W::new(self, 8)
    }
}
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0123;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
