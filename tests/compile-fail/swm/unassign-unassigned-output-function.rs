use lpc8xx_hal::{
    Peripherals,
    pins::{
        self,
        Pin,
    },
    swm,
};


fn main() {
    let mut p = Peripherals::take().unwrap();

    let     swm    = p.SWM.split();
    let mut syscon = p.SYSCON.split();

    #[cfg(feature = "82x")]
    let mut swm_handle = swm.handle;
    #[cfg(feature = "845")]
    let mut swm_handle = swm.handle.enable(&mut syscon.handle);

    let pio0_0: Pin<_, pins::state::Unused> = p.pins.pio0_0;

    let u0_txd: swm::Function<_, swm::state::Unassigned> =
        swm.movable_functions.u0_txd;

    let (u0_txd, pio0_0) =
        u0_txd.assign(pio0_0.into_swm_pin(), &mut swm_handle);
    let (u0_txd, pio0_0) =
        u0_txd.unassign(pio0_0, &mut swm_handle);
    // Should fail: Already unassigned the function.
    let (u0_txd, pio0_0) =
        u0_txd.unassign(pio0_0, &mut swm_handle);
}
