#[doc = "Register `RXFIFOSZ` reader"]
pub type R = crate::R<RxfifoszSpec>;
#[doc = "Register `RXFIFOSZ` writer"]
pub type W = crate::W<RxfifoszSpec>;
#[doc = "maximum packet size to be allowed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Szselect {
    #[doc = "0: 8 BYTES"]
    _8bytes = 0,
    #[doc = "1: 16 BYTES"]
    _16bytes = 1,
    #[doc = "2: 32 BYTES"]
    _32bytes = 2,
    #[doc = "3: 64 BYTES"]
    _64bytes = 3,
    #[doc = "4: 128 BYTES"]
    _128bytes = 4,
    #[doc = "5: 256 BYTES"]
    _256bytes = 5,
    #[doc = "6: 512 BYTES"]
    _512bytes = 6,
    #[doc = "7: 1024 BYTES"]
    _1024bytes = 7,
    #[doc = "8: 2048 BYTES"]
    _2048bytes = 8,
    #[doc = "9: 4096 BYTES"]
    _4096bytes = 9,
}
impl From<Szselect> for u8 {
    #[inline(always)]
    fn from(variant: Szselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Szselect {
    type Ux = u8;
}
impl crate::IsEnum for Szselect {}
#[doc = "Field `SZ` reader - maximum packet size to be allowed"]
pub type SzR = crate::FieldReader<Szselect>;
impl SzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Szselect> {
        match self.bits {
            0 => Some(Szselect::_8bytes),
            1 => Some(Szselect::_16bytes),
            2 => Some(Szselect::_32bytes),
            3 => Some(Szselect::_64bytes),
            4 => Some(Szselect::_128bytes),
            5 => Some(Szselect::_256bytes),
            6 => Some(Szselect::_512bytes),
            7 => Some(Szselect::_1024bytes),
            8 => Some(Szselect::_2048bytes),
            9 => Some(Szselect::_4096bytes),
            _ => None,
        }
    }
    #[doc = "8 BYTES"]
    #[inline(always)]
    pub fn is_8bytes(&self) -> bool {
        *self == Szselect::_8bytes
    }
    #[doc = "16 BYTES"]
    #[inline(always)]
    pub fn is_16bytes(&self) -> bool {
        *self == Szselect::_16bytes
    }
    #[doc = "32 BYTES"]
    #[inline(always)]
    pub fn is_32bytes(&self) -> bool {
        *self == Szselect::_32bytes
    }
    #[doc = "64 BYTES"]
    #[inline(always)]
    pub fn is_64bytes(&self) -> bool {
        *self == Szselect::_64bytes
    }
    #[doc = "128 BYTES"]
    #[inline(always)]
    pub fn is_128bytes(&self) -> bool {
        *self == Szselect::_128bytes
    }
    #[doc = "256 BYTES"]
    #[inline(always)]
    pub fn is_256bytes(&self) -> bool {
        *self == Szselect::_256bytes
    }
    #[doc = "512 BYTES"]
    #[inline(always)]
    pub fn is_512bytes(&self) -> bool {
        *self == Szselect::_512bytes
    }
    #[doc = "1024 BYTES"]
    #[inline(always)]
    pub fn is_1024bytes(&self) -> bool {
        *self == Szselect::_1024bytes
    }
    #[doc = "2048 BYTES"]
    #[inline(always)]
    pub fn is_2048bytes(&self) -> bool {
        *self == Szselect::_2048bytes
    }
    #[doc = "4096 BYTES"]
    #[inline(always)]
    pub fn is_4096bytes(&self) -> bool {
        *self == Szselect::_4096bytes
    }
}
#[doc = "Field `SZ` writer - maximum packet size to be allowed"]
pub type SzW<'a, REG> = crate::FieldWriter<'a, REG, 4, Szselect>;
impl<'a, REG> SzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 BYTES"]
    #[inline(always)]
    pub fn _8bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Szselect::_8bytes)
    }
    #[doc = "16 BYTES"]
    #[inline(always)]
    pub fn _16bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Szselect::_16bytes)
    }
    #[doc = "32 BYTES"]
    #[inline(always)]
    pub fn _32bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Szselect::_32bytes)
    }
    #[doc = "64 BYTES"]
    #[inline(always)]
    pub fn _64bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Szselect::_64bytes)
    }
    #[doc = "128 BYTES"]
    #[inline(always)]
    pub fn _128bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Szselect::_128bytes)
    }
    #[doc = "256 BYTES"]
    #[inline(always)]
    pub fn _256bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Szselect::_256bytes)
    }
    #[doc = "512 BYTES"]
    #[inline(always)]
    pub fn _512bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Szselect::_512bytes)
    }
    #[doc = "1024 BYTES"]
    #[inline(always)]
    pub fn _1024bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Szselect::_1024bytes)
    }
    #[doc = "2048 BYTES"]
    #[inline(always)]
    pub fn _2048bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Szselect::_2048bytes)
    }
    #[doc = "4096 BYTES"]
    #[inline(always)]
    pub fn _4096bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Szselect::_4096bytes)
    }
}
#[doc = "Field `DPB` reader - double packet buffering enabled or not"]
pub type DpbR = crate::BitReader;
#[doc = "Field `DPB` writer - double packet buffering enabled or not"]
pub type DpbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - maximum packet size to be allowed"]
    #[inline(always)]
    pub fn sz(&self) -> SzR {
        SzR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - double packet buffering enabled or not"]
    #[inline(always)]
    pub fn dpb(&self) -> DpbR {
        DpbR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - maximum packet size to be allowed"]
    #[inline(always)]
    pub fn sz(&mut self) -> SzW<'_, RxfifoszSpec> {
        SzW::new(self, 0)
    }
    #[doc = "Bit 4 - double packet buffering enabled or not"]
    #[inline(always)]
    pub fn dpb(&mut self) -> DpbW<'_, RxfifoszSpec> {
        DpbW::new(self, 4)
    }
}
#[doc = "Controls The Size Of The Selected RX Endpoint FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifosz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfifosz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfifoszSpec;
impl crate::RegisterSpec for RxfifoszSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxfifosz::R`](R) reader structure"]
impl crate::Readable for RxfifoszSpec {}
#[doc = "`write(|w| ..)` method takes [`rxfifosz::W`](W) writer structure"]
impl crate::Writable for RxfifoszSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXFIFOSZ to value 0"]
impl crate::Resettable for RxfifoszSpec {}
