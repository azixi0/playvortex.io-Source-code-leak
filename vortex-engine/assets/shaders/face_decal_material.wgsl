// -- leaked by @azixi0 on github
#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    mesh_functions,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var decal_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var decal_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var<uniform> skin_color: vec4<f32>;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    let skin = skin_color.rgb;
    var rgb = skin;
#ifdef VERTEX_UVS_A
    if skin_color.a > 0.5 {
        let decal_uv = vec2<f32>(1.0 - in.uv.x, in.uv.y);
        let decal = textureSample(decal_texture, decal_sampler, decal_uv);
        var front = 1.0;
#ifdef VERTEX_COLORS
        front = in.color.r;
#endif
        rgb = mix(skin, decal.rgb, decal.a * front);
    }
#endif

    pbr_input.material.base_color = vec4<f32>(rgb, pbr_input.material.base_color.a);

#ifdef PREPASS_PIPELINE
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    return out;
}
