use std::cmp::Ordering;
use std::mem;
use std::sync::Arc;
use std::ops::Range;

use rand::{thread_rng, Rng};

use crate::hit::{Hit, HitRecord};
use crate::hit::aabb::{AABB};
use crate::ray::Ray;
use super::world::World;

// a hittable — just like lists of hittables. 
// It’s really a container, but it can respond to the query “does this ray hit you?”. 
// design question: 
// 1. do we have two classes, one for the tree, and one for the nodes in the tree; 
// 2. do we have just one class and have the root just be a node we point to. 
// hit function is pretty straightforward: check whether the box for the node is hit, 
// and if so, check the children and sort out any details.

pub struct BVH {
    left: Arc<Box<dyn Hit>>,
    right: Arc<Box<dyn Hit>>,
    bounding_box: AABB
}

impl BVH {
    // as long as the list of objects in a bvh_node gets divided into two sub-lists, the hit function will work. 
    // at each node split the list along one axis

    // 1. randomly choose an axis
    // 2. sort the primitives (using std::sort)
    // 3.put half in each subtree
    // when list coming in is two elements, put one in each subtree and end the recursion. 
    // traversal algorithm should be smooth and not have to check for null pointers, 
    // so if just have one element, duplicate it in each subtree. 
    // following code uses three methods — box_x_compare, box_y_compare_, and box_z_compare.
    pub fn new(objects: &mut World, time_range: Range<f64>) -> BVH {
        Self::from_range(&mut objects.to_owned(), time_range)
    }

    pub fn from_range(objects: &mut World, time_range: Range<f64>) -> BVH {
        let axis = thread_rng().gen_range(0..=2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => unreachable!(),
        };
        let (left, right) = match objects.len() {
            0 => unreachable!(),
            1 => {
                let left = Arc::clone(objects.first().unwrap());
                let right = Arc::clone(objects.last().unwrap());
                (left, right)
            }
            2 => {
                let mut left = Arc::clone(objects.first().unwrap());
                let mut right = Arc::clone(objects.last().unwrap());
                if let Ordering::Greater = comparator(&left, &right) {
                    mem::swap(&mut left, &mut right)
                }
                (left, right)
            }
            len => {
                objects.sort_unstable_by(comparator);
                let mid = len / 2;
                let left: Arc<Box<dyn Hit>> = Arc::new(
                    Box::new(
                        Self::from_range(&mut objects[..mid].to_vec(), time_range.clone())
                    )
                );
                let right: Arc<Box<dyn Hit>> = Arc::new(
                    Box::new(
                        Self::from_range(&mut objects[mid..].to_vec(), time_range.clone())
                    )
                );
                (left, right)
            }
        };

        let box_left = left.bounding_box(time_range.clone());
        let box_right = right.bounding_box(time_range);
        let bounding_box = AABB::surrounding_box(box_left, box_right);

        Self {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hit for BVH {
    fn hit(&self, r: &Ray, time_range: Range<f64>) -> Option<HitRecord> {
        if !self.bounding_box.hit(r, time_range.clone()) {
            return None;
        }

        let hit_left = self.left.hit(r, time_range.clone());
        let tr = hit_left
                .as_ref()
                .map_or(time_range.clone(), |rec| time_range.start..rec.t);
        let hit_right = self.right.hit(r, tr);
        hit_left.or(hit_right)
    }
    
    fn bounding_box(&self, time_range: Range<f64>) -> AABB {
        self.bounding_box.clone()
    }
}

fn box_x_compare(a: &Arc<Box<dyn Hit>>, b: &Arc<Box<dyn Hit>>) -> Ordering {
    let a = a.bounding_box(0.0..1.0).get_minimum().x();
    let b = b.bounding_box(0.0..1.0).get_minimum().x();
    a.partial_cmp(&b).expect("unexpected NaN in bounding x")
}

fn box_y_compare(a: &Arc<Box<dyn Hit>>, b: &Arc<Box<dyn Hit>>) -> Ordering {
    let a = a.bounding_box(0.0..1.0).get_minimum().y();
    let b = b.bounding_box(0.0..1.0).get_minimum().y();
    a.partial_cmp(&b).expect("unexpected NaN in bounding y")
}

fn box_z_compare(a: &Arc<Box<dyn Hit>>, b: &Arc<Box<dyn Hit>>) -> Ordering {
    let a = a.bounding_box(0.0..1.0).get_minimum().z();
    let b = b.bounding_box(0.0..1.0).get_minimum().z();
    a.partial_cmp(&b).expect("unexpected NaN in bounding z")
}
