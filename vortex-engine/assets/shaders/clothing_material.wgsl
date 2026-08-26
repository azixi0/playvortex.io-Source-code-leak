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
var clothing_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var clothing_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var<uniform> body_color: vec4<f32>;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    var rgb = body_color.rgb;
    let has_texture = body_color.a;
#ifdef VERTEX_UVS_A
    if has_texture > 0.5 {
        let sample = textureSample(clothing_texture, clothing_sampler, in.uv);
        rgb = mix(body_color.rgb, sample.rgb, sample.a);
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
