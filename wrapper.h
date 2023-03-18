/**
 * This is a copy of LinuxCOM.h, with some hacks to make rust happy.
 * Implementation of REFIID has been refined
 */

#define __LINUX_COM_H_

struct REFIID2
{
    unsigned char byte0;
    unsigned char byte1;
    unsigned char byte2;
    unsigned char byte3;
    unsigned char byte4;
    unsigned char byte5;
    unsigned char byte6;
    unsigned char byte7;
    unsigned char byte8;
    unsigned char byte9;
    unsigned char byte10;
    unsigned char byte11;
    unsigned char byte12;
    unsigned char byte13;
    unsigned char byte14;
    unsigned char byte15;
};

typedef struct REFIID2 CFUUIDBytes;
#define CFUUIDGetUUIDBytes(x) x

#define REFIID struct REFIID2

typedef int HRESULT;
typedef unsigned long ULONG;
typedef void *LPVOID;

#define SUCCEEDED(Status) ((HRESULT)(Status) >= 0)
#define FAILED(Status) ((HRESULT)(Status) < 0)

#define IS_ERROR(Status) ((unsigned long)(Status) >> 31 == SEVERITY_ERROR)
#define HRESULT_CODE(hr) ((hr)&0xFFFF)
#define HRESULT_FACILITY(hr) (((hr) >> 16) & 0x1fff)
#define HRESULT_SEVERITY(hr) (((hr) >> 31) & 0x1)
#define SEVERITY_SUCCESS 0
#define SEVERITY_ERROR 1

#define MAKE_HRESULT(sev, fac, code) ((HRESULT)(((unsigned long)(sev) << 31) | ((unsigned long)(fac) << 16) | ((unsigned long)(code))))

#define S_OK ((HRESULT)0x00000000L)
#define S_FALSE ((HRESULT)0x00000001L)
#define E_UNEXPECTED ((HRESULT)0x8000FFFFL)
#define E_NOTIMPL ((HRESULT)0x80000001L)
#define E_OUTOFMEMORY ((HRESULT)0x80000002L)
#define E_INVALIDARG ((HRESULT)0x80000003L)
#define E_NOINTERFACE ((HRESULT)0x80000004L)
#define E_POINTER ((HRESULT)0x80000005L)
#define E_HANDLE ((HRESULT)0x80000006L)
#define E_ABORT ((HRESULT)0x80000007L)
#define E_FAIL ((HRESULT)0x80000008L)
#define E_ACCESSDENIED ((HRESULT)0x80000009L)

#define STDMETHODCALLTYPE

#define IID_IUnknown                                                                                   \
    (REFIID)                                                                                           \
    {                                                                                                  \
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46 \
    }
#define IUnknownUUID IID_IUnknown

#ifndef BMD_PUBLIC
#define BMD_PUBLIC
#endif

#ifdef __cplusplus
class BMD_PUBLIC IUnknown
{
public:
    virtual HRESULT STDMETHODCALLTYPE QueryInterface(REFIID iid, LPVOID *ppv) = 0;
    virtual ULONG STDMETHODCALLTYPE AddRef(void) = 0;
    virtual ULONG STDMETHODCALLTYPE Release(void) = 0;
};
#endif

#include "./sdk/include/DeckLinkAPI.h"
