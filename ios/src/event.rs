use winit::event_loop::EventLoopProxy;
use objc::{
    declare::ClassDecl,
    runtime::{
        Object,
        Class,
        Sel,
    },
};

use uikit_sys::id;



#[derive(PartialEq, Clone, Debug)]
pub struct WidgetEvent {
    pub widget_id: u64,
    pub id: usize,
}

#[derive(Debug)]
pub struct EventHandler {
    pub id: id,
    pub widget_id: u64,
}

static mut PROXY : Option<EventLoopProxy<WidgetEvent>> = None;
static mut COUNTER: Option<u64> = None;
impl EventHandler {
    pub fn init(proxy: EventLoopProxy<WidgetEvent>) {
        unsafe {
            COUNTER = Some(0);
            PROXY = Some(proxy);
        }
    }
    pub fn new(objc_id: id) -> Self
    {
        let mut widget_id = 0;
        // TODO: Figure out how to make this unsafe block much smaller.
        let obj = unsafe {
            let obj: id = objc::msg_send![Self::class(), alloc];
            let obj: id = objc::msg_send![obj, init];

            if let Some(mut counter) = COUNTER {
                counter += 1;
                COUNTER = Some(counter);
                widget_id = counter;
                (*obj).set_ivar::<u64>("widget_id", widget_id);
            }
            (*obj).set_ivar::<id>("objc_id", objc_id);
            obj
        };
        trace!("NEW EVENTHANDLER WITH WIDGET ID :{:?}", widget_id);
        Self{id: obj, widget_id}
    }
    extern "C" fn event(this: &Object, _cmd: objc::runtime::Sel)
    {

        // TODO: Figure out how to make this unsafe block smaller.
        unsafe {
            if let Some(ref proxy) = PROXY {
                let widget_id = *this.get_ivar::<u64>("widget_id");
                let id = *this.get_ivar::<id>("objc_id");
                let _ = proxy.send_event(WidgetEvent { widget_id, id: id as usize} );
            }
        }
    }

    fn class() -> &'static Class
    {
        let cls_name = "RustEventHandler";
        match Class::get(cls_name) {
            Some(cls) => cls,
            None => {
                let superclass = objc::class!(NSObject);
                let mut decl = ClassDecl::new(cls_name, superclass).unwrap();
                unsafe {
                    decl.add_method(
                        objc::sel!(sendEvent),
                        Self::event as extern "C" fn(&Object, Sel),
                    );
                }
                decl.add_ivar::<u64>("widget_id");
                decl.add_ivar::<id>("objc_id");
                decl.register()
            }
        }
    }
}