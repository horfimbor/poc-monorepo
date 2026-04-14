use horfimbor_client_derive::WebComponent;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use weblog::console_info;
use yew::prelude::*;

// FIXME PLACEHOLDER AI SLOP TO REMOVE
// FIXME PLACEHOLDER AI SLOP TO REMOVE
// FIXME PLACEHOLDER AI SLOP TO REMOVE
// FIXME PLACEHOLDER AI SLOP TO REMOVE
// FIXME PLACEHOLDER AI SLOP TO REMOVE
// FIXME PLACEHOLDER AI SLOP TO REMOVE
// FIXME PLACEHOLDER AI SLOP TO REMOVE
// FIXME PLACEHOLDER AI SLOP TO REMOVE
// FIXME PLACEHOLDER AI SLOP TO REMOVE
// FIXME PLACEHOLDER AI SLOP TO REMOVE

// ─── Biome palette constants ────────────────────────────────────────────────
// Three anchor palettes: cold (0.0), temperate (0.5), hot (1.0)
// Each has four elevation zones: ocean, lowland, highland, peak

const COLD_OCEAN: [u8; 3] = [70, 100, 140];
const COLD_LOWLAND: [u8; 3] = [200, 215, 230];
const COLD_HIGHLAND: [u8; 3] = [220, 230, 245];
const COLD_PEAK: [u8; 3] = [245, 248, 255];

const TEMP_OCEAN: [u8; 3] = [30, 80, 160];
const TEMP_LOWLAND: [u8; 3] = [60, 130, 60];
const TEMP_HIGHLAND: [u8; 3] = [110, 85, 60];
const TEMP_PEAK: [u8; 3] = [180, 175, 170];

const HOT_OCEAN: [u8; 3] = [20, 15, 10];
const HOT_LOWLAND: [u8; 3] = [60, 40, 20];
const HOT_HIGHLAND: [u8; 3] = [120, 50, 10];
const HOT_PEAK: [u8; 3] = [200, 90, 5];

// ─── Noise helpers ───────────────────────────────────────────────────────────

fn hash_u32(mut x: u32) -> u32 {
    x = x.wrapping_add(0x9e3779b9);
    x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3b);
    x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3b);
    (x >> 16) ^ x
}

fn hash3(ix: i32, iy: i32, iz: i32, seed: u64) -> f32 {
    let s = seed as u32;
    let h = hash_u32(
        hash_u32(ix.wrapping_add(s as i32) as u32)
            ^ hash_u32(iy as u32)
            ^ hash_u32(iz as u32 ^ (seed >> 32) as u32),
    );
    (h as f32) / (u32::MAX as f32)
}

fn smoothstep(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

fn value_noise_3d(x: f32, y: f32, z: f32, seed: u64) -> f32 {
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    let iz = z.floor() as i32;
    let fx = smoothstep(x - x.floor());
    let fy = smoothstep(y - y.floor());
    let fz = smoothstep(z - z.floor());
    let v = |dx: i32, dy: i32, dz: i32| hash3(ix + dx, iy + dy, iz + dz, seed);
    lerp(
        lerp(
            lerp(v(0, 0, 0), v(1, 0, 0), fx),
            lerp(v(0, 1, 0), v(1, 1, 0), fx),
            fy,
        ),
        lerp(
            lerp(v(0, 0, 1), v(1, 0, 1), fx),
            lerp(v(0, 1, 1), v(1, 1, 1), fx),
            fy,
        ),
        fz,
    )
}

fn fbm(x: f32, y: f32, z: f32, octaves: u32, seed: u64) -> f32 {
    let mut val = 0.0f32;
    let mut amp = 0.5f32;
    let mut freq = 1.0f32;
    let mut max = 0.0f32;
    for _ in 0..octaves {
        val += amp * value_noise_3d(x * freq, y * freq, z * freq, seed);
        max += amp;
        amp *= 0.5;
        freq *= 2.0;
    }
    val / max
}

// ─── Biome palette ───────────────────────────────────────────────────────────

struct Palette {
    ocean: [u8; 3],
    lowland: [u8; 3],
    highland: [u8; 3],
    peak: [u8; 3],
}

fn lerp_color(a: [u8; 3], b: [u8; 3], t: f32) -> [u8; 3] {
    [
        (a[0] as f32 + t * (b[0] as f32 - a[0] as f32)) as u8,
        (a[1] as f32 + t * (b[1] as f32 - a[1] as f32)) as u8,
        (a[2] as f32 + t * (b[2] as f32 - a[2] as f32)) as u8,
    ]
}

fn get_palette(temperature: f32) -> Palette {
    if temperature <= 0.5 {
        let t = temperature / 0.5;
        Palette {
            ocean: lerp_color(COLD_OCEAN, TEMP_OCEAN, t),
            lowland: lerp_color(COLD_LOWLAND, TEMP_LOWLAND, t),
            highland: lerp_color(COLD_HIGHLAND, TEMP_HIGHLAND, t),
            peak: lerp_color(COLD_PEAK, TEMP_PEAK, t),
        }
    } else {
        let t = (temperature - 0.5) / 0.5;
        Palette {
            ocean: lerp_color(TEMP_OCEAN, HOT_OCEAN, t),
            lowland: lerp_color(TEMP_LOWLAND, HOT_LOWLAND, t),
            highland: lerp_color(TEMP_HIGHLAND, HOT_HIGHLAND, t),
            peak: lerp_color(TEMP_PEAK, HOT_PEAK, t),
        }
    }
}

fn elevation_color(elevation: f32, palette: &Palette) -> [u8; 3] {
    if elevation < 0.40 {
        palette.ocean
    } else if elevation < 0.55 {
        let t = smoothstep((elevation - 0.40) / 0.15);
        lerp_color(palette.ocean, palette.lowland, t)
    } else if elevation < 0.75 {
        let t = smoothstep((elevation - 0.55) / 0.20);
        lerp_color(palette.lowland, palette.highland, t)
    } else {
        let t = smoothstep(((elevation - 0.75) / 0.25).min(1.0));
        lerp_color(palette.highland, palette.peak, t)
    }
}

// ─── UUID → seed + temperature ───────────────────────────────────────────────

fn parse_id(id: &str) -> Option<(u64, f32)> {
    console_info!(id);

    let uuid = id.char_indices().rev().nth(35).map(|(i, _)| &id[i..])?;
    console_info!(uuid);
    let uuid = uuid::Uuid::parse_str(uuid).ok()?;
    let bytes = uuid.as_bytes();
    let seed = u64::from_le_bytes(bytes[0..8].try_into().ok()?);
    let temp_raw = u16::from_le_bytes([bytes[8], bytes[9]]);
    Some((seed, temp_raw as f32 / 65535.0))
}

// ─── Planet pixel renderer ───────────────────────────────────────────────────

fn render_planet(seed: u64, temperature: f32) -> Vec<u8> {
    const SIZE: u32 = 300;
    const CX: f32 = 149.5;
    const CY: f32 = 149.5;
    const RADIUS: f32 = 148.0;

    // Light direction (normalized): (0.5, 0.5, 0.7) / ||(0.5, 0.5, 0.7)||
    // magnitude = sqrt(0.25 + 0.25 + 0.49) ≈ 0.9950
    const LX: f32 = 0.5 / 0.9950;
    const LY: f32 = 0.5 / 0.9950;
    const LZ: f32 = 0.7 / 0.9950;

    let cloud_seed = seed ^ 0xDEAD_BEEF_1234_5678u64;
    let palette = get_palette(temperature);

    let mut pixels = vec![0u8; (SIZE * SIZE * 4) as usize];

    for py in 0..SIZE {
        for px in 0..SIZE {
            let nx = (px as f32 - CX) / RADIUS;
            let ny = -((py as f32 - CY) / RADIUS); // flip Y so north is up
            let dist2 = nx * nx + ny * ny;

            if dist2 > 1.0 {
                // outside sphere: leave transparent (already zero)
                continue;
            }

            let nz = (1.0 - dist2).sqrt();

            // Terrain FBM (5 octaves, scale 3.0 gives continent-sized features)
            let elevation = fbm(nx * 3.0, ny * 3.0, nz * 3.0, 5, seed);
            let base_color = elevation_color(elevation, &palette);

            // Lambert shading
            let diffuse = (nx * LX + ny * LY + nz * LZ).max(0.0);
            let shade = 0.3 + 0.7 * diffuse;

            let mut r = (base_color[0] as f32 * shade) as u8;
            let mut g = (base_color[1] as f32 * shade) as u8;
            let mut b = (base_color[2] as f32 * shade) as u8;

            // Cloud overlay (4 octaves, scale 4.0 for tighter wisps)
            let cloud = fbm(nx * 4.0, ny * 4.0, nz * 4.0, 4, cloud_seed);
            if cloud > 0.60 {
                let cloud_alpha = ((cloud - 0.60) / 0.40).min(1.0) * shade;
                r = lerp(r as f32, 230.0, cloud_alpha) as u8;
                g = lerp(g as f32, 230.0, cloud_alpha) as u8;
                b = lerp(b as f32, 240.0, cloud_alpha) as u8;
            }

            let idx = ((py * SIZE + px) * 4) as usize;
            pixels[idx] = r;
            pixels[idx + 1] = g;
            pixels[idx + 2] = b;
            pixels[idx + 3] = 255;
        }
    }

    pixels
}

// ─── Yew component ───────────────────────────────────────────────────────────

#[derive(WebComponent)]
#[component(PlanetVisual)]
#[derive(Default, Properties, PartialEq, Deserialize, Clone)]
pub struct PlanetVisualProps {
    pub id: AttrValue,
}

#[function_component(PlanetVisual)]
pub fn planet_visual(props: &PlanetVisualProps) -> Html {
    let canvas_ref = use_node_ref();

    use_effect_with(
        (canvas_ref.clone(), props.id.clone()),
        |(canvas_ref, id)| {
            let Some(canvas) = canvas_ref.cast::<web_sys::HtmlCanvasElement>() else {
                return;
            };
            let Ok(Some(obj)) = canvas.get_context("2d") else {
                return;
            };
            let Ok(ctx) = obj.dyn_into::<web_sys::CanvasRenderingContext2d>() else {
                return;
            };
            let Some((seed, temperature)) = parse_id(id.as_str()) else {
                return;
            };

            let pixels = render_planet(seed, temperature);

            let Ok(image_data) = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
                wasm_bindgen::Clamped(pixels.as_slice()),
                300,
                300,
            ) else {
                return;
            };

            let _ = ctx.put_image_data(&image_data, 0.0, 0.0);
        },
    );

    html! {
        <canvas ref={canvas_ref} width="300" height="300" />
    }
}
