use core::assert_eq;
use gx21m15::{get_temperature_hysteresis_bytes_from_value, get_temperature_hysteresis_value_from_bytes, get_temperature_value_from_bytes};

#[test]
fn test_get_temperature() {
    let bytes = (0x3f8u16 << 5).to_be_bytes();
    let value = get_temperature_value_from_bytes(bytes);
    assert_eq!(value, 127f32);

    let bytes = (0x3f7u16 << 5).to_be_bytes();
    let value = get_temperature_value_from_bytes(bytes);
    assert_eq!(value, 126.875f32);

    let bytes = (0x3f1u16 << 5).to_be_bytes();
    let value = get_temperature_value_from_bytes(bytes);
    assert_eq!(value, 126.125);

    let bytes = (0x3e8u16 << 5).to_be_bytes();
    let value = get_temperature_value_from_bytes(bytes);
    assert_eq!(value, 125f32);

    let bytes = (0x0c8u16 << 5).to_be_bytes();
    let value = get_temperature_value_from_bytes(bytes);
    assert_eq!(value, 25f32);

    let bytes = (0x001u16 << 5).to_be_bytes();
    let value = get_temperature_value_from_bytes(bytes);
    assert_eq!(value, 0.125f32);

    let bytes = (0x000u16 << 5).to_be_bytes();
    let value = get_temperature_value_from_bytes(bytes);
    assert_eq!(value, 0f32);

    let bytes = (0x7ffu16 << 5).to_be_bytes();
    let value = get_temperature_value_from_bytes(bytes);
    assert_eq!(value, -0.125f32);

    let bytes = (0x738u16 << 5).to_be_bytes();
    let value = get_temperature_value_from_bytes(bytes);
    assert_eq!(value, -25f32);

    let bytes = (0x649u16 << 5).to_be_bytes();
    let value = get_temperature_value_from_bytes(bytes);
    assert_eq!(value, -54.875f32);

    let bytes = (0x648u16 << 5).to_be_bytes();
    let value = get_temperature_value_from_bytes(bytes);
    assert_eq!(value, -55f32);
}

#[test]
fn test_get_temperature_hysteresis() {
    let bytes = (0x0FAu16 << 7).to_be_bytes();
    let value = get_temperature_hysteresis_value_from_bytes(bytes);
    assert_eq!(value, 125f32);

    let bytes = (0x32u16 << 7).to_be_bytes();
    let value = get_temperature_hysteresis_value_from_bytes(bytes);
    assert_eq!(value, 25f32);

    let bytes = (0x001u16 << 7).to_be_bytes();
    let value = get_temperature_hysteresis_value_from_bytes(bytes);
    assert_eq!(value, 0.5f32);

    let bytes = (0x000u16 << 7).to_be_bytes();
    let value = get_temperature_hysteresis_value_from_bytes(bytes);
    assert_eq!(value, 0f32);

    let bytes = (0x1FFu16 << 7).to_be_bytes();
    let value = get_temperature_hysteresis_value_from_bytes(bytes);
    assert_eq!(value, -0.5f32);

    let bytes = (0x1CEu16 << 7).to_be_bytes();
    let value = get_temperature_hysteresis_value_from_bytes(bytes);
    assert_eq!(value, -25f32);

    let bytes = (0x192u16 << 7).to_be_bytes();
    let value = get_temperature_hysteresis_value_from_bytes(bytes);
    assert_eq!(value, -55f32);
}

#[test]
fn test_get_temperature_hysteresis_bytes() {
    let bytes = get_temperature_hysteresis_bytes_from_value(125f32);
    assert_eq!(bytes, (0x0FAu16 << 7).to_be_bytes());

    let bytes = get_temperature_hysteresis_bytes_from_value(25f32);
    assert_eq!(bytes, (0x32u16 << 7).to_be_bytes());

    let bytes = get_temperature_hysteresis_bytes_from_value(0.5f32);
    assert_eq!(bytes, (0x001u16 << 7).to_be_bytes());

    let bytes = get_temperature_hysteresis_bytes_from_value(0f32);
    assert_eq!(bytes, (0x000u16 << 7).to_be_bytes());

    let bytes = get_temperature_hysteresis_bytes_from_value(-0.5f32);
    assert_eq!(bytes, (0x1FFu16 << 7).to_be_bytes());

    let bytes = get_temperature_hysteresis_bytes_from_value(-25f32);
    assert_eq!(bytes, (0x1CEu16 << 7).to_be_bytes());

    let bytes = get_temperature_hysteresis_bytes_from_value(-55f32);
    assert_eq!(bytes, (0x192u16 << 7).to_be_bytes());
}
