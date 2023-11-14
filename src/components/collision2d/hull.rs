use std::fmt;
use std::cmp;

use glam::Vec2;



/// #### 한국어 </br>
/// 볼록 도형 충돌체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// It is convex shape collider. </br>
/// 
#[derive(Clone, PartialEq)]
pub struct ConvexHull {
    points: Vec<Vec2>,
}

impl ConvexHull {
    /// #### 한국어 </br>
    /// 주어진 점들로 부터 볼록 도형을 생성합니다. </br>
    /// <b>주어진 점의 갯수가 3개 미만일 경우 애플리케이션 실행을 중단시킵니다.</b></br>
    /// 
    /// #### English (Translation) </br>
    /// Creates a convex shape from given points. </br>
    /// <b>If the number of given points is less than 3, aborts application running.</b></br>
    /// 
    pub fn new<I>(points: I) -> Self 
    where 
        I: IntoIterator<Item = Vec2>,
        I::IntoIter: ExactSizeIterator 
    {
        use std::collections::VecDeque;

        let mut points: Vec<_> = points.into_iter().collect();
        assert!(points.len() >= 3, "The number of given points must be at least 3.");

        // (한국어) 가장 최하단 점을 찾습니다.
        // (English Translation) Find the lowest point.
        for idx in 1..points.len() {
            if comp(&points[idx], &points[0]).is_lt() {
                points.swap(idx, 0);
            }
        }

        // (한국어) 점들을 최하단 점을 기준으로 반시계 방향으로 정렬합니다.
        // (English Translation) Sort the points countercolockwise based on the lowest point.
        let pivot = points[0];
        points[1..].sort_by(|lhs, rhs| {
            ccw_comp(&pivot, lhs, rhs)
        });

        // (한국어) 볼록 도형을 만족하는 점들을 스택에 집어넣습니다.
        // (English Translation) Insert the points that satisfy the convex shape into the stack.
        let mut stack = VecDeque::with_capacity(points.len());
        for point in points.into_iter() {
            while stack.len() > 1 {
                let top = stack.pop_back().unwrap();
                let next_to_top = stack.back().unwrap();

                if ccw(next_to_top, &top, &point).is_gt() {
                    stack.push_back(top);
                    break;
                }
            }
            stack.push_back(point);
        }

        Self { points: stack.into_iter().collect() }
    }


    /// #### 한국어 </br>
    /// 두 볼록 도형이 서로 충돌할 경우 `true`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns `true` if two convex shapes collide with each other. </br>
    /// 
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        gjk_intersection(self, other, Vec2::X)
    }
}

impl fmt::Debug for ConvexHull {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConvexHull")
            .field("points", &self.points)
            .finish()
    }
}



#[inline]
fn comp(lhs: &Vec2, rhs: &Vec2) -> cmp::Ordering {
    use std::f32::EPSILON;

    if (lhs.y - rhs.y).abs() <= EPSILON {
        lhs.x.total_cmp(&rhs.x)
    } else {
        lhs.y.total_cmp(&rhs.y)
    }
}


#[inline]
fn ccw(pivot: &Vec2, a: &Vec2, b: &Vec2) -> cmp::Ordering {
    use glam::vec3a;

    let v0 = *a - *pivot;
    let v1 = *b - *a;
    let v2 = vec3a(v0.x, v0.y, 0.0).cross(vec3a(v1.x, v1.y, 0.0));
    v2.z.total_cmp(&0.0)
}


#[inline]
fn ccw_comp(pivot: &Vec2, lhs: &Vec2, rhs: &Vec2) -> cmp::Ordering {
    match ccw(pivot, lhs, rhs) {
        cmp::Ordering::Greater => cmp::Ordering::Less,
        cmp::Ordering::Equal => comp(lhs, rhs),
        cmp::Ordering::Less => cmp::Ordering::Greater,
    }
}


/// #### 한국어 </br>
/// 모든 점들과 방향벡터를 내적하여 값이 가장 큰 점을 찾습니다. </br>
/// 
/// #### English (Translation) </br>
/// Find the point with the largest value by dot producting all points and the direction vector. </br>
/// 
fn get_support_point(points: &[Vec2], dir: Vec2) -> Vec2 {
    assert!(points.is_empty(), "The given points are empty!");

    let mut point = points[0];
    let mut maximum = points[0].dot(dir);
    for idx in 1..points.len() {
        let value = points[idx].dot(dir);
        if maximum < value {
            point = points[idx];
            maximum = value;
        }
    }

    return point;
}


/// #### 한국어 </br>
/// 두 도형의 서로 반대 방향 벡터로 얻은 Support Point의 차를 반환합니다. </br>
/// 
/// #### English (Translation) </br>
/// Returns the difference in Support Point obtained by vectors in opposite directions of two shapes. </br>
/// 
#[inline]
fn minkowsk_support_point(a: &ConvexHull, b: &ConvexHull, dir: Vec2) -> Vec2 {
    let p0 = get_support_point(&a.points, dir);
    let p1 = get_support_point(&b.points, -dir);
    return p0 - p1;
}


/// #### 한국어 </br>
/// [Gilbert–Johnson–Keerthi distance algorithm](https://en.wikipedia.org/wiki/Gilbert%E2%80%93Johnson%E2%80%93Keerthi_distance_algorithm) 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is [Gilbert–Johnson–Keerthi distance algorithm](https://en.wikipedia.org/wiki/Gilbert%E2%80%93Johnson%E2%80%93Keerthi_distance_algorithm) function. </br>
/// 
fn gjk_intersection(a: &ConvexHull, b: &ConvexHull, initial_axis: Vec2) -> bool {
    use glam::{vec2, vec3a};

    let v_oa = minkowsk_support_point(a, b, initial_axis);
    let v_ao = -v_oa;
    let v_ob = minkowsk_support_point(a, b, -v_ao);
    if v_ob.dot(v_ao) < 0.0 {
        return false;
    }

    let v_ab = v_ao + v_ob;
    let dir = (vec3a(v_ab.x, v_ab.y, 0.0).cross(vec3a(v_ao.x, v_ao.y, 0.0))).cross(vec3a(v_ab.x, v_ab.y, 0.0));
    let dir = vec2(dir.x, dir.y);
    let v_oc = minkowsk_support_point(a, b, dir);
    if v_oc.dot(dir) < 0.0 {
        return false;
    }

    let v_ca = v_oa - v_oc;
    let v_ca = vec3a(v_ca.x, v_ca.y, 0.0);
    let v_cb = v_ob - v_oc;
    let v_cb = vec3a(v_cb.x, v_cb.y, 0.0);
    let v_co = -vec3a(v_oc.x, v_oc.y, 0.0);
    if ((v_ca.cross(v_cb)).cross(v_cb)).dot(v_co) < 0.0
    && ((v_cb.cross(v_ca)).cross(v_ca)).dot(v_co) < 0.0 {
        return true;
    } else {
        return false;
    }
}
