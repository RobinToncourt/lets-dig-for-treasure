use yew::prelude::*;
use yew_autoprops::autoprops;

const SPRITES_PATH: &str = "assets/img/sprites.jpg";

const SHEET_W: f64 = 7474.0;
const SHEET_H: f64 = 459.0;

#[autoprops]
#[component]
pub fn CardSprite(
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    #[prop_or(220.0)] target_w: f64,
    #[prop_or(308.0)] target_h: f64,
) -> Html {
    let scale_x = target_w / w as f64;
    let scale_y = target_h / h as f64;

    let bg_w = SHEET_W * scale_x;
    let bg_h = SHEET_H * scale_y;
    let bg_x = -(x as f64) * scale_x;
    let bg_y = -(y as f64) * scale_y;

    let style = format!(
        "width: {target_w}px; height: {target_h}px; \
         background-image: url('{SPRITES_PATH}'); \
         background-repeat: no-repeat; \
         background-size: {bg_w:.2}px {bg_h:.2}px; \
         background-position: {bg_x:.2}px {bg_y:.2}px;",
    );

    html! {
        <div class="card-sprite" style={style} />
    }
}
