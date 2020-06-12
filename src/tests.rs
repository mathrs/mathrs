pub use crate::*;

/// All of the test cases for the `structs` and `functions` of **mathrs**.
#[cfg(test)]
mod mathrs_tests {

    use super::*;

    #[test]
    fn vector() {
        assert_eq!(
            Vector { x: 1.0, y: 1.0 }[0] * Vector { x: 1.0, y: 1.0 }[1],
            1.0
        );

        assert_eq!(!Vector { x: 1.0, y: 1.0 }, Vector { x: -1.0, y: -1.0 });

        assert_eq!(
            Vector { x: 1.0, y: 1.0 } + Vector { x: 1.0, y: 1.0 },
            Vector { x: 2.0, y: 2.0 }
        );
        assert_eq!(
            Vector { x: 3.14, y: 3.14 } - Vector { x: 3.14, y: 3.14 },
            Vector { x: 0.0, y: 0.0 }
        );
        assert_eq!(
            Vector { x: 0.5, y: 0.5 } * Vector { x: 2.0, y: 2.0 },
            Vector { x: 1.0, y: 1.0 }
        );

        assert_eq!(
            Vector { x: 1.0, y: 1.0 } * 2.22,
            Vector { x: 2.22, y: 2.22 }
        );

        assert_eq!(
            Vector { x: 9.99, y: 9.99 } / Vector { x: 9.99, y: 9.99 },
            Vector { x: 1.0, y: 1.0 }
        );

        assert_eq!(
            Vector { x: 1.0, y: 0.5 }.dot(Vector { x: 0.5, y: 1.0 }),
            3.0
        );

        assert_eq!(
            Vector { x: 1.0, y: 1.0 }.cross(),
            Vector { x: -1.0, y: 1.0 }
        );
    }

    #[test]
    fn vector3d() {
        assert_eq!(
            Vector { x: 1.0, y: 1.0 }[0] * Vector { x: 1.0, y: 1.0 }[1],
            1.0
        );

        assert_eq!(!Vector { x: 1.0, y: 1.0 }, Vector { x: -1.0, y: -1.0 });

        assert_eq!(
            Vector { x: 1.0, y: 1.0 } + Vector { x: 1.0, y: 1.0 },
            Vector { x: 2.0, y: 2.0 }
        );
        assert_eq!(
            Vector { x: 3.14, y: 3.14 } - Vector { x: 3.14, y: 3.14 },
            Vector { x: 0.0, y: 0.0 }
        );
        assert_eq!(
            Vector { x: 0.5, y: 0.5 } * Vector { x: 2.0, y: 2.0 },
            Vector { x: 1.0, y: 1.0 }
        );

        assert_eq!(
            Vector { x: 1.0, y: 1.0 } * 2.22,
            Vector { x: 2.22, y: 2.22 }
        );

        assert_eq!(
            Vector { x: 9.99, y: 9.99 } / Vector { x: 9.99, y: 9.99 },
            Vector { x: 1.0, y: 1.0 }
        );

        assert_eq!(
            Vector { x: 1.0, y: 0.5 }.dot(Vector { x: 0.5, y: 1.0 }),
            3.0
        );

        assert_eq!(
            Vector { x: 1.0, y: 1.0 }.cross(),
            Vector { x: -1.0, y: 1.0 }
        );
    }

    #[test]
    fn array() {
        //
    }
}
