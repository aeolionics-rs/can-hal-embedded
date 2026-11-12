#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `CMPHIT` reader - Compare Hit Enable"]
pub type CmphitR = crate::BitReader;
#[doc = "Field `CMPHIT` writer - Compare Hit Enable"]
pub type CmphitW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SOVFL` reader - Synchronizer Overflow Enable"]
pub type SovflR = crate::BitReader;
#[doc = "Field `SOVFL` writer - Synchronizer Overflow Enable"]
pub type SovflW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `CHRDYC` reader - Current Channel Ready Enable"]
pub type ChrdycR = crate::BitReader;
#[doc = "Field `CHRDYC` writer - Current Channel Ready Enable"]
pub type ChrdycW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `FLTRDY` reader - Filter Ready Enable"]
pub type FltrdyR = crate::BitReader;
#[doc = "Field `FLTRDY` writer - Filter Ready Enable"]
pub type FltrdyW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `CHNERRC` reader - Channel Overwrite Enable"]
pub type ChnerrcR = crate::BitReader;
#[doc = "Field `CHNERRC` writer - Channel Overwrite Enable"]
pub type ChnerrcW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `EOSRDY` reader - End of Scan Enable"]
pub type EosrdyR = crate::BitReader;
#[doc = "Field `EOSRDY` writer - End of Scan Enable"]
pub type EosrdyW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `CHRDY` reader - Channel Ready Enable"]
pub type ChrdyR = crate::FieldReader<u16>;
#[doc = "Field `CHRDY` writer - Channel Ready Enable"]
pub type ChrdyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 4 - Compare Hit Enable"]
    #[inline(always)]
    pub fn cmphit(&self) -> CmphitR {
        CmphitR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronizer Overflow Enable"]
    #[inline(always)]
    pub fn sovfl(&self) -> SovflR {
        SovflR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Current Channel Ready Enable"]
    #[inline(always)]
    pub fn chrdyc(&self) -> ChrdycR {
        ChrdycR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter Ready Enable"]
    #[inline(always)]
    pub fn fltrdy(&self) -> FltrdyR {
        FltrdyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel Overwrite Enable"]
    #[inline(always)]
    pub fn chnerrc(&self) -> ChnerrcR {
        ChnerrcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End of Scan Enable"]
    #[inline(always)]
    pub fn eosrdy(&self) -> EosrdyR {
        EosrdyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Channel Ready Enable"]
    #[inline(always)]
    pub fn chrdy(&self) -> ChrdyR {
        ChrdyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - Compare Hit Enable"]
    #[inline(always)]
    pub fn cmphit(&mut self) -> CmphitW<'_, IntensetSpec> {
        CmphitW::new(self, 4)
    }
    #[doc = "Bit 7 - Synchronizer Overflow Enable"]
    #[inline(always)]
    pub fn sovfl(&mut self) -> SovflW<'_, IntensetSpec> {
        SovflW::new(self, 7)
    }
    #[doc = "Bit 8 - Current Channel Ready Enable"]
    #[inline(always)]
    pub fn chrdyc(&mut self) -> ChrdycW<'_, IntensetSpec> {
        ChrdycW::new(self, 8)
    }
    #[doc = "Bit 9 - Filter Ready Enable"]
    #[inline(always)]
    pub fn fltrdy(&mut self) -> FltrdyW<'_, IntensetSpec> {
        FltrdyW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel Overwrite Enable"]
    #[inline(always)]
    pub fn chnerrc(&mut self) -> ChnerrcW<'_, IntensetSpec> {
        ChnerrcW::new(self, 10)
    }
    #[doc = "Bit 11 - End of Scan Enable"]
    #[inline(always)]
    pub fn eosrdy(&mut self) -> EosrdyW<'_, IntensetSpec> {
        EosrdyW::new(self, 11)
    }
    #[doc = "Bits 16:31 - Channel Ready Enable"]
    #[inline(always)]
    pub fn chrdy(&mut self) -> ChrdyW<'_, IntensetSpec> {
        ChrdyW::new(self, 16)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_0f90;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
