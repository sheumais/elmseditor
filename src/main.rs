use std::collections::HashMap;
use stylist::{css, Style};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, HtmlInputElement, MouseEvent, WheelEvent};
use yew::prelude::*;
use yew_icons::{Icon, IconId};

mod marker;
mod zone;

use crate::marker::{ALL_ELMS_ICONS, ALL_M0R_ICONS, BreadcrumbLine, ElmMarker, Marker::{self, Elms, M0r}, MarkerTypes, Position3D, build_elms_string, build_m0r_string, get_marker_id, hex_to_rgba, lines_to_string, m0r_texture_to_og, parse_elms_string, parse_lines_string, parse_m0r_string, rgba_to_hex_string, set_marker_active};
use crate::zone::{Map, populate_zone_data};

#[derive(Properties, PartialEq)]
pub struct CanvasMapProps {
    pub map: Map,
    pub markers: Vec<Marker>,
    pub lines: Vec<BreadcrumbLine>,
    pub zoom: f64,
    pub pan: (f64, f64),
    pub width: u32,
    pub height: u32,
    pub force_redraw: bool,
}
#[function_component(CanvasMap)]
fn canvas_map(props: &CanvasMapProps) -> Html {
    let map_canvas_ref = use_node_ref();
    let marker_canvas_ref = use_node_ref();

    let map = props.map.clone();
    let markers = props.markers.clone();
    let zoom = props.zoom;
    let pan = props.pan;
    let canvas_width = props.width;
    let canvas_height = props.height;
    let force_redraw = props.force_redraw;
    let lines = props.lines.clone();

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
        let map_canvas_ref = map_canvas_ref.clone();
        let tile_images = tile_images.clone();
        use_effect_with((tile_images.clone(), zoom, pan, canvas_width, canvas_height),
            move |(tile_images, zoom, pan, canvas_width, canvas_height)| {
                let canvas = map_canvas_ref.cast::<HtmlCanvasElement>().unwrap();
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

                || ()
        });
    }

    {
        let marker_canvas_ref = marker_canvas_ref.clone();
        let markers = markers.clone();
        let lines = lines.clone();

        use_effect_with((markers.clone(), lines.clone(), zoom, pan, canvas_width, canvas_height, force_redraw),
            move |(markers, lines, zoom, pan, canvas_width, canvas_height, _force_redraw)| {
                let canvas = marker_canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                let ctx = canvas
                    .get_context("2d").unwrap().unwrap()
                    .dyn_into::<CanvasRenderingContext2d>().unwrap();

                canvas.set_width(*canvas_width);
                canvas.set_height(*canvas_height);

                let w = canvas.width() as f64;
                let h = canvas.height() as f64;

                ctx.set_transform(*zoom, 0.0, 0.0, *zoom, pan.0, pan.1).unwrap();
                ctx.set_image_smoothing_enabled(true);
                ctx.clear_rect(0.0, 0.0, w / zoom, h / zoom);

                let project = |p: &Position3D| -> (f64, f64) {
                    let nx = (p.x as f64 - map.scale_data.min_x as f64)
                        / (map.scale_data.max_x as f64 - map.scale_data.min_x as f64);
                    let nz = (p.z as f64 - map.scale_data.min_z as f64)
                        / (map.scale_data.max_z as f64 - map.scale_data.min_z as f64);
                    (nx * w, nz * h)
                };

                for line in lines.iter() {
                    if !line.active || line.map_id != map.map_id { continue; }

                    let (x1, y1) = project(&line.position1);
                    let (x2, y2) = project(&line.position2);

                    let (r, g, b, _a) = line.colour;
                    let rgba = format!("rgba({},{},{},{})", r, g, b, 0.9);

                    ctx.begin_path();
                    ctx.set_stroke_style_str(&rgba);
                    ctx.set_line_width(2.0 / zoom.max(0.0001));
                    ctx.move_to(x1, y1);
                    ctx.line_to(x2, y2);
                    ctx.stroke();
                    ctx.close_path();
                }

                let base = *canvas_width as f64 / 30.0;
                for marker in markers.iter() {
                    match marker {
                        Marker::Elms(elms_marker) => {
                            if !elms_marker.active { continue; }

                            let (mx, mz) = {
                                let nx = (elms_marker.position.x as f64 - map.scale_data.min_x as f64)
                                    / (map.scale_data.max_x as f64 - map.scale_data.min_x as f64);
                                let nz = (elms_marker.position.z as f64 - map.scale_data.min_z as f64)
                                    / (map.scale_data.max_z as f64 - map.scale_data.min_z as f64);
                                (nx * w, nz * h)
                            };

                            let display_size = base * (1.0 / zoom) * (elms_marker.size as f64);
                            let dx = mx - display_size / 2.0;
                            let dy = mz - display_size / 2.0;
                            let icon_img = HtmlImageElement::new().unwrap();
                            icon_img.set_src(&format!("static/icons/{}", String::from(elms_marker.icon)));

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
                        },
                        Marker::M0r(m0r_marker) => {
                            if !m0r_marker.active { continue; }
                            let offscreen: HtmlCanvasElement = web_sys::window()
                                .unwrap()
                                .document()
                                .unwrap()
                                .create_element("canvas")
                                .unwrap()
                                .dyn_into::<HtmlCanvasElement>()
                                .unwrap();

                            let off_ctx: CanvasRenderingContext2d = offscreen
                                .get_context("2d")
                                .unwrap()
                                .unwrap()
                                .dyn_into::<CanvasRenderingContext2d>()
                                .unwrap();
                            let (mx, mz) = {
                                let nx = (m0r_marker.position.x as f64 - map.scale_data.min_x as f64)
                                    / (map.scale_data.max_x as f64 - map.scale_data.min_x as f64);
                                let nz = (m0r_marker.position.z as f64 - map.scale_data.min_z as f64)
                                    / (map.scale_data.max_z as f64 - map.scale_data.min_z as f64);
                                (nx * w, nz * h)
                            };

                            //let display_size = ((1.0 + m0r_marker.size as f64) * 100.0 / map_x_range) * w;
                            let display_size = base * (1.0 / zoom) * (1.0 + (m0r_marker.size as f64 / 5.0));
                            let dx = mx - display_size / 2.0;
                            let dy = mz - display_size / 2.0;

                            let icon_img = HtmlImageElement::new().unwrap();
                            icon_img.set_src(&format!("static/icons/m0r/{}", String::from(m0r_marker.background_texture.clone())));
                            let rgba = format!("rgba({},{},{},{})", m0r_marker.colour.0, m0r_marker.colour.1, m0r_marker.colour.2, m0r_marker.colour.3);

                            let draw_marker = move |ctx: &CanvasRenderingContext2d, icon: &HtmlImageElement| {
                                offscreen.set_width(base as u32);
                                offscreen.set_height(base as u32);

                                off_ctx.set_global_composite_operation("multiply").unwrap();
                                off_ctx.set_fill_style_str(&rgba);
                                off_ctx.fill_rect(0.0, 0.0, base, base);
                                off_ctx.set_global_composite_operation("destination-in").unwrap();
                                off_ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                                    icon,
                                    0.0, 0.0,
                                    icon.width() as f64,
                                    icon.height() as f64,
                                    0.0, 0.0,
                                    base, base,
                                ).unwrap();

                                ctx.save();
                                ctx.translate(dx + display_size / 2.0, dy + display_size / 2.0).unwrap();
                                ctx.scale(display_size / base, display_size / base).unwrap();
                                ctx.draw_image_with_html_canvas_element(&offscreen, -base/2.0, -base/2.0).unwrap();
                                ctx.restore();
                            };

                            if icon_img.complete() {
                                draw_marker(&ctx, &icon_img);
                            } else {
                                let ctx_clone = ctx.clone();
                                let icon_clone = icon_img.clone();
                                let draw_closure = Closure::wrap(Box::new(move || {
                                    draw_marker(&ctx_clone, &icon_clone);
                                }) as Box<dyn Fn()>);
                                icon_img.set_onload(Some(draw_closure.as_ref().unchecked_ref()));
                                draw_closure.forget();
                            }

                            if let Some(text) = &m0r_marker.text {
                                ctx.save();
                                ctx.set_font(format!("{}px Univers", 12.0/zoom).as_str());
                                ctx.set_fill_style_str("white");
                                ctx.set_stroke_style_str("black");
                                ctx.set_line_width(3.0/zoom);
                                ctx.set_text_align("center");
                                ctx.set_text_baseline("top");

                                let text_x = dx + display_size / 2.0;
                                let text_y = dy + display_size / 2.0;

                                ctx.stroke_text(&text, text_x, text_y).unwrap();
                                ctx.fill_text(&text, text_x, text_y).unwrap();
                                ctx.restore();
                            }
                        },
                    }
                }

                || ()
        });
    }

    html! {
        <div style={format!(
            "position: relative; min-width: 475px; min-height: 475px; cursor: grab; width: {}px; height: {}px;",
            canvas_width, canvas_height
        )}>
            <canvas ref={map_canvas_ref} 
                style="position: absolute; top: 0; left: 0; width: 100%; height: 100%;" />
            <canvas ref={marker_canvas_ref} 
                style="position: absolute; top: 0; left: 0; width: 100%; height: 100%;" />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MarkerListPanelProps {
    pub zone_markers: Vec<Marker>,
    pub current_markers: Vec<Marker>,
    pub on_update: Callback<Vec<Marker>>,
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
    // web_sys::console::log_1(&format!("current: {current:?}").into());

    let delete_style = Style::new(css!(r#"
        color: #fff;
        transition: color 0.3s, scale 0.3s;
        cursor: pointer;
        margin-left: 1em;
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
                    "x" => if let Ok(x) = val.parse() { 
                        match m {
                            Marker::Elms(marker) => {marker.position.x = x},
                            Marker::M0r(marker) => {marker.position.x = x}
                        } 
                    },
                    "y" => if let Ok(y) = val.parse() { 
                        match m {
                            Marker::Elms(marker) => {marker.position.y = y},
                            Marker::M0r(marker) => {marker.position.y = y}
                        } 
                    },
                    "z" => if let Ok(z) = val.parse() { 
                        match m {
                            Marker::Elms(marker) => {marker.position.z = z},
                            Marker::M0r(marker) => {marker.position.z = z}
                        } 
                    },
                    "icon" => {
                        match m {
                            Marker::Elms(marker) => {marker.icon = val.as_str().into()},
                            Marker::M0r(marker) => {marker.background_texture = val.as_str().into()}
                        }
                    },
                    "size" => {
                        match m {
                            Marker::M0r(marker) => {marker.size = val.parse::<f32>().unwrap_or(1.0)}
                            _ => {},
                        }
                    },
                    "pitch" => {
                        match m {
                            Marker::M0r(marker) => {
                                let new_pitch = val.parse::<i8>();
                                if new_pitch.is_err() {marker.orientation = None} else {
                                    let pitch = new_pitch.unwrap().clamp(-90, 90);
                                    if let Some(existing) = marker.orientation {
                                        marker.orientation = Some((pitch, existing.1));
                                    } else {
                                        marker.orientation = Some((pitch, 0));
                                    }
                                }
                            }
                            _ => {},
                        }
                    },
                    "yaw" => {
                        match m {
                            Marker::M0r(marker) => {
                                let new_yaw = val.parse::<i16>();
                                if new_yaw.is_err() {marker.orientation = None} else {
                                    let yaw = new_yaw.unwrap().clamp(0, 360);
                                    if let Some(existing) = marker.orientation {
                                        marker.orientation = Some((existing.0, yaw));
                                    } else {
                                        marker.orientation = Some((0, yaw));
                                    }
                                }
                            }
                            _ => {},
                        }
                    },
                    "text" => {
                        match m {
                            Marker::M0r(marker) => {
                                if val.is_empty() {
                                    marker.text = None;
                                } else {
                                    marker.text = Some(val);
                                }
                            }
                            _ => {},
                        }
                    },
                    "colour" => {
                        match m {
                            Marker::M0r(marker) => {
                                let x = val.trim_start_matches('#').parse::<u32>().unwrap_or(65535);
                                marker.colour = hex_to_rgba(x);
                            }
                            _ => {},
                        }
                    },
                    "colour_tuple" => {
                        match m {
                            Marker::M0r(marker) => {
                                let nums: Vec<u8> = val
                                    .trim()
                                    .trim_start_matches('(')
                                    .trim_end_matches(')')
                                    .split(',')
                                    .filter_map(|p| p.trim().parse::<u8>().ok())
                                    .collect();
                                if nums.len() >= 4 {
                                    let r = nums[0];
                                    let g = nums[1];
                                    let b = nums[2];
                                    let a = nums[3];
                                    marker.colour = (r, g, b, a);
                                }
                            }
                            _ => {},
                        }
                    },
                    _ => {}
                }
            }
            current.set(new_current.clone());
            let rebuilt: Vec<Marker> = zone_for_upd
                .iter()
                .map(|zm| {
                    new_current
                        .iter()
                        .find(|cm| get_marker_id(*cm) == get_marker_id(zm))
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
                set_marker_active(m, is_on);
            }
            current.set(new_current.clone());
            let rebuilt = zone_for_toggle
                .iter()
                .map(|zm| {
                    new_current
                        .iter()
                        .find(|cm| get_marker_id(*cm) == get_marker_id(zm))
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
            if let Some(to_delete) = (*current).get(pos).map(|m| get_marker_id(m)) {
                let filtered_full: Vec<Marker> = zone_for_delete
                    .iter()
                    .cloned()
                    .filter(|zm| get_marker_id(zm) != to_delete)
                    .collect();

                let new_current: Vec<Marker> = (*current)
                    .iter()
                    .filter(|cm| get_marker_id(&**cm) != to_delete)
                    .cloned()
                    .collect();
                current.set(new_current);

                emit_for_delete.emit(filtered_full);
            }
        })
    };

    let input_text_style = css!("width: 5em; height: 1em; flex-basis: 26%;");

    html! {
        <div style="display:flex;flex-direction:column;max-height:85vh;margin-bottom:5vh;text-shadow: 2px 1.5px black;">
            <h1 style="text-align:center;">{"Markers"}</h1>
            <div style="overflow-y:auto;">
                <ul style="padding:0;margin:0;list-style:none;display:flex;flex-direction:column;">
                { for current.iter().enumerate().map(|(i, marker)| {
                    let upd = update_marker.clone();
                    let tog = toggle_active.clone();
                    let del = delete_marker.clone();
                    let up_size = update_marker.clone();
                    let up_pitch = update_marker.clone();
                    let up_yaw = update_marker.clone();
                    let up_text = update_marker.clone();
                    let picker = icon_picker_for.clone();
                    let input_text_style = input_text_style.clone();
                    let input_text_style_clone = input_text_style.clone();
                    match marker {
                        Marker::Elms(marker) => {
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
                                        "x" => marker.position.x,
                                        "y" => marker.position.y,
                                        "z" => marker.position.z,
                                        _ => 0,
                                    };

                                    let (min, max) = match axis {
                                        "x" => (props.world_bounds.0 as i32, props.world_bounds.1 as i32),
                                        "z" => (props.world_bounds.2 as i32, props.world_bounds.3 as i32),
                                        _ => (-1e6 as i32, 1e6 as i32),
                                    };

                                    let val_str = val.to_string();
                                    let is_out_of_bounds = val < min || val > max || val == 0;

                                    html! {
                                    <label>
                                        {format!("{}: ", axis.to_uppercase())}
                                        <input
                                            type="number"
                                            min={min.to_string()}
                                            max={max.to_string()}
                                            step="25"
                                            title=""
                                            value={val_str.clone()}
                                            oninput={Callback::from(move |e: InputEvent| {
                                                let inp: HtmlInputElement = e.target_unchecked_into();
                                                up.emit((i, axis.to_string(), inp.value()));
                                            })}
                                            style={format!(
                                                "width:6em;{}",
                                                if is_out_of_bounds { " outline: 1px solid red;" } else { "" }
                                            )}
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
                        }
                        Marker::M0r(marker) => {
                            html!{
                                <li key={marker.id.clone()} style="display:flex;align-items:center;gap:1em;justify-content:center;padding:4px;">
                                    <img
                                        src={format!("static/icons/m0r/{}", String::from(marker.background_texture.clone()))}
                                        style="height:2em;cursor:pointer;"
                                        onclick={Callback::from(move |_| picker.set(Some(i)))}
                                    />

                                    <div class={css!("display:flex;flex-flow: column wrap; max-height: 4em;")}>
                                        {for ["x","y","z"].iter().map(move |&axis| {
                                            let up = upd.clone();
                                            let val = match axis {
                                                "x" => marker.position.x,
                                                "y" => marker.position.y,
                                                "z" => marker.position.z,
                                                _ => 0,
                                            };

                                            let (min, max) = match axis {
                                                "x" => (props.world_bounds.0 as i32, props.world_bounds.1 as i32),
                                                "z" => (props.world_bounds.2 as i32, props.world_bounds.3 as i32),
                                                _ => (-1e6 as i32, 1e6 as i32),
                                            };

                                            let val_str = val.to_string();
                                            let is_out_of_bounds = val < min || val > max || val == 0;

                                            html! {
                                                <label>
                                                    <input
                                                        type="number"
                                                        min={min.to_string()}
                                                        max={max.to_string()}
                                                        step="25"
                                                        title={format!("{}", axis.to_uppercase())}
                                                        value={val_str.clone()}
                                                        oninput={Callback::from(move |e: InputEvent| {
                                                            let inp: HtmlInputElement = e.target_unchecked_into();
                                                            up.emit((i, axis.to_string(), inp.value()));
                                                        })}
                                                        class={
                                                            if is_out_of_bounds { classes!(css!("outline: 1px solid red;"), input_text_style.clone()) } else { classes!(input_text_style.clone()) }
                                                        }
                                                    />
                                                </label>
                                            }
                                        })}
                                    </div>
                                    <div class={css!("display:flex;flex-flow: column wrap; max-height: 4em; text-align: right;")}>
                                        <label>
                                            <input
                                                type="number"
                                                min="0"
                                                max="10"
                                                step="0.1"
                                                title={format!("{} ({} to {})", "Size", "0.0", "10.0")}
                                                value={marker.size.to_string().clone()}
                                                oninput={Callback::from(move |e: InputEvent| {
                                                    let inp: HtmlInputElement = e.target_unchecked_into();
                                                    up_size.emit((i, "size".to_string(), inp.value()));
                                                })}
                                                class={classes!(input_text_style_clone.clone())}
                                            />
                                        </label>
                                        <label>
                                            <input
                                                type="number"
                                                min="-90"
                                                max="90"
                                                step="1"
                                                title={format!("{} ({} to {})", "Pitch", "-90", "90")}
                                                value={
                                                    if let Some(existing) = marker.orientation {
                                                        existing.0.to_string()
                                                    } else {
                                                        "-".to_string()
                                                    }
                                                }
                                                oninput={Callback::from(move |e: InputEvent| {
                                                    let inp: HtmlInputElement = e.target_unchecked_into();
                                                    up_pitch.emit((i, "pitch".to_string(), inp.value()));
                                                })}
                                                class={classes!(input_text_style_clone.clone())}
                                            />
                                        </label>
                                        <label>
                                            <input
                                                type="number"
                                                min="0"
                                                max="360"
                                                step="1"
                                                title={format!("{} ({} to {})", "Yaw", "0", "360")}
                                                value={
                                                    if let Some(existing) = marker.orientation {
                                                        existing.1.to_string()
                                                    } else {
                                                        "-".to_string()
                                                    }
                                                }
                                                oninput={Callback::from(move |e: InputEvent| {
                                                    let inp: HtmlInputElement = e.target_unchecked_into();
                                                    up_yaw.emit((i, "yaw".to_string(), inp.value()));
                                                })}
                                                class={classes!(input_text_style_clone.clone())}
                                            />
                                        </label>
                                    </div>
                                    <textarea
                                        oninput={Callback::from(move |e: InputEvent| {
                                            let inp: HtmlInputElement = e.target_unchecked_into();
                                            up_text.emit((i, "text".to_string(), inp.value()));
                                        })}
                                        value={marker.text.clone().unwrap_or("".to_string())}
                                        placeholder=""
                                        title={"Marker text"}
                                        style="
                                            width: 10em;
                                            height: 3em;
                                            padding: 0.5em;
                                            resize: none;
                                        "
                                    />
                                    <label>
                                        <input
                                            type="checkbox"
                                            checked={marker.active}
                                            style="cursor:pointer;"
                                            title="Enabled"
                                            onchange={Callback::from(move |e: Event| {
                                                let inp: HtmlInputElement = e.target_unchecked_into();
                                                tog.emit((i, inp.checked()));
                                            })}
                                        />
                                    </label>
                                    <Icon
                                        class={delete_style.clone()}
                                        style="cursor:pointer;"
                                        title={"Delete"}
                                        width={"1em"}
                                        height={"1em"}
                                        icon_id={IconId::BootstrapXLg}
                                        onclick={Callback::from(move |_| del.emit(i))}
                                    />
                                </li>
                            }
                        },
                    }
                }) }
                </ul>
            </div>

            {
                if let Some(idx) = *icon_picker_for {
                    let upd_icon = update_marker.clone();
                    let upd_colour = update_marker.clone();
                    let upd_colour_rgba = update_marker.clone();
                    let marker = current.get(idx).unwrap();
                    let marker_type = match marker {
                        Marker::Elms(_) => {
                            MarkerTypes::Elms
                        },
                        Marker::M0r(_) => {
                            MarkerTypes::M0r
                        }
                    };
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
                            <h2 style="text-align:center;margin:10px 0px 1em 0px;">{"Select Icon"}</h2>
                            <div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(50px,1fr));gap:8px;">
                                {
                                    match marker_type {
                                        MarkerTypes::Elms => html! {
                                            for ALL_ELMS_ICONS.iter().map(|icon| {
                                                let choose = choose.clone();
                                                let icon_name = String::from(icon.clone());
                                                let src = format!("static/icons/{}", icon_name.clone());
                                                html! {
                                                    <div style="text-align:center;">
                                                        <img
                                                            src={src}
                                                            style="width:2em;height:2em;cursor:pointer;"
                                                            onclick={Callback::from(move |_| choose.emit(icon_name.clone()))}
                                                        />
                                                    </div>
                                                }
                                            })
                                        },
                                        MarkerTypes::M0r => html! {
                                            for ALL_M0R_ICONS.iter().map(|icon| {
                                                let choose = choose.clone();
                                                let icon_name = String::from(icon.clone());
                                                let src = format!("static/icons/m0r/{}", icon_name.clone());
                                                html! {
                                                    <div style="text-align:center;">
                                                        <img
                                                            src={src}
                                                            style="width:2em;height:2em;cursor:pointer;"
                                                            onclick={Callback::from(move |_| choose.emit(m0r_texture_to_og(icon)))}
                                                        />
                                                    </div>
                                                }
                                            })
                                        }
                                    }
                                }
                            </div>
                            {
                                match marker {
                                    M0r(m) => html!{
                                        <>
                                            <h3 style="text-align:center;margin:10px 0px 1em 0px;">{"Use Custom File Path"}</h3>
                                            <textarea
                                                oninput={Callback::from(move |e: InputEvent| {
                                                    let inp: HtmlInputElement = e.target_unchecked_into();
                                                    upd_icon.emit((idx, "icon".to_string(), inp.value()));
                                                })}
                                                // value={m0r_texture_to_og(&m.background_texture)}
                                                placeholder="esoui/art/icons/blank.dds"
                                                title={"Custom icon"}
                                                style="
                                                    width: 20em;
                                                    height: 1em;
                                                    padding: 0.5em;
                                                    resize: none;
                                                "
                                            />
                                            <h3 style="text-align:center;margin:10px 0px 1em 0px;">{"Change Colour"}</h3>
                                            <textarea
                                                oninput={Callback::from(move |e: InputEvent| {
                                                    let inp: HtmlInputElement = e.target_unchecked_into();
                                                    upd_colour.emit((idx, "colour".to_string(), inp.value()));
                                                })}
                                                placeholder={
                                                    let (r, g, b, a) = m.colour;
                                                    let rgba_hex = rgba_to_hex_string((r, g, b, a));
                                                    format!("{}", rgba_hex)
                                                }
                                                title={"Hex Code"}
                                                style="
                                                    width: min-content;
                                                    height: 1em;
                                                    padding: 0.5em;
                                                    resize: none;
                                                "
                                            />
                                            <textarea
                                                oninput={Callback::from(move |e: InputEvent| {
                                                    let inp: HtmlInputElement = e.target_unchecked_into();
                                                    let raw = inp.value();
                                                    let nums: Vec<u8> = raw
                                                        .trim()
                                                        .trim_start_matches('(')
                                                        .trim_end_matches(')')
                                                        .split(',')
                                                        .filter_map(|p| p.trim().parse::<u8>().ok())
                                                        .collect();
                                                    if nums.len() >= 4 {
                                                        let r = nums[0];
                                                        let g = nums[1];
                                                        let b = nums[2];
                                                        let a = nums[3];
                                                        upd_colour_rgba.emit((idx, "colour_tuple".to_string(), format!("{}, {}, {}, {}", r, g, b, a)));
                                                    }
                                                })}
                                                placeholder={
                                                    let (r, g, b, a) = m.colour;
                                                    format!("({}, {}, {}, {})", r, g, b, a)
                                                }
                                                title={"R, G, B, A"}
                                                style="
                                                    min-width: min-content;
                                                    height: 1em;
                                                    padding: 0.5em;
                                                    resize: none;
                                                "
                                            />
                                        </>
                                    },
                                    _ => {html!{}}
                                }
                            }
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
    let marker_input = use_state(|| String::new());
    let parsed_markers = use_state(|| HashMap::<u16, Vec<Marker>>::new());
    let parsed_lines = use_state(|| HashMap::<u16, Vec<BreadcrumbLine>>::new());
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
                    let pos_y = scale.y.unwrap_or(0.0);
                    let pos_z = scale.min_z as f64 + nz * (scale.max_z - scale.min_z) as f64;

                    let marker = ElmMarker {
                        active: true,
                        icon: "squares/marker_lightblue.png".into(),
                        position: Position3D {
                            x: pos_x.round() as i32,
                            y: pos_y.round() as i32,
                            z: pos_z.round() as i32,
                        },
                        size: 1,
                        id: parsed_markers.get(&zone.id).map(|v| v.len() as u16).unwrap_or(0),
                        map_id: map.map_id,
                    };

                    let mut new_map = (*parsed_markers).clone();
                    let entry = new_map.entry(zone.id).or_default();
                    entry.push(Elms(marker));
                    parsed_markers.set(new_map);
                }
            }
        })
    };

    let oninput = {
        let elms_input = marker_input.clone();
        let parsed_markers = parsed_markers.clone();
        let parsed_lines = parsed_lines.clone();
        let zones = zones.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let v = input.value();
            // web_sys::console::log_1(&format!("Textarea input: '{}'", v).into());

            elms_input.set(v.clone());

            if !v.is_empty() {
                let new_m0r_markers = parse_m0r_string(&v, zones.clone());

                let new_elms_markers = parse_elms_string(&v, zones.clone());

                let mut combined_markers = new_m0r_markers;

                for (key, mut markers) in new_elms_markers {
                    combined_markers
                        .entry(key)
                        .and_modify(|existing| existing.append(&mut markers))
                        .or_insert(markers);
                }

                for (_zone_id, markers) in combined_markers.iter_mut() {
                    let mut next_id: u16 = 0;
                    for m in markers.iter_mut() {
                        match m {
                            Marker::Elms(el) => el.id = next_id,
                            Marker::M0r(m0r) => m0r.id = next_id,
                        }
                        next_id = next_id.saturating_add(1);
                    }
                }
                
                web_sys::console::log_1(&format!("{:?}", combined_markers).into());
                parsed_markers.set(combined_markers);

                let new_lines = parse_lines_string(&v, zones.clone());
                parsed_lines.set(new_lines);
            } else {
                parsed_markers.set(HashMap::new());
                parsed_lines.set(HashMap::new());
            }
        })
    };

    {
        let parsed_markers = parsed_markers.clone();
        let parsed_lines = parsed_lines.clone();
        let zone_ids = zone_ids.clone();
        let selected_zone_index = selected_zone_index.clone();
        let selected_map_index = selected_map_index.clone();
        let zoom = zoom.clone();
        let pan = pan.clone();
        use_effect_with((parsed_markers.clone(), parsed_lines.clone()),
            move |(parsed_markers, parsed_lines)| {
                let mut zones: Vec<u16> = parsed_markers.keys().cloned().collect();
                zones.extend(parsed_lines.keys().cloned());
                zones.sort();
                zones.dedup();
                // web_sys::console::log_1(&format!("use_effect parsed_markers zones: {:?}", zones).into());

                if !zones.is_empty() {
                    let current_zone = zone_ids[*selected_zone_index];
                    let first_zone = zones[0];

                    if first_zone != current_zone {
                        if let Some(idx) = zone_ids.iter().position(|&z| z == first_zone) {
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

    let do_redraw = use_state(|| false);
    let update_elms_input = {
        let parsed_markers = parsed_markers.clone();
        let parsed_lines = parsed_lines.clone();
        let elms_input = marker_input.clone();
        let selected_zone_index = selected_zone_index.clone();
        let redraw = do_redraw.clone();
        let zone_ids = zone_ids.clone();

        Callback::from(move |update: (Option<Vec<Marker>>, Option<Vec<BreadcrumbLine>>)| {
            let (maybe_markers, maybe_lines) = update;
            let zone_id = zone_ids[*selected_zone_index];

            if let Some(markers) = maybe_markers {
                let mut map = (*parsed_markers).clone();
                map.insert(zone_id, markers);
                parsed_markers.set(map);
            }
            if let Some(lines) = maybe_lines {
                let mut map = (*parsed_lines).clone();
                map.insert(zone_id, lines);
                parsed_lines.set(map);
            }
            let markers_map = (*parsed_markers).clone();
            let lines_map = (*parsed_lines).clone();

            let m0r_markers_str = build_m0r_string(&markers_map);
            let elms_markers_str = build_elms_string(&markers_map);
            let lines_str = lines_to_string(&lines_map);

            let combined = format!("{}\n{}\n{}", m0r_markers_str, elms_markers_str, lines_str);
            elms_input.set(combined);
            redraw.set(!*redraw);
        })
    };

    let update_markers = {
        let cb = update_elms_input.clone();
        Callback::from(move |markers: Vec<Marker>| {
            cb.emit((Some(markers), None));
        })
    };

    let update_lines = {
        let cb = update_elms_input.clone();
        Callback::from(move |lines: Vec<BreadcrumbLine>| {
            cb.emit((None, Some(lines)));
        })
    };

    
    {
        let update_markers = update_markers.clone();
        let update_lines = update_lines.clone();
        let zone_ids_markers = zone_ids.clone();
        let zone_ids_lines = zone_ids.clone();
        let selected_zone_markers = selected_zone_index.clone();
        let selected_zone_lines = selected_zone_index.clone();

        use_effect_with(parsed_markers.clone(), move |pm| {
            let current_zone_id = zone_ids_markers[*selected_zone_markers];
            if let Some(markers) = pm.get(&current_zone_id).cloned() {
                update_markers.emit(markers);
            }
            || ()
        });
        use_effect_with(parsed_lines.clone(), move |pl| {
            let current_zone_id = zone_ids_lines[*selected_zone_lines];
            if let Some(lines) = pl.get(&current_zone_id).cloned() {
                update_lines.emit(lines);
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
                    let size = (w.min(h).max(475.0)) as u32;
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
    // web_sys::console::log_1(&format!("zone_markers: {:?}", zone_markers).into());
    let zone_marker_clone = zone_markers.clone();
    let other_current_markers: Vec<Marker> = zone_markers.into_iter()
        .filter(|m| {
            match m {
                Elms(elms_marker) => {
                    elms_marker.map_id == map.map_id
                }
                M0r(m0r_marker) => {
                    m0r_marker.map_id == map.map_id
                }
            }
        })
        .collect();

    let zone_lines = parsed_lines.get(&zone.id).cloned().unwrap_or_default();
    let zone_lines_clone = zone_lines.clone();
    let current_lines: Vec<BreadcrumbLine> = zone_lines.into_iter()
        .filter(|l| {l.map_id == map.map_id})
        .collect();

    html! {
        <div style={format!("display: flex; background-color: #333; color: #fff; font-family: 'Univers', sans-serif; max-height: {}px; flex-wrap: wrap;", *canvas_size)}>
            <div
                {onwheel}
                {onmousedown}
                {onmouseup}
                {onmousemove}
                {oncontextmenu}
                style={format!("
                    display: flex;
                    flex-flow: column nowrap;
                    box-sizing: border-box;
                    flex-shrink: 1;
                    flex-basis: 475px;
                    justify-content: center;
                    text-align: center;
                    width: {}px;
                    height: {}px;
                ", canvas_width, canvas_height)}>
                <CanvasMap
                    map={map.clone()}
                    markers={other_current_markers.clone()}
                    lines = {current_lines.clone()}
                    zoom={zoom}
                    pan={pan}
                    width={canvas_width}
                    height={canvas_height}
                    force_redraw={*do_redraw}
                />
            </div>

            <div style="position: absolute; top: 1em; left: 1em;">
            if zone_marker_clone.len() > 0 || zone_lines_clone.len() > 0 {
                if zone.maps.len() > 1 {
                    <select onchange={on_map_change}>
                        {
                            for zone.maps.iter().enumerate().map(|(i, map)| {
                                let marker_count = zone_marker_clone
                                    .iter()
                                    .filter(|m| {
                                        match m {
                                            Elms(elms_marker) => {
                                                elms_marker.map_id == map.map_id
                                            }
                                            M0r(m0r_marker) => {
                                                m0r_marker.map_id == map.map_id
                                            }
                                        }
                                    })
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
                        value={(*marker_input).clone()}
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
                    <MarkerListPanel zone_markers={zone_marker_clone} current_markers={other_current_markers} on_update={update_markers} world_bounds={(map.scale_data.min_x, map.scale_data.max_x, map.scale_data.min_z, map.scale_data.max_z)} />
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
