use super::script;

com::interfaces! {
    #[uuid("626FC520-A41E-11cf-A731-00A0C9082637")]
    pub unsafe interface IHtmlDocument : $IDispatch {
        pub fn get_Script(&self, out: *mut u32) -> com::sys::HRESULT;
    }

    #[uuid("332c4425-26cb-11d0-b483-00c04fd90119")]
    pub unsafe interface IHtmlDocument2 : IHtmlDocument {
        pub fn get_All(&self) -> com::sys::HRESULT;
        pub fn get_Body(&self) -> com::sys::HRESULT;
        pub fn get_ActiveElement(&self) -> com::sys::HRESULT;
        pub fn get_Images(&self) -> com::sys::HRESULT;
        pub fn get_Applets(&self) -> com::sys::HRESULT;
        pub fn get_Links(&self) -> com::sys::HRESULT;
        pub fn get_Forms(&self) -> com::sys::HRESULT;
        pub fn get_Anchors(&self) -> com::sys::HRESULT;
        pub fn set_Title(&self) -> com::sys::HRESULT;
        pub fn get_Title(&self) -> com::sys::HRESULT;
        pub fn get_Scripts(&self) -> com::sys::HRESULT;
        pub fn set_DesignMode(&self) -> com::sys::HRESULT;
        pub fn get_DesignMode(&self) -> com::sys::HRESULT;
        pub fn get_Selection(&self) -> com::sys::HRESULT;
        pub fn get_ReadyState(&self) -> com::sys::HRESULT;
        pub fn get_Frames(&self) -> com::sys::HRESULT;
        pub fn get_Embeds(&self) -> com::sys::HRESULT;
        pub fn get_Plugins(&self) -> com::sys::HRESULT;
        pub fn set_ALinkColor(&self) -> com::sys::HRESULT;
        pub fn get_ALinkColor(&self) -> com::sys::HRESULT;
        pub fn set_BgColor(&self) -> com::sys::HRESULT;
        pub fn get_BgColor(&self) -> com::sys::HRESULT;
        pub fn set_FgColor(&self) -> com::sys::HRESULT;
        pub fn get_FgColor(&self) -> com::sys::HRESULT;
        pub fn set_LinkColor(&self) -> com::sys::HRESULT;
        pub fn get_LinkColor(&self) -> com::sys::HRESULT;
        pub fn set_VLinkColor(&self) -> com::sys::HRESULT;
        pub fn get_VLinkColor(&self) -> com::sys::HRESULT;
        pub fn get_Referrer(&self) -> com::sys::HRESULT;
        pub fn get_Location(&self) -> com::sys::HRESULT;
        pub fn get_LastModified(&self) -> com::sys::HRESULT;
        pub fn set_Url(&self) -> com::sys::HRESULT;
        pub fn get_Url(&self) -> com::sys::HRESULT;
        pub fn set_Domain(&self) -> com::sys::HRESULT;
        pub fn get_Domain(&self) -> com::sys::HRESULT;
        pub fn set_Cookie(&self) -> com::sys::HRESULT;
        pub fn get_Cookie(&self) -> com::sys::HRESULT;
        pub fn set_Expando(&self) -> com::sys::HRESULT;
        pub fn get_Expando(&self) -> com::sys::HRESULT;
        pub fn set_Charset(&self) -> com::sys::HRESULT;
        pub fn get_Charset(&self) -> com::sys::HRESULT;
        pub fn set_DefaultCharset(&self) -> com::sys::HRESULT;
        pub fn get_DefaultCharset(&self) -> com::sys::HRESULT;
        pub fn get_MimeType(&self) -> com::sys::HRESULT;
        pub fn get_FileSize(&self) -> com::sys::HRESULT;
        pub fn get_FileCreatedDate(&self) -> com::sys::HRESULT;
        pub fn get_FileModifiedDate(&self) -> com::sys::HRESULT;
        pub fn get_FileUpdatedDate(&self) -> com::sys::HRESULT;
        pub fn get_Security(&self) -> com::sys::HRESULT;
        pub fn get_Protocol(&self) -> com::sys::HRESULT;
        pub fn get_NameProp(&self) -> com::sys::HRESULT;
        pub fn Write(&self) -> com::sys::HRESULT;
        pub fn WriteLn(&self) -> com::sys::HRESULT;
        pub fn Open(&self) -> com::sys::HRESULT;
        pub fn Close(&self) -> com::sys::HRESULT;
        pub fn Clear(&self) -> com::sys::HRESULT;
        pub fn QueryCommandSupported(&self) -> com::sys::HRESULT;
        pub fn QueryCommandEnabled(&self) -> com::sys::HRESULT;
        pub fn QueryCommandState(&self) -> com::sys::HRESULT;
        pub fn QueryCommandIndeterm(&self) -> com::sys::HRESULT;
        pub fn QueryCommandText(&self) -> com::sys::HRESULT;
        pub fn QueryCommandValue(&self) -> com::sys::HRESULT;
        pub fn ExecCommand(&self) -> com::sys::HRESULT;
        pub fn ExecCommandShowHelp(&self) -> com::sys::HRESULT;
        pub fn CreateElement(&self) -> com::sys::HRESULT;
        pub fn set_OnHelp(&self) -> com::sys::HRESULT;
        pub fn get_OnHelp(&self) -> com::sys::HRESULT;
        pub fn set_OnClick(&self) -> com::sys::HRESULT;
        pub fn get_OnClick(&self) -> com::sys::HRESULT;
        pub fn set_OnDblClick(&self) -> com::sys::HRESULT;
        pub fn get_OnDblClick(&self) -> com::sys::HRESULT;
        pub fn set_OnKeyUp(&self) -> com::sys::HRESULT;
        pub fn get_OnKeyUp(&self) -> com::sys::HRESULT;
        pub fn set_OnKeyDown(&self) -> com::sys::HRESULT;
        pub fn get_OnKeyDown(&self) -> com::sys::HRESULT;
        pub fn set_OnKeyPress(&self) -> com::sys::HRESULT;
        pub fn get_OnKeyPress(&self) -> com::sys::HRESULT;
        pub fn get_OnMouseUp(&self) -> com::sys::HRESULT;
        pub fn set_OnMouseUp(&self) -> com::sys::HRESULT;
        pub fn get_OnMouseDown(&self) -> com::sys::HRESULT;
        pub fn set_OnMouseDown(&self) -> com::sys::HRESULT;
        pub fn get_OnMouseMove(&self) -> com::sys::HRESULT;
        pub fn set_OnMouseMove(&self) -> com::sys::HRESULT;
        pub fn get_OnMouseOut(&self) -> com::sys::HRESULT;
        pub fn set_OnMouseOut(&self) -> com::sys::HRESULT;
        pub fn get_OnMouseOver(&self) -> com::sys::HRESULT;
        pub fn set_OnMouseOver(&self) -> com::sys::HRESULT;
        pub fn get_OnReadyStateChange(&self) -> com::sys::HRESULT;
        pub fn set_OnReadyStateChange(&self) -> com::sys::HRESULT;
        pub fn get_OnAfterUpdate(&self) -> com::sys::HRESULT;
        pub fn set_OnAfterUpdate(&self) -> com::sys::HRESULT;
        pub fn get_OnRowExit(&self) -> com::sys::HRESULT;
        pub fn set_OnRowExit(&self) -> com::sys::HRESULT;
        pub fn get_OnRowEnter(&self) -> com::sys::HRESULT;
        pub fn set_OnRowEnter(&self) -> com::sys::HRESULT;
        pub fn get_OnDragStart(&self) -> com::sys::HRESULT;
        pub fn set_OnDragStart(&self) -> com::sys::HRESULT;
        pub fn get_OnSelectStart(&self) -> com::sys::HRESULT;
        pub fn set_OnSelectStart(&self) -> com::sys::HRESULT;
        pub fn ElementFromPoint(&self) -> com::sys::HRESULT;
        pub fn get_ParentWindow(&self) -> com::sys::HRESULT;
        pub fn get_StyleSheets(&self) -> com::sys::HRESULT;
        pub fn set_OnBeforeUpdate(&self) -> com::sys::HRESULT;
        pub fn get_OnBeforeUpdate(&self) -> com::sys::HRESULT;
        pub fn set_OnErrorUpdate(&self) -> com::sys::HRESULT;
        pub fn get_OnErrorUpdate(&self) -> com::sys::HRESULT;
        pub fn ToString(&self) -> com::sys::HRESULT;
        pub fn CreateStyleSheet(&self) -> com::sys::HRESULT;
    }
}

com::class! {
    pub class HtmlDocument
        : IHtmlDocument2(IHtmlDocument($IDispatch))
    {}

    impl IHtmlDocument for HtmlDocument {
        pub fn get_Script(&self, out: *mut u32) -> com::sys::HRESULT {
            let script = script::Script::allocate();
            let idispatch = script.query_interface::<com::interfaces::IDispatch>().unwrap();
            std::mem::forget(idispatch.clone());
            unsafe {
                *out = std::mem::transmute(idispatch);
            }
            com::sys::S_OK
        }
    }

    impl IHtmlDocument2 for HtmlDocument {
        pub fn get_All(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Body(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_ActiveElement(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Images(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Applets(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Links(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Forms(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Anchors(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_Title(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Title(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Scripts(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_DesignMode(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_DesignMode(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Selection(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_ReadyState(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Frames(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Embeds(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Plugins(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_ALinkColor(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_ALinkColor(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_BgColor(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_BgColor(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_FgColor(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_FgColor(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_LinkColor(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_LinkColor(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_VLinkColor(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_VLinkColor(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Referrer(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Location(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_LastModified(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_Url(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Url(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_Domain(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Domain(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_Cookie(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Cookie(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_Expando(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Expando(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_Charset(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Charset(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_DefaultCharset(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_DefaultCharset(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_MimeType(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_FileSize(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_FileCreatedDate(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_FileModifiedDate(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_FileUpdatedDate(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Security(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_Protocol(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_NameProp(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn Write(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn WriteLn(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn Open(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn Close(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn Clear(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn QueryCommandSupported(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn QueryCommandEnabled(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn QueryCommandState(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn QueryCommandIndeterm(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn QueryCommandText(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn QueryCommandValue(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn ExecCommand(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn ExecCommandShowHelp(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn CreateElement(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnHelp(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnHelp(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnClick(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnClick(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnDblClick(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnDblClick(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnKeyUp(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnKeyUp(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnKeyDown(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnKeyDown(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnKeyPress(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnKeyPress(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnMouseUp(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnMouseUp(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnMouseDown(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnMouseDown(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnMouseMove(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnMouseMove(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnMouseOut(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnMouseOut(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnMouseOver(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnMouseOver(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnReadyStateChange(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnReadyStateChange(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnAfterUpdate(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnAfterUpdate(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnRowExit(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnRowExit(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnRowEnter(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnRowEnter(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnDragStart(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnDragStart(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnSelectStart(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnSelectStart(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn ElementFromPoint(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_ParentWindow(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_StyleSheets(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnBeforeUpdate(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnBeforeUpdate(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn set_OnErrorUpdate(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn get_OnErrorUpdate(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn ToString(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        pub fn CreateStyleSheet(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
    }
}

