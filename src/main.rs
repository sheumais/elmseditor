use std::collections::{HashMap, HashSet};
use stylist::css;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, HtmlInputElement, MouseEvent, WheelEvent};
use yew::prelude::*;
use regex::Regex;
use yew_icons::{Icon, IconId};

mod marker;
mod zone;

use crate::{marker::{icon_number_to_string, string_to_icon_number, MarkerFlat, MarkerFormat, Position3D}, zone::{populate_zone_data, Map}};

fn parse_elms_string(elms_string: &str) -> HashMap<u16, Vec<MarkerFlat>> {
    let re = Regex::new(r"/(?P<zone>\d+)//(?P<x>\d+),(?P<y>\d+),(?P<z>\d+),(?P<icon>\d+)/").unwrap();
    let mut result: HashMap<u16, Vec<MarkerFlat>> = HashMap::new();
    let mut seen: HashMap<u16, HashSet<MarkerFlat>> = HashMap::new();

    let mut i = 0;
    for caps in re.captures_iter(elms_string) {
        let zone_id: u16 = caps["zone"].parse().unwrap();
        let x = caps["x"].parse().unwrap();
        let y = caps["y"].parse().unwrap();
        let z = caps["z"].parse().unwrap();
        let icon = icon_number_to_string(caps["icon"].parse().unwrap());
        let marker = MarkerFlat { position: Position3D { x, y, z }, icon, size: 1, active: true, id: i, format: MarkerFormat::Bitrock};

        let set = seen.entry(zone_id).or_default();
        if set.insert(marker.clone()) {
            result.entry(zone_id).or_default().push(marker);
            i += 1;
        }
    }

    for markers in result.values_mut() {
        markers.sort_by_key(|m| string_to_icon_number(&m.icon));
    }

    result
}

#[derive(Properties, PartialEq)]
pub struct CanvasMapProps {
    pub map: Map,
    pub markers: Vec<MarkerFlat>,
    pub zoom: f64,
    pub pan: (f64, f64),
    pub width: u32,
    pub height: u32,
}

#[function_component(CanvasMap)]
fn canvas_map(props: &CanvasMapProps) -> Html {
    let canvas_ref = use_node_ref();
    let map = props.map.clone();
    let markers = props.markers.clone();
    let zoom = props.zoom;
    let pan = props.pan;
    let canvas_width = props.width;
    let canvas_height = props.height;
    // web_sys::console::log_1(&format!("canvas map map: {:?}", map).into());

    let tile_images = {
        let tiles = map.tiles.clone();
        use_memo(tiles.clone(), move |tiles: &Vec<_>| {
            tiles.iter().map(|tile| {
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

            ctx.set_transform(zoom, 0.0, 0.0, zoom, pan.0, pan.1).unwrap();
            ctx.set_image_smoothing_enabled(false);
            ctx.clear_rect(0.0, 0.0, w / zoom, h / zoom);

            let tile_size = w / (map.count as f64);

            for (i, img) in tile_images.iter().enumerate() {
                let row = (i as u8) / map.count;
                let col = (i as u8) % map.count;

                let raw_x1 = col as f64 * tile_size;
                let raw_y1 = row as f64 * tile_size;
                let raw_x2 = (col as f64 + 1.0) * tile_size;
                let raw_y2 = (row as f64 + 1.0) * tile_size;
                let x = raw_x1.floor();
                let y = raw_y1.floor();
                let dw = raw_x2.ceil() - x;
                let dh = raw_y2.ceil() - y;

                if img.complete() {
                    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        img,
                        0.0, 0.0,
                        img.width() as f64,
                        img.height() as f64,
                        x, y,
                        dw, dh,
                    ).unwrap();
                } else {
                    let ctx_clone = ctx.clone();
                    let img_clone = img.clone();
                    let draw = Closure::wrap(Box::new(move || {
                        ctx_clone.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                            &img_clone,
                            0.0, 0.0,
                            img_clone.width() as f64,
                            img_clone.height() as f64,
                            x, y,
                            dw, dh,
                        ).unwrap();
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
                    let nx = (x - map.scale_data.min_x as f64)
                        / (map.scale_data.max_x as f64 - map.scale_data.min_x as f64);
                    let nz = (z - map.scale_data.min_z as f64)
                        / (map.scale_data.max_z as f64 - map.scale_data.min_z as f64);
                    (nx * w, nz * h)
                };

                let display_size = base * (1.0 / zoom) * (marker.size as f64);
                let dx = mx - display_size / 2.0;
                let dy = mz - display_size / 2.0;
                let icon_img = HtmlImageElement::new().unwrap();
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
    pub zone_markers: Vec<MarkerFlat>,
    pub current_markers: Vec<MarkerFlat>,
    pub on_update: Callback<Vec<MarkerFlat>>,
}

#[function_component(MarkerListPanel)]
fn marker_list_panel(props: &MarkerListPanelProps) -> Html {
    let current = use_state(|| props.current_markers.clone());
    {
        let cm_clone = props.current_markers.clone();
        let current = current.clone();
        use_effect_with(props.current_markers.clone(), move |_| {
            current.set(cm_clone);
            || ()
        });
    }

    let zone_template = props.zone_markers.clone();
    let on_update_cb  = props.on_update.clone();
    let update_marker = {
        let current       = current.clone();
        let zone_for_upd  = zone_template.clone();
        let emit_for_upd  = on_update_cb.clone();
        Callback::from(move |(pos, field, val): (usize, String, String)| {
            let mut new_current = (*current).clone();
            if let Some(m) = new_current.get_mut(pos) {
                match field.as_str() {
                    "x" => if let Ok(x) = val.parse() { m.position.x = x },
                    "y" => if let Ok(y) = val.parse() { m.position.y = y },
                    "z" => if let Ok(z) = val.parse() { m.position.z = z },
                    "icon" => m.icon = val.clone(),
                    _ => {}
                }
            }
            current.set(new_current.clone());
            let rebuilt: Vec<MarkerFlat> = zone_for_upd
                .iter()
                .map(|zm| {
                    new_current
                        .iter()
                        .find(|cm| cm.id == zm.id)
                        .cloned()
                        .unwrap_or_else(|| zm.clone())
                })
                .collect();
            emit_for_upd.emit(rebuilt);
        })
    };

    let toggle_active = {
        let current        = current.clone();
        let zone_for_toggle = zone_template.clone();
        let emit_for_toggle = on_update_cb.clone();
        Callback::from(move |(pos, is_on): (usize, bool)| {
            let mut new_current = (*current).clone();
            if let Some(m) = new_current.get_mut(pos) {
                m.active = is_on;
            }
            current.set(new_current.clone());

            let rebuilt = zone_for_toggle
                .iter()
                .map(|zm| {
                    new_current
                        .iter()
                        .find(|cm| cm.id == zm.id)
                        .cloned()
                        .unwrap_or_else(|| zm.clone())
                })
                .collect();
            emit_for_toggle.emit(rebuilt);
        })
    };

    html! {
        <div style="
            display: flex;
            flex-direction: column;
            max-height: 85vh;
            margin-bottom: 5vh;">
        <h1 style="text-align:center;">{"Markers"}</h1>
        <div style="
            min-height: 0;
            overflow-y: auto;">
            <ul style="padding:0; margin:0;">
            { for current.iter().enumerate().map(|(i, marker)| {
                let upd = update_marker.clone();
                let tog = toggle_active.clone();
                html! {
                <li key={marker.id} style="display: flex; align-items: center; gap: 1em; justify-content: center;">
                    <img src={format!("static/icons/{}", marker.icon)} style="height: 2em;"/>
                    { for ["x","y","z"].iter().map(move |&axis| {
                        let updt = upd.clone();
                        let val = match axis {
                            "x" => marker.position.x.to_string(),
                            "y" => marker.position.y.to_string(),
                            "z" => marker.position.z.to_string(),
                            _   => String::new(),
                        };
                        html! {
                        <label>
                            {format!("{}: ", axis.to_uppercase())}
                            <input
                            type="number"
                            value={val.clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let inp: HtmlInputElement = e.target_unchecked_into();
                                updt.emit((i, axis.to_string(), inp.value()));
                            })}
                            style="width:75px;"
                            />
                        </label>
                        }
                    }) }
                    <label>
                        <input
                        type="checkbox"
                        checked={marker.active}
                        onchange={Callback::from(move |e: Event| {
                            let inp: HtmlInputElement = e.target_unchecked_into();
                            tog.emit((i, inp.checked()));
                        })}
                        style="margin-left:8px;"
                        />
                    </label>
                </li>
                }
            }) }
            </ul>
        </div>
      </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let zones = populate_zone_data();
    let zone_ids: Vec<u16> = zones.iter().map(|z| z.id).collect();
    let selected_zone_index = use_state(|| 0_usize);
    let selected_map_index = use_state(|| 0_usize);
    let elms_input = use_state(|| String::new());
    let parsed_markers = use_state(|| HashMap::<u16, Vec<MarkerFlat>>::new());
    let zoom = use_state(|| 1.0);
    let pan = use_state(|| (0.0, 0.0));
    let canvas_size = use_state(|| 0);
    let window_width = use_state(|| 0.0);
    let window_height = use_state(|| 0.0);

    let on_map_change = {
        let selected_map_index = selected_map_index.clone();
        let zoom = zoom.clone();
        let pan = pan.clone();
        Callback::from(move |e: Event| {
            let sel: HtmlInputElement = e.target_unchecked_into();
            if let Ok(idx) = sel.value().parse::<usize>() {
                selected_map_index.set(idx);
                zoom.set(1.0);
                pan.set( (0.0, 0.0) );
            }
        })
    };

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
            let max_zoom = 20.0_f64;
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
        let parsed_markers = parsed_markers.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let v = input.value();
            // web_sys::console::log_1(&format!("Textarea input: '{}'", v).into());

            elms_input.set(v.clone());

            if !v.is_empty() {
                let new_map = parse_elms_string(&v);
                // web_sys::console::log_1(&format!("parsed_markers map entries: {}", new_map.len()).into());
                parsed_markers.set(new_map);
            } else {
                parsed_markers.set(HashMap::new());
            }
        })
    };

    {
        let parsed_markers = parsed_markers.clone();
        let zone_ids = zone_ids.clone();
        let selected_zone_index = selected_zone_index.clone();
        let selected_map_index = selected_map_index.clone();
        let zoom = zoom.clone();
        let pan = pan.clone();
        use_effect_with(parsed_markers.clone(),
            move |parsed_markers| {
                let zones: Vec<u16> = parsed_markers.keys().cloned().collect();
                // web_sys::console::log_1(&format!("use_effect parsed_markers zones: {:?}", zones).into());

                if !parsed_markers.is_empty() {
                    let current_zone = zone_ids[*selected_zone_index];
                    let mut keys = zones.clone();
                    keys.sort();
                    let first_zone = keys[0];
                    if first_zone != current_zone {
                        if let Some(idx) = zone_ids.iter().position(|&z| z == first_zone) {
                            // web_sys::console::log_1(&format!("Switching to zone {} at index {}", first_zone, idx).into());
                            selected_zone_index.set(idx);
                            selected_map_index.set(0);
                            zoom.set(1.0);
                            pan.set((0f64, 0f64));
                            last.set((0f64, 0f64));
                        }
                    }
                }

                || ()
            },
        );
    }

    let update_markers = {
        let parsed_markers = parsed_markers.clone();
        let elms_input = elms_input.clone();
        let selected_zone_index = selected_zone_index.clone();
        let zone_ids = zone_ids.clone();
        Callback::from(move |new_markers: Vec<MarkerFlat>| {
            let mut new_map = (*parsed_markers).clone();
            let zone_id = zone_ids[*selected_zone_index];
            new_map.insert(zone_id, new_markers.clone());
            let mut all_zones: Vec<u16> = new_map.keys().cloned().collect();
            all_zones.sort();

            let mut result = String::new();
            for zone in all_zones {
                if let Some(markers) = new_map.get(&zone) {
                    let mut active: Vec<_> = markers.iter()
                        .filter(|m| m.active)
                        .collect();
                    active.sort_by_key(|m| string_to_icon_number(&m.icon));

                    for m in active {
                        result.push_str(&format!(
                            "/{}//{},{},{},{}/",
                            zone,
                            m.position.x,
                            m.position.y,
                            m.position.z,
                            string_to_icon_number(&m.icon),
                        ));
                    }
                }
            }
            let normalized = result.trim();
            if normalized != elms_input.trim() {
                elms_input.set(normalized.to_string());
            }
            parsed_markers.set(new_map);
        })
    };

    {
        let canvas_size = canvas_size.clone();
        let window_width = window_width.clone();
        let window_height = window_height.clone();
        use_effect_with((),
            move |_| {
                let win = web_sys::window().expect("no global `window` exists");
                let win2 = win.clone();
                let set_size = move || {
                    let w = win2
                        .inner_width()
                        .ok()
                        .and_then(|v| v.as_f64())
                        .unwrap_or(950.0);
                    let h = win2
                        .inner_height()
                        .ok()
                        .and_then(|v| v.as_f64())
                        .unwrap_or(950.0);
                    let size = (w.min(h)) as u32;
                    window_width.set(w);
                    window_height.set(h);
                    canvas_size.set(size);
                };

                set_size();

                let resize_closure = Closure::wrap(Box::new(move |_ev: web_sys::Event| {
                    set_size();
                }) as Box<dyn FnMut(_)>);

                win
                    .add_event_listener_with_callback(
                        "resize",
                        resize_closure.as_ref().unchecked_ref(),
                    )
                    .expect("failed to register resize listener");

                resize_closure.forget();
                || ()
            },
        );
    }


    let logo_style = css!(r#"
        width: 2em;
        height: 2em;
        color: #fff;
    "#);

    let zone = zones.get(*selected_zone_index).unwrap().clone();
    // web_sys::console::log_1(&format!("zone_index: {}, zone: {:?}", *selected_zone_index, zone).into());
    let map = zone.maps.get(*selected_map_index).unwrap().clone();

    let canvas_width = *canvas_size;
    let canvas_height = *canvas_size;

    let w = canvas_width as f64;
    let h = canvas_height as f64;
    let zoom = *zoom.clone();
    let pan = *pan.clone();

    let view_min_x = -pan.0 / zoom;
    let view_max_x = (w - pan.0) / zoom;
    let view_min_z = -pan.1 / zoom;
    let view_max_z = (h - pan.1) / zoom;
    let zone_markers = parsed_markers.get(&zone.id).cloned().unwrap_or_default();
    let zone_marker_clone = zone_markers.clone();
    let current_markers: Vec<MarkerFlat> = zone_markers.into_iter()
        .filter(|m| {
            let nx = (m.position.x as f64 - map.scale_data.min_x as f64)
                / (map.scale_data.max_x as f64 - map.scale_data.min_x as f64);
            let nz = (m.position.z as f64 - map.scale_data.min_z as f64)
                / (map.scale_data.max_z as f64 - map.scale_data.min_z as f64);
            let world_x = nx * w;
            let world_z = nz * h;

            world_x >= view_min_x
            && world_x <= view_max_x
            && world_z >= view_min_z
            && world_z <= view_max_z
        })
        .collect();

    html! {
        <div style={format!("display: flex; background-color: #333; color: #fff; font-family: 'Univers', sans-serif; max-height: {}px; overflow: hidden;", *canvas_size)}>
            <div
                {onwheel}
                {onmousedown}
                {onmouseup}
                {onmousemove}
                style="
                    display: flex;
                    flex-flow: column nowrap;
                    box-sizing: border-box;
                    overflow: hidden;
                ">
                <div>
                    <CanvasMap
                        map={map.clone()}
                        markers={current_markers.clone()}
                        zoom={zoom}
                        pan={pan}
                        width={canvas_width}
                        height={canvas_height}
                    />
                </div>
            </div>

            if zone.maps.len() > 1 {
                <div style="position: absolute; top: 1em; left: 1em;">
                    <select onchange={on_map_change}>
                        {
                            for zone.maps.iter().enumerate().map(|(i, map)| html! {
                                <option value={i.to_string()} selected={i == *selected_map_index}>
                                    { &map.name }
                                </option>
                            })
                        }
                    </select>
                </div>
            }

            <div style={format!("width: {}px; height: {}px; padding: 1em;", window_height.max(*window_width) - (*canvas_size as f64), *window_height)}>
                <div> 
                    <textarea
                        oninput={oninput}
                        value={(*elms_input).clone()}
                        autofocus=true
                        placeholder="Elm's string"
                        style="
                            width: 100%;
                            border-radius: 1em;
                            height: 4em;
                            padding-left: 0.5em;
                            resize: none;"
                    />
                    <MarkerListPanel zone_markers={zone_marker_clone} current_markers={current_markers} on_update={update_markers} />
                </div>
                
                <div style="position: absolute; bottom: 1em; right: 1em; display: flex; gap: 1em;">
                    <a
                        href={"https://discord.gg/FjJjXHjUQ4"}
                        target="_blank"
                        rel="noopener noreferrer">
                        <Icon icon_id={IconId::BootstrapDiscord} class={logo_style.clone()} />
                    </a>
                    <a
                        href={"https://github.com/sheumais/elmseditor"}
                        target="_blank"
                        rel="noopener noreferrer">
                        <Icon icon_id={IconId::BootstrapGithub} class={logo_style.clone()} />
                    </a>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
