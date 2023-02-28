//! ================================================================================== //
//! ============================= Main implementation ================================ //
//! ================================================================================== //
//!
//! This component is responsible for the main operations of the wrapper app around tikzpaint. This calculates the svg,
//! and handles all clicks and translate them into actions on the figure.

use gloo::console::log;
use gloo::utils::head;
use stylist::Style;
use stylist::css;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::figures::*;
use crate::app::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Clone, Copy)]
struct Transform {
    /// Screen size according to inner_width and inner_height
    screen_size: (i32, i32),

    /// scale x is the number such that 1 unit in the coordinate system horizontally = scale_x pixels
    scale_x: f64,

    /// scale y is the number such that 1 unit in the coordinate system vertically = scale_y pixels
    scale_y: f64,

    /// Where the origin ought to be in world coordinates. Coordinates can be very big or negative - means the origin is out of sight
    origin: (i32, i32),

    margins: (i32, i32, i32, i32),

    initialized: bool,
}

impl Transform {
    /// Abuse of notation
    pub fn new(header_height: usize, sidebar_width: usize, terminal_height: usize) -> Self {
        let h = header_height as i32;
        let w = sidebar_width as i32;
        let th = terminal_height as i32;

        Self {
            screen_size: (1920, 1080),
            scale_x: 30.,
            scale_y: 30.,
            origin: (0, 0),
            margins: (h, 0, th, w),
            initialized: false,
        }
    }

    pub fn get_self_size(&self) -> (i32, i32) {
        let (x, y) = self.screen_size;
        let (top, right, bottom, left) = self.margins;
        let w = (x - left - right);
        let h = (y - right - top);
        (w, h)
    }

    /// Sets the origin right at the middle of the transform
    pub fn reset_origin(&mut self) {
        let (top, right, bottom, left) = self.margins;
        let (x, y) = self.get_self_size();
        let ox = left + x/2;
        let oy = top + y/2;
        self.origin = (ox, oy);
    }

    pub fn set_screen_size(&mut self, x: i32, y: i32) {
        self.screen_size = (x, y);
        if !self.initialized {
            self.initialized = true;
            self.reset_origin();
        }
    }

    pub fn set_margin_top(&mut self, m: i32) {
        self.margins.0 = m;
    }

    pub fn set_margin_right(&mut self, m: i32) {
        self.margins.1 = m;
    }

    pub fn set_margin_down(&mut self, m: i32) {
        self.margins.2 = m;
    }

    pub fn set_margin_left(&mut self, m: i32) {
        self.margins.3 = m;
    }

    pub fn set_scale_x(&mut self, m: f64) {
        self.scale_x = m;
    }

    pub fn set_scale_y(&mut self, m: f64) {
        self.scale_y = m;
    }

    /// Transforms screen_x and screen_y into local_coordinates
    pub fn world_to_local(&self, x: i32, y: i32) -> (f64, f64) {
        // Satisfies x = origin + a * scale_x;
        let a = (x - self.origin.0) as f64/self.scale_x;
        let b = (y - self.origin.1) as f64/self.scale_y;
        (a, b)
    }

    /// Transforms local x and y to screen_x, screen_y
    pub fn local_to_world(&self, a: f64, b: f64) -> (i32, i32) {
        let x = self.origin.0 as f64 + a * self.scale_x;
        let y = self.origin.1 as f64 + a * self.scale_y;

        (x.round() as i32, y.round() as i32)
    }

    /// Transforms local x and y to client coordinates (render coordinates)
    pub fn local_to_client(&self, a: f64, b: f64) -> (f64, f64) {
        let (top, right, bottom, left) = self.margins;
        let x = self.origin.0 as f64 + a * self.scale_x - top as f64;
        let y = self.origin.1 as f64 + a * self.scale_y - left as f64;

        (x, y)
    }
}

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
    let w = props.side_bar_width;
    let th = props.terminal_height;

    let button_css = format!(r#"
    {{
        bottom: {th}px;
        right: 0;
        width: calc(100% - {w}px);
        height: calc(100% - {h}px - {th}px);
    }}"#);

    let svg_css = button_css.clone();

    let main_canvas_pos = Style::new(format!("& button {} & svg {}", button_css, svg_css))
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

#[derive(Clone)]
struct Handles {
    fig: Rc<RefCell<Figure>>,
    transform: UseStateHandle<Transform>,
    text: UseStateHandle<TerminalText>
}

fn get_canvas_sensor_cb(props: &CanvasManagerProps, h: Handles, canvas_sensor_id: &'static str) -> Callback<CanvasSensorEvent> {
    // Handles main canvas sensor events
    let canvas_sensor_cb = Callback::from(move |event: CanvasSensorEvent| {
        let mut fig = h.fig.borrow_mut();

        // We need to spawn a point. Thus we need do perform the following:
        // 1. Get the coordinates of the click. Transform that into the canvas coordinates
        // 2. Spawn a point at the canvas coordinates
        // 3. Pass the figure to the renderer and perform the rendering of the svg

        let (x, y) = event.mouse_click_event.screen_pos;
        let (local_x, local_y) = h.transform.world_to_local(x, y);

        let p = Point::new(Coordinates::new(vec![local_x, local_y])).wrap();
        fig.draw(p);

        // Update the terminal - TODO
    });

    return canvas_sensor_cb;
}

fn get_header_cb(props: &CanvasManagerProps, h: Handles) -> Callback<HeaderBarEvent> {
    // Handles header bar events
    let header_cb = Callback::from(move |event: HeaderBarEvent| {
        let fig = h.fig.borrow_mut();
    });

    return header_cb;
}

fn get_sidebar_cb(props: &CanvasManagerProps, h: Handles) -> Callback<SideBarEvent> {
    let sidebar_cb = Callback::from(move |event: SideBarEvent| {
        let fig = h.fig.borrow_mut();
    });

    return sidebar_cb;
}

fn get_terminal_cb(props: &CanvasManagerProps, h: Handles) -> Callback<TerminalEvent> {
    let terminal_cb = Callback::from(move |event: TerminalEvent| {

    });

    return terminal_cb;
}

fn get_resize_cb(props: &CanvasManagerProps, h: Handles) -> Callback<WindowResizeEvent> {
    let resize_cb = Callback::from(move |event: WindowResizeEvent| {
        let (x, y) = (event.new_size.x, event.new_size.y);
        let mut orig = *h.transform;
        orig.set_screen_size(x, y);
        h.transform.set(orig);
    });

    return resize_cb;
}

/// The main app is a coordinator component that coordinates all three main components
/// i.e. the header bar, the side bar, and the canvas
#[function_component(CanvasManager)]
pub fn canvas_manager(props: &CanvasManagerProps) -> Html {
    let debug_mode = is_true(props.debug);

    // Dimensions of the page
    let h = props.header_height;
    let w = props.side_bar_width;
    let th = props.terminal_height;

    // Process figure and callbacks
    let dims = props.figure_dims;
    let fig = Figure::new(dims);
    let fig_state = use_mut_ref(|| fig);

    // The terminal text
    let terminal_text = use_state(|| TerminalText::new());

    // We need to keep track of the world coordinates and figure coordinates conversion.
    // so we basically need to keep track of the transforms of this world. We need to keep track of
    // the position of (0, 0), (0, 1) and (1, 0)

    // Pass a unique ID down to the mouse sensor and use get_element_by_ID
    let canvas_sensor_id = "canvas-sensor";

    // Load the transform - i.e. basis axis. If we can get the sensor element then use that as reference
    // Otherwise we fall back to calculating the midpoint using the window size
    // Make a blanket initialization first due to where use_state can be called
    let transform = use_state(|| Transform::new(h, w, th));

    let handles = Handles {
        fig: fig_state.clone(),
        transform: transform.clone(),
        text: terminal_text.clone()
    };


    // Get all callbacks
    let canvas_sensor_cb = get_canvas_sensor_cb(props, handles.clone(), canvas_sensor_id);
    let header_cb = get_header_cb(props, handles.clone());
    let sidebar_cb = get_sidebar_cb(props, handles.clone());
    let terminal_cb = get_terminal_cb(props, handles.clone());
    let resize_cb = get_resize_cb(props, handles.clone());

    // Process CSS
    let class_id = get_css(props);

    html!{
        <>
            <HeaderBar height={h} cb={header_cb}/>
            <SideBar header_height={h} width={w} cb={sidebar_cb}/>
            <Terminal height={th} text_box_height={37} sidebar_width={w} text={(*terminal_text).clone()} cb={terminal_cb}/>
            <WindowResizeListener cb={resize_cb}/>
            <div class={class_id}>
                <CanvasSensor top={h} left={w} cb={canvas_sensor_cb} id={canvas_sensor_id}/>
            </div>
        </>
    }
}