// -- leaked by @azixi0 on github
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct FlareUniform {
    flare_pos: vec2<f32>,
    intensity: f32,
    aspect: f32,
    color: vec4<f32>,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> flare: FlareUniform;

fn ghost(uv: vec2<f32>, pos: vec2<f32>, radius: f32, softness: f32, aspect: f32) -> f32 {
    let d = length((uv - pos) * vec2<f32>(aspect, 1.0));
    return clamp(1.0 - smoothstep(radius * (1.0 - softness), radius, d), 0.0, 1.0);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    if (flare.intensity <= 0.0) {
        discard;
    }

    let uv = in.uv;
    let d = (uv - flare.flare_pos) * vec2<f32>(flare.aspect, 1.0);
    let dist = length(d);

    let core = exp(-dist * 30.0) * 0.85;
    let halo = exp(-dist * 8.0) * 0.18;

    let angle = atan2(d.y, d.x);
    let spikes = pow(abs(cos(angle * 3.0)), 10.0) * 0.6
               + pow(abs(cos(angle * 3.0 + 1.0472)), 14.0) * 0.3;
    let streak = spikes * exp(-dist * 11.0) * 0.45;

    let horiz = exp(-(d.y * d.y) * 9000.0) * exp(-abs(d.x) * 6.0) * 0.30;
    let vert = exp(-(d.x * d.x) * 22000.0) * exp(-abs(d.y) * 9.0) * 0.12;

    let center = vec2<f32>(0.5, 0.5);
    let to_center = center - flare.flare_pos;
    var ghosts = 0.0;
    ghosts += ghost(uv, flare.flare_pos + to_center * 0.4, 0.09, 0.85, flare.aspect) * 0.22;
    ghosts += ghost(uv, flare.flare_pos + to_center * 0.9, 0.06, 0.85, flare.aspect) * 0.16;
    ghosts += ghost(uv, flare.flare_pos + to_center * 1.4, 0.14, 0.9, flare.aspect) * 0.13;
    ghosts += ghost(uv, flare.flare_pos + to_center * 1.9, 0.045, 0.85, flare.aspect) * 0.15;
    ghosts += ghost(uv, flare.flare_pos + to_center * 1.15, 0.03, 0.85, flare.aspect) * 0.18;

    let strength = (core + halo + streak + horiz + vert + ghosts) * flare.intensity;

    return vec4<f32>(flare.color.rgb * strength, clamp(strength, 0.0, 1.0));
}
