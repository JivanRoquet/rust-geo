use num_traits::{Float, FromPrimitive};
use ::Point;

pub trait VincentyDistance<T, Rhs = Self> {
    fn vincenty_distance(&self, rhs: &Rhs) -> T;
}

impl<T> VincentyDistance<T, Point<T>> for Point<T>
where
    T: Float + FromPrimitive,
{
    /// Number returned is in meters.
    #[allow(non_snake_case)]
    fn vincenty_distance(&self, rhs: &Point<T>) -> T {
        // Magic numbers

        let t_0 = T::zero();
        let t_1 = T::one();
        let t_2 = T::from(2).unwrap();
        let t_3 = T::from(3).unwrap();
        let t_4 = T::from(4).unwrap();
        let t_6 = T::from(6).unwrap();
        let t_47 = T::from(47).unwrap();
        let t_16 = T::from(16).unwrap();
        let t_74 = T::from(74).unwrap();
        let t_128 = T::from(128).unwrap();
        let t_175 = T::from(175).unwrap();
        let t_256 = T::from(256).unwrap();
        let t_320 = T::from(320).unwrap();
        let t_768 = T::from(768).unwrap();
        let t_1024 = T::from(1024).unwrap();
        let t_4096 = T::from(4096).unwrap();
        let t_16384 = T::from(16384).unwrap();

        // Beginning of vincenty algorithm

        let a = T::from(6378137.0).unwrap();
        let b = T::from(6356752.314245).unwrap();
        let f = T::from(298.257223563).unwrap().recip();
        let L = (rhs.lng() - self.lng()).to_radians();
        let U1 = ((t_1 - f) * self.lat().to_radians().tan()).atan();
        let U2 = ((t_1 - f) * rhs.lat().to_radians().tan()).atan();
        let sinU1 = U1.sin();
        let cosU1 = U1.cos();
        let sinU2 = U2.sin();
        let cosU2 = U2.cos();
        let mut cosSqAlpha;
        let mut sinSigma;
        let mut cos2SigmaM;
        let mut cosSigma;
        let mut sigma;
        let mut lambda = L;
        let mut lambdaP;
        let mut iterLimit = 100;

        loop {
            let sinLambda = lambda.sin();
            let cosLambda = lambda.cos();
            sinSigma = (
                (cosU2 * sinLambda)
                    * (cosU2 * sinLambda)
                    + (cosU1 * sinU2 - sinU1 * cosU2 * cosLambda)
                    * (cosU1 * sinU2 - sinU1 * cosU2 * cosLambda)
            ).sqrt();
            if sinSigma.is_zero() {
                return t_0;
            }
            cosSigma = sinU1 * sinU2 + cosU1 * cosU2 * cosLambda;
            sigma = sinSigma.atan2(cosSigma);
            let sinAlpha = cosU1 * cosU2 * sinLambda / sinSigma;
            cosSqAlpha = t_1 - sinAlpha * sinAlpha;
            cos2SigmaM = cosSigma - t_2 * sinU1 * sinU2 / cosSqAlpha;
            let C = f / t_16 * cosSqAlpha * (t_4 + f * (t_4 - t_3 * cosSqAlpha));
            lambdaP = lambda;
            lambda = L + (t_1 - C) * f * sinAlpha * (
                sigma + C * sinSigma * (
                    cos2SigmaM + C * cosSigma * (-t_1 + t_2 * cos2SigmaM * cos2SigmaM)
                )
            );

            if (lambda - lambdaP).abs() > T::from(1e-12).unwrap() {
                iterLimit -= 1;
                if iterLimit > 0 {
                    continue;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if iterLimit == 0 {
            return t_0;
        }

        let uSq = cosSqAlpha * (a * a - b * b) / (b * b);
        let A = t_1 + uSq / t_16384 * (
            t_4096 + uSq * (
                -t_768 + uSq * (
                    t_320 - t_175 * uSq
                )
            )
        );
        let B = uSq / t_1024 * (
            t_256 + uSq * (
                -t_128 + uSq * (
                    t_74 - t_47 * uSq
                )
            )
        );

        let deltaSigma =
            B * sinSigma
                * (cos2SigmaM + B / t_4
                    * (cosSigma
                        * (-t_1 + t_2 * cos2SigmaM * cos2SigmaM) - B / t_6 * cos2SigmaM
                            * (-t_3 + t_4 * sinSigma * sinSigma)
                                * (-t_3 + t_4 * cos2SigmaM * cos2SigmaM)));

        let s = b * A * (sigma - deltaSigma);

        return s;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vincenty_distance_1() {
        let a = Point::<f64>::new(17.072561, 48.154563);
        let b = Point::<f64>::new(17.072562, 48.154564);
        assert_relative_eq!(
            a.vincenty_distance(&b),
            0.13378944117648012
        );
    }


    #[test]
    fn test_vincenty_distance_2() {
        let a = Point::<f64>::new(17.072561, 48.154563);
        let b = Point::<f64>::new(17.064064, 48.158800);
        assert_relative_eq!(
            a.vincenty_distance(&b),
            788.4148295236967
        );
    }

    #[test]
    fn test_vincenty_distance_3() {
        let a = Point::<f64>::new(17.107558, 48.148636);
        let b = Point::<f64>::new(16.372477, 48.208810);
        assert_relative_eq!(
            a.vincenty_distance(&b),
            55073.68246366003
        );
    }

    use ::test::{black_box, Bencher};

    const NITER: usize = 10_000;

    #[bench]
    fn bench_vincenty(bencher: &mut Bencher) {
        let a = Point::<f64>::new(17.107558, 48.148636);
        let b = Point::<f64>::new(16.372477, 48.208810);
        bencher.iter(|| {
            for _ in 0..NITER {
                black_box(a.vincenty_distance(&b));
            }
        });
    }
}
