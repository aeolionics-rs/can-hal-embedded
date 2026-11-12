#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `CMPHIT` reader - Compare Hit Disable"]
pub type CmphitR = crate::BitReader;
#[doc = "Field `CMPHIT` writer - Compare Hit Disable"]
pub type CmphitW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SOVFL` reader - Synchonizer Overflow Disable"]
pub type SovflR = crate::BitReader;
#[doc = "Field `SOVFL` writer - Synchonizer Overflow Disable"]
pub type SovflW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CHRDYC` reader - Core Current Channel Disable"]
pub type ChrdycR = crate::BitReader;
#[doc = "Field `CHRDYC` writer - Core Current Channel Disable"]
pub type ChrdycW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FLTRDY` reader - Filter Ready Disable"]
pub type FltrdyR = crate::BitReader;
#[doc = "Field `FLTRDY` writer - Filter Ready Disable"]
pub type FltrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CHNERRC` reader - Channel Overwrite Error Disable"]
pub type ChnerrcR = crate::BitReader;
#[doc = "Field `CHNERRC` writer - Channel Overwrite Error Disable"]
pub type ChnerrcW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EOSRDY` reader - Endo of Scan Disable"]
pub type EosrdyR = crate::BitReader;
#[doc = "Field `EOSRDY` writer - Endo of Scan Disable"]
pub type EosrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CHRDY` reader - Channel Ready Disable"]
pub type ChrdyR = crate::FieldReader<u16>;
#[doc = "Field `CHRDY` writer - Channel Ready Disable"]
pub type ChrdyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 4 - Compare Hit Disable"]
    #[inline(always)]
    pub fn cmphit(&self) -> CmphitR {
        CmphitR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchonizer Overflow Disable"]
    #[inline(always)]
    pub fn sovfl(&self) -> SovflR {
        SovflR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Core Current Channel Disable"]
    #[inline(always)]
    pub fn chrdyc(&self) -> ChrdycR {
        ChrdycR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter Ready Disable"]
    #[inline(always)]
    pub fn fltrdy(&self) -> FltrdyR {
        FltrdyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel Overwrite Error Disable"]
    #[inline(always)]
    pub fn chnerrc(&self) -> ChnerrcR {
        ChnerrcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endo of Scan Disable"]
    #[inline(always)]
    pub fn eosrdy(&self) -> EosrdyR {
        EosrdyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:27 - Channel Ready Disable"]
    #[inline(always)]
    pub fn chrdy(&self) -> ChrdyR {
        ChrdyR::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - Compare Hit Disable"]
    #[inline(always)]
    pub fn cmphit(&mut self) -> CmphitW<'_, IntenclrSpec> {
        CmphitW::new(self, 4)
    }
    #[doc = "Bit 7 - Synchonizer Overflow Disable"]
    #[inline(always)]
    pub fn sovfl(&mut self) -> SovflW<'_, IntenclrSpec> {
        SovflW::new(self, 7)
    }
    #[doc = "Bit 8 - Core Current Channel Disable"]
    #[inline(always)]
    pub fn chrdyc(&mut self) -> ChrdycW<'_, IntenclrSpec> {
        ChrdycW::new(self, 8)
    }
    #[doc = "Bit 9 - Filter Ready Disable"]
    #[inline(always)]
    pub fn fltrdy(&mut self) -> FltrdyW<'_, IntenclrSpec> {
        FltrdyW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel Overwrite Error Disable"]
    #[inline(always)]
    pub fn chnerrc(&mut self) -> ChnerrcW<'_, IntenclrSpec> {
        ChnerrcW::new(self, 10)
    }
    #[doc = "Bit 11 - Endo of Scan Disable"]
    #[inline(always)]
    pub fn eosrdy(&mut self) -> EosrdyW<'_, IntenclrSpec> {
        EosrdyW::new(self, 11)
    }
    #[doc = "Bits 12:27 - Channel Ready Disable"]
    #[inline(always)]
    pub fn chrdy(&mut self) -> ChrdyW<'_, IntenclrSpec> {
        ChrdyW::new(self, 12)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0fff_ff90;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
