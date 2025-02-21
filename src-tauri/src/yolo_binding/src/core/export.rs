use std::error::Error;
use tch::Tensor;
fn filter_confidence(
    tensor: &Tensor,
    confidence: f64,
) -> Vec<(i64, i64, i64, i64, i64, f64)> {
    //Tensor [84, 8400]
    let pred = tensor.transpose(1, 0); //Tensor [8400, 84]
    let (npreds, pred_size) = pred.size2().unwrap();
    let full_xywh = pred.slice(1, 0, 4, 1); //Tensor [8400, 4]
    let mut filtered_results = Vec::new();
    for index in 0..npreds {
        // iterate all predictions
        let pred = pred.get(index); // Tensor [84]
        let max_conf_index = pred
            .narrow(0, 4, pred_size - 4)
            .argmax(0, true)
            .double_value(&[0]) as i64;
        let max_conf = pred.double_value(&[max_conf_index + 4]);
        if max_conf > confidence {
            let class_index = max_conf_index;
            filtered_results.push((
                full_xywh.double_value(&[index, 0]).round() as i64,
                full_xywh.double_value(&[index, 1]).round() as i64,
                full_xywh.double_value(&[index, 2]).round() as i64,
                full_xywh.double_value(&[index, 3]).round() as i64,
                class_index,
                max_conf,
            ));
        }
    }
    filtered_results
}
fn nms(
    mut boxes: Vec<(i64, i64, i64, i64, i64, f64)>,
    threshold: f64,
) -> Vec<(i64, i64, i64, i64, i64, f64)> {
    boxes.sort_unstable_by(|a, b| b.5.partial_cmp(&a.5).unwrap_or(std::cmp::Ordering::Equal));
    let mut suppressed = vec![false; boxes.len()];
    let to_xyxy = |x: i64, y: i64, w: i64, h: i64| {
        let (x, y, w, h) = (x as f64, y as f64, w as f64, h as f64);
        let x1 = x - w / 2.0;
        let y1 = y - h / 2.0;
        let x2 = x + w / 2.0;
        let y2 = y + h / 2.0;
        (x1, y1, x2, y2)
    };
    let compute_iou =
        |a: &(i64, i64, i64, i64, i64, f64), b: &(i64, i64, i64, i64, i64, f64)| -> f64 {
            let a_rect = to_xyxy(a.0, a.1, a.2, a.3);
            let b_rect = to_xyxy(b.0, b.1, b.2, b.3);
            // 计算交集区域
            let inter_x1 = a_rect.0.max(b_rect.0);
            let inter_y1 = a_rect.1.max(b_rect.1);
            let inter_x2 = a_rect.2.min(b_rect.2);
            let inter_y2 = a_rect.3.min(b_rect.3);

            let inter_area = (inter_x2 - inter_x1).max(0.0) * (inter_y2 - inter_y1).max(0.0);

            let a_area = (a_rect.2 - a_rect.0) * (a_rect.3 - a_rect.1);
            let b_area = (b_rect.2 - b_rect.0) * (b_rect.3 - b_rect.1);

            let union_area = a_area + b_area - inter_area;

            if union_area == 0.0 {
                0.0
            } else {
                inter_area / union_area
            }
        };

    for i in 0..boxes.len() {
        if suppressed[i] {
            continue;
        }
        for j in (i + 1)..boxes.len() {
            if suppressed[j] {
                continue;
            }
            if boxes[i].4 != boxes[j].4 {
                continue;
            }
            let iou = compute_iou(&boxes[i], &boxes[j]);
            if iou > threshold {
                suppressed[j] = true;
            }
        }
    }
    boxes //
        .into_iter()
        .enumerate()
        .filter(|(i, _)| !suppressed[*i])
        .map(|(_, b)| b)
        .collect()
}

pub fn post_process(
    tensor: &Tensor,
    confidence: f64,
    threshold: f64,
) -> Result<Vec<(i64, i64, i64, i64, i64, f64)>, Box<dyn Error>> {
    //！ Post-process the output tensor to get the bounding boxes
    //！ tensor: the output tensor from the model [n, X, 8400]
    //！ confidence: the confidence threshold
    //！ threshold: the NMS threshold
    let res = filter_confidence(&tensor, confidence);
    let res = nms(res, threshold);
    Ok(res)
}
