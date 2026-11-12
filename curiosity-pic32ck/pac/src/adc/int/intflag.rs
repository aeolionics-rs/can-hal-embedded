#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `CMPINTID` reader - Compare Channel ID"]
pub type CmpintidR = crate::FieldReader;
#[doc = "Field `CMPINTID` writer - Compare Channel ID"]
pub type CmpintidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CMPHIT` reader - Compare Hit"]
pub type CmphitR = crate::BitReader;
#[doc = "Field `CMPHIT` writer - Compare Hit"]
pub type CmphitW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SOVFL` reader - Synchronizer Overflow"]
pub type SovflR = crate::BitReader;
#[doc = "Field `SOVFL` writer - Synchronizer Overflow"]
pub type SovflW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CHRDYC` reader - Current Channel Ready"]
pub type ChrdycR = crate::BitReader;
#[doc = "Field `CHRDYC` writer - Current Channel Ready"]
pub type ChrdycW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FLTRDY` reader - Filter Ready"]
pub type FltrdyR = crate::BitReader;
#[doc = "Field `FLTRDY` writer - Filter Ready"]
pub type FltrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CHNERRC` reader - Channel Overwrite Error"]
pub type ChnerrcR = crate::BitReader;
#[doc = "Field `CHNERRC` writer - Channel Overwrite Error"]
pub type ChnerrcW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EOSRDY` reader - End of Scan Ready"]
pub type EosrdyR = crate::BitReader;
#[doc = "Field `EOSRDY` writer - End of Scan Ready"]
pub type EosrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CRDYID` reader - Channel Ready ID"]
pub type CrdyidR = crate::FieldReader;
#[doc = "Field `CRDYID` writer - Channel Ready ID"]
pub type CrdyidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHRDY` reader - Channel Ready"]
pub type ChrdyR = crate::FieldReader<u16>;
#[doc = "Field `CHRDY` writer - Channel Ready"]
pub type ChrdyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Compare Channel ID"]
    #[inline(always)]
    pub fn cmpintid(&self) -> CmpintidR {
        CmpintidR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Compare Hit"]
    #[inline(always)]
    pub fn cmphit(&self) -> CmphitR {
        CmphitR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronizer Overflow"]
    #[inline(always)]
    pub fn sovfl(&self) -> SovflR {
        SovflR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Current Channel Ready"]
    #[inline(always)]
    pub fn chrdyc(&self) -> ChrdycR {
        ChrdycR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter Ready"]
    #[inline(always)]
    pub fn fltrdy(&self) -> FltrdyR {
        FltrdyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel Overwrite Error"]
    #[inline(always)]
    pub fn chnerrc(&self) -> ChnerrcR {
        ChnerrcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End of Scan Ready"]
    #[inline(always)]
    pub fn eosrdy(&self) -> EosrdyR {
        EosrdyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Channel Ready ID"]
    #[inline(always)]
    pub fn crdyid(&self) -> CrdyidR {
        CrdyidR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Channel Ready"]
    #[inline(always)]
    pub fn chrdy(&self) -> ChrdyR {
        ChrdyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Compare Channel ID"]
    #[inline(always)]
    pub fn cmpintid(&mut self) -> CmpintidW<'_, IntflagSpec> {
        CmpintidW::new(self, 0)
    }
    #[doc = "Bit 4 - Compare Hit"]
    #[inline(always)]
    pub fn cmphit(&mut self) -> CmphitW<'_, IntflagSpec> {
        CmphitW::new(self, 4)
    }
    #[doc = "Bit 7 - Synchronizer Overflow"]
    #[inline(always)]
    pub fn sovfl(&mut self) -> SovflW<'_, IntflagSpec> {
        SovflW::new(self, 7)
    }
    #[doc = "Bit 8 - Current Channel Ready"]
    #[inline(always)]
    pub fn chrdyc(&mut self) -> ChrdycW<'_, IntflagSpec> {
        ChrdycW::new(self, 8)
    }
    #[doc = "Bit 9 - Filter Ready"]
    #[inline(always)]
    pub fn fltrdy(&mut self) -> FltrdyW<'_, IntflagSpec> {
        FltrdyW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel Overwrite Error"]
    #[inline(always)]
    pub fn chnerrc(&mut self) -> ChnerrcW<'_, IntflagSpec> {
        ChnerrcW::new(self, 10)
    }
    #[doc = "Bit 11 - End of Scan Ready"]
    #[inline(always)]
    pub fn eosrdy(&mut self) -> EosrdyW<'_, IntflagSpec> {
        EosrdyW::new(self, 11)
    }
    #[doc = "Bits 12:15 - Channel Ready ID"]
    #[inline(always)]
    pub fn crdyid(&mut self) -> CrdyidW<'_, IntflagSpec> {
        CrdyidW::new(self, 12)
    }
    #[doc = "Bits 16:31 - Channel Ready"]
    #[inline(always)]
    pub fn chrdy(&mut self) -> ChrdyW<'_, IntflagSpec> {
        ChrdyW::new(self, 16)
    }
}
#[doc = "Interrupt Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ff9f;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
