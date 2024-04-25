//图像回调处理函数
static void GX_STDC OnFrameCallbackFun(GX_FRAME_CALLBACK_PARAM* pFrame)
{
if (pFrame->status == GX_FRAME_STATUS_ SUCCESS)
{
2.编程指引
版权所有©中国大恒（集团）有限公司北京图像视觉技术分公司 25
//对图像进行某些操作
}
return;
}

//注册图像处理回调函数
status = GXRegisterCaptureCallback(hDevice, NULL,OnFrameCallbackFun);
2.编程指引
版权所有©中国大恒（集团）有限公司北京图像视觉技术分公司 26
//---------------------
//
//在这个区间图像会通过 OnFrameCallbackFun 接口返给用户
//
//---------------------
//注销采集回调
status = GXUnregisterCaptureCallback(hDevice);

请你在rust里面实现这个C的功能

typedef void (GX_STDC * GXCaptureCallBack)
(GX_FRAME_CALLBACK_PARAM *pFrameData)
采集回调函数类型

中文讲解，谢谢！