//
// DelayHlp.cpp
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  Implement the delay load helper routines.
//

// Build instructions
// cl -c -O1 -Z7 -Zl -W3 delayhlp.cpp
//
// For ISOLATION_AWARE_ENABLED calls to LoadLibrary(), you will need to add
// a definition for ISOLATION_AWARE_ENABLED to the command line above, eg:
// cl -c -O1 -Z7 -Zl -W3 -DISOLATION_AWARE_ENABLED=1 delayhlp.cpp
//
//
// Then, you can either link directly with this new object file, or replace the one in
// delayimp.lib with your new one, eg:
// lib /out:delayimp.lib delayhlp.obj
//

#define WIN32_LEAN_AND_MEAN
#define STRICT
#include <Windows.h>

//
// DelayImp.h
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  Define structures and prototypes necessary for delay loading of imports
//
#pragma once

#define _DELAY_IMP_VER  2

#if defined(__cplusplus)
#define ExternC extern "C"
#else
#define ExternC extern
#endif

typedef IMAGE_THUNK_DATA *          PImgThunkData;
typedef const IMAGE_THUNK_DATA *    PCImgThunkData;
typedef DWORD                       RVA;

typedef struct ImgDelayDescr {
    DWORD           grAttrs;        // attributes
    RVA             rvaDLLName;     // RVA to dll name
    RVA             rvaHmod;        // RVA of module handle
    RVA             rvaIAT;         // RVA of the IAT
    RVA             rvaINT;         // RVA of the INT
    RVA             rvaBoundIAT;    // RVA of the optional bound IAT
    RVA             rvaUnloadIAT;   // RVA of optional copy of original IAT
    DWORD           dwTimeStamp;    // 0 if not bound,
                                    // O.W. date/time stamp of DLL bound to (Old BIND)
    } ImgDelayDescr, * PImgDelayDescr;

typedef const ImgDelayDescr *   PCImgDelayDescr;

enum DLAttr {                   // Delay Load Attributes
    dlattrRva = 0x1,                // RVAs are used instead of pointers
                                    // Having this set indicates a VC7.0
                                    // and above delay load descriptor.
    };

//
// Delay load import hook notifications
//
enum {
    dliStartProcessing,             // used to bypass or note helper only
    dliNoteStartProcessing = dliStartProcessing,

    dliNotePreLoadLibrary,          // called just before LoadLibrary, can
                                    //  override w/ new HMODULE return val
    dliNotePreGetProcAddress,       // called just before GetProcAddress, can
                                    //  override w/ new FARPROC return value
    dliFailLoadLib,                 // failed to load library, fix it by
                                    //  returning a valid HMODULE
    dliFailGetProc,                 // failed to get proc address, fix it by
                                    //  returning a valid FARPROC
    dliNoteEndProcessing,           // called after all processing is done, no
                                    //  bypass possible at this point except
                                    //  by longjmp()/throw()/RaiseException.
    };

typedef struct DelayLoadProc {
    BOOL                fImportByName;
    union {
        LPCSTR          szProcName;
        DWORD           dwOrdinal;
        };
    } DelayLoadProc;

typedef struct DelayLoadInfo {
    DWORD               cb;         // size of structure
    PCImgDelayDescr     pidd;       // raw form of data (everything is there)
    FARPROC *           ppfn;       // points to address of function to load
    LPCSTR              szDll;      // name of dll
    DelayLoadProc       dlp;        // name or ordinal of procedure
    HMODULE             hmodCur;    // the hInstance of the library we have loaded
    FARPROC             pfnCur;     // the actual function that will be called
    DWORD               dwLastError;// error received (if an error notification)
    } DelayLoadInfo, * PDelayLoadInfo;

typedef FARPROC (WINAPI *PfnDliHook)(
    unsigned        dliNotify,
    PDelayLoadInfo  pdli
    );

//
// Unload support
//

// routine definition; takes a pointer to a name to unload
//
ExternC
BOOL WINAPI
__FUnloadDelayLoadedDLL2(LPCSTR szDll);

//
// Snap load support
//
ExternC
HRESULT WINAPI
__HrLoadAllImportsForDll(LPCSTR szDll);


//
// Exception information
//
//#define FACILITY_VISUALCPP  ((LONG)0x6d)  now defined in winerror.h
#define VcppException(sev,err)  ((sev) | (FACILITY_VISUALCPP<<16) | err)

//
// Hook pointers
//

// The "notify hook" gets called for every call to the
// delay load helper.  This allows a user to hook every call and
// skip the delay load helper entirely.
//
// dliNotify == {
//  dliStartProcessing |
//  dliNotePreLoadLibrary  |
//  dliNotePreGetProc |
//  dliNoteEndProcessing}
//  on this call.
//
// Prior to Visual Studio 2015 Update 3, these hooks were non-const.  They were
// made const to improve security (global, writable function pointers are bad).
// If for backwards compatibility you require the hooks to be writable, define
// the macro DELAYIMP_INSECURE_WRITABLE_HOOKS prior to including this header and
// provide your own non-const definition of the hooks.
ExternC
#ifndef DELAYIMP_INSECURE_WRITABLE_HOOKS
const
#endif
PfnDliHook   __pfnDliNotifyHook2 = NULL;

// This is the failure hook, dliNotify = {dliFailLoadLib|dliFailGetProc}
ExternC
#ifndef DELAYIMP_INSECURE_WRITABLE_HOOKS
const
#endif
PfnDliHook   __pfnDliFailureHook2 = NULL;


#define DLOAD_UNLOAD 1
#include "dloadsup.h"

//
// Local copies of strlen, memcmp, and memcpy to make sure we do not need the CRT
//

static inline size_t
__strlen(const char * sz) {
    const char *szEnd = sz;

    while( *szEnd++ ) {
        ;
        }

    return szEnd - sz - 1;
    }

static inline int
__memcmp(const void * pv1, const void * pv2, size_t cb) {
    if (!cb) {
        return 0;
        }

    while ( --cb && *(char *)pv1 == *(char *)pv2 ) {
        pv1 = (char *)pv1 + 1;
        pv2 = (char *)pv2 + 1;
        }

    return  *((unsigned char *)pv1) - *((unsigned char *)pv2);
    }

static inline void *
__memcpy(void * pvDst, const void * pvSrc, size_t cb) {

    void * pvRet = pvDst;

    //
    // copy from lower addresses to higher addresses
    //
    while (cb--) {
        *(char *)pvDst = *(char *)pvSrc;
        pvDst = (char *)pvDst + 1;
        pvSrc = (char *)pvSrc + 1;
        }

    return pvRet;
    }


// utility function for calculating the index of the current import
// for all the tables (INT, BIAT, UIAT, and IAT).
inline unsigned
IndexFromPImgThunkData(PCImgThunkData pitdCur, PCImgThunkData pitdBase) {
    return (unsigned) (pitdCur - pitdBase);
    }

// C++ template utility function for converting RVAs to pointers
//
extern "C"
const IMAGE_DOS_HEADER __ImageBase;

template <class X>
X PFromRva(RVA rva) {
    return X(PBYTE(&__ImageBase) + rva);
    }

// utility function for calculating the count of imports given the base
// of the IAT.  NB: this only works on a valid IAT!
inline unsigned
CountOfImports(PCImgThunkData pitdBase) {
    unsigned        cRet = 0;
    PCImgThunkData  pitd = pitdBase;
    while (pitd->u1.Function) {
        pitd++;
        cRet++;
        }
    return cRet;
    }

// For our own internal use, we convert to the old
// format for convenience.
//
struct InternalImgDelayDescr {
    DWORD           grAttrs;        // attributes
    LPCSTR          szName;         // pointer to dll name
    HMODULE *       phmod;          // address of module handle
    PImgThunkData   pIAT;           // address of the IAT
    PCImgThunkData  pINT;           // address of the INT
    PCImgThunkData  pBoundIAT;      // address of the optional bound IAT
    PCImgThunkData  pUnloadIAT;     // address of optional copy of original IAT
    DWORD           dwTimeStamp;    // 0 if not bound,
                                    // O.W. date/time stamp of DLL bound to (Old BIND)
    };

typedef InternalImgDelayDescr *         PIIDD;
typedef const InternalImgDelayDescr *   PCIIDD;

static inline
PIMAGE_NT_HEADERS WINAPI
PinhFromImageBase(HMODULE hmod) {
    return PIMAGE_NT_HEADERS(PBYTE(hmod) + PIMAGE_DOS_HEADER(hmod)->e_lfanew);
    }

static inline
void WINAPI
OverlayIAT(PImgThunkData pitdDst, PCImgThunkData pitdSrc) {
    __memcpy(pitdDst, pitdSrc, CountOfImports(pitdDst) * sizeof IMAGE_THUNK_DATA);
    }

static inline
DWORD WINAPI
TimeStampOfImage(PIMAGE_NT_HEADERS pinh) {
    return pinh->FileHeader.TimeDateStamp;
    }

static inline
bool WINAPI
FLoadedAtPreferredAddress(PIMAGE_NT_HEADERS pinh, HMODULE hmod) {
    return UINT_PTR(hmod) == pinh->OptionalHeader.ImageBase;
    }

static
PCImgDelayDescr
PiddFromDllName(LPCSTR szDll) {
    PCImgDelayDescr     piddRet = NULL;
    PIMAGE_NT_HEADERS   pinh = PinhFromImageBase(HMODULE(&__ImageBase));

    // Scan the Delay load IAT/INT for the dll in question
    //
    if (pinh->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT].Size != 0) {
        PCImgDelayDescr pidd = PFromRva<PCImgDelayDescr>(
            pinh->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT].VirtualAddress
            );

        // Check all of the dlls listed up to the NULL one.
        //
        while (pidd->rvaDLLName != 0) {
            // Check to see if it is the DLL we want
            // Intentionally case sensitive to avoid complication of using the CRT
            // for those that don't use the CRT...the user can replace this with
            // a variant of a case insensitive comparison routine.
            //
            LPCSTR  szDllCur = PFromRva<LPCSTR>(pidd->rvaDLLName);
            size_t  cchDllCur = __strlen(szDllCur);
            if (cchDllCur == __strlen(szDll) && __memcmp(szDll, szDllCur, cchDllCur) == 0) {
                piddRet = pidd;
                break;
                }

            pidd++;
            }
        }
    return piddRet;
    }

// Do the InterlockedExchange magic
//
#ifdef  _M_IX86

#undef  InterlockedExchangePointer
#define InterlockedExchangePointer(Target, Value) \
    (PVOID)(LONG_PTR)InterlockedExchange((PLONG)(Target), (LONG)(LONG_PTR)(Value))

typedef unsigned long *PULONG_PTR;

#endif

extern "C"
FARPROC WINAPI
__delayLoadHelper2(
    PCImgDelayDescr     pidd,
    FARPROC *           ppfnIATEntry
    ) {

    DloadAcquireSectionWriteAccess();

    // Set up some data we use for the hook procs but also useful for
    // our own use
    //
    InternalImgDelayDescr   idd = {
        pidd->grAttrs,
        PFromRva<LPCSTR>(pidd->rvaDLLName),
        PFromRva<HMODULE*>(pidd->rvaHmod),
        PFromRva<PImgThunkData>(pidd->rvaIAT),
        PFromRva<PCImgThunkData>(pidd->rvaINT),
        PFromRva<PCImgThunkData>(pidd->rvaBoundIAT),
        PFromRva<PCImgThunkData>(pidd->rvaUnloadIAT),
        pidd->dwTimeStamp
        };

    DelayLoadInfo   dli = {
        sizeof DelayLoadInfo,
        pidd,
        ppfnIATEntry,
        idd.szName,
        { 0 },
        0,
        0,
        0
        };

    if (0 == (idd.grAttrs & dlattrRva)) {
        PDelayLoadInfo  rgpdli[1] = { &dli };

        DloadReleaseSectionWriteAccess();

        RaiseException(
            VcppException(ERROR_SEVERITY_ERROR, ERROR_INVALID_PARAMETER),
            0,
            1,
            PULONG_PTR(rgpdli)
            );
        return 0;
        }

    HMODULE hmod = *idd.phmod;

    // Calculate the index for the IAT entry in the import address table
    // N.B. The INT entries are ordered the same as the IAT entries so
    // the calculation can be done on the IAT side.
    //
    const unsigned  iIAT = IndexFromPImgThunkData(PCImgThunkData(ppfnIATEntry), idd.pIAT);
    const unsigned  iINT = iIAT;

    PCImgThunkData  pitd = &(idd.pINT[iINT]);

    dli.dlp.fImportByName = !IMAGE_SNAP_BY_ORDINAL(pitd->u1.Ordinal);

    if (dli.dlp.fImportByName) {
        dli.dlp.szProcName = LPCSTR(PFromRva<PIMAGE_IMPORT_BY_NAME>(RVA(UINT_PTR(pitd->u1.AddressOfData)))->Name);
        }
    else {
        dli.dlp.dwOrdinal = DWORD(IMAGE_ORDINAL(pitd->u1.Ordinal));
        }

    // Call the initial hook.  If it exists and returns a function pointer,
    // abort the rest of the processing and just return it for the call.
    //
    FARPROC pfnRet = NULL;

    if (__pfnDliNotifyHook2) {
        pfnRet = ((*__pfnDliNotifyHook2)(dliStartProcessing, &dli));

        if (pfnRet != NULL) {
            goto HookBypass;
            }
        }

    // Check to see if we need to try to load the library.
    //
    if (hmod == 0) {
        if (__pfnDliNotifyHook2) {
            hmod = HMODULE(((*__pfnDliNotifyHook2)(dliNotePreLoadLibrary, &dli)));
            }
        if (hmod == 0) {
            hmod = ::LoadLibraryEx(dli.szDll, NULL, 0);
            }
        if (hmod == 0) {
            dli.dwLastError = ::GetLastError();
            if (__pfnDliFailureHook2) {
                // when the hook is called on LoadLibrary failure, it will
                // return 0 for failure and an hmod for the lib if it fixed
                // the problem.
                //
                hmod = HMODULE((*__pfnDliFailureHook2)(dliFailLoadLib, &dli));
                }

            if (hmod == 0) {
                PDelayLoadInfo  rgpdli[1] = { &dli };

                DloadReleaseSectionWriteAccess();
                RaiseException(
                    VcppException(ERROR_SEVERITY_ERROR, ERROR_MOD_NOT_FOUND),
                    0,
                    1,
                    PULONG_PTR(rgpdli)
                    );

                // If we get to here, we blindly assume that the handler of the exception
                // has magically fixed everything up and left the function pointer in
                // dli.pfnCur.
                //
                return dli.pfnCur;
                }
            }

        // Store the library handle.  If it is already there, we infer
        // that another thread got there first, and we need to do a
        // FreeLibrary() to reduce the refcount
        //
        HMODULE hmodT = HMODULE(InterlockedExchangePointer((PVOID *) idd.phmod, PVOID(hmod)));
        if (hmodT == hmod) {
            ::FreeLibrary(hmod);
            }
        }

    // Go for the procedure now.
    //
    dli.hmodCur = hmod;
    if (__pfnDliNotifyHook2) {
        pfnRet = (*__pfnDliNotifyHook2)(dliNotePreGetProcAddress, &dli);
        }
    if (pfnRet == 0) {
        if (pidd->rvaBoundIAT && pidd->dwTimeStamp) {
            // bound imports exist...check the timestamp from the target image
            //
            PIMAGE_NT_HEADERS   pinh(PinhFromImageBase(hmod));

            if (pinh->Signature == IMAGE_NT_SIGNATURE &&
                TimeStampOfImage(pinh) == idd.dwTimeStamp &&
                FLoadedAtPreferredAddress(pinh, hmod)) {

                // Everything is good to go, if we have a decent address
                // in the bound IAT!
                //
                pfnRet = FARPROC(UINT_PTR(idd.pBoundIAT[iIAT].u1.Function));
                if (pfnRet != 0) {
                    goto SetEntryHookBypass;
                    }
                }
            }

        pfnRet = ::GetProcAddress(hmod, dli.dlp.szProcName);
        }

    if (pfnRet == 0) {
        dli.dwLastError = ::GetLastError();
        if (__pfnDliFailureHook2) {
            // when the hook is called on GetProcAddress failure, it will
            // return 0 on failure and a valid proc address on success
            //
            pfnRet = (*__pfnDliFailureHook2)(dliFailGetProc, &dli);
            }
        if (pfnRet == 0) {
            PDelayLoadInfo  rgpdli[1] = { &dli };

            DloadReleaseSectionWriteAccess();

            RaiseException(
                VcppException(ERROR_SEVERITY_ERROR, ERROR_PROC_NOT_FOUND),
                0,
                1,
                PULONG_PTR(rgpdli)
                );

            DloadAcquireSectionWriteAccess();

            // If we get to here, we blindly assume that the handler of the exception
            // has magically fixed everything up and left the function pointer in
            // dli.pfnCur.
            //
            pfnRet = dli.pfnCur;
            }
        }

SetEntryHookBypass:
    *ppfnIATEntry = pfnRet;

HookBypass:
    if (__pfnDliNotifyHook2) {
        dli.dwLastError = 0;
        dli.hmodCur = hmod;
        dli.pfnCur = pfnRet;
        (*__pfnDliNotifyHook2)(dliNoteEndProcessing, &dli);
        }

    DloadReleaseSectionWriteAccess();

    return pfnRet;
    }

extern "C"
BOOL WINAPI
__FUnloadDelayLoadedDLL2(LPCSTR szDll) {
    BOOL        fRet = FALSE;
    PCImgDelayDescr pidd = PiddFromDllName(szDll);

    if ((pidd != NULL) &&
        (pidd->rvaUnloadIAT != 0)) {
        HMODULE *           phmod = PFromRva<HMODULE*>(pidd->rvaHmod);
        HMODULE             hmod = *phmod;
        if (hmod != NULL) {

            DloadAcquireSectionWriteAccess();

            OverlayIAT(
                PFromRva<PImgThunkData>(pidd->rvaIAT),
                PFromRva<PCImgThunkData>(pidd->rvaUnloadIAT)
                );
            ::FreeLibrary(hmod);
            *phmod = NULL;

            DloadReleaseSectionWriteAccess();

            fRet = TRUE;
            }

        }
    return fRet;
    }

extern "C"
HRESULT WINAPI
__HrLoadAllImportsForDll(LPCSTR szDll) {
    HRESULT             hrRet = HRESULT_FROM_WIN32(ERROR_MOD_NOT_FOUND);
    PCImgDelayDescr     pidd = PiddFromDllName(szDll);

    if (pidd != NULL) {
        // Found a matching DLL name, now process it.
        //
        // Set up the internal structure
        //
        FARPROC *   ppfnIATEntry = PFromRva<FARPROC*>(pidd->rvaIAT);
        size_t      cpfnIATEntries = CountOfImports(PCImgThunkData(ppfnIATEntry));
        FARPROC *   ppfnIATEntryMax = ppfnIATEntry + cpfnIATEntries;

        for (;ppfnIATEntry < ppfnIATEntryMax; ppfnIATEntry++) {
            __delayLoadHelper2(pidd, ppfnIATEntry);
            }

        // Done, indicate some semblance of success
        //
        hrRet = S_OK;
        }
    return hrRet;
    }
