#[doc = "Register `DMACNTL` reader"]
pub type R = crate::R<DmacntlSpec>;
#[doc = "Register `DMACNTL` writer"]
pub type W = crate::W<DmacntlSpec>;
#[doc = "Field `DMAEN` reader - dma transfer enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - dma transfer enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "transfer direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmadirselect {
    #[doc = "0: DMA Write (RX Endpoint)"]
    Write = 0,
    #[doc = "1: DMA Read (TX Endpoint)"]
    Read = 1,
}
impl From<Dmadirselect> for bool {
    #[inline(always)]
    fn from(variant: Dmadirselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMADIR` reader - transfer direction"]
pub type DmadirR = crate::BitReader<Dmadirselect>;
impl DmadirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmadirselect {
        match self.bits {
            false => Dmadirselect::Write,
            true => Dmadirselect::Read,
        }
    }
    #[doc = "DMA Write (RX Endpoint)"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Dmadirselect::Write
    }
    #[doc = "DMA Read (TX Endpoint)"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Dmadirselect::Read
    }
}
#[doc = "Field `DMADIR` writer - transfer direction"]
pub type DmadirW<'a, REG> = crate::BitWriter<'a, REG, Dmadirselect>;
impl<'a, REG> DmadirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA Write (RX Endpoint)"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadirselect::Write)
    }
    #[doc = "DMA Read (TX Endpoint)"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Dmadirselect::Read)
    }
}
#[doc = "transfer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmamodeselect {
    #[doc = "0: Mode 0 Transfer"]
    Mode0 = 0,
    #[doc = "1: Mode1 Transfer"]
    Mode1 = 1,
}
impl From<Dmamodeselect> for bool {
    #[inline(always)]
    fn from(variant: Dmamodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMODE` reader - transfer mode"]
pub type DmamodeR = crate::BitReader<Dmamodeselect>;
impl DmamodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmamodeselect {
        match self.bits {
            false => Dmamodeselect::Mode0,
            true => Dmamodeselect::Mode1,
        }
    }
    #[doc = "Mode 0 Transfer"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == Dmamodeselect::Mode0
    }
    #[doc = "Mode1 Transfer"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Dmamodeselect::Mode1
    }
}
#[doc = "Field `DMAMODE` writer - transfer mode"]
pub type DmamodeW<'a, REG> = crate::BitWriter<'a, REG, Dmamodeselect>;
impl<'a, REG> DmamodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode 0 Transfer"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamodeselect::Mode0)
    }
    #[doc = "Mode1 Transfer"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamodeselect::Mode1)
    }
}
#[doc = "Field `DMAIE` reader - interrupt enable"]
pub type DmaieR = crate::BitReader;
#[doc = "Field `DMAIE` writer - interrupt enable"]
pub type DmaieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEP` reader - endpoint number this channel is assigned to"]
pub type DmaepR = crate::FieldReader;
#[doc = "Field `DMAEP` writer - endpoint number this channel is assigned to"]
pub type DmaepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAERR` reader - bus error bit"]
pub type DmaerrR = crate::BitReader;
#[doc = "Field `DMAERR` writer - bus error bit"]
pub type DmaerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "burst mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmabrstmselect {
    #[doc = "0: Burst Mode 0"]
    Bm0 = 0,
    #[doc = "1: Burst Mode 1"]
    Bm1 = 1,
    #[doc = "2: Burst Mode 2"]
    Bm2 = 2,
    #[doc = "3: Burst Mode 3"]
    Bm3 = 3,
}
impl From<Dmabrstmselect> for u8 {
    #[inline(always)]
    fn from(variant: Dmabrstmselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmabrstmselect {
    type Ux = u8;
}
impl crate::IsEnum for Dmabrstmselect {}
#[doc = "Field `DMABRSTM` reader - burst mode"]
pub type DmabrstmR = crate::FieldReader<Dmabrstmselect>;
impl DmabrstmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmabrstmselect {
        match self.bits {
            0 => Dmabrstmselect::Bm0,
            1 => Dmabrstmselect::Bm1,
            2 => Dmabrstmselect::Bm2,
            3 => Dmabrstmselect::Bm3,
            _ => unreachable!(),
        }
    }
    #[doc = "Burst Mode 0"]
    #[inline(always)]
    pub fn is_bm0(&self) -> bool {
        *self == Dmabrstmselect::Bm0
    }
    #[doc = "Burst Mode 1"]
    #[inline(always)]
    pub fn is_bm1(&self) -> bool {
        *self == Dmabrstmselect::Bm1
    }
    #[doc = "Burst Mode 2"]
    #[inline(always)]
    pub fn is_bm2(&self) -> bool {
        *self == Dmabrstmselect::Bm2
    }
    #[doc = "Burst Mode 3"]
    #[inline(always)]
    pub fn is_bm3(&self) -> bool {
        *self == Dmabrstmselect::Bm3
    }
}
#[doc = "Field `DMABRSTM` writer - burst mode"]
pub type DmabrstmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmabrstmselect, crate::Safe>;
impl<'a, REG> DmabrstmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Burst Mode 0"]
    #[inline(always)]
    pub fn bm0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabrstmselect::Bm0)
    }
    #[doc = "Burst Mode 1"]
    #[inline(always)]
    pub fn bm1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabrstmselect::Bm1)
    }
    #[doc = "Burst Mode 2"]
    #[inline(always)]
    pub fn bm2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabrstmselect::Bm2)
    }
    #[doc = "Burst Mode 3"]
    #[inline(always)]
    pub fn bm3(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabrstmselect::Bm3)
    }
}
impl R {
    #[doc = "Bit 0 - dma transfer enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - transfer direction"]
    #[inline(always)]
    pub fn dmadir(&self) -> DmadirR {
        DmadirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - transfer mode"]
    #[inline(always)]
    pub fn dmamode(&self) -> DmamodeR {
        DmamodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt enable"]
    #[inline(always)]
    pub fn dmaie(&self) -> DmaieR {
        DmaieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - endpoint number this channel is assigned to"]
    #[inline(always)]
    pub fn dmaep(&self) -> DmaepR {
        DmaepR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - bus error bit"]
    #[inline(always)]
    pub fn dmaerr(&self) -> DmaerrR {
        DmaerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - burst mode"]
    #[inline(always)]
    pub fn dmabrstm(&self) -> DmabrstmR {
        DmabrstmR::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - dma transfer enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, DmacntlSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - transfer direction"]
    #[inline(always)]
    pub fn dmadir(&mut self) -> DmadirW<'_, DmacntlSpec> {
        DmadirW::new(self, 1)
    }
    #[doc = "Bit 2 - transfer mode"]
    #[inline(always)]
    pub fn dmamode(&mut self) -> DmamodeW<'_, DmacntlSpec> {
        DmamodeW::new(self, 2)
    }
    #[doc = "Bit 3 - interrupt enable"]
    #[inline(always)]
    pub fn dmaie(&mut self) -> DmaieW<'_, DmacntlSpec> {
        DmaieW::new(self, 3)
    }
    #[doc = "Bits 4:7 - endpoint number this channel is assigned to"]
    #[inline(always)]
    pub fn dmaep(&mut self) -> DmaepW<'_, DmacntlSpec> {
        DmaepW::new(self, 4)
    }
    #[doc = "Bit 8 - bus error bit"]
    #[inline(always)]
    pub fn dmaerr(&mut self) -> DmaerrW<'_, DmacntlSpec> {
        DmaerrW::new(self, 8)
    }
    #[doc = "Bits 9:10 - burst mode"]
    #[inline(always)]
    pub fn dmabrstm(&mut self) -> DmabrstmW<'_, DmacntlSpec> {
        DmabrstmW::new(self, 9)
    }
}
#[doc = "DMA Transfer Control For Each Channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacntlSpec;
impl crate::RegisterSpec for DmacntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacntl::R`](R) reader structure"]
impl crate::Readable for DmacntlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacntl::W`](W) writer structure"]
impl crate::Writable for DmacntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACNTL to value 0"]
impl crate::Resettable for DmacntlSpec {}
