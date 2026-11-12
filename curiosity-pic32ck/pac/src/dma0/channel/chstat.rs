#[doc = "Register `CHSTAT` reader"]
pub type R = crate::R<ChstatSpec>;
#[doc = "Field `BLKBUSY` reader - Channel Block Transfer Busy Status Bit (0: none; 1: busy)"]
pub type BlkbusyR = crate::BitReader;
#[doc = "Field `CELLBUSY` reader - Channel Cell Transfer Busy Status Bit (0: none; 1: busy)"]
pub type CellbusyR = crate::BitReader;
#[doc = "Field `DREAD` reader - Descriptor Read Status Bit (0: not read or not avail, 1: read and loaded)"]
pub type DreadR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel Block Transfer Busy Status Bit (0: none; 1: busy)"]
    #[inline(always)]
    pub fn blkbusy(&self) -> BlkbusyR {
        BlkbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Cell Transfer Busy Status Bit (0: none; 1: busy)"]
    #[inline(always)]
    pub fn cellbusy(&self) -> CellbusyR {
        CellbusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Descriptor Read Status Bit (0: not read or not avail, 1: read and loaded)"]
    #[inline(always)]
    pub fn dread(&self) -> DreadR {
        DreadR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "CHANNEL STATUS REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChstatSpec;
impl crate::RegisterSpec for ChstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chstat::R`](R) reader structure"]
impl crate::Readable for ChstatSpec {}
#[doc = "`reset()` method sets CHSTAT to value 0"]
impl crate::Resettable for ChstatSpec {}
