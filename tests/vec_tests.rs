use haje::vec2::Vec2;
use haje::vec3::Vec3;
use haje::vec4::Vec4;

#[test]
fn test_vec2_operations() {
    let v1 = Vec2 { x: 1.0, y: 2.0 };
    let v2 = Vec2 { x: 3.0, y: 4.0 };
    
    let sum = v1 + v2;
    assert_eq!(sum.x, 4.0);
    assert_eq!(sum.y, 6.0);
    
    let dot = v1.dot(v2);
    assert_eq!(dot, 11.0);
    
    let mag = v1.magnitude();
    assert_eq!(mag, (5.0f64).sqrt());
}

#[test]
fn test_vec3_operations() {
    let v1 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    let v2 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    
    let cross = v1.cross(v2);
    assert_eq!(cross.x, 0.0);
    assert_eq!(cross.y, 0.0);
    assert_eq!(cross.z, 1.0);
}

#[test]
fn test_vec4_operations() {
    let v1 = Vec4 { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    let mag = v1.magnitude();
    assert_eq!(mag, 2.0);
}
