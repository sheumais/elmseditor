use std::collections::{HashMap, HashSet};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement, MouseEvent, WheelEvent};
use yew::prelude::*;
use regex::Regex;

mod marker;
mod zone;

use crate::{marker::{icon_number_to_string, string_to_icon_number, MarkerFlat, Position3D}, zone::{populate_zone_data, Zone}};

fn parse_elms_string(elms_string: &str) -> HashMap<u16, Vec<MarkerFlat>> {
    let re = Regex::new(r"/(?P<zone>\d+)//(?P<x>\d+),(?P<y>\d+),(?P<z>\d+),(?P<icon>\d+)/").unwrap();
    let mut result: HashMap<u16, Vec<MarkerFlat>> = HashMap::new();
    let mut seen: HashMap<u16, HashSet<MarkerFlat>> = HashMap::new();
    for caps in re.captures_iter(elms_string) {
        let zone_id: u16 = caps["zone"].parse().unwrap();
        let x = caps["x"].parse().unwrap();
        let y = caps["y"].parse().unwrap();
        let z = caps["z"].parse().unwrap();
        let icon = icon_number_to_string(caps["icon"].parse().unwrap());
        let marker = MarkerFlat { position: Position3D{x,y,z}, icon, size:1, active:true};
        let set = seen.entry(zone_id).or_default();
        if set.insert(marker.clone()) {
            result.entry(zone_id).or_default().push(marker);
        }
    }
    result
}

#[derive(Properties, PartialEq)]
pub struct CanvasMapProps {
    pub zone: Zone,
    pub markers: Vec<MarkerFlat>,
    pub zoom: f64,
    pub pan: (f64, f64),
    pub width: u32,
    pub height: u32,
}

#[function_component(CanvasMap)]
fn canvas_map(props: &CanvasMapProps) -> Html {
    let canvas_ref = use_node_ref();
    let zone = props.zone.clone();
    let markers = props.markers.clone();
    let zoom = props.zoom;
    let pan = props.pan;
    let canvas_width = props.width;
    let canvas_height = props.height;

    let tile_images = {
        let tiles = zone.tiles.clone();
        use_memo((), move |_| {
            tiles.into_iter().map(|tile| {
                let img = web_sys::HtmlImageElement::new().unwrap();
                img.set_src(&format!("static/maps/{}", tile.path));
                img
            }).collect::<Vec<_>>()
        })
    };

    {
        let canvas_ref = canvas_ref.clone();
        let tile_images = tile_images.clone();
        use_effect(move || {
            let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let ctx = canvas
                .get_context("2d").unwrap().unwrap()
                .dyn_into::<CanvasRenderingContext2d>().unwrap();
            canvas.set_width(canvas_width);
            canvas.set_height(canvas_height);
            let w = canvas.width() as f64;
            let h = canvas.height() as f64;
            let zoom = zoom.clone();

            ctx.set_transform(zoom, 0.0, 0.0, zoom, pan.0, pan.1).unwrap();
            ctx.clear_rect(0.0, 0.0, w / zoom, h / zoom);

            let tile_size = w / (zone.count as f64);
            let overlap = 1.0;

            for (i, img) in tile_images.iter().enumerate() {
                let row = (i as u8) / zone.count;
                let col = (i as u8) % zone.count;
                let x = col as f64 * tile_size;
                let y = row as f64 * tile_size;

                if img.complete() {
                    ctx
                        .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                            img,
                            0.0, 0.0,
                            img.width() as f64,
                            img.height() as f64,
                            x, y,
                            tile_size + overlap,
                            tile_size + overlap,
                        )
                        .unwrap();
                } else {
                    let ctx_clone = ctx.clone();
                    let img_clone = img.clone();
                    let draw = Closure::wrap(Box::new(move || {
                        ctx_clone
                            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                                &img_clone,
                                0.0, 0.0,
                                img_clone.width() as f64,
                                img_clone.height() as f64,
                                x, y,
                                tile_size + overlap,
                                tile_size + overlap,
                            )
                            .unwrap();
                    }) as Box<dyn Fn()>);
                    img.set_onload(Some(draw.as_ref().unchecked_ref()));
                    draw.forget();
                }
            }

            let base = canvas_width as f64 / 30.0;
            for marker in markers.iter() {
                if !marker.active {continue;}
                let (mx, mz) = {
                    let x = marker.position.x as f64;
                    let z = marker.position.z as f64;
                    let nx = (x - zone.scale_data.min_x as f64)
                        / (zone.scale_data.max_x as f64 - zone.scale_data.min_x as f64);
                    let nz = (z - zone.scale_data.min_z as f64)
                        / (zone.scale_data.max_z as f64 - zone.scale_data.min_z as f64);
                    (nx * w, nz * h)
                };

                let display_size = base * (1.0 / zoom.clone()) * (marker.size as f64);
                let dx = mx - display_size / 2.0;
                let dy = mz - display_size / 2.0;
                let icon_img = web_sys::HtmlImageElement::new().unwrap();
                icon_img.set_src(&format!("static/icons/{}", marker.icon));

                if icon_img.complete() {
                    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        &icon_img,
                        0.0, 0.0,
                        icon_img.width() as f64,
                        icon_img.height() as f64,
                        dx, dy,
                        display_size,
                        display_size,
                    ).unwrap();
                } else {
                    let ctx_clone = ctx.clone();
                    let icon_clone = icon_img.clone();
                    let draw = Closure::wrap(Box::new(move || {
                        ctx_clone.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                            &icon_clone,
                            0.0, 0.0,
                            icon_clone.width() as f64,
                            icon_clone.height() as f64,
                            dx, dy,
                            display_size,
                            display_size,
                        ).unwrap();
                    }) as Box<dyn Fn()>);
                    icon_img.set_onload(Some(draw.as_ref().unchecked_ref()));
                    draw.forget();
                }
            }

            || ()
        });
    }

    html! {
        <canvas ref={canvas_ref} style="cursor: grab;" />
    }
}


#[derive(Properties, PartialEq)]
pub struct MarkerListPanelProps {
    pub markers: Vec<MarkerFlat>,
    pub on_update: Callback<Vec<MarkerFlat>>,
}

#[function_component(MarkerListPanel)]
fn marker_list_panel(props: &MarkerListPanelProps) -> Html {
    let markers = use_state(|| props.markers.clone());

    {
        let markers = markers.clone();
        let props_markers = props.markers.clone();
        use_effect_with(props.markers.clone(), move |_| {
            markers.set(props_markers);
            || ()
        });
    }

    let on_update = props.on_update.clone();
    let update_marker = {
        let markers = markers.clone();
        Callback::from(move |(index, field, value): (usize, String, String)| {
            let mut new_markers = (*markers).clone();
            if let Some(marker) = new_markers.get_mut(index) {
                match field.as_str() {
                    "x" => if let Ok(v) = value.parse() { marker.position.x = v },
                    "y" => if let Ok(v) = value.parse() { marker.position.y = v },
                    "z" => if let Ok(v) = value.parse() { marker.position.z = v },
                    "icon" => marker.icon = value,
                    _ => {}
                }
            }
            markers.set(new_markers.clone());
            on_update.emit(new_markers);
        })
    };

    let on_toggle = {
        let markers = markers.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |(i, active): (usize, bool)| {
            let mut new_markers = (*markers).clone();
            if let Some(marker) = new_markers.get_mut(i) {
                marker.active = active;
            }
            markers.set(new_markers.clone());
            on_update.emit(new_markers);
        })
    };

    html! {
        <div style="width: 670px; padding: 10px; overflow-y: auto;">
            <h1 style="text-align:center;">{"Markers"}</h1>
            <ul>
                { for markers.iter().enumerate().map(|(i, marker)| {
                    let on_toggle = on_toggle.clone();
                    let update_marker = update_marker.clone();
                    let update_marker2 = update_marker.clone();
                    let update_marker3 = update_marker.clone();
                    let marker_clone = marker.clone();
                    html! {
                        <li key={i} style="margin-bottom: 10px;display: flex; height:20px;">
                            <div style="display: flex; align-items: center; gap: 4px;">
                                <img src={format!("static/icons/{}", marker_clone.icon)} style="height: 100%;" />
                            </div>
                            <label style="margin-left:5px;">{"X: "}<input
                                type="number"
                                style="width: 100px; margin-left:5px;"
                                value={marker_clone.position.x.to_string()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    update_marker.emit((i, "x".to_string(), input.value()));
                                })}
                            /></label>
                            <label style="margin-left:5px;">{" Y: "}<input
                                type="number"
                                style="width: 100px; margin-left:5px;"
                                value={marker_clone.position.y.to_string()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    update_marker2.emit((i, "y".to_string(), input.value()));
                                })}
                            /></label>
                            <label style="margin-left:5px;">{" Z: "}<input
                                type="number"
                                style="width: 100px; margin-left:5px;"
                                value={marker_clone.position.z.to_string()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    update_marker3.emit((i, "z".to_string(), input.value()));
                                })}
                            /></label>
                            <label style="margin-left:5px;">{" Active: "}<input
                                type="checkbox"
                                style="margin-left:5px;"
                                checked={marker_clone.active}
                                onchange={Callback::from(move |e: Event| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    on_toggle.emit((i, input.checked()));
                                })}
                            /></label>
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let zones = populate_zone_data();
    let current_zone = zones[0].clone();
    let elms_input = use_state(|| String::new());
    let parsed = use_state(|| HashMap::<u16, Vec<MarkerFlat>>::new());
    let zoom = use_state(|| 1.0);
    let pan = use_state(|| (0.0, 0.0));

    let onwheel = {
        let pan = pan.clone();
        let zoom = zoom.clone();
        Callback::from(move |e: WheelEvent| {
            let target = e.target();
            let Some(canvas) = target.and_then(|t| t.dyn_into::<HtmlCanvasElement>().ok()) else {
                return;
            };

            let rect = canvas.get_bounding_client_rect();
            let cx = e.client_x() as f64;
            let cy = e.client_y() as f64;

            if cx < rect.left() || cx > rect.right() || cy < rect.top() || cy > rect.bottom() {
                return;
            }
            e.prevent_default();
            let mx = e.offset_x() as f64;
            let my = e.offset_y() as f64;
            let old_zoom = *zoom;
            let world_x = (mx - pan.0) / old_zoom;
            let world_y = (my - pan.1) / old_zoom;

            let delta = if e.delta_y() > 0.0 { 0.9_f64 } else { 1.1_f64 };
            let min_zoom = 1.0_f64;
            let max_zoom = 5.0_f64;
            let new_zoom = (old_zoom * delta).clamp(min_zoom, max_zoom);

            let new_pan_x = mx - world_x * new_zoom;
            let new_pan_y = my - world_y * new_zoom;

            let w = canvas.width() as f64;
            let h = canvas.height() as f64;
            let world_px = w;
            let scaled = world_px * new_zoom;
            let clamped_x = new_pan_x.clamp(w - scaled, 0.0);
            let clamped_y = new_pan_y.clamp(h - scaled, 0.0);

            zoom.set(new_zoom);
            pan.set((clamped_x, clamped_y));
        })
    };

    let dragging = use_state(|| false);
    let last = use_state(|| (0.0, 0.0));

    let onmousedown = {
        let dragging = dragging.clone();
        let last = last.clone();
        Callback::from(move |e: MouseEvent| {
            dragging.set(true);
            last.set((e.client_x() as f64, e.client_y() as f64));
        })
    };

    let onmouseup = {
        let dragging = dragging.clone();
        Callback::from(move |_| dragging.set(false))
    };

    let onmousemove = {
        let dragging = dragging.clone();
        let last = last.clone();
        let pan = pan.clone();
        let zoom = zoom.clone();
        Callback::from(move |e: MouseEvent| {
            if *dragging {
                let (lx, ly) = *last;
                let nx = e.client_x() as f64;
                let ny = e.client_y() as f64;
                let dx = nx - lx;
                let dy = ny - ly;
                let mut new_x = pan.0 + dx;
                let mut new_y = pan.1 + dy;
                let canvas = e
                    .target_unchecked_into::<HtmlCanvasElement>();
                let w = canvas.width() as f64;
                let world_px = w;
                let scaled = world_px * *zoom;
                new_x = new_x.clamp(w - scaled, 0.0);
                new_y = new_y.clamp(w - scaled, 0.0);
                pan.set((new_x, new_y));
                last.set((nx, ny));
            }
        })
    };

    let oninput = {
        let elms_input = elms_input.clone();
        let parsed = parsed.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let v = input.value();
            if v != *elms_input  {
                elms_input.set(v.clone());
            }
            if !v.is_empty() {
                parsed.set(parse_elms_string(&v));
            } else {
                parsed.set(HashMap::new());
            }
        })
    };

    let update_markers = {
        let parsed = parsed.clone();
        let elms_input = elms_input.clone();
        Callback::from(move |new_markers: Vec<MarkerFlat>| {
            let mut new_map = (*parsed).clone();
            new_map.insert(current_zone.id, new_markers.clone());
            parsed.set(new_map);

            let mut result = String::new();
            for (zone_id, markers) in &*parsed {
                for marker in markers {
                    if marker.active {
                        result.push_str(&format!(
                            "/{zone}//{x},{y},{z},{icon}/",
                            zone = zone_id,
                            x = marker.position.x,
                            y = marker.position.y,
                            z = marker.position.z,
                            icon = string_to_icon_number(&marker.icon),
                        ));
                    }
                }
            }
            let normalized_result = result.trim();
            let normalized_input = elms_input.trim();

            if normalized_result != normalized_input && !elms_input.is_empty() {
                elms_input.set(normalized_result.to_string());
            }
        })
    };

    let current_markers = parsed.get(&current_zone.id).cloned().unwrap_or_default();

    let canvas_width = 950;
    let canvas_height = 950;

    html! {
        <div style="display: flex; flex-direction: row; background-color: #333; color: #fff; font-family: 'Univers', sans-serif;">
            <div
                {onwheel}
                {onmousedown}
                {onmouseup}
                {onmousemove}
                style="
                    display: flex;
                    flex-flow: column nowrap;
                    box-sizing: border-box;
                    margin-left: 5%;
                    overflow: hidden;
                ">
                <div>
                    <CanvasMap
                        zone={current_zone.clone()}
                        markers={current_markers.clone()}
                        zoom={*zoom}
                        pan={*pan}
                        width={canvas_width}
                        height={canvas_height}
                    />
                </div>

                <textarea
                    oninput={oninput}
                    value={(*elms_input).clone()}
                    autofocus=true
                    placeholder="Elm's string"
                    style={format!("
                        width: {}px;
                        box-sizing: border-box;
                        height: 50px;
                        margin-top: 10px;", canvas_width)}
                />
            </div>

            <MarkerListPanel markers={current_markers} on_update={update_markers} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
