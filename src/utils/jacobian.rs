use crate::{Float, Point2};

/// 2×2 Jacobian matrix for a bilinear quad with nodes [A,B,C,D] at
/// reference coords (-1,-1), (1,-1), (1,1), (-1,1).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Jacobian2x2<T: Float> {
    /// Entry (1,1) of the Jacobian matrix
    pub m11: T,
    /// Entry (1,2) of the Jacobian matrix
    pub m12: T,
    /// Entry (2,1) of the Jacobian matrix
    pub m21: T,
    /// Entry (2,2) of the Jacobian matrix
    pub m22: T,
}

impl<T: Float> Jacobian2x2<T> {
    /// Compute J(xi,eta) = ∑ Ni,xi * Xi for i=1..4
    pub fn for_quad(
        xi: T,
        eta: T,
        a: Point2<T>,
        b: Point2<T>,
        c: Point2<T>,
        d: Point2<T>,
    ) -> Self {
        // shape‐fn derivatives
        let one = T::one();
        let d_n1_dxi = -one + eta; let d_n1_deta = -one + xi; // node A
        let d_n2_dxi =  one - eta; let d_n2_deta = -one - xi; // node B
        let d_n3_dxi =  one + eta; let d_n3_deta =  one + xi; // node C
        let d_n4_dxi = -one - eta; let d_n4_deta =  one - xi; // node D
        // physical grads
        let m11 = d_n1_dxi * a.x + d_n2_dxi * b.x + d_n3_dxi * c.x + d_n4_dxi * d.x;
        let m12 = d_n1_deta * a.x + d_n2_deta * b.x + d_n3_deta * c.x + d_n4_deta * d.x;
        let m21 = d_n1_dxi * a.y + d_n2_dxi * b.y + d_n3_dxi * c.y + d_n4_dxi * d.y;
        let m22 = d_n1_deta * a.y + d_n2_deta * b.y + d_n3_deta * c.y + d_n4_deta * d.y;
        Jacobian2x2 { m11, m12, m21, m22 }
    }

    /// Determinant det(J)
    pub fn det(self) -> T {
        self.m11 * self.m22 - self.m12 * self.m21
    }

    /// Inverse J⁻¹
    pub fn inverse(self) -> Option<Jacobian2x2<T>> {
        let d = self.det();
        if d.abs() < T::epsilon() { return None; }
        let inv = T::one() / d;
        Some(Jacobian2x2 {
            m11:  self.m22 * inv,
            m12: -self.m12 * inv,
            m21: -self.m21 * inv,
            m22:  self.m11 * inv,
        })
    }
}

/// Given physical point `p` and quad corners `a,b,c,d`, find (xi,eta) via Newton:
pub fn invert_quad_mapping<T: Float>(
    mut xi: T,
    mut eta: T,
    p: Point2<T>,
    a: Point2<T>,
    b: Point2<T>,
    c: Point2<T>,
    d: Point2<T>,
    tol: T,
    max_iters: usize,
) -> Option<(T, T)> {
    for _ in 0..max_iters {
        // shape‐fn values
        let n1 = (T::one()-xi)*(T::one()-eta)/T::from(4.0).unwrap();
        let n2 = (T::one()+xi)*(T::one()-eta)/T::from(4.0).unwrap();
        let n3 = (T::one()+xi)*(T::one()+eta)/T::from(4.0).unwrap();
        let n4 = (T::one()-xi)*(T::one()+eta)/T::from(4.0).unwrap();
        // physical point at (xi,eta)
        let x = a.x*n1 + b.x*n2 + c.x*n3 + d.x*n4;
        let y = a.y*n1 + b.y*n2 + c.y*n3 + d.y*n4;
        // residual
        let rx = x - p.x;
        let ry = y - p.y;
        if rx.abs() < tol && ry.abs() < tol {
            return Some((xi, eta));
        }
        // Jacobian & inverse
        let j = Jacobian2x2::for_quad(xi, eta, a, b, c, d);
        let inv_j = match j.inverse() {
            Some(inv) => inv,
            None => return None,
        };
        // Newton update: [Δxi; Δeta] = invJ * [rx; ry]
        let dxi = inv_j.m11 * rx + inv_j.m12 * ry;
        let deta = inv_j.m21 * rx + inv_j.m22 * ry;
        xi = xi - dxi;
        eta = eta - deta;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point2;
    use num_traits::Zero;

    #[test]
    fn jacobian_quad_det_nonzero() {
        let a = Point2::new(0.0,0.0);
        let b = Point2::new(2.0,0.0);
        let c = Point2::new(2.0,1.0);
        let d = Point2::new(0.0,1.0);
        let j = Jacobian2x2::for_quad(0.0_f64, 0.0, a,b,c,d );
        assert!(!j.det().is_zero());
    }

    #[test]
    fn invert_quad_identity() {
        // unit square maps xi,eta -> x,y one-to-one
        let a=Point2::new(0.0,0.0);
        let b=Point2::new(1.0,0.0);
        let c=Point2::new(1.0,1.0);
        let d=Point2::new(0.0,1.0);
        let p = Point2::new(0.3, 0.7);
        let got = invert_quad_mapping(0.0_f64, 0.0, p, a,b,c,d, 1e-6, 50 );
        assert!(got.is_some());
        let (xi,eta)=got.unwrap();
        assert!((xi - (-0.4)).abs() < 1e-3);
        assert!((eta - 0.4).abs() < 1e-3);
    }
}
