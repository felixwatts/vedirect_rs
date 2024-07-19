# What?

I have a victron BlueSolar mppt charger for a long term science project I'm running.  I wanted a way to keep tabs 
on the solar status, but the BLE output from the victron is kind of sketchy and prone to stop working in my install.  
I decided to buy a VE.Direct to USB cable and then figure out how to get meaningful data from it.  This project is the result of that.

# Components

## vedirect_rs

This crate utilizes the `nom` parser to convert the serial output of a VE.Direct serial connection into a usable data structure.

## vedirect_to_json

This is a test app for vedirect_rs that connects to /dev/ttyUSB0 and parses the output of the VE.Direct serial stream therein.


# Thanks
Special thanks to @gshipilov for helping me with understanding how the `nom` crate works for payload parsing.  Without his help, 
I would have likely given up.
