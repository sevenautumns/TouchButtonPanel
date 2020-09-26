EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A3 16535 11693
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L STM32F401RCT6:STM32F401RCT6 U4
U 1 1 5F6EFB44
P 6750 1300
F 0 "U4" H 8050 1687 60  0000 C CNN
F 1 "STM32F401RCT6" H 8050 1581 60  0000 C CNN
F 2 "STM32F401RCT6:STM32F401RCT6" H 8050 1540 60  0001 C CNN
F 3 "" H 6750 1300 60  0000 C CNN
	1    6750 1300
	1    0    0    -1  
$EndComp
$Comp
L Connector:USB_C_Plug_USB2.0 P1
U 1 1 5F6FB855
P 1150 1550
F 0 "P1" H 1257 2417 50  0000 C CNN
F 1 "USB_C_Plug_USB2.0" H 1257 2326 50  0000 C CNN
F 2 "Connector_USB:USB_C_Receptacle_GCT_USB4085" H 1300 1550 50  0001 C CNN
F 3 "https://www.usb.org/sites/default/files/documents/usb_type-c.zip" H 1300 1550 50  0001 C CNN
	1    1150 1550
	1    0    0    -1  
$EndComp
Text GLabel 6750 3900 0    50   Input ~ 0
BOOT1
Text GLabel 9350 1700 2    50   Input ~ 0
BOOT0
Text GLabel 6750 1900 0    50   Input ~ 0
RESET
$Comp
L Device:R R3
U 1 1 5F70B262
P 2200 1150
F 0 "R3" V 1993 1150 50  0000 C CNN
F 1 "5.1 k" V 2084 1150 50  0000 C CNN
F 2 "Resistor_SMD:R_0603_1608Metric_Pad1.05x0.95mm_HandSolder" V 2130 1150 50  0001 C CNN
F 3 "~" H 2200 1150 50  0001 C CNN
	1    2200 1150
	0    1    1    0   
$EndComp
Wire Wire Line
	1750 1150 2050 1150
Text GLabel 1750 1450 2    50   Input ~ 0
D_MINUS
Text GLabel 1750 1650 2    50   Input ~ 0
D_PLUS
Text GLabel 1750 950  2    50   Input ~ 0
5V
Text GLabel 2750 2500 2    50   Input ~ 0
GND
Wire Wire Line
	1150 2450 1150 2500
Wire Wire Line
	850  2450 850  2500
Wire Wire Line
	850  2500 1150 2500
Connection ~ 1150 2500
Wire Wire Line
	2350 1150 2350 2500
Wire Wire Line
	1150 2500 2350 2500
Connection ~ 2350 2500
Wire Wire Line
	2350 2500 2750 2500
Wire Notes Line
	3100 2700 3100 600 
Wire Notes Line
	3100 600  650  600 
Wire Notes Line
	650  600  650  2700
Wire Notes Line
	650  2700 3100 2700
Text Notes 3000 750  2    50   ~ 0
USB Port\n
$Comp
L Regulator_Linear:AMS1117-3.3 U3
U 1 1 5F6FAC90
P 4250 1000
F 0 "U3" H 4250 1242 50  0000 C CNN
F 1 "AMS1117-3.3" H 4250 1151 50  0000 C CNN
F 2 "Package_TO_SOT_SMD:SOT-223-3_TabPin2" H 4250 1200 50  0001 C CNN
F 3 "http://www.advanced-monolithic.com/pdf/ds1117.pdf" H 4350 750 50  0001 C CNN
	1    4250 1000
	1    0    0    -1  
$EndComp
$Comp
L Device:CP C2
U 1 1 5F718F7A
P 4900 1300
F 0 "C2" H 5018 1346 50  0000 L CNN
F 1 "22 uF" H 5018 1255 50  0000 L CNN
F 2 "Capacitor_SMD:C_0603_1608Metric_Pad1.05x0.95mm_HandSolder" H 4938 1150 50  0001 C CNN
F 3 "~" H 4900 1300 50  0001 C CNN
	1    4900 1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	3950 1000 3550 1000
Wire Wire Line
	4550 1000 4900 1000
Wire Wire Line
	4900 1000 4900 1150
Text GLabel 4450 1850 2    50   Input ~ 0
GND
Wire Wire Line
	4250 1300 4250 1650
Wire Wire Line
	4250 1850 4450 1850
Wire Wire Line
	3550 1450 3550 1650
Wire Wire Line
	3550 1650 4250 1650
Connection ~ 4250 1650
Wire Wire Line
	4250 1650 4900 1650
Wire Wire Line
	4900 1650 4900 1450
Wire Wire Line
	3550 1000 3550 1150
$Comp
L Device:CP C1
U 1 1 5F71849F
P 3550 1300
F 0 "C1" H 3668 1346 50  0000 L CNN
F 1 "10 uF" H 3668 1255 50  0000 L CNN
F 2 "Capacitor_SMD:C_0603_1608Metric_Pad1.05x0.95mm_HandSolder" H 3588 1150 50  0001 C CNN
F 3 "~" H 3550 1300 50  0001 C CNN
	1    3550 1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	4250 1650 4250 1850
$Comp
L power:Earth #PWR0101
U 1 1 5F71D75C
P 4250 1950
F 0 "#PWR0101" H 4250 1700 50  0001 C CNN
F 1 "Earth" H 4250 1800 50  0001 C CNN
F 2 "" H 4250 1950 50  0001 C CNN
F 3 "~" H 4250 1950 50  0001 C CNN
	1    4250 1950
	1    0    0    -1  
$EndComp
Wire Wire Line
	4250 1950 4250 1850
Connection ~ 4250 1850
Wire Notes Line
	3350 600  5300 600 
Wire Notes Line
	5300 600  5300 2100
Wire Notes Line
	3350 600  3350 2100
Wire Notes Line
	3350 2100 5300 2100
Text Notes 5200 750  2    50   ~ 0
Voltage Regulator\n
Text GLabel 5000 1000 2    50   Input ~ 0
3.3V
Wire Wire Line
	5000 1000 4900 1000
Connection ~ 4900 1000
Text GLabel 6750 2100 0    50   Input ~ 0
SPI2_LED
Text GLabel 6750 2200 0    50   Input ~ 0
SPI2_MISO
Text GLabel 6750 2300 0    50   Input ~ 0
SPI2_MOSI
Text GLabel 6750 2600 0    50   Input ~ 0
CHANGE0
Text GLabel 6750 2700 0    50   Input ~ 0
RESET0
Text GLabel 6750 2800 0    50   Input ~ 0
BTN0
Text GLabel 6750 2900 0    50   Input ~ 0
BTN1
Text GLabel 6750 3200 0    50   Input ~ 0
BTN2
Text GLabel 6750 3300 0    50   Input ~ 0
BTN3
Text GLabel 6750 3400 0    50   Input ~ 0
BTN4
Text GLabel 6750 3500 0    50   Input ~ 0
BTN5
Text GLabel 6750 3600 0    50   Input ~ 0
BTN_LED
Text GLabel 6750 4100 0    50   Input ~ 0
SPI2_SCK
Text GLabel 9350 3600 2    50   Input ~ 0
BTN6
Text GLabel 9350 3500 2    50   Input ~ 0
BTN7
Text GLabel 9350 3300 2    50   Input ~ 0
D_MINUS
Text GLabel 9350 3200 2    50   Input ~ 0
D_PLUS
Text GLabel 9350 3100 2    50   Input ~ 0
USB_LED
Text GLabel 9350 2800 2    50   Input ~ 0
RESET1
Text GLabel 9350 2700 2    50   Input ~ 0
CHANGE1
Text GLabel 9350 2600 2    50   Input ~ 0
SPI3_SCK
Text GLabel 9350 2500 2    50   Input ~ 0
SPI3_MISO
Text GLabel 9350 2400 2    50   Input ~ 0
SPI3_MOSI
Text GLabel 9350 2300 2    50   Input ~ 0
SPI3_LED
Text GLabel 6750 4300 0    50   Input ~ 0
GND
Text GLabel 6750 4400 0    50   Input ~ 0
3.3V
Text GLabel 6750 3000 0    50   Input ~ 0
GND
Text GLabel 6750 3100 0    50   Input ~ 0
3.3V
Text GLabel 6750 2400 0    50   Input ~ 0
GND
Text GLabel 6750 2500 0    50   Input ~ 0
3.3V
Text GLabel 9350 1300 2    50   Input ~ 0
3.3V
Text GLabel 9350 1400 2    50   Input ~ 0
GND
Text GLabel 9350 2900 2    50   Input ~ 0
3.3V
Text GLabel 9350 3000 2    50   Input ~ 0
GND
Text GLabel 6750 1300 0    50   Input ~ 0
3.3V
$Comp
L AT42QT1110-AUR:AT42QT1110-AUR U1
U 1 1 5F6F0F49
P 1600 3600
F 0 "U1" H 2800 3987 60  0000 C CNN
F 1 "AT42QT1110-AUR" H 2800 3881 60  0000 C CNN
F 2 "AT42QT1110-AUR:AT42QT1110-AUR" H 2800 3840 60  0001 C CNN
F 3 "" H 1600 3600 60  0000 C CNN
	1    1600 3600
	1    0    0    -1  
$EndComp
$Comp
L AT42QT1110-AUR:AT42QT1110-AUR U2
U 1 1 5F6F1E78
P 1600 6000
F 0 "U2" H 2800 6387 60  0000 C CNN
F 1 "AT42QT1110-AUR" H 2800 6281 60  0000 C CNN
F 2 "AT42QT1110-AUR:AT42QT1110-AUR" H 2800 6240 60  0001 C CNN
F 3 "" H 1600 6000 60  0000 C CNN
	1    1600 6000
	1    0    0    -1  
$EndComp
Text GLabel 4000 5000 2    50   Input ~ 0
5V
Text GLabel 1600 3900 0    50   Input ~ 0
5V
Text GLabel 1600 4000 0    50   Input ~ 0
GND
Text GLabel 4000 4700 2    50   Input ~ 0
GND
Text GLabel 1250 5100 0    50   Input ~ 0
SPI2_MISO
Text GLabel 1600 5000 0    50   Input ~ 0
SPI2_MOSI
Wire Wire Line
	1600 5100 1450 5100
$Comp
L Device:R R1
U 1 1 5F7325AE
P 1450 5250
F 0 "R1" H 1520 5296 50  0000 L CNN
F 1 "100k" H 1520 5205 50  0000 L CNN
F 2 "Resistor_SMD:R_0603_1608Metric_Pad1.05x0.95mm_HandSolder" V 1380 5250 50  0001 C CNN
F 3 "~" H 1450 5250 50  0001 C CNN
	1    1450 5250
	1    0    0    -1  
$EndComp
Connection ~ 1450 5100
Wire Wire Line
	1450 5100 1250 5100
Text GLabel 1250 5400 0    50   Input ~ 0
GND
Wire Wire Line
	1450 5400 1250 5400
Text GLabel 1250 7500 0    50   Input ~ 0
SPI3_MISO
Wire Wire Line
	1600 7500 1450 7500
$Comp
L Device:R R2
U 1 1 5F7372D0
P 1450 7650
F 0 "R2" H 1520 7696 50  0000 L CNN
F 1 "100k" H 1520 7605 50  0000 L CNN
F 2 "Resistor_SMD:R_0603_1608Metric_Pad1.05x0.95mm_HandSolder" V 1380 7650 50  0001 C CNN
F 3 "~" H 1450 7650 50  0001 C CNN
	1    1450 7650
	1    0    0    -1  
$EndComp
Connection ~ 1450 7500
Wire Wire Line
	1450 7500 1250 7500
Text GLabel 1250 7800 0    50   Input ~ 0
GND
Wire Wire Line
	1450 7800 1250 7800
Text GLabel 1600 7400 0    50   Input ~ 0
SPI3_MOSI
Text GLabel 4000 7100 2    50   Input ~ 0
GND
Text GLabel 4000 7400 2    50   Input ~ 0
5V
Text GLabel 1600 6300 0    50   Input ~ 0
5V
Text GLabel 1600 6400 0    50   Input ~ 0
GND
Text GLabel 4000 6300 2    50   Input ~ 0
RESET1
Text GLabel 4000 6400 2    50   Input ~ 0
CHANGE1
Text GLabel 4000 3900 2    50   Input ~ 0
RESET0
Text GLabel 4000 4000 2    50   Input ~ 0
CHANGE0
Text GLabel 1600 4900 0    50   Input ~ 0
SS0
Text GLabel 1600 7300 0    50   Input ~ 0
SS1
Text GLabel 4000 7500 2    50   Input ~ 0
SPI3_SCK
Text GLabel 4000 5100 2    50   Input ~ 0
SPI2_SCK
Text GLabel 4000 3600 2    50   Input ~ 0
Key0
Text GLabel 4000 3700 2    50   Input ~ 0
Key10K
Text GLabel 4000 3800 2    50   Input ~ 0
Key10
Text GLabel 4000 4100 2    50   Input ~ 0
Key9K
Text GLabel 4000 4200 2    50   Input ~ 0
Key9
Text GLabel 4000 4300 2    50   Input ~ 0
Key8K
Text GLabel 4000 4400 2    50   Input ~ 0
Key8
Text GLabel 4000 4500 2    50   Input ~ 0
Key7
Text GLabel 4000 4600 2    50   Input ~ 0
Key7K
Text GLabel 4000 4800 2    50   Input ~ 0
Key6
Text GLabel 4000 4900 2    50   Input ~ 0
Key6K
Text GLabel 1600 3600 0    50   Input ~ 0
Key0K
Text GLabel 1600 3700 0    50   Input ~ 0
Key1
Text GLabel 1600 3800 0    50   Input ~ 0
Key1K
Text GLabel 1600 4100 0    50   Input ~ 0
Key2K
Text GLabel 1600 4200 0    50   Input ~ 0
Key2
Text GLabel 1600 4300 0    50   Input ~ 0
Key3
Text GLabel 1600 4400 0    50   Input ~ 0
Key3K
Text GLabel 1600 4500 0    50   Input ~ 0
Key4
Text GLabel 1600 4600 0    50   Input ~ 0
Key4K
Text GLabel 1600 4700 0    50   Input ~ 0
Key5
Text GLabel 1600 4800 0    50   Input ~ 0
Key5K
Text GLabel 4000 6000 2    50   Input ~ 0
Key11
Text GLabel 1600 6000 0    50   Input ~ 0
Key11K
Text GLabel 1600 6100 0    50   Input ~ 0
Key12
Text GLabel 1600 6200 0    50   Input ~ 0
Key12K
Text GLabel 1600 6500 0    50   Input ~ 0
Key13K
Text GLabel 1600 6600 0    50   Input ~ 0
Key13
Text GLabel 1600 6700 0    50   Input ~ 0
Key14
Text GLabel 1600 6800 0    50   Input ~ 0
Key14K
Text GLabel 1600 6900 0    50   Input ~ 0
key15
Text GLabel 1600 7000 0    50   Input ~ 0
Key15K
Text GLabel 1600 7100 0    50   Input ~ 0
Key16
Text GLabel 1600 7200 0    50   Input ~ 0
Key16K
Wire Notes Line
	4750 8150 800  8150
Wire Notes Line
	800  8150 800  3000
Wire Notes Line
	800  3000 4750 3000
Wire Notes Line
	4750 3000 4750 8150
Text Notes 4300 3100 0    50   ~ 0
Touch ICs
$Comp
L Device:C C3
U 1 1 5F75DC2F
P 6150 4350
F 0 "C3" H 6265 4396 50  0000 L CNN
F 1 "2.2 uF" H 6265 4305 50  0000 L CNN
F 2 "Capacitor_SMD:C_0603_1608Metric_Pad1.05x0.95mm_HandSolder" H 6188 4200 50  0001 C CNN
F 3 "~" H 6150 4350 50  0001 C CNN
	1    6150 4350
	1    0    0    -1  
$EndComp
Text GLabel 5950 4500 0    50   Input ~ 0
GND
Wire Notes Line
	9900 4850 5650 4850
Wire Notes Line
	5650 4850 5650 700 
Wire Notes Line
	5650 700  9900 700 
Wire Notes Line
	9900 700  9900 4850
Text Notes 9650 850  0    50   ~ 0
MCU\n
Wire Wire Line
	6150 4200 6750 4200
Wire Wire Line
	6150 4500 5950 4500
Text Notes 5800 5150 0    50   ~ 0
100nF als decouple für VBAT\n
$EndSCHEMATC
