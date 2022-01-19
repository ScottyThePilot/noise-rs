pub use self::{
    cache::*, combiners::*, generators::*, modifiers::*, selectors::*, transformers::*,
};
use alloc::boxed::Box;

mod cache;
mod combiners;
mod generators;
mod modifiers;
mod selectors;
mod transformers;

/// Base trait for noise functions.
///
/// A noise function is a object that calculates and outputs a value given a
/// n-Dimensional input value, where n is (2,3,4).
///
/// Each type of noise function uses a specific method to calculate an output
/// value. Some of these methods include:
///
/// * Calculating a value using a coherent-noise function or some other
///     mathematical function.
/// * Mathematically changing the output value from another noise function
///     in various ways.
/// * Combining the output values from two noise functions in various ways.
pub trait NoiseFn<T, const DIM: usize> {
    fn get(&self, point: [T; DIM]) -> f64;

    fn abs(self) -> Abs<T, Self, DIM>
    where
        Self: Sized,
    {
        Abs::new(self)
    }

    fn add<Other>(self, other: Other) -> Add<T, Self, Other, DIM>
    where
        Self: Sized,
        Other: NoiseFn<T, DIM>,
    {
        Add::new(self, other)
    }

    fn blend<Other, Control>(self, other: Other, control: Control) -> Blend<T, Self, Other, Control, DIM>
    where
        Self: Sized,
        Other: NoiseFn<T, DIM>,
        Control: NoiseFn<T, DIM>
    {
        Blend::new(self, other, control)
    }

    fn cache(self) -> Cache<Self>
    where
        Self: Sized,
    {
        Cache::new(self)
    }

    fn clamp(self) -> Clamp<T, Self, DIM>
    where
        Self: Sized,
    {
        Clamp::new(self)
    }

    fn curve(self) -> Curve<T, Self, DIM>
    where
        Self: Sized,
    {
        Curve::new(self)
    }

    fn displace_xy<XDisplace, YDisplace>(
        self,
        x_displace: XDisplace,
        y_displace: YDisplace,
    ) -> Displace<Self, XDisplace, YDisplace, (), ()>
    where
        Self: NoiseFn<f64, 2> + Sized,
        XDisplace: NoiseFn<f64, 2>,
        YDisplace: NoiseFn<f64, 2>,
    {
        Displace::new(self, x_displace, y_displace, (), ())
    }

    fn displace_xyz<XDisplace, YDisplace, ZDisplace>(
        self,
        x_displace: XDisplace,
        y_displace: YDisplace,
        z_displace: ZDisplace,
    ) -> Displace<Self, XDisplace, YDisplace, ZDisplace, ()>
    where
        Self: NoiseFn<f64, 3> + Sized,
        XDisplace: NoiseFn<f64, 3>,
        YDisplace: NoiseFn<f64, 3>,
        ZDisplace: NoiseFn<f64, 3>,
    {
        Displace::new(self, x_displace, y_displace, z_displace, ())
    }

    fn displace_xyzu<XDisplace, YDisplace, ZDisplace, UDisplace>(
        self,
        x_displace: XDisplace,
        y_displace: YDisplace,
        z_displace: ZDisplace,
        u_displace: UDisplace,
    ) -> Displace<Self, XDisplace, YDisplace, ZDisplace, UDisplace>
    where
        Self: NoiseFn<f64, 4> + Sized,
        XDisplace: NoiseFn<f64, 4>,
        YDisplace: NoiseFn<f64, 4>,
        ZDisplace: NoiseFn<f64, 4>,
        UDisplace: NoiseFn<f64, 4>,
    {
        Displace::new(self, x_displace, y_displace, z_displace, u_displace)
    }

    fn exponent(self) -> Exponent<T, Self, DIM>
    where
        Self: Sized,
    {
        Exponent::new(self)
    }

    fn max<Other>(self, other: Other) -> Max<T, Self, Other, DIM>
    where
        Self: Sized,
        Other: NoiseFn<T, DIM>
    {
        Max::new(self, other)
    }

    fn min<Other>(self, other: Other) -> Min<T, Self, Other, DIM>
    where
        Self: Sized,
        Other: NoiseFn<T, DIM>
    {
        Min::new(self, other)
    }

    fn multiply<Other>(self, other: Other) -> Multiply<T, Self, Other, DIM>
    where
        Self: Sized,
        Other: NoiseFn<T, DIM>
    {
        Multiply::new(self, other)
    }

    fn negate(self) -> Negate<T, Self, DIM>
    where
        Self: Sized,
    {
        Negate::new(self)
    }

    fn power<Other>(self, other: Other) -> Power<T, Self, Other, DIM>
    where
        Self: Sized,
        Other: NoiseFn<T, DIM>
    {
        Power::new(self, other)
    }

    fn rotate_point(self) -> RotatePoint<Self>
    where
        Self: Sized,
    {
        RotatePoint::new(self)
    }

    fn scale_bias(self) -> ScaleBias<T, Self, DIM>
    where
        Self: Sized,
    {
        ScaleBias::new(self)
    }

    fn scale_point(self) -> ScalePoint<Self>
    where
        Self: Sized,
    {
        ScalePoint::new(self)
    }

    fn select<Other, Control>(self, other: Other, control: Control) -> Select<T, Self, Other, Control, DIM>
    where
        Self: Sized,
        Other: NoiseFn<T, DIM>,
        Control: NoiseFn<T, DIM>
    {
        Select::new(self, other, control)
    }

    fn terrace(self) -> Terrace<T, Self, DIM>
    where
        Self: Sized,
    {
        Terrace::new(self)
    }

    fn translate_point(self) -> TranslatePoint<Self>
    where
        Self: Sized,
    {
        TranslatePoint::new(self)
    }

    fn turbulence<F>(self) -> Turbulence<Self, F>
    where
        Self: Sized,
        F: Default + Seedable,
    {
        Turbulence::new(self)
    }
}

impl<'a, T, M, const DIM: usize> NoiseFn<T, DIM> for &'a M
where
    M: NoiseFn<T, DIM> + ?Sized,
{
    #[inline]
    fn get(&self, point: [T; DIM]) -> f64 {
        M::get(*self, point)
    }
}

impl<T, M, const DIM: usize> NoiseFn<T, DIM> for Box<M>
where
    M: NoiseFn<T, DIM> + ?Sized,
{
    #[inline]
    fn get(&self, point: [T; DIM]) -> f64 {
        M::get(self, point)
    }
}

/// Trait for functions that require a seed before generating their values
pub trait Seedable {
    /// Set the seed for the function implementing the `Seedable` trait
    fn set_seed(self, seed: u32) -> Self;

    /// Getter to retrieve the seed from the function
    fn seed(&self) -> u32;
}
