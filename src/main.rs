use std::collections::{HashMap, HashSet};
use stylist::{css, Style};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, HtmlInputElement, MouseEvent, WheelEvent};
use yew::prelude::*;
use regex::Regex;
use yew_icons::{Icon, IconId};

mod marker;
mod zone;

use crate::{marker::{MarkerFlat, MarkerFormat, MarkerIcon, Position3D, ALL_ICONS}, zone::{populate_zone_data, Map, Zone}};

fn parse_elms_string(elms_string: &str, zones: Vec<Zone>) -> HashMap<u16, Vec<MarkerFlat>> {
    let re = Regex::new(r"/(?P<zone>\d+)//(?P<x>\d+),(?P<y>\d+),(?P<z>\d+),(?P<icon>\d+)/").unwrap();
    let mut result: HashMap<u16, Vec<MarkerFlat>> = HashMap::new();
    let mut seen: HashMap<u16, HashSet<MarkerFlat>> = HashMap::new();
    let mut id_counter = 0;

    for caps in re.captures_iter(elms_string) {
        let zone_id: u16 = caps["zone"].parse().unwrap();
        let x: i32 = caps["x"].parse().unwrap();
        let y: i32 = caps["y"].parse().unwrap();
        let z: i32 = caps["z"].parse().unwrap();
        let icon_number: u16 = caps["icon"].parse().unwrap();
        let icon_enum = MarkerIcon::try_from(icon_number).unwrap_or(MarkerIcon::Unknown);
        let icon = icon_enum.into();

        if let Some(zone_obj) = zones.iter().find(|zone| zone.id == zone_id) {
            let matching_maps: Vec<&Map> = zone_obj.maps.iter().filter(|map| {
                x >= map.scale_data.min_x as i32 && x <= map.scale_data.max_x as i32
                && z >= map.scale_data.min_z as i32 && z <= map.scale_data.max_z as i32
            }).collect();

            if let Some(best_map) = matching_maps.into_iter().min_by_key(|map| {
                    let width  = (map.scale_data.max_x - map.scale_data.min_x) as u32;
                    let depth  = (map.scale_data.max_z - map.scale_data.min_z) as u32;
                    let area   = width.saturating_mul(depth);

                    let (unknown_flag, y_offset) = match map.scale_data.y {
                        Some(by) => {
                            let dy = (y as f32 - by).abs() as u32;
                            (0u32, dy)
                        }
                        None => {
                            (1u32, u32::MAX)
                        }
                    };

                    (unknown_flag, y_offset, area)
                }) {
                let marker = MarkerFlat {
                    position: Position3D { x, y, z },
                    icon,
                    size: 1,
                    active: true,
                    id: id_counter,
                    format: MarkerFormat::Bitrock,
                    map_id: best_map.map_id,
                };

                let entry_set = seen.entry(zone_id).or_default();
                if entry_set.insert(marker.clone()) {
                    result.entry(zone_id).or_default().push(marker);
                    id_counter += 1;
                }
            }
        }
    }

    for markers in result.values_mut() {
        markers.sort_by_key(|m| -> u16 { (&m.icon).into() });
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
    pub map_index: usize,
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
    let map_index = props.map_index;
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
        use_effect_with((tile_images.clone(),markers.clone(),zoom,pan,canvas_width,canvas_height,map_index),
            move |(tile_images, markers, zoom, pan, canvas_width, canvas_height, _)| {
            
            let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let ctx = canvas
                .get_context("2d").unwrap().unwrap()
                .dyn_into::<CanvasRenderingContext2d>().unwrap();
            canvas.set_width(*canvas_width);
            canvas.set_height(*canvas_height);
            let w = canvas.width() as f64;
            let h = canvas.height() as f64;

            ctx.set_transform(*zoom, 0.0, 0.0, *zoom, pan.0, pan.1).unwrap();
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

            let base = *canvas_width as f64 / 30.0;
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
                icon_img.set_src(&format!("static/icons/{}", String::from(marker.icon)));

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
        <canvas ref={canvas_ref} style="cursor: grab; min-width: 300px; min-height: 300px;" />
    }
}

#[derive(Properties, PartialEq)]
pub struct MarkerListPanelProps {
    pub zone_markers: Vec<MarkerFlat>,
    pub current_markers: Vec<MarkerFlat>,
    pub on_update: Callback<Vec<MarkerFlat>>,
    pub world_bounds: (f32, f32, f32, f32),
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

    let delete_style = Style::new(css!(r#"
        color: #fff;
        transition: color 0.3s, scale 0.3s;
        cursor: pointer;
        &:hover {
            color: #ff0000;
            transform: scale(1.5);
        }
    "#)).expect("Couldn't create delete_style");

    let zone_template = props.zone_markers.clone();
    let on_update_cb  = props.on_update.clone();
    let icon_picker_for = use_state(|| None::<usize>);

    let update_marker = {
        let current = current.clone();
        let zone_for_upd  = zone_template.clone();
        let emit_for_upd  = on_update_cb.clone();
        Callback::from(move |(pos, field, val): (usize, String, String)| {
            let mut new_current = (*current).clone();
            if let Some(m) = new_current.get_mut(pos) {
                match field.as_str() {
                    "x" => if let Ok(x) = val.parse() { m.position.x = x },
                    "y" => if let Ok(y) = val.parse() { m.position.y = y },
                    "z" => if let Ok(z) = val.parse() { m.position.z = z },
                    "icon" => m.icon = val.as_str().into(),
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
        let current = current.clone();
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

    let delete_marker = {
        let current = current.clone();
        let zone_for_delete = zone_template.clone();
        let emit_for_delete = on_update_cb.clone();
        Callback::from(move |pos: usize| {
            if let Some(to_delete) = (*current).get(pos).map(|m| m.id.clone()) {
                let filtered_full: Vec<MarkerFlat> = zone_for_delete
                    .iter()
                    .cloned()
                    .filter(|zm| zm.id != to_delete)
                    .collect();

                let new_current: Vec<MarkerFlat> = (*current)
                    .iter()
                    .filter(|cm| cm.id != to_delete)
                    .cloned()
                    .collect();
                current.set(new_current);

                emit_for_delete.emit(filtered_full);
            }
        })
    };

    html! {
        <div style="display:flex;flex-direction:column;max-height:85vh;margin-bottom:5vh;text-shadow: 2px 1.5px black;">
            <h1 style="text-align:center;">{"Markers"}</h1>
            <div style="overflow-y:auto;">
                <ul style="padding:0;margin:0;list-style:none;">
                { for current.iter().enumerate().map(|(i, marker)| {
                    let upd = update_marker.clone();
                    let tog = toggle_active.clone();
                    let del = delete_marker.clone();
                    let picker = icon_picker_for.clone();
                    html! {
                    <li key={marker.id.clone()} style="display:flex;align-items:center;gap:1em;justify-content:center;padding:4px;">
                        <img
                            src={format!("static/icons/{}", String::from(marker.icon))}
                            style="height:2em;cursor:pointer;"
                            onclick={Callback::from(move |_| picker.set(Some(i)))}
                        />

                        { for ["x","y","z"].iter().map(move |&axis| {
                            let up = upd.clone();
                            let val = match axis {
                                "x" => marker.position.x.to_string(),
                                "y" => marker.position.y.to_string(),
                                "z" => marker.position.z.to_string(),
                                _ => String::new(),
                            };
                            html! {
                            <label>
                                {format!("{}: ", axis.to_uppercase())}
                                <input
                                    type="number"
                                    min={match axis {
                                        "x" => props.world_bounds.0.to_string(),
                                        "z" => props.world_bounds.2.to_string(),
                                        _ => f32::MIN.to_string(),
                                    }}
                                    max={match axis {
                                        "x" => props.world_bounds.1.to_string(),
                                        "z" => props.world_bounds.3.to_string(),
                                        _ => f32::MAX.to_string(),
                                    }}
                                    step={"25"}
                                    value={val.clone()}
                                    oninput={Callback::from(move |e: InputEvent| {
                                        let inp: HtmlInputElement = e.target_unchecked_into();
                                        up.emit((i, axis.to_string(), inp.value()));
                                    })}
                                    style="width:6em;"
                                />
                            </label>
                            }
                        }) }

                        <label>
                            <input
                                type="checkbox"
                                checked={marker.active}
                                style="cursor:pointer;"
                                onchange={Callback::from(move |e: Event| {
                                    let inp: HtmlInputElement = e.target_unchecked_into();
                                    tog.emit((i, inp.checked()));
                                })}
                            />
                        </label>

                        <Icon
                            class={delete_style.clone()}
                            style="cursor:pointer;"
                            width={"1em"}
                            height={"1em"}
                            icon_id={IconId::BootstrapXLg}
                            onclick={Callback::from(move |_| del.emit(i))}
                        />
                    </li>
                    }
                }) }
                </ul>
            </div>

            {
                if let Some(idx) = *icon_picker_for {
                    let close = {
                        let picker = icon_picker_for.clone();
                        Callback::from(move |_| picker.set(None))
                    };
                    let choose = {
                        let upd = update_marker.clone();
                        let picker = icon_picker_for.clone();
                        Callback::from(move |icon_name: String| {
                            upd.emit((idx, "icon".into(), icon_name));
                            picker.set(None);
                        })
                    };

                    html! {
                    <div class="modal-overlay" onclick={close.clone()} style="position:fixed;top:0;left:0;width:100%;height:100%;background:rgba(0,0,0,0.4);">
                        <div class="modal-content" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                            style="background:#333;padding:1em;border-radius:8px;max-width:40vw;max-height:80vh;overflow:auto;margin:5vh auto;">
                            <h2 style="text-align:center;margin:0px 0px 1em 0px;">{"Select Icon"}</h2>
                            <div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(50px,1fr));gap:8px;">
                                { for ALL_ICONS.iter().map(move |&icon| {
                                    let choose = choose.clone();
                                    html! {
                                    <div style="text-align:center;">
                                        <img
                                            src={format!("static/icons/{}", icon.to_string())}
                                            style="width:2em;height:2em;cursor:pointer;"
                                            onclick={Callback::from(move |_| choose.emit(icon.to_string()))}
                                        />
                                    </div>
                                    }
                                }) }
                            </div>
                        </div>
                    </div>
                    }
                } else {
                    html! {}
                }
            }
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
                // web_sys::console::log_1(&format!("2Setting map index to '{}'", idx).into());
                zoom.set(1.0);
                pan.set( (0.0, 0.0) );
            }
        })
    };

    let on_zone_change = {
        let selected_zone_index = selected_zone_index.clone();
        let selected_map_index = selected_map_index.clone();
        let zoom = zoom.clone();
        let pan = pan.clone();
        Callback::from(move |e: Event| {
            let sel: HtmlInputElement = e.target_unchecked_into();
            if let Ok(idx) = sel.value().parse::<usize>() {
                selected_zone_index.set(idx);
                selected_map_index.set(0);
                // web_sys::console::log_1(&format!("2Setting zone index to '{}'", idx).into());
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

    let oncontextmenu = {
        let zones = zones.clone();
        let selected_zone_index = selected_zone_index.clone();
        let selected_map_index = selected_map_index.clone();
        let parsed_markers = parsed_markers.clone();
        let canvas_size = canvas_size.clone();
        let zoom = zoom.clone();
        let pan = pan.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if let Some(canvas) = e.target_dyn_into::<HtmlCanvasElement>() {
                let canvas_rect = canvas.get_bounding_client_rect();
                let mx = e.client_x() as f64 - canvas_rect.left();
                let my = e.client_y() as f64 - canvas_rect.top();

                let zoom = *zoom;
                let pan = *pan;
                let world_x = (mx - pan.0) / zoom;
                let world_z = (my - pan.1) / zoom;

                let size = *canvas_size as f64;

                let zone = &zones[*selected_zone_index];
                let map = zone.maps.get(*selected_map_index).cloned();
                if let Some(map) = map {
                    let scale = &map.scale_data;
                    let nx = world_x / size;
                    let nz = world_z / size;
                    let pos_x = scale.min_x as f64 + nx * (scale.max_x - scale.min_x) as f64;
                    let pos_z = scale.min_z as f64 + nz * (scale.max_z - scale.min_z) as f64;

                    let marker = MarkerFlat {
                        active: true,
                        icon: "squares/marker_lightblue.png".into(),
                        position: Position3D {
                            x: pos_x.round() as i32,
                            y: 0,
                            z: pos_z.round() as i32,
                        },
                        size: 1,
                        id: parsed_markers.get(&zone.id).map(|v| v.len() as u16).unwrap_or(0),
                        format: MarkerFormat::Bitrock,
                        map_id: map.map_id,
                    };

                    let mut new_map = (*parsed_markers).clone();
                    let entry = new_map.entry(zone.id).or_default();
                    entry.push(marker);
                    parsed_markers.set(new_map);
                }
            }
        })
    };

    let oninput = {
        let elms_input = elms_input.clone();
        let parsed_markers = parsed_markers.clone();
        let zones = zones.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let v = input.value();
            // web_sys::console::log_1(&format!("Textarea input: '{}'", v).into());

            elms_input.set(v.clone());

            if !v.is_empty() {
                let new_map = parse_elms_string(&v, zones.clone());
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
                    // web_sys::console::log_1(&format!("current_zone id: {}, selected zone index: {}", current_zone, *selected_zone_index).into());
                    let keys = zones.clone();
                    let first_zone = keys[0];
                    // web_sys::console::log_1(&format!("first_zone: {}", first_zone).into());
                    if first_zone != current_zone {
                        if let Some(idx) = zone_ids.iter().position(|&z| z == first_zone) {
                            // web_sys::console::log_1(&format!("Switching to zone {} at index {}", first_zone, idx).into());
                            selected_zone_index.set(idx);
                            // web_sys::console::log_1(&format!("1Setting zone index to '{}'", idx).into());
                            selected_map_index.set(0);
                            // web_sys::console::log_1(&format!("1Setting map index to '{}'", 0).into());
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
            let mut new_map = HashMap::new();
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
                    active.sort_by_key(|m| -> u16 { (&m.icon).into() });

                    for m in active {
                        result.push_str(&format!(
                            "/{}//{},{},{},{}/",
                            zone,
                            m.position.x,
                            m.position.y,
                            m.position.z,
                            u16::from(&m.icon),
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
        let update_markers = update_markers.clone();
        let zone_ids = zone_ids.clone();
        let selected_zone = selected_zone_index.clone();

        use_effect_with(parsed_markers.clone(), move |pm| {
            let current_zone_id = zone_ids[*selected_zone];
            if let Some(markers) = pm.get(&current_zone_id).cloned() {
                update_markers.emit(markers);
            }
            || ()
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

    let zone = zones.get(*selected_zone_index).unwrap_or_else(|| {selected_zone_index.set(0); zones.get(0).unwrap()}).clone();
    // web_sys::console::log_1(&format!("zone_index: {}, zone: {:?}", *selected_zone_index, zone).into());
    let map = zone.maps.get(*selected_map_index).unwrap_or_else(|| {selected_map_index.set(0); zone.maps.get(0).unwrap()}).clone();

    let canvas_width = *canvas_size;
    let canvas_height = *canvas_size;

    let zoom = *zoom.clone();
    let pan = *pan.clone();

    let zone_markers = parsed_markers.get(&zone.id).cloned().unwrap_or_default();
    let zone_marker_clone = zone_markers.clone();
    let current_markers: Vec<MarkerFlat> = zone_markers.into_iter()
        .filter(|m| {m.map_id == map.map_id})
        .collect();

    html! {
        <div style={format!("display: flex; background-color: #333; color: #fff; font-family: 'Univers', sans-serif; max-height: {}px; flex-wrap: wrap;", *canvas_size)}>
            <div
                {onwheel}
                {onmousedown}
                {onmouseup}
                {onmousemove}
                {oncontextmenu}
                style="
                    display: flex;
                    flex-flow: column nowrap;
                    box-sizing: border-box;
                    flex-grow: 1;
                    flex-shrink: 1;
                    flex-basis: 300px;
                    justify-content: center;
                    text-align: center;
                ">
                <CanvasMap
                    map={map.clone()}
                    markers={current_markers.clone()}
                    zoom={zoom}
                    pan={pan}
                    width={canvas_width}
                    height={canvas_height}
                    map_index={*selected_map_index.clone()}
                />
            </div>

            <div style="position: absolute; top: 1em; left: 1em;">
            if zone_marker_clone.len() > 0 {
                if zone.maps.len() > 1 {
                    <select onchange={on_map_change}>
                        {
                            for zone.maps.iter().enumerate().map(|(i, map)| {
                                let marker_count = zone_marker_clone
                                    .iter()
                                    .filter(|m| m.map_id == map.map_id)
                                    .count();

                                let label = if marker_count > 0 {
                                    format!("[{}] {}", marker_count, &map.name)
                                } else {
                                    map.name.clone()
                                };

                                html! {
                                    <option value={i.to_string()} selected={i == *selected_map_index}>
                                        { label }
                                    </option>
                                }
                            })
                        }
                    </select>
                }
            } else {
                <select onchange={on_zone_change}>
                    {
                        for zones.iter().enumerate().map(|(i, zone)| html! {
                            <option value={i.to_string()} selected={i == *selected_zone_index}>
                                { &zone.name }
                            </option>
                        })
                    }
                </select>
            }
            </div>

            <div style={format!("width: {}px; height: {}px; min-width: 300px; flex-grow: 1; flex-shrink: 1; flex-basis: 300px; text-align: center;", window_height.max(*window_width) - (*canvas_size as f64), *window_height)}>
                <div> 
                    <textarea
                        oninput={oninput}
                        value={(*elms_input).clone()}
                        autofocus=true
                        placeholder="Paste an Elm's string to view and modify it. This will change the selected zone automatically. Right click to place a new marker. Note that a player is approximately 100 units (1 metre) wide."
                        style="
                            width: 80%;
                            border-radius: 1em;
                            height: 4em;
                            padding: 0.5em;
                            resize: none;
                            margin-top: 1em;"
                    />
                    <MarkerListPanel zone_markers={zone_marker_clone} current_markers={current_markers} on_update={update_markers} world_bounds={(map.scale_data.min_x, map.scale_data.max_x, map.scale_data.min_z, map.scale_data.max_z)} />
                </div>
                
                <div style="position: fixed; bottom: 1em; right: 1em; display: flex; gap: 1em;">
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
