#[doc = "Register `CLKCON` reader"]
pub type R = crate::R<ClkconSpec>;
#[doc = "Register `CLKCON` writer"]
pub type W = crate::W<ClkconSpec>;
#[doc = "Field `EN` reader - This bit is set to 0 when the SPI Driver is not using the SPI Controller. The SPI Controller should stop its internal clock to go very low power state. Still, registers will be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the SPI Controller will set Internal Clock Stable in this register to 1. 1 - Enable 0 - Disable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - This bit is set to 0 when the SPI Driver is not using the SPI Controller. The SPI Controller should stop its internal clock to go very low power state. Still, registers will be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the SPI Controller will set Internal Clock Stable in this register to 1. 1 - Enable 0 - Disable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STABLE` reader - After Reset, this bit is set to 1 when SPI Clock is stable after writing Internal Clock Enable in this register to 1. The Clock Stable indication will continue unless the Clock Divisor Value is changed, in which case it will wait until Clock is stable again. Stability is reached when the Clock Divisor period has been reached. The SPI Host Driver should wait until this bit is set to 1 (and the Enable is 1) before programming the SPI Control register. 1 - Stable 0 - Not stable"]
pub type StableR = crate::BitReader;
#[doc = "Field `STABLE` writer - After Reset, this bit is set to 1 when SPI Clock is stable after writing Internal Clock Enable in this register to 1. The Clock Stable indication will continue unless the Clock Divisor Value is changed, in which case it will wait until Clock is stable again. Stability is reached when the Clock Divisor period has been reached. The SPI Host Driver should wait until this bit is set to 1 (and the Enable is 1) before programming the SPI Control register. 1 - Stable 0 - Not stable"]
pub type StableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIV` reader - SPI Clock Frequency Select 400h - base clock divided by 2048 ,200h - base clock divided by 1024 ,100h - base clock divided by 512 ,080h - base clock divided by 256 ,040h - base clock divided by 128 ,020h - base clock divided by 64 ,010h - base clock divided by 32 ,008h - base clock divided by 16 ,004h - base clock divided by 8 ,002h - base clock divided by 4 ,001h - base clock divided by 2 ,000h - base clock Setting 000h specifies the highest frequency of the SPI Clock. NOTE: When DDR mode is active, the output SPI_CLK is further divided by two (so that internal clocking remains rise edge active internally) Max output SPI_CLK for DDR is 66Mhz."]
pub type ClkdivR = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV` writer - SPI Clock Frequency Select 400h - base clock divided by 2048 ,200h - base clock divided by 1024 ,100h - base clock divided by 512 ,080h - base clock divided by 256 ,040h - base clock divided by 128 ,020h - base clock divided by 64 ,010h - base clock divided by 32 ,008h - base clock divided by 16 ,004h - base clock divided by 8 ,002h - base clock divided by 4 ,001h - base clock divided by 2 ,000h - base clock Setting 000h specifies the highest frequency of the SPI Clock. NOTE: When DDR mode is active, the output SPI_CLK is further divided by two (so that internal clocking remains rise edge active internally) Max output SPI_CLK for DDR is 66Mhz."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - This bit is set to 0 when the SPI Driver is not using the SPI Controller. The SPI Controller should stop its internal clock to go very low power state. Still, registers will be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the SPI Controller will set Internal Clock Stable in this register to 1. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - After Reset, this bit is set to 1 when SPI Clock is stable after writing Internal Clock Enable in this register to 1. The Clock Stable indication will continue unless the Clock Divisor Value is changed, in which case it will wait until Clock is stable again. Stability is reached when the Clock Divisor period has been reached. The SPI Host Driver should wait until this bit is set to 1 (and the Enable is 1) before programming the SPI Control register. 1 - Stable 0 - Not stable"]
    #[inline(always)]
    pub fn stable(&self) -> StableR {
        StableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:18 - SPI Clock Frequency Select 400h - base clock divided by 2048 ,200h - base clock divided by 1024 ,100h - base clock divided by 512 ,080h - base clock divided by 256 ,040h - base clock divided by 128 ,020h - base clock divided by 64 ,010h - base clock divided by 32 ,008h - base clock divided by 16 ,004h - base clock divided by 8 ,002h - base clock divided by 4 ,001h - base clock divided by 2 ,000h - base clock Setting 000h specifies the highest frequency of the SPI Clock. NOTE: When DDR mode is active, the output SPI_CLK is further divided by two (so that internal clocking remains rise edge active internally) Max output SPI_CLK for DDR is 66Mhz."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 8) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set to 0 when the SPI Driver is not using the SPI Controller. The SPI Controller should stop its internal clock to go very low power state. Still, registers will be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the SPI Controller will set Internal Clock Stable in this register to 1. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, ClkconSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - After Reset, this bit is set to 1 when SPI Clock is stable after writing Internal Clock Enable in this register to 1. The Clock Stable indication will continue unless the Clock Divisor Value is changed, in which case it will wait until Clock is stable again. Stability is reached when the Clock Divisor period has been reached. The SPI Host Driver should wait until this bit is set to 1 (and the Enable is 1) before programming the SPI Control register. 1 - Stable 0 - Not stable"]
    #[inline(always)]
    pub fn stable(&mut self) -> StableW<'_, ClkconSpec> {
        StableW::new(self, 1)
    }
    #[doc = "Bits 8:18 - SPI Clock Frequency Select 400h - base clock divided by 2048 ,200h - base clock divided by 1024 ,100h - base clock divided by 512 ,080h - base clock divided by 256 ,040h - base clock divided by 128 ,020h - base clock divided by 64 ,010h - base clock divided by 32 ,008h - base clock divided by 16 ,004h - base clock divided by 8 ,002h - base clock divided by 4 ,001h - base clock divided by 2 ,000h - base clock Setting 000h specifies the highest frequency of the SPI Clock. NOTE: When DDR mode is active, the output SPI_CLK is further divided by two (so that internal clocking remains rise edge active internally) Max output SPI_CLK for DDR is 66Mhz."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, ClkconSpec> {
        ClkdivW::new(self, 8)
    }
}
#[doc = "SPI Clock Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkconSpec;
impl crate::RegisterSpec for ClkconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcon::R`](R) reader structure"]
impl crate::Readable for ClkconSpec {}
#[doc = "`write(|w| ..)` method takes [`clkcon::W`](W) writer structure"]
impl crate::Writable for ClkconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKCON to value 0x01"]
impl crate::Resettable for ClkconSpec {
    const RESET_VALUE: u32 = 0x01;
}
