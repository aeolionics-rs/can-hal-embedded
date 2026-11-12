#[doc = "Register `MEMSTAT` reader"]
pub type R = crate::R<MemstatSpec>;
#[doc = "Register `MEMSTAT` writer"]
pub type W = crate::W<MemstatSpec>;
#[doc = "Field `STATCMD` reader - Status Data: The first byte to be sent must be in LSB. For example if the user wants to send 0F first and 0C second, then we should program 16?h0C0F."]
pub type StatcmdR = crate::FieldReader<u16>;
#[doc = "Field `STATCMD` writer - Status Data: The first byte to be sent must be in LSB. For example if the user wants to send 0F first and 0C second, then we should program 16?h0C0F."]
pub type StatcmdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `STATBYTES` reader - No. of bytes to send: 00 - Reserved ,01 - 1 byte ,10 - 2 bytes ,11 - Reserved"]
pub type StatbytesR = crate::FieldReader;
#[doc = "Field `STATBYTES` writer - No. of bytes to send: 00 - Reserved ,01 - 1 byte ,10 - 2 bytes ,11 - Reserved"]
pub type StatbytesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TYPESTAT` reader - Status Lane: Indicates the number of lanes (single/dual/quad) in which the Status command/ Read Status Register value is transmitted to /received from flash. 2?b00 - single lane, 2?b01 - dual lane, 2?b10 - quad lane"]
pub type TypestatR = crate::FieldReader;
#[doc = "Field `TYPESTAT` writer - Status Lane: Indicates the number of lanes (single/dual/quad) in which the Status command/ Read Status Register value is transmitted to /received from flash. 2?b00 - single lane, 2?b01 - dual lane, 2?b10 - quad lane"]
pub type TypestatW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STATPOS` reader - Indicates whether the BUSY bit position in Status Register of flash is 0 or 7. 1?b0 - 0th bit position (bit 0 is valid for busy status), 1?b1 - 7th bit position (bit7 is valid for busy status) The Controller continuously reads the Status register value from flash until this bit which indicates the flash busy status goes low."]
pub type StatposR = crate::BitReader;
#[doc = "Field `STATPOS` writer - Indicates whether the BUSY bit position in Status Register of flash is 0 or 7. 1?b0 - 0th bit position (bit 0 is valid for busy status), 1?b1 - 7th bit position (bit7 is valid for busy status) The Controller continuously reads the Status register value from flash until this bit which indicates the flash busy status goes low."]
pub type StatposW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Status Data: The first byte to be sent must be in LSB. For example if the user wants to send 0F first and 0C second, then we should program 16?h0C0F."]
    #[inline(always)]
    pub fn statcmd(&self) -> StatcmdR {
        StatcmdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - No. of bytes to send: 00 - Reserved ,01 - 1 byte ,10 - 2 bytes ,11 - Reserved"]
    #[inline(always)]
    pub fn statbytes(&self) -> StatbytesR {
        StatbytesR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Status Lane: Indicates the number of lanes (single/dual/quad) in which the Status command/ Read Status Register value is transmitted to /received from flash. 2?b00 - single lane, 2?b01 - dual lane, 2?b10 - quad lane"]
    #[inline(always)]
    pub fn typestat(&self) -> TypestatR {
        TypestatR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Indicates whether the BUSY bit position in Status Register of flash is 0 or 7. 1?b0 - 0th bit position (bit 0 is valid for busy status), 1?b1 - 7th bit position (bit7 is valid for busy status) The Controller continuously reads the Status register value from flash until this bit which indicates the flash busy status goes low."]
    #[inline(always)]
    pub fn statpos(&self) -> StatposR {
        StatposR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Status Data: The first byte to be sent must be in LSB. For example if the user wants to send 0F first and 0C second, then we should program 16?h0C0F."]
    #[inline(always)]
    pub fn statcmd(&mut self) -> StatcmdW<'_, MemstatSpec> {
        StatcmdW::new(self, 0)
    }
    #[doc = "Bits 16:17 - No. of bytes to send: 00 - Reserved ,01 - 1 byte ,10 - 2 bytes ,11 - Reserved"]
    #[inline(always)]
    pub fn statbytes(&mut self) -> StatbytesW<'_, MemstatSpec> {
        StatbytesW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Status Lane: Indicates the number of lanes (single/dual/quad) in which the Status command/ Read Status Register value is transmitted to /received from flash. 2?b00 - single lane, 2?b01 - dual lane, 2?b10 - quad lane"]
    #[inline(always)]
    pub fn typestat(&mut self) -> TypestatW<'_, MemstatSpec> {
        TypestatW::new(self, 18)
    }
    #[doc = "Bit 20 - Indicates whether the BUSY bit position in Status Register of flash is 0 or 7. 1?b0 - 0th bit position (bit 0 is valid for busy status), 1?b1 - 7th bit position (bit7 is valid for busy status) The Controller continuously reads the Status register value from flash until this bit which indicates the flash busy status goes low."]
    #[inline(always)]
    pub fn statpos(&mut self) -> StatposW<'_, MemstatSpec> {
        StatposW::new(self, 20)
    }
}
#[doc = "SPI Status Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`memstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemstatSpec;
impl crate::RegisterSpec for MemstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memstat::R`](R) reader structure"]
impl crate::Readable for MemstatSpec {}
#[doc = "`write(|w| ..)` method takes [`memstat::W`](W) writer structure"]
impl crate::Writable for MemstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEMSTAT to value 0x0001_0005"]
impl crate::Resettable for MemstatSpec {
    const RESET_VALUE: u32 = 0x0001_0005;
}
