// A ring buffer connected to a DAC
// We need three things to configure it
// Number of samples, sample bit size, and drain rate in Hz
// Like usual, we drive the thing with a stepper
// It should drain once every n steps, where m is the
// world clock Hz, defined as the LCM of all clocks in the system,
// d is the drain rate in Hz, and n = m / d
