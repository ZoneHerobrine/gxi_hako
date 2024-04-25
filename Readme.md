# Index
尝试用Rust基于GxIAPI的库进行大恒工业相机的接口开发

目前已实现
1. 获取打印设备信息
2. 单张图片采图
3. 回调实时推流

# Todolist
- Get-Set 函数补全
- 项目重构，等有闲的时候重构
- 发布crate，重构完就发布
- 补全更多函数
- Linux平台支持

# DLL里面的实现了的与暂未实现的全部接口
- [x] 302    0 0001C020 GXCloseDevice
- [x] 101    1 0001BBC0 GXCloseLib
- [] 700    2 0001E9E0 GXExportConfigFile
- [] 707    3 0001EA50 GXExportConfigFileW
- [] 602    4 0001E920 GXFlushEvent
- [] 505    5 0001E6E0 GXFlushQueue
- [x] 201    6 0001BDE0 GXGetAllDeviceBaseInfo
- [] 414    7 0001D5F0 GXGetBool
- [] 419    8 0001E080 GXGetBuffer
- [] 418    9 0001DF50 GXGetBufferLength
- [] 205    A 0001BE80 GXGetDeviceIPInfo
- [] 423    B 0001C0B0 GXGetDevicePersistentIpAddress
- [x] 411    C 0001D3C0 GXGetEnum
- [] 410    D 0001CF50 GXGetEnumDescription
- [] 409    E 0001CE20 GXGetEnumEntryNums
- [] 506    F 0001E970 GXGetEventNumInQueue
- [] 422   10 0001C1E0 GXGetFeatureName
- [] 408   11 0001CCF0 GXGetFloat
- [] 406   12 0001C960 GXGetFloatRange
- [x] 504   13 0001E670 GXGetImage
- [x] 404   14 0001C730 GXGetInt
- [] 403   15 0001C590 GXGetIntRange
- [] 204   16 0001BC40 GXGetLastError
- [] 709   17 0001F370 GXGetOptimalPacketSize
- [] 416   18 0001DAA0 GXGetString
- [] 415   19 0001D820 GXGetStringLength
- [] 425   1A 0001D970 GXGetStringMaxLength
- [] 705   1B 0001EEF0 GXGigEForceIp
- [] 704   1C 0001ECC0 GXGigEIpConfiguration
- [] 706   1D 0001F170 GXGigEResetDevice
- [] 701   1E 0001EAC0 GXImportConfigFile
- [] 708   1F 0001EB40 GXImportConfigFileW
- [x] 100   20 0001BB70 GXInitLib
- [] 400   21 0001C260 GXIsImplemented
- [] 401   22 0001C370 GXIsReadable
- [] 402   23 0001C480 GXIsWritable
- [] 301   24 0001BFB0 GXOpenDevice
- [x] 300   25 0001BF10 GXOpenDeviceByIndex
- [] 702   26 0001EBC0 GXReadRemoteDevicePort
- [] 710   27 0001F3E0 GXReadRemoteDevicePortStacked
- [x] 500   28 0001E5B0 GXRegisterCaptureCallback
- [] 600   29 0001E730 GXRegisterDeviceOfflineCallback
- [] 603   2A 0001E820 GXRegisterFeatureCallback
- [x] 421   2B 0001E480 GXSendCommand
- [] 507   2C 0001F100 GXSetAcqusitionBufferNumber
- [] 413   2D 0001D720 GXSetBool
- [] 420   2E 0001E350 GXSetBuffer
- [] 424   2F 0001C160 GXSetDevicePersistentIpAddress
- [] 412   30 0001D4F0 GXSetEnum
- [] 407   31 0001CBE0 GXSetFloat
- [] 405   32 0001C860 GXSetInt
- [] 417   33 0001DDC0 GXSetString
- [x] 501   34 0001E620 GXUnregisterCaptureCallback
- [] 601   35 0001E7B0 GXUnregisterDeviceOfflineCallback
- [] 604   36 0001E8B0 GXUnregisterFeatureCallback
- [] 206   37 0001BD70 GXUpdateAllDeviceList
- [] 200   38 0001BD00 GXUpdateDeviceList
- [] 703   39 0001EC40 GXWriteRemoteDevicePort
- [] 711   3A 0001F450 GXWriteRemoteDevicePortStacked