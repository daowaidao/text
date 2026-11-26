use kurbo::{BezPath, Cap, Join, Stroke, StrokeOpts};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn outline_svg_path(svg_d: &str, width: f64) -> Result<String, JsValue> {
    if !width.is_finite() || width <= 0.0 {
        return Err(JsValue::from_str("线宽必须大于 0"));
    }
    let bp = BezPath::from_svg(svg_d)
        .map_err(|e| JsValue::from_str(&format!("SVG 路径解析失败: {e}")))?;
    if bp.elements().is_empty() {
        return Ok(String::new());
    }
    let style = Stroke::new(width)
        .with_caps(Cap::Round)
        .with_join(Join::Round)
        .with_miter_limit(4.0);
    let stroked = kurbo::stroke(bp.iter(), &style, &StrokeOpts::default(), 1e-3);
    Ok(stroked.to_svg())
}
