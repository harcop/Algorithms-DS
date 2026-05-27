/// LeetCode #1453 - Maximum Number Of Darts Inside Of A Circular Dartboard
fn num_points(darts: Vec<Vec<i32>>, r: i32) -> i32 {
    let pts: Vec<(f64, f64)> = darts.iter().map(|p| (p[0] as f64, p[1] as f64)).collect();
    let r = r as f64;
    let n = pts.len();
    if n == 0 { return 0; }
    let inside = |cx: f64, cy: f64| -> i32 {
        pts.iter().filter(|&&(x, y)| (x - cx).powi(2) + (y - cy).powi(2) <= r * r + 1e-6).count() as i32
    };
    let mut best = 1;
    for i in 0..n { best = best.max(inside(pts[i].0, pts[i].1)); }
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1, x2, y2) = (pts[i].0, pts[i].1, pts[j].0, pts[j].1);
            let d2 = (x1 - x2).powi(2) + (y1 - y2).powi(2);
            if d2 > 4.0 * r * r + 1e-6 { continue; }
            let mx = (x1 + x2) / 2.0;
            let my = (y1 + y2) / 2.0;
            if d2 < 1e-12 {
                best = best.max(inside(mx, my));
            } else {
                let d = d2.sqrt();
                let h = (r * r - d2 / 4.0).max(0.0).sqrt();
                let dx = (y2 - y1) / d;
                let dy = (x1 - x2) / d;
                for s in [-1.0f64, 1.0] {
                    best = best.max(inside(mx + s * h * dx, my + s * h * dy));
                }
            }
        }
        for j in i + 1..n {
            for k in j + 1..n {
                let (x1,y1,x2,y2,x3,y3)=(pts[i].0,pts[i].1,pts[j].0,pts[j].1,pts[k].0,pts[k].1);
                let a2=(x1-x2).powi(2)+(y1-y2).powi(2); let b2=(x1-x3).powi(2)+(y1-y3).powi(2); let c2=(x2-x3).powi(2)+(y2-y3).powi(2);
                let a=a2.sqrt(); let b=b2.sqrt(); let c=c2.sqrt();
                let s2=(a+b+c)/2.0; let ar2=s2*(s2-a)*(s2-b)*(s2-c);
                if ar2<=1e-14 { continue; }
                let ar=ar2.sqrt(); let rad=a*b*c/(4.0*ar);
                if rad>r+1e-6 { continue; }
                let d=2.0*(x1*(y2-y3)+x2*(y3-y1)+x3*(y1-y2));
                if d.abs()<1e-14 { continue; }
                let s1=x1*x1+y1*y1; let s2v=x2*x2+y2*y2; let s3=x3*x3+y3*y3;
                let ux=(s1*(y2-y3)+s2v*(y3-y1)+s3*(y1-y2))/d;
                let uy=(s1*(x3-x2)+s2v*(x1-x3)+s3*(x2-x1))/d;
                best=best.max(inside(ux,uy));
            }
        }
    }
    best
}
fn main() { println!("{}", num_points(vec![vec![-2,0],vec![4,0],vec![0,2],vec![0,-2]], 2)); }
#[cfg(test)]
mod tests {
    use super::num_points;
    #[test]
    fn example_one() { assert_eq!(num_points(vec![vec![-2,0],vec![4,0],vec![0,2],vec![0,-2]], 2), 3); }
    #[test]
    fn example_two() { assert_eq!(num_points(vec![vec![3,4],vec![5,-3],vec![-2,4]], 5), 3); }
}