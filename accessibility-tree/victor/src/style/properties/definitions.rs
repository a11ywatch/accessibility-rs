use crate::style::values::*;
use cssparser::{Color, RGBA};

type FourSidesAuto = FourSides<SpecifiedLengthOrPercentageOrAuto>;
type FourSidesAutoPercentage = FourSides<SpecifiedLengthOrPercentage>;
type FourSidesLineStyle = FourSides<LineStyle>;
type FourSidesColor = FourSides<Color>;
type FourSidesLineWidth = FourSides<SpecifiedLineWidth>;

properties! {
    type Discriminant = u8;

    inherited struct font {
        @early font_size { "font-size", FontSize, initial = Length { px: 16. } }
    }

    inherited struct color {
        // FIXME: support currentColor here
        color { "color", RGBA, initial = BLACK }
    }

    reset struct box_ {
        position { "position", Position, initial = Position::Static }
        float { "float", Float, initial = Float::None }
        display { "display", Display, initial = Display::INITIAL }
        top { "top", LengthOrPercentageOrAuto, initial = LengthOrPercentageOrAuto::Auto }
        left { "left", LengthOrPercentageOrAuto, initial = LengthOrPercentageOrAuto::Auto }
        bottom { "bottom", LengthOrPercentageOrAuto, initial = LengthOrPercentageOrAuto::Auto }
        right { "right", LengthOrPercentageOrAuto, initial = LengthOrPercentageOrAuto::Auto }
        width { "width", LengthOrPercentageOrAuto, initial = LengthOrPercentageOrAuto::Auto }
        height { "height", LengthOrPercentageOrAuto, initial = LengthOrPercentageOrAuto::Auto }
    }

    reset struct margin {
        margin_top { "margin-top", LengthOrPercentageOrAuto, initial = Length::zero() }
        margin_left { "margin-left", LengthOrPercentageOrAuto, initial = Length::zero() }
        margin_bottom { "margin-bottom", LengthOrPercentageOrAuto, initial = Length::zero() }
        margin_right { "margin-right", LengthOrPercentageOrAuto, initial = Length::zero() }
    }

    reset struct padding {
        padding_top { "padding-top", LengthOrPercentage, initial = Length::zero() }
        padding_left { "padding-left", LengthOrPercentage, initial = Length::zero() }
        padding_bottom { "padding-bottom", LengthOrPercentage, initial = Length::zero() }
        padding_right { "padding-right", LengthOrPercentage, initial = Length::zero() }
    }

    reset struct border {
        border_top_color { "border-top-color", Color, initial = Color::CurrentColor }
        border_left_color { "border-left-color", Color, initial = Color::CurrentColor }
        border_bottom_color { "border-bottom-color", Color, initial = Color::CurrentColor }
        border_right_color { "border-color-right", Color, initial = Color::CurrentColor }

        border_top_style { "border-top-style", LineStyle, initial = LineStyle::None }
        border_left_style { "border-left-style", LineStyle, initial = LineStyle::None }
        border_bottom_style { "border-bottom-style", LineStyle, initial = LineStyle::None }
        border_right_style { "border-right-style", LineStyle, initial = LineStyle::None }

        border_top_width { "border-top-width", LineWidth, initial = LineWidth::MEDIUM }
        border_left_width { "border-left-width", LineWidth, initial = LineWidth::MEDIUM }
        border_bottom_width { "border-bottom-width", LineWidth, initial = LineWidth::MEDIUM }
        border_right_width { "border-right-width", LineWidth, initial = LineWidth::MEDIUM }
    }

    reset struct background {
        background_color { "background-color", Color, initial = Color::RGBA(RGBA::transparent()) }
    }

    @shorthands {
        "margin" => FourSidesAuto {
            top: margin_top,
            left: margin_left,
            bottom: margin_bottom,
            right: margin_right,
        }
        "padding" => FourSidesAutoPercentage {
            top: padding_top,
            left: padding_left,
            bottom: padding_bottom,
            right: padding_right,
        }
        "border-style" => FourSidesLineStyle {
            top: border_top_style,
            left: border_left_style,
            bottom: border_bottom_style,
            right: border_right_style,
        }
        "border-color" => FourSidesColor {
            top: border_top_color,
            left: border_left_color,
            bottom: border_bottom_color,
            right: border_right_color,
        }
        "border-width" => FourSidesLineWidth {
            top: border_top_width,
            left: border_left_width,
            bottom: border_bottom_width,
            right: border_right_width,
        }
        "border-top" => BorderSide {
            style: border_top_style,
            color: border_top_color,
            width: border_top_width,
        }
        "background" => Background {
            color: background_color,
        }
    }
}
