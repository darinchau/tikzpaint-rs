//! ================================================================================== //
//! ============================= Main implementation ================================ //
//! ================================================================================== //
//!
//! This component is responsible for the main operations of the wrapper app around tikzpaint. This calculates the svg,
//! and handles all clicks and translate them into actions on the figure.

use gloo::console::log;
use stylist::Style;
use stylist::css;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::figures::*;
use crate::app::*;
use crate::renderer::*;
use std::cell::RefCell;
use std::rc::Rc;

// ================================================================================================================
// =============================== Implement Canvas manager and props =============================================
// ================================================================================================================

/// Dictates the height of the header and the maximum width of the side bar
#[derive(PartialEq, Properties)]
pub struct CanvasManagerProps {
    pub header_height: usize,
    pub side_bar_width: usize,
    pub terminal_height: usize,
    pub figure_dims: usize,
    pub debug: Option<bool>
}

/// Gets css properties of main canvas
fn get_css(props: &CanvasManagerProps) -> String {
    let debug_mode = is_true(props.debug);
    let h = props.header_height;
    let th = props.terminal_height;
    let w = props.side_bar_width;

    let button_css = format!(r#"
    {{
        bottom: {th}px;
        right: 0;
        width: calc(100% - {w}px);
        height: calc(100% - {h}px - {th}px);
    }}"#);

    let canvas_css = format!(r#"
    {{
        top: {h}px;
        left: {w}px;
    }}
    "#);

    let main_canvas_pos = Style::new(format!("& button {button_css} & canvas {canvas_css}"))
        .unwrap_or_else(|_| {
            log!("Failed to load main canvas position style");
            Style::new("").unwrap()
        });

    if debug_mode {
        format!("main-canvas-debug {}", main_canvas_pos.get_class_name())
    }
    else {
        format!("main-canvas {}", main_canvas_pos.get_class_name())
    }
}

pub enum CanvasManagerMessage {
    /// Means something changed in the figure. In this case we only need to load the newly drawn objects
    ChangedFigure,

    /// Means we probably have to redraw everything since the dimensions are different
    ChangedWindowSize,
}

macro_rules! mborrow {
    ($x: ident) => {
        (*(*$x.clone()).borrow_mut())
    };
}

macro_rules! rerender {
    ($x: ident) => {
        ctx.link().send_message($x)
    };
}

/// The main app is a coordinator component that coordinates all three main components
/// i.e. the header bar, the side bar, and the canvas
pub struct CanvasManager {
    fig: Rc<RefCell<FigureComplex>>,
    tf: Rc<RefCell<Transform>>,
    csh: CanvasStateHandle,
}

impl CanvasManager {
    fn get_canvas_sensor_cb(&self, props: &CanvasManagerProps, ctx: &Context<Self>) -> Callback<CanvasSensorEvent> {
        let f = self.fig.clone();
        let tf = self.tf.clone();
        let link = ctx.link().clone();

        // Handles main canvas sensor events
        let canvas_sensor_cb = Callback::from(move |event: CanvasSensorEvent| {
            // Suppose we need to spawn a point. We need do perform the following:
            // 1. Get the coordinates of the click. Transform that into the canvas coordinates
            // 2. Spawn a point at the canvas coordinates
            // 3. Pass the figure to the renderer and perform the rendering of the svg
            match event.mouse_click_event.click_type {
                MouseClickType::LeftClick => {
                    let (x, y) = event.mouse_click_event.screen_pos;
                    log!(format!("Recieved mouse click at ({x}, {y})"));


                    let (local_x, local_y) = deref_get(tf.clone()).world_to_local(x, y);

                    let p = Point::new(Coordinates::new(vec![local_x, local_y]));
                    let repr = p.into_str();
                    let pt = FigureObjectComplex::new(p.wrap(), repr);
                    mborrow!(f).draw(pt);

                    log!(format!("Drawing a point at ({local_x}, {local_y})"));

                    link.send_message(CanvasManagerMessage::ChangedFigure);
                },

                _ => ()
            }
        });

        return canvas_sensor_cb;
    }

    fn get_header_cb(&self, props: &CanvasManagerProps, ctx: &Context<Self>) -> Callback<HeaderBarEvent> {
        // Handles header bar events
        let header_cb = Callback::from(move |event: HeaderBarEvent| {

        });

        return header_cb;
    }

    fn get_sidebar_cb(&self, props: &CanvasManagerProps, ctx: &Context<Self>) -> Callback<SideBarEvent> {
        let sidebar_cb = Callback::from(move |event: SideBarEvent| {

        });

        return sidebar_cb;
    }

    fn get_terminal_cb(&self, props: &CanvasManagerProps, ctx: &Context<Self>) -> Callback<TerminalEvent> {
        let terminal_cb = Callback::from(move |event: TerminalEvent| {

        });

        return terminal_cb;
    }

    fn get_resize_cb(&self, props: &CanvasManagerProps, ctx: &Context<Self>) -> Callback<WindowResizeEvent> {
        let debug_mode = is_true(props.debug);

        let tf = self.tf.clone();
        let link = ctx.link().clone();

        let resize_cb = Callback::from(move |event: WindowResizeEvent| {
            if debug_mode {
                log!(format!("Windows resized to ({}, {})", event.new_size.x, event.new_size.y));
            }
            let (x, y) = (event.new_size.x, event.new_size.y);
            mborrow!(tf).set_screen_size(x, y);

            // Trigger rerender
            link.send_message(CanvasManagerMessage::ChangedWindowSize);
        });

        return resize_cb;
    }
}

impl Component for CanvasManager {
    type Message = CanvasManagerMessage;
    type Properties = CanvasManagerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        let debug_mode = is_true(props.debug);

        // Dimensions of the page
        let h = props.header_height;
        let w = props.side_bar_width;
        let th = props.terminal_height;

        // Process figure and callbacks
        let dims = props.figure_dims;
        let fig_state = FigureComplex::new(dims);

        // We need to keep track of the world coordinates and figure coordinates conversion.
        // so we basically need to keep track of the transforms of this world. We need to keep track of
        // the position of (0, 0), (0, 1) and (1, 0)

        let (x, y) = get_size().unwrap_or_else(|x| {
            log!(format!("Cannot get window size value. Reason: {x}"));
            ASSUMPTION
        });

        // Load the transform - i.e. basis axis. If we can get the sensor element then use that as reference
        // Otherwise we fall back to calculating the midpoint using the window size
        // Make a blanket initialization first due to where use_state can be called
        let mut tf = Transform::new(h, w, th);
        tf.set_screen_size(x, y);

        let t_ptr = Rc::new(RefCell::new(tf));

        CanvasManager {
            fig: Rc::new(RefCell::new(fig_state)),
            tf: t_ptr.clone(),
            csh: CanvasStateHandle::new(t_ptr.clone())
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let fig = &*self.fig.borrow_mut();

        match msg {
            CanvasManagerMessage::ChangedFigure => {fig.render(self.csh.clone());}
            CanvasManagerMessage::ChangedWindowSize => {fig.rerender(self.csh.clone());}
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let debug_mode = is_true(props.debug);

        // Dimensions of the page
        let h = props.header_height;
        let w = props.side_bar_width;
        let th = props.terminal_height;

        // Get all callbacks
        let canvas_sensor_cb = self.get_canvas_sensor_cb(props, ctx);
        let header_cb = self.get_header_cb(props, ctx);
        let sidebar_cb = self.get_sidebar_cb(props, ctx);
        let terminal_cb = self.get_terminal_cb(props, ctx);
        let resize_cb = self.get_resize_cb(props, ctx);

        // Process CSS
        let class_id = get_css(props);

        // Make copies of stuff to pass into html
        let fg = &*(*self.fig).borrow();
        let terminal_text = fg.unpack_html();

        let other_t = *(*self.tf.clone()).borrow();

        let tf = *(*self.tf.clone()).borrow();

        let csh = self.csh.clone();

        html!{
            <>
                <HeaderBar id={"header-bar"} height={h} cb={header_cb}/>
                <SideBar id={"side-bar"} header_height={h} width={w} cb={sidebar_cb}/>
                <Terminal id={"terminal"} height={th} text_box_height={37} sidebar_width={w} cb={terminal_cb}>
                    {terminal_text}
                </Terminal>
                <WindowResizeListener cb={resize_cb}/>
                <div class={class_id}>
                    <CanvasSensor id={"canvas-sensor"} top={h} left={w} cb={canvas_sensor_cb}/>
                    <CanvasRenderer id={"canvas-renderer"} tf={tf} canvas={csh}/>
                </div>
            </>
        }
    }
}
