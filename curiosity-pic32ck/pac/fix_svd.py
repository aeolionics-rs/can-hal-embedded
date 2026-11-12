"""Patch Microchip's SVD so svd2rust's output compiles. Writes a copy.

- Drops the Arm core peripherals (and their `_NS` aliases): svd2rust leaves
  core peripherals out (the `cortex-m` crate has them), so the aliases
  derived from them would point at nothing.
- Renames EVSYS's `CHANNEL` register inside the `CHANNEL[%s]` cluster to
  `CHCTRL`: the same name for both clashes in the generated code.

Usage: python3 fix_svd.py in.svd out.svd
"""

import sys
import xml.etree.ElementTree as ET

CORE = {"CoreDebug", "DIB", "DWT", "FPB", "FPU", "ICB", "ITM", "MPU", "NVIC", "SAU", "SCB", "SysTick", "TPI"}

tree = ET.parse(sys.argv[1])
peripherals = tree.getroot().find("peripherals")
for p in list(peripherals):
    name = p.findtext("name")
    base = p.get("derivedFrom")
    if name.removesuffix("_NS") in CORE or (base and base in CORE):
        peripherals.remove(p)
for p in peripherals:
    if p.findtext("name") != "EVSYS":
        continue
    for c in p.iter("cluster"):
        if c.findtext("name").startswith("CHANNEL"):
            for r in c.findall("register"):
                if r.findtext("name") == "CHANNEL":
                    r.find("name").text = "CHCTRL"
tree.write(sys.argv[2], xml_declaration=True, encoding="utf-8")
